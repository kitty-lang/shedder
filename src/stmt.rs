use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use super::expr::Expr;

#[derive(Debug)]
pub enum Stmt<'s> {
    Expr(Expr<'s>),
}

impl<'s> Display for Stmt<'s> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "stmt::")?;
        match self {
            Stmt::Expr(expr) => write!(fmt, "{}", expr),
        }
    }
}
