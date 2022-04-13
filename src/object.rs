
pub struct Object {
    
}

pub enum ObjectKind {
    Number(NumberKind),
    Boolean(bool),
    Pair{
        car: Box<Object>,
        cdr: Box<Object>
    },
    Empty,
    Procedure(Procedure),
    Symbol(String),
    String(String),
}

pub enum NumberKind {
    Int(i64),
    Float(f64)
}

pub struct Procedure {

}
