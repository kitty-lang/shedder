use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::lexer::Ident;
use crate::lexer::Token;
use crate::lexer::TokenTy;

use super::decl::Decl;
use super::decl::Func;
use super::error::*;
use super::split;

#[derive(Debug)]
pub struct Module<'m> {
    pub name: Ident<'m>,
    pub funcs: Vec<Func<'m>>,
}

impl<'m> Module<'m> {
    pub(super) fn new(name: Ident<'m>) -> Module<'m> {
        Module {
            name,
            funcs: vec![],
        }
    }

    pub(super) fn handled() -> Vec<TokenTy> {
        let mut handled = Decl::handled();
        handled.push(TokenTy::EOF);

        handled
    }

    pub(super) fn parse(&mut self, tokens: &'m [Token<'m>]) -> Result<'m, usize> {
        let mut t = 0;
        loop {
            if t >= tokens.len() {
                return Err(Error::missing_token(vec![TokenTy::EOF], None));
            }

            if tokens[t].is_eof() {
                t += 1;
                break;
            }

            match Decl::parse(split(tokens, t))? {
                (t_, Decl::Func(func)) => {
                    self.funcs.push(func);
                    t += t_;
                }
            }
        }

        Ok(t)
    }
}

impl<'m> Display for Module<'m> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "module(name={}, funcs=[", self.name.inner())?;

        for func in &self.funcs {
            write!(fmt, " {} ", func.name.inner())?;
        }

        write!(fmt, "])")
    }
}
