use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::lexer::Ident;
use crate::stmt::Stmt;
use crate::ty::Ty;

#[derive(Debug)]
pub enum Decl<'d> {
    Func(Func<'d>),
}

#[derive(Debug)]
pub struct Func<'f> {
    pub name: Ident<'f>,
    // TODO: params
    pub ret: Ty<'f>,
    pub stmts: Vec<Stmt<'f>>,
}

impl<'d> Display for Decl<'d> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "decl::")?;
        match self {
            Decl::Func(func) => write!(fmt, "{}", func),
        }
    }
}

impl<'f> Display for Func<'f> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "func(name={}, args=[", self.name.inner())?;

        // TODO

        write!(fmt, "], ret={})", self.ret)
    }
}
