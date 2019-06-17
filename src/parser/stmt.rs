use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::lexer::Ident;
use crate::lexer::Keyword;
use crate::lexer::Symbol;
use crate::lexer::Token;
use crate::lexer::TokenTy;

use super::error::*;
use super::expr::Expr;
use super::split;
use super::try_eq_keyword;
use super::try_eq_symbol;
use super::try_get_ident;

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

impl<'s> Stmt<'s> {
    pub(super) fn handled() -> Vec<TokenTy> {
        let mut handled = vec![];
        handled.append(&mut Let::handled());
        handled.append(&mut Return::handled());
        handled.append(&mut Expr::handled());
        handled
    }

    pub(super) fn parse(tokens: &'s [Token<'s>]) -> Result<(usize, Stmt<'s>)> {
        if tokens.is_empty() {
            return Err(Error::missing_token(Self::handled(), None));
        }

        let mut error = Error::multiple(vec![]);

        match Return::parse(tokens) {
            Ok((t, ret)) => {
                if let Err(err) = try_eq_symbol(tokens, t, Symbol::SemiColon).map_err(|mut err| {
                    err.max_after(tokens.get(t - 1).map(|token| token.pos));
                    err
                }) {
                    error = error.concat(err);
                } else {
                    return Ok((t + 1, Stmt::Return(ret)));
                }
            }
            Err(mut err) => {
                error = error.concat({
                    err.max_after(tokens.get(0).map(|token| token.pos));
                    err
                })
            }
        }

        match Let::parse(tokens) {
            Ok((t, let_)) => {
                if let Err(err) = try_eq_symbol(tokens, t, Symbol::SemiColon).map_err(|mut err| {
                    err.max_after(tokens.get(t - 1).map(|token| token.pos));
                    err
                }) {
                    error = error.concat(err);
                } else {
                    return Ok((t + 1, Stmt::Let(let_)));
                }
            }
            Err(mut err) => {
                error = error.concat({
                    err.max_after(tokens.get(0).map(|token| token.pos));
                    err
                })
            }
        }

        match Expr::parse(tokens) {
            Ok((t, expr)) => {
                if let Err(err) = try_eq_symbol(tokens, t, Symbol::SemiColon).map_err(|mut err| {
                    err.max_after(tokens.get(t - 1).map(|token| token.pos));
                    err
                }) {
                    error = error.concat(err);
                } else {
                    return Ok((t + 1, Stmt::Expr(expr)));
                }
            }
            Err(mut err) => {
                error = error.concat({
                    err.max_after(tokens.get(0).map(|token| token.pos));
                    err
                })
            }
        }

        Err(error)
    }
}

impl<'l> Let<'l> {
    pub fn as_ref(&'l self) -> Let<'l> {
        Let {
            name: self.name.as_ref(),
            value: self.value.as_ref(),
        }
    }

    fn handled() -> Vec<TokenTy> {
        vec![TokenTy::Keyword(Keyword::Let)]
    }

    fn parse(tokens: &'l [Token<'l>]) -> Result<(usize, Let<'l>)> {
        try_eq_keyword(tokens, 0, Keyword::Let)?;

        let mut t = 1;
        let name = try_get_ident(tokens, t)
            .map_err(|mut err| {
                err.max_after(tokens.get(t - 1).map(|token| token.pos));
                err
            })?
            .as_ref();

        t += 1;
        try_eq_symbol(tokens, t, Symbol::Equal).map_err(|mut err| {
            err.max_after(tokens.get(t - 1).map(|token| token.pos));
            err
        })?;

        t += 1;
        let (t_, value) = Expr::parse(split(tokens, t)).map_err(|mut err| {
            err.max_after(tokens.get(t - 1).map(|token| token.pos));
            err
        })?;

        Ok((t + t_, Let { name, value }))
    }
}

impl<'r> Return<'r> {
    pub fn as_ref(&'r self) -> Return<'r> {
        Return(self.0.as_ref())
    }

    fn handled() -> Vec<TokenTy> {
        vec![TokenTy::Keyword(Keyword::Return)]
    }

    fn parse(tokens: &'r [Token<'r>]) -> Result<(usize, Return<'r>)> {
        try_eq_keyword(tokens, 0, Keyword::Return)?;

        let (t, expr) = Expr::parse(split(tokens, 1)).map_err(|mut err| {
            err.max_after(tokens.get(0).map(|token| token.pos));
            err
        })?;

        Ok((t + 1, Return(expr)))
    }
}

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
