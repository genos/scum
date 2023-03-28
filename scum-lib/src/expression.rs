#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Identifier(pub String);

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Bool(bool),
    Float(f64),
    Int(i64),
    Str(String),
    Symbol(Identifier),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression<'a> {
    Constant(Atom),
    Define(&'a Expression<'a>, &'a Expression<'a>),
    If(&'a Expression<'a>, &'a Expression<'a>, &'a Expression<'a>),
    List(Vec<Expression<'a>>),
}
