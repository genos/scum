use crate::expression::{Atom, Define, Environment, Expression, Identifier, If, Lambda};

#[derive(Debug, thiserror::Error)]
pub enum ReadingError {
    #[error("{0}")]
    ParseError(#[from] peg::error::ParseError<peg::str::LineCol>),
}

pub fn read(input: &str, env: &Environment) -> Result<Expression, ReadingError> {
    scum_parser::expression(input, env).map_err(Into::into)
}

peg::parser! {
    grammar scum_parser(env: &Environment) for str {
        pub rule expression() -> Expression =
            constant() / define() / ifte() / lambda() / list()

        // see https://groups.csail.mit.edu/mac/ftpdir/scheme-reports/r5rs-html/r5rs_9.html

        // identifiers
        rule identifier() -> Identifier =
            i:$(initial() subsequent()* / "+" / "-") { Identifier::from(i) }
        rule initial() -> char =
            ['a'..='z' | 'A'..='Z' | '!' | '$' | '%' | '&' | '*' | '/' | ':' | '<'
            | '=' | '>' | '?' | '^' | '_' | '~']
        rule subsequent() -> char = initial() / ['0'..='9' | '+' | '-' | '.' | '@']

        // atoms
        rule boolean() -> bool = c:$("#t" / "#f") { c == "#t" }
        rule exp() -> &'input str = $("e" ['+' | '-']? ['0'..='9']+)
        rule float() -> f64 =
            x:$("-"? ((['0'..='9']+ "." ['0'..='9']* exp()?) / (['0'..='9']+ exp())))
            {? x.parse().or(Err("float")) }
        rule int() -> i64 =
            n:$("-"? ("0" / ['1'..='9']['0'..='9']*))
            {? n.parse().or(Err("int")) }
        rule string() -> &'input str = $("\"" character()* "\"")
        rule character() -> &'input str =
            !['"' |'\\'] c:$[_] { c }
            / "\\" c:$['"' | '\\' | '/' | 'b' | 'f' | 'n' | 'r' | 't'] { c }
            / "\\" c:$("u" ['0'..='9' | 'a'..='f' | 'A'..='F']*<4>) { c }
        rule symbol() -> Identifier = identifier()
        rule atom() -> Atom =
            b:boolean()  { Atom::Bool(b) }
            / f:float()  { Atom::Float(f) }
            / i:int()    { Atom::Int(i) }
            / s:string() { Atom::Str(s.into()) }
            / s:symbol() { Atom::Symbol(s) }

        // expressions
        rule _() = [' ' | '\t' | '\r' | '\n']*
        rule constant() -> Expression = a:atom() { Expression::Constant(a) }
        rule define() -> Expression =
            "(" _ "define" _ name:expression() _ value:expression() _ ")"
            { Define {name, value}.into() }
        rule ifte() -> Expression =
            "(" _ "if" _ cond:expression() _ true_:expression() _ false_:expression() ")"
            { If { cond, true_, false_ }.into() }
        rule params() -> Vec<Identifier> =
            "(" _ ps:(identifier() ** _) _ ")" { ps }
        rule lambda() -> Expression =
            "(" _ "lambda" _ params:params() _ body:expression() _ ")"
            { Lambda { params, body, env: env.clone() }.into() }
        rule list() -> Expression =
            "(" _ es:(expression() ** _) _ ")" { Expression::List(es.into()) }
    }
}
