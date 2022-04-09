use regex::Regex;
use once_cell::sync::Lazy;
use anyhow::{Context, Result, anyhow, bail};

pub enum Token {
    Num(i32),
    Bool(bool),
    Id(String),
    Str(String),
    Pair{car: Box<Token>, cdr: Box<Token>},
    Empty,
    Symbol(Box<Token>),
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
        let end_str = *cursor + mat.end();
        
        if !(end_str < self.input.len()) {
            Err(anyhow!("read error: unterminated string"))
        } else {
            let res = Token::Str(self.input.get(*cursor..(end_str)).unwrap().to_string());
            *cursor = end_str + 1;
            Ok(res)
        }
    }

    fn token_id_or_literal(&self, cursor: &mut usize) -> Result<Token> {
        todo!()
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
