use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::lexer::Ident;

use super::expr::Expr;

#[derive(Debug)]
pub enum Stmt<'s> {
    Let(Let<'s>),
    Return(Return<'s>),
    Expr(Expr<'s>),
}

#[derive(Debug)]
pub struct Let<'l> {
    pub name: Ident<'l>,
    pub value: Expr<'l>,
}

#[derive(Debug)]
pub struct Return<'r>(pub Expr<'r>);

impl<'s> Display for Stmt<'s> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "stmt::")?;
        match self {
            Stmt::Let(let_) => write!(fmt, "{}", let_),
            Stmt::Return(ret) => write!(fmt, "{}", ret),
            Stmt::Expr(expr) => write!(fmt, "{}", expr),
        }
    }
}

impl<'l> Display for Let<'l> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "let(name={}, value={})", self.name, self.value)
    }
}

impl<'r> Display for Return<'r> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "return({})", self.0)
    }
}
