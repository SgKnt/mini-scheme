use std::fmt;
use regex::Regex;
use once_cell::sync::Lazy;
use anyhow::{Context, Result, anyhow, bail};

pub enum Token {
    Int(i32),
    Float(f64),
    Bool(bool),
    Id(String),
    Str(String),
    Pair{car: Box<Token>, cdr: Box<Token>},
    Empty,
    Symbol(Box<Token>),
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Int(i) => write!(f, "{}[int]", i),
            Token::Float(fl) => write!(f, "{}[float]", fl),
            Token::Bool(b) if *b => write!(f, "#t[bool]"),
            Token::Bool(_) => write!(f, "#f[bool]"),
            Token::Id(id) => write!(f, "{}[id]", id),
            Token::Str(s) => write!(f, "\"{}\"[string]", s),
            Token::Pair{car, cdr} => match **cdr {
                Token::Pair{..} | Token::Empty => {
                    let cdr = format!("{:?}", cdr);
                    write!(f, "({:?} {}", car, cdr.split_at(1).1)
                },
                _ => {
                    write!(f, "({:?} {:?})", car, cdr)
                }
            }
            Token::Empty => write!(f, "()"),
            Token::Symbol(s) => write!(f, "'{:?}", s)
        }
    }
}

pub struct Lexer {
    input: String
}

static RE_PERIOD: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\.[\(\)'\s]").unwrap());

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer{input: input}
    }

    pub fn build_tokens(&self) -> Vec<Result<Token>> {
        let mut cursor = 0;
        let mut res = Vec::new();

        while let Some(()) = self.skip_whitespace(&mut cursor) {
            res.push(self.token(&mut cursor));
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
            _ if RE_PERIOD.is_match(self.input.split_at(*cursor).1) => 
                Err(anyhow!("read error: dot in wrong context")),
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
        
        if !(end < self.input.len()) {
            Err(anyhow!("read error: unterminated string"))
        } else {
            *cursor = end + 1;
            Ok(Token::Str(self.input.get(start..end).unwrap().to_string()))
        }
    }

    fn token_id_or_literal(&self, cursor: &mut usize) -> Result<Token> {
        static RE_FLOAT: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[\+-]?(\d*\.\d*([Ee][\+-]?\d+)?|\d+e[\+-]?\d+)[\(\)'\s]").unwrap());
        static RE_FLOAT_NOINT: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(?P<sign>[\+-]).").unwrap());
        static RE_INT: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d+[\(\)'\s]").unwrap());
        static RE_BOOL: Lazy<Regex> = Lazy::new(|| Regex::new(r"^#[ft][\(\)'\s]").unwrap());
        static RE_ID: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[[:alnum:]!\$%&\*\+-\./<=>\?@\^_]+[\(\)'\s]").unwrap());
        static RE_SEP: Lazy<Regex> = Lazy::new(|| Regex::new(r"[\(\)\s']").unwrap());

        let input_from_cursor = self.input.split_at(*cursor).1;
        if let Some(_) = RE_PERIOD.find(input_from_cursor) {
            *cursor += 1;
            Err(anyhow!("read error: dot in wrong context"))
        } else if let Some(mat) = RE_FLOAT.find(input_from_cursor) {
            let (start, end) = (*cursor, *cursor + mat.end() - 1);
            *cursor = end;
            Ok(Token::Float(RE_FLOAT_NOINT
                .replace(self.input.get(start..end).unwrap(), "${sign}0.")
                .parse()
                .unwrap()))
        } else if let Some(mat) = RE_INT.find(input_from_cursor) {
            let (start, end) = (*cursor, *cursor + mat.end() - 1);
            *cursor = end;
            Ok(Token::Int(self.input
                .get(start..end)
                .unwrap()
                .parse()
                .unwrap()))
        } else if let Some(_) = RE_BOOL.find(input_from_cursor) {
            *cursor += 2;
            if input_from_cursor.as_bytes()[1] == b'f' {
                Ok(Token::Bool(false))
            } else {
                Ok(Token::Bool(true))
            }
        } else if let Some(mat) = RE_ID.find(input_from_cursor) {
            let (start, end) = (*cursor, *cursor + mat.end() - 1);
            *cursor = end + 1;
            Ok(Token::Id(self.input.get(start..end).unwrap().to_string()))
        } else {
            let end = RE_SEP.find(input_from_cursor).map(|m| m.end());
            *cursor = if let Some(end) = end {*cursor + end} else {self.input.len()};
            Err(anyhow!("read error: invalid character"))
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
