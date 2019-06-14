use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::lexer::Ident;
use crate::lexer::Keyword;
use crate::lexer::Symbol;
use crate::lexer::Token;
use crate::lexer::TokenTy;
use crate::lexer::Ty;

use super::error::*;
use super::split;
use super::stmt::Stmt;
use super::try_eq_keyword;
use super::try_eq_symbol;
use super::try_get_ident;
use super::try_get_ty;

#[derive(Debug)]
pub enum Decl<'d> {
    Func(Func<'d>),
}

#[derive(Debug)]
pub struct Func<'f> {
    pub name: Ident<'f>,
    pub args: Vec<Arg<'f>>,
    pub ret: Ty,
    pub stmts: Vec<Stmt<'f>>,
}

#[derive(Debug)]
pub struct Arg<'a> {
    pub name: Ident<'a>,
    pub ty: Ty,
}

impl<'d> Decl<'d> {
    pub(super) fn handled() -> Vec<TokenTy> {
        let mut handled = vec![];
        handled.append(&mut Func::handled());
        handled
    }

    pub(super) fn parse(tokens: &'d [Token<'d>]) -> Result<(usize, Decl<'d>)> {
        if tokens.is_empty() {
            return Err(Error::missing_token(Self::handled(), None));
        }

        let mut error = Error::multiple(vec![]);

        match Func::parse(tokens) {
            Ok((t, func)) => return Ok((t, Decl::Func(func))),
            Err(err) => error = error.concat(err),
        }

        Err(error)
    }
}

impl<'f> Func<'f> {
    fn handled() -> Vec<TokenTy> {
        vec![TokenTy::Keyword(Keyword::Func)]
    }

    fn parse(tokens: &'f [Token<'f>]) -> Result<(usize, Func<'f>)> {
        try_eq_keyword(tokens, 0, Keyword::Func)?;

        let mut t = 1;
        let name = try_get_ident(tokens, t)
            .map_err(|mut err| {
                err.max_after(tokens.get(t - 1).map(|token| token.pos));
                err
            })?
            .as_ref();

        t += 1;
        try_eq_symbol(tokens, t, Symbol::LeftParen).map_err(|mut err| {
            err.max_after(tokens.get(t - 1).map(|token| token.pos));
            err
        })?;

        let mut args = vec![];

        t += 1;
        let mut arg = None;
        loop {
            if t >= tokens.len() {
                let mut handled = Stmt::handled();
                handled.push(TokenTy::Symbol(Symbol::RightParen));
                return Err(Error::missing_token(
                    handled,
                    tokens.get(t - 1).map(|token| token.pos),
                ));
            }

            if tokens[t].eq_symbol(Symbol::RightParen) {
                break;
            }

            if arg.is_none() {
                arg = Some(try_get_ident(tokens, t)
                    .map_err(|mut err| {
                        err.max_after(tokens.get(t - 1).map(|token| token.pos));
                        err
                    })?.as_ref()
                );
            } else {
                try_eq_symbol(tokens, t, Symbol::Colon).map_err(|mut err| {
                    err.max_after(tokens.get(t - 1).map(|token| token.pos));
                    err
                })?;

                t += 1;
                let ty = try_get_ty(tokens, t).map_err(|mut err| {
                    err.max_after(tokens.get(t - 1).map(|token| token.pos));
                    err
                })?;

                args.push(Arg { name: arg.unwrap(), ty });
                arg = None;
            }

            t += 1;
        }

        t += 1;
        let ty = match try_eq_symbol(tokens, t, Symbol::Colon) {
            Ok(()) => {
                t += 1;
                let ty = Some(try_get_ty(tokens, t).map_err(|mut err| {
                    err.max_after(tokens.get(t - 1).map(|token| token.pos));
                    err
                })?);

                t += 1;
                ty
            }
            Err(ref err) if err.is_wrong_token() => None,
            Err(mut err) => {
                err.max_after(tokens.get(t - 1).map(|token| token.pos));
                return Err(err);
            }
        };

        try_eq_symbol(tokens, t, Symbol::LeftBrace).map_err(|mut err| {
            err.max_after(tokens.get(t - 1).map(|token| token.pos));
            err
        })?;

        let mut stmts = vec![];

        t += 1;
        loop {
            if t >= tokens.len() {
                let mut handled = Stmt::handled();
                handled.push(TokenTy::Symbol(Symbol::RightBrace));
                return Err(Error::missing_token(
                    handled,
                    tokens.get(t - 1).map(|token| token.pos),
                ));
            }

            if tokens[t].eq_symbol(Symbol::RightBrace) {
                t += 1;
                break;
            }

            let (t_, stmt) = Stmt::parse(split(tokens, t)).map_err(|mut err| {
                err.max_after(Some(tokens[t].pos));
                err
            })?;

            stmts.push(stmt);
            t += t_;
        }

        Ok((t, Func {
            name,
            args,
            ret: ty.unwrap_or(Ty::Void),
            stmts,
        }))
    }
}

impl<'a> Arg<'a> {
    pub fn as_ref(&'a self) -> Arg<'a> {
        Arg {
            name: self.name.as_ref(),
            ty: self.ty,
        }
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

        for arg in &self.args {
            write!(fmt, " {} ", arg)?;
        }

        write!(fmt, "], ret={})", self.ret)
    }
}

impl<'a> Display for Arg<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "arg(name={}, ty={})", self.name.inner(), self.ty)
    }
}
