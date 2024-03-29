use regex::Regex;
use once_cell::sync::Lazy;
use anyhow::{Context, Result, anyhow, bail};

use super::token::Token;


pub struct Parser {
    input: String
}

static RE_PERIOD: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\.([\(\)'\s]|$)").unwrap());

impl Parser {
    pub fn new(input: String) -> Self {
        Parser{input: input}
    }

    pub fn build_tokens(&self) -> Vec<Result<Token>> {
        let mut cursor = 0;
        let mut res = Vec::new();

        while let Some(()) = self.skip_whitespace(&mut cursor) {
            if self.input.as_bytes()[cursor] == b')' {
                cursor += 1;
                res.push(Err(anyhow!("read error: extra close parenthesis")))
            } else {
                res.push(self.token(&mut cursor));
            }
        }
        
        res
    }

    fn token(&self, cursor: &mut usize) -> Result<Token> {
        if !(*cursor < self.input.len()) {
            bail!("lexical analyzer error: index out of bounds");
        }

        match self.input.as_bytes()[*cursor] {
            b'('  => {
                *cursor += 1;
                self.skip_whitespace(cursor).context("read error: unterminated parenthesis")?;
                self.token_pair(cursor)
            },
            b')'  => Err(anyhow!("read error: extra close parenthesis")),
            b'\'' => {
                *cursor += 1;
                self.skip_whitespace(cursor).context("read error: unterminated quote")?;
                self.token(cursor)
                    .map(|t| Token::Symbol(Box::new(t)))
            },
            b'"'  => {
                *cursor += 1;
                self.token_str(cursor)
            },
            _ if RE_PERIOD.is_match(self.input.split_at(*cursor).1) => {
                *cursor += 1;
                Err(anyhow!("read error: dot in wrong context"))
            }
            _     => self.token_id_or_literal(cursor)
        }
    }
    
    fn token_pair(&self, cursor: &mut usize) -> Result<Token> {
        if self.input.as_bytes()[*cursor] == b')' {
            // ()
            *cursor += 1;
            return Ok(Token::Empty);
        }

        let car = self.token(cursor)?;
        
        self.skip_whitespace(cursor).context("read error: unterminated parenthesis")?;
        if self.input.split_at(*cursor).1.starts_with(".)") {
            // (a .)
            *cursor += 2;
            Err(anyhow!("read error: dot in wrong context"))

        } else if RE_PERIOD.is_match(self.input.split_at(*cursor).1) {
            // (a . b)
            *cursor += 1;
            self.skip_whitespace(cursor).context("read error: unterminated parenthesis")?;
            let cdr = self.token(cursor)?;
            self.skip_whitespace(cursor).context("read error: unterminated parenthesis")?;
            if self.input.as_bytes()[*cursor] == b')' {
                *cursor += 1;
                Ok(Token::Pair{car: Box::new(car), cdr: Box::new(cdr)})
            } else {
                // (a . b c)
                *cursor += 1;
                Err(anyhow!("read error: bad dot syntax"))
            }

        } else {
            // (a b)
            let cdr = self.token_pair(cursor)?;
            Ok(Token::Pair{car: Box::new(car), cdr: Box::new(cdr)})
        }
    }

    fn token_str(&self, cursor: &mut usize) -> Result<Token> {
        static RE_STRING: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^(\\"|[^"])*"#).unwrap());
        let mat = RE_STRING.find(self.input.split_at(*cursor).1).unwrap();
        let start = *cursor;
        let end = *cursor + mat.end();
        
        if end == self.input.len() {
            Err(anyhow!("read error: unterminated string"))
        } else {
            *cursor = end + 1;
            Ok(Token::String(self.input.get(start..end).unwrap().to_string()))
        }
    }

    fn token_id_or_literal(&self, cursor: &mut usize) -> Result<Token> {
        static RE_FLOAT: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^[\+-]?(\d*\.\d*([Ee][\+-]?\d+)?|\d+e[\+-]?\d+)([\(\)'"\s]|$)"#).unwrap());
        static RE_FLOAT_NOINT: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(?P<sign>[\+-])\.").unwrap()); // .123 (must be changed to 0.123)
        static RE_INT: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^[\+-]?\d+([\(\)'"\s]|$)"#).unwrap());
        static RE_BOOL: Lazy<Regex> = Lazy::new(|| Regex::new(r##"^#[ft]([\(\)'"#\s]|$)"##).unwrap());
        static RE_ID: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^[[:alnum:]!\$%&\*\+-\./<=>\?@\^_]+([\(\)'"\s]|$)"#).unwrap());
        static RE_DELIMITER: Lazy<Regex> = Lazy::new(|| Regex::new(r#"[\(\)\s'"]"#).unwrap());

        let input_from_cursor = self.input.split_at(*cursor).1;
        if RE_PERIOD.is_match(input_from_cursor) {
            *cursor += 1;
            Err(anyhow!("read error: dot in wrong context"))
        
        } else if let Some(mat) = RE_FLOAT.find(input_from_cursor) {
            let (start, mut end) = (*cursor, *cursor + mat.end() - 1);
            if !RE_DELIMITER.is_match_at(&self.input, end) {
                // match `$` (end)
                end += 1;
            }
            *cursor = end;
            Ok(Token::Float(RE_FLOAT_NOINT
                .replace(self.input.get(start..end).unwrap(), "${sign}0.")
                .parse()
                .unwrap()))

        } else if let Some(mat) = RE_INT.find(input_from_cursor) {
            let (start, mut end) = (*cursor, *cursor + mat.end() - 1);
            if !RE_DELIMITER.is_match_at(&self.input, end) {
                end += 1;
            }
            *cursor = end;
            Ok(Token::Int(self.input
                .get(start..end)
                .unwrap()
                .parse()
                .context("overflow")?))

        } else if RE_BOOL.is_match(input_from_cursor) {
            *cursor += 2;
            if input_from_cursor.as_bytes()[1] == b'f' {
                Ok(Token::Boolean(false))
            } else {
                Ok(Token::Boolean(true))
            }

        } else if let Some(mat) = RE_ID.find(input_from_cursor) {
            let (start, mut end) = (*cursor, *cursor + mat.end() - 1);
            if !RE_DELIMITER.is_match_at(&self.input, end) {
                end += 1;
            }
            *cursor = end;
            Ok(Token::Id(self.input.get(start..end).unwrap().to_string()))

        } else {
            *cursor = RE_DELIMITER.find(input_from_cursor).map_or(self.input.len(), |m| *cursor + m.end());
            Err(anyhow!("read error: invalid symbol name"))
        }
    }

    fn skip_whitespace(&self, cursor: &mut usize) -> Option<()> {
        loop {
            if *cursor >= self.input.len() {
                return None;
            } else if self.input.as_bytes()[*cursor].is_ascii_whitespace() {
                *cursor += 1;
            } else {
                return Some(());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_float() {
        let lex = Parser::new(r"23.4 -.0098e3 .".to_string());
        let tokens = lex.build_tokens();
        assert_eq!(format!("{:?}", tokens.get(0).unwrap().as_ref().unwrap()), "23.4[float]");
        assert_eq!(format!("{:?}", tokens.get(1).unwrap().as_ref().unwrap()), "-9.8[float]");
        //assert_eq!(format!("{:?}", tokens.get(2).unwrap().as_ref().err().unwrap()), ("read error: dot in wrong context"));
    }

    #[test]
    fn lex_int() {
        let lex = Parser::new(r"234 0098".to_string());
        let tokens = lex.build_tokens();
        assert_eq!(format!("{:?}", tokens.get(0).unwrap().as_ref().unwrap()), "234[int]");
        assert_eq!(format!("{:?}", tokens.get(1).unwrap().as_ref().unwrap()), "98[int]");
    }

    #[test]
    fn lex_string() {
        let lex = Parser::new(r#""hoge" "fああueo84()79" "hogefuga"#.to_string());
        let tokens = lex.build_tokens();
        assert_eq!(format!("{:?}", tokens.get(0).unwrap().as_ref().unwrap()), r#""hoge"[string]"#);
        assert_eq!(format!("{:?}", tokens.get(1).unwrap().as_ref().unwrap()), r#""fああueo84()79"[string]"#);
        //assert_eq!(format!("{:?}", tokens.get(2).unwrap().as_ref().err().unwrap()), "read error: unterminated string");
    }

    #[test]
    fn lex_bool() {
        let lex = Parser::new(r"#t #f #t42 #f#t".to_string());
        let tokens = lex.build_tokens();
        assert_eq!(format!("{:?}", tokens.get(0).unwrap().as_ref().unwrap()), "#t[boolean]");
        assert_eq!(format!("{:?}", tokens.get(1).unwrap().as_ref().unwrap()), "#f[boolean]");
        //assert_eq!(format!("{:?}", tokens.get(2).unwrap().as_ref().err().unwrap()), "read error: invalid symbol name");
        assert_eq!(format!("{:?}", tokens.get(3).unwrap().as_ref().unwrap()), "#f[boolean]");
        assert_eq!(format!("{:?}", tokens.get(4).unwrap().as_ref().unwrap()), "#t[boolean]");
    }

    #[test]
    fn lex_id() {
        let lex = Parser::new(r"hoge fuga あ #123 piyo".to_string());
        let tokens = lex.build_tokens();
        assert_eq!(format!("{:?}", tokens.get(0).unwrap().as_ref().unwrap()), "hoge[id]");
        assert_eq!(format!("{:?}", tokens.get(1).unwrap().as_ref().unwrap()), "fuga[id]");
        //assert_eq!(format!("{:?}", tokens.get(2).unwrap().as_ref().err().unwrap()), "read error: invalid symbol name");
        //assert_eq!(format!("{:?}", tokens.get(3).unwrap().as_ref().err().unwrap()), "read error: invalid symbol name");
        assert_eq!(format!("{:?}", tokens.get(4).unwrap().as_ref().unwrap()), "piyo[id]");
    }

    #[test]
    fn lex_symble() {
        let lex = Parser::new(r#"'123 '1.23e-3 '"foo" '#t '''bar"#.to_string());
        let tokens = lex.build_tokens();
        assert_eq!(format!("{:?}", tokens.get(0).unwrap().as_ref().unwrap()), "'123[int]");
        assert_eq!(format!("{:?}", tokens.get(1).unwrap().as_ref().unwrap()), "'0.00123[float]");
        assert_eq!(format!("{:?}", tokens.get(2).unwrap().as_ref().unwrap()), "'\"foo\"[string]");
        assert_eq!(format!("{:?}", tokens.get(3).unwrap().as_ref().unwrap()), "'#t[boolean]");
        assert_eq!(format!("{:?}", tokens.get(4).unwrap().as_ref().unwrap()), "'''bar[id]");
    }

    #[test]
    fn lex_pair() {
        let lex1 = Parser::new(r#"() (() ()) (() . (() . ())))"#.to_string());
        let tokens1 = lex1.build_tokens();
        assert_eq!(format!("{:?}", tokens1.get(0).unwrap().as_ref().unwrap()), "()");
        assert_eq!(format!("{:?}", tokens1.get(1).unwrap().as_ref().unwrap()), "(() ())");
        assert_eq!(format!("{:?}", tokens1.get(2).unwrap().as_ref().unwrap()), "(() ())");
        //assert_eq!(format!("{:?}", tokens1.get(3).unwrap().as_ref().err().unwrap()), "read error: extra close parenthesis");

        let lex2 = Parser::new(r#"(define ls '(1 2 3 4))"#.to_string());
        let tokens2 = lex2.build_tokens();
        assert_eq!(format!("{:?}", tokens2.get(0).unwrap().as_ref().unwrap()), "(define[id] ls[id] '(1[int] 2[int] 3[int] 4[int]))");

        let lex3 = Parser::new(
        r#"
        (define (fact n)
          (if (eq? n 0)
            1
            (* n (fact (- n 1)))
          )
        )
        "#.to_string());
        let tokens3 = lex3.build_tokens();
        assert_eq!(format!("{:?}", tokens3.get(0).unwrap().as_ref().unwrap()),
        "(define[id] (fact[id] n[id]) (if[id] (eq?[id] n[id] 0[int]) 1[int] (*[id] n[id] (fact[id] (-[id] n[id] 1[int])))))");
    }
}
