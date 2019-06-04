use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::lexer::Ident;
use crate::lexer::Token;
use crate::lexer::TokenTy;

#[derive(Debug)]
pub enum Expr<'e> {
    Func(Func<'e>),
}

#[derive(Debug)]
pub struct Func<'f> {
    pub name: &'f Ident,
    pub args: Vec<&'f Token<'f>>,
}

impl<'e> Expr<'e> {
    pub(super) fn handled() -> Vec<TokenTy> {
        vec![TokenTy::Ident]
    }
}

impl<'e> Display for Expr<'e> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "expr::")?;
        match self {
            Expr::Func(func) => write!(fmt, "{}", func),
        }
    }
}

impl<'f> Display for Func<'f> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "func(name={}, args=[", self.name)?;

        for arg in &self.args {
            write!(fmt, " {} ", arg)?;
        }

        write!(fmt, "]")
    }
}
