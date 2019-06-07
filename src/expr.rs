use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::lexer::Ident;
use crate::lexer::Literal as LexLiteral;

#[derive(Debug)]
pub enum Expr<'e> {
    Literal(Literal<'e>),
    Func(Func<'e>),
    Var(&'e Ident),
}

#[derive(Debug)]
pub struct Literal<'l> {
    pub name: Ident,
    pub lit: &'l LexLiteral<'l>,
}

#[derive(Debug)]
pub struct Func<'f> {
    pub name: &'f Ident,
    pub args: Vec<Expr<'f>>,
}

impl<'e> Display for Expr<'e> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "expr::")?;
        match self {
            Expr::Literal(lit) => write!(fmt, "{}", lit),
            Expr::Func(func) => write!(fmt, "{}", func),
            Expr::Var(var) => write!(fmt, "{}", var),
        }
    }
}

impl<'l> Display for Literal<'l> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}", self.lit)
    }
}

impl<'f> Display for Func<'f> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "func(name={}, args=[", self.name)?;

        for arg in &self.args {
            write!(fmt, " {} ", arg)?;
        }

        write!(fmt, "])")
    }
}
