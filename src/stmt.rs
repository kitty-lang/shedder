use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::lexer::Ident;

use super::expr::Expr;

#[derive(Debug)]
pub enum Stmt<'s> {
    Expr(Expr<'s>),
    Let(Let<'s>),
}

#[derive(Debug)]
pub struct Let<'l> {
    pub name: Ident<'l>,
    pub value: Expr<'l>,
}

impl<'s> Display for Stmt<'s> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "stmt::")?;
        match self {
            Stmt::Expr(expr) => write!(fmt, "{}", expr),
            Stmt::Let(let_) => write!(fmt, "{}", let_),
        }
    }
}

impl<'l> Display for Let<'l> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "let(name={}, value={})", self.name, self.value)
    }
}
