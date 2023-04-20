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
pub enum Expression {
    Constant(Atom),
    Define(Box<Expression>, Box<Expression>),
    If(Box<Expression>, Box<Expression>, Box<Expression>),
    Function(fn(Vec<Expression>) -> Result<Expression, FunctionError>),
    Lambda(Box<Expression>, Box<Expression>),
    List(Vec<Expression>),
}

#[derive(Debug, thiserror::Error)]
pub enum FunctionError {
    #[error("Expected {0} arguments, received {1}")]
    WrongNumberOfArgs(usize, usize),
    #[error("Expected two args with the same type, received {0} and {1}")]
    TypeMismatch(Expression, Expression),
    #[error("Expected two numeric args, received {0} and {1}")]
    NonNumericArgs(Expression, Expression),
}
