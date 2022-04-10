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