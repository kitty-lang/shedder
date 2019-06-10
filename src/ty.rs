use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::lexer::Ident;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Ty<'t> {
    Str,
    Void,
    User(Ident<'t>),
}

impl<'t> Display for Ty<'t> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "ty::")?;
        match self {
            Ty::Str => write!(fmt, "str"),
            Ty::Void => write!(fmt, "void"),
            Ty::User(ty) => write!(fmt, "user({})", ty.inner()),
        }
    }
}
