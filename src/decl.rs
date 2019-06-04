use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::lexer::Ident;
use crate::lexer::Keyword;
use crate::lexer::TokenTy;
use crate::stmt::Stmt;

#[derive(Debug)]
pub enum Decl<'d> {
    Func(Func<'d>),
}

#[derive(Debug)]
pub struct Func<'f> {
    pub name: &'f Ident,
    // TODO: params
    // TODO: ret
    pub stmts: Vec<Stmt<'f>>,
}

impl<'d> Decl<'d> {
    pub(super) fn handled() -> Vec<TokenTy> {
        vec![TokenTy::Keyword(Keyword::Func)]
    }
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

        write!(fmt, "])")
    }
}
