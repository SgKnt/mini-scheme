use std::fmt;

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

impl Token {
    // (a ..) -> a
    pub fn first(&self) -> Option<&Self> {
        match &self {
            Token::Pair{car, ..} => Some(car.as_ref()),
            _ => None,
        }
    }

    // (a ..) -> ..
    // e.g. (a b c).next() -> (b c)
    //      (a b c).next().first() -> b
    pub fn next(&self) -> Option<&Self> {
        if let Token::Pair{car: _, cdr} = &self {
            if let Token::Pair{..} = &**cdr {
                Some(cdr)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Int(i) => write!(f, "{}", i),
            Token::Float(fl) => write!(f, "{}", fl),
            Token::Bool(b) if *b => write!(f, "#t"),
            Token::Bool(_) => write!(f, "#f"), 
            Token::Id(id) => write!(f, "{}", id),
            Token::Str(s) => write!(f, "{}", s),
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
            Token::Bool(b) if *b => write!(f, "#t[bool]"),
            Token::Bool(_) => write!(f, "#f[bool]"),
            Token::Id(id) => write!(f, "{}[id]", id),
            Token::Str(s) => write!(f, "\"{}\"[string]", s),
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

struct TokenIter<'a> {
    token: &'a Token,
}

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
