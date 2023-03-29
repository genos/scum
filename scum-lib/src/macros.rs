use crate::expression::{Atom, Expression, Identifier};

macro_rules! ident {
    ($s:literal) => {
        Identifier($s.to_string())
    }
}

pub(crate) use ident;
