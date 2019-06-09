use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use fnv::FnvHashMap;

use crate::decl::Decl;
use crate::decl::Func;
use crate::lexer::Ident;
use crate::lexer::Token;
use crate::lexer::TokenTy;

use super::error::*;
use super::split;

pub struct Module<'m> {
    pub name: Ident<'m>,
    pub funcs: FnvHashMap<Ident<'m>, Func<'m>>,
}

impl<'m> Module<'m> {
    pub(super) fn new(name: Ident<'m>) -> Self {
        Module {
            name,
            funcs: FnvHashMap::default(),
        }
    }

    pub(super) fn handled() -> Vec<TokenTy> {
        let mut handled = Decl::handled();
        handled.push(TokenTy::EOF);

        handled
    }

    // TODO: parse

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
                    self.funcs.insert(func.name.clone(), func);
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

        for func in self.funcs.keys() {
            write!(fmt, " {} ", func.inner())?;
        }

        write!(fmt, "])")
    }
}
