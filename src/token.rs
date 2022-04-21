use std::fmt;

pub enum Token {
    Int(i64),
    Float(f64),
    Boolean(bool),
    Id(String),
    String(String),
    Pair{car: Box<Token>, cdr: Box<Token>},
    Empty,
    Symbol(Box<Token>),
}

impl Token {
    // (a ..) -> a
    pub fn elem(&self) -> Option<&Self> {
        match self {
            Token::Pair{car, ..} => Some(&**car),
            _ => None,
        }
    }

    // (a ..) -> ..
    pub fn next(&self) -> Option<&Self> {
        if let Token::Pair{car: _, cdr} = self {
            match &**cdr {
                Token::Pair{..} | Token::Empty => Some(cdr),
                _ => None
            } 
        } else {
            None
        }
    }

    pub fn nth(&self, n: usize) -> Option<&Self> {
        let mut t = self;
        for _ in 0..n {
            if let Token::Pair{car: _, cdr} = t {
                t = &**cdr;
            } else {
                return None;
            }
        }
        t.elem()
    }

    pub fn is_empty(&self) -> bool {
        if let Token::Empty = self {
            true
        } else {
            false
        }
    }

    pub fn is_list(&self) -> bool {
        let mut t = self;
        loop {
            match t {
                Token::Pair{car: _, cdr} => {
                    t = &&*cdr;
                    continue;
                }
                Token::Empty => {
                    return true;
                }
                _ => {
                    return false;
                }
            }
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Int(i) => write!(f, "{}", i),
            Token::Float(fl) => write!(f, "{}", fl),
            Token::Boolean(b) if *b => write!(f, "#t"),
            Token::Boolean(_) => write!(f, "#f"), 
            Token::Id(id) => write!(f, "{}", id),
            Token::String(s) => write!(f, "{}", s),
            Token::Pair{car, cdr} => match **cdr {
                Token::Pair{..} => {
                    let cdr = format!("{}", cdr);
                    write!(f, "({} {}", car, cdr.split_at(1).1)
                },
                Token::Empty => write!(f, "({})", car),
                _ => write!(f, "({} . {})", car, cdr),
            },
            Token::Empty => write!(f, "()"),
            Token::Symbol(s) => write!(f, "'{}", s),
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Int(i) => write!(f, "{}[int]", i),
            Token::Float(fl) => write!(f, "{}[float]", fl),
            Token::Boolean(b) if *b => write!(f, "#t[boolean]"),
            Token::Boolean(_) => write!(f, "#f[boolean]"),
            Token::Id(id) => write!(f, "{}[id]", id),
            Token::String(s) => write!(f, "\"{}\"[string]", s),
            Token::Pair{car, cdr} => match **cdr {
                Token::Pair{..} => {
                    let cdr = format!("{:?}", cdr);
                    write!(f, "({:?} {}", car, cdr.split_at(1).1) // (car . (...)) -> (car ...) 
                },
                Token::Empty => write!(f, "({:?})", car),
                _ => {
                    write!(f, "({:?} . {:?})", car, cdr)
                }
            }
            Token::Empty => write!(f, "()"),
            Token::Symbol(s) => write!(f, "'{:?}", s)
        }
    }
}

impl<'a> std::iter::IntoIterator for &'a Token {
    type Item = &'a Token;
    type IntoIter = TokenIter<'a>;
    fn into_iter(self) -> TokenIter<'a> {
        TokenIter{token: self}
    }
}

pub struct TokenIter<'a> {
    token: &'a Token,
}

// (a b c) -> a, b, c
impl<'a> Iterator for TokenIter<'a> {
    type Item = &'a Token;
    fn next(&mut self) -> Option<&'a Token> {
        if let Token::Pair{car, cdr} = self.token {
            self.token = &**cdr;
            Some(&**car)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nth_test1() {
        let t = Token::Pair{
            car: Box::new(Token::Int(0)),
            cdr: Box::new(Token::Pair { 
                car: Box::new(Token::Boolean(false)), 
                cdr: Box::new(Token::Empty)
            })
        };
        assert_eq!(format!("{}", t.nth(0).unwrap()), "0");
        assert_eq!(format!("{}", t.nth(1).unwrap()), "#f");
    }
    #[test]
    #[should_panic]
    fn nth_test2() {
        let t = Token::Pair{
            car: Box::new(Token::Int(0)),
            cdr: Box::new(Token::Pair { 
                car: Box::new(Token::Boolean(false)), 
                cdr: Box::new(Token::Empty)
            })
        };
        t.nth(2).unwrap();
    }
}
