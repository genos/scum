use crate::expression::{Atom, Expression, Identifier};
use smol_str::SmolStr;

macro_rules! ident {
    ($s:literal) => {
        Identifier(SmolStr::new($s))
    };
}

pub(crate) use ident;
