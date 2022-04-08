use regex::Regex;
use anyhow::{Context, Result, anyhow, bail};

pub struct Token {
    value: String,
    kind: TokenKind,
}

pub enum TokenKind {
    Num,
    Bool,
    Id,
    Str,
    Pair{car: Box<Token>, cdr: Box<Token>},
    Empty,
    Symbol(Box<Token>),
}

impl Token {
    pub fn new(value: Option<&str>, kind: TokenKind) -> Self {
        match kind {
            TokenKind::Num | TokenKind::Bool | TokenKind::Id | TokenKind::Str
                => Token{value: value.unwrap().into(), kind: kind},
            TokenKind::Pair{..} | TokenKind::Empty | TokenKind::Symbol{..}
                => Token{value: String::new(), kind: kind}
        }
    }
}

pub struct Lexer {
    input: String
}

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
                self.skip_whitespace(cursor).context("syntax error: unterminated parenthesis")?;
                self.token_pair(cursor)
            },
            b')'  => Err(anyhow!("syntax error: extra close parenthesis")),
            b'\'' => {
                *cursor += 1;
                self.skip_whitespace(cursor).context("syntax error: unterminated quote")?;
                self.token(cursor)
                    .map(|t| Token::new(None, TokenKind::Symbol(Box::new(t))))
            },
            b'"'  => {
                *cursor += 1;
                self.token_str(cursor)
            },
            _     => self.token_id_or_literal(cursor)
        }
    }
    
    fn token_pair(&self, cursor: &mut usize) -> Result<Token> {
        if self.input.as_bytes()[*cursor] == b')' {
            // ()
            *cursor += 1;
            return Ok(Token::new(None, TokenKind::Empty));
        }

        let car = self.token(cursor)?;
        
        self.skip_whitespace(cursor).context("syntex error: unterminated parenthesis")?;

        if self.input.as_bytes()[*cursor] == b'.' {
            *cursor += 1;
            self.skip_whitespace(cursor).context("syntax error: unterminated parenthesis")?;
            let cdr = self.token(cursor)?;
            self.skip_whitespace(cursor).context("syntax error: unterminated parenthesis")?;
            *cursor += 1;
            Ok(Token::new(None, TokenKind::Pair{car: Box::new(car), cdr: Box::new(cdr)}))

        } else {
            let cdr = self.token_pair(cursor)?;
            Ok(Token::new(None, TokenKind::Pair{car: Box::new(car), cdr: Box::new(cdr)}))
        }
    }

    fn token_str(&self, cursor: &mut usize) -> Result<Token> {
        todo!()
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
