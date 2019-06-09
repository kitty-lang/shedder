use crate::expr::Expr;
use crate::lexer::Keyword;
use crate::lexer::Symbol;
use crate::lexer::Token;
use crate::lexer::TokenTy;
use crate::stmt::Let;
use crate::stmt::Stmt;

use super::error::*;
use super::split;
use super::try_eq_keyword;
use super::try_eq_symbol;
use super::try_get_ident;

impl<'s> Stmt<'s> {
    pub(super) fn handled() -> Vec<TokenTy> {
        let mut handled = vec![];
        handled.append(&mut Expr::handled());
        handled.append(&mut Let::handled());
        handled
    }

    pub(super) fn parse(tokens: &'s [Token<'s>]) -> Result<(usize, Self)> {
        if tokens.is_empty() {
            return Err(Error::missing_token(Self::handled(), None));
        }

        let mut error = Error::multiple(vec![]);

        match Expr::parse(tokens) {
            Ok((t, expr)) => return Ok((t, Stmt::Expr(expr))),
            Err(mut err) => {
                error = error.concat({
                    err.max_after(tokens.get(0).map(|token| token.pos));
                    err
                })
            }
        }

        match Let::parse(tokens) {
            Ok((t, let_)) => return Ok((t, Stmt::Let(let_))),
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
    fn handled() -> Vec<TokenTy> {
        vec![TokenTy::Keyword(Keyword::Let)]
    }

    fn parse(tokens: &'l [Token<'l>]) -> Result<(usize, Self)> {
        try_eq_keyword(tokens, 0, Keyword::Let)?;

        let name = try_get_ident(tokens, 1)
            .map_err(|mut err| {
                err.max_after(tokens.get(0).map(|token| token.pos));
                err
            })?
            .clone();

        try_eq_symbol(tokens, 2, Symbol::Equal).map_err(|mut err| {
            err.max_after(tokens.get(1).map(|token| token.pos));
            err
        })?;

        let (t, value) = Expr::parse(split(tokens, 3)).map_err(|mut err| {
            err.max_after(tokens.get(2).map(|token| token.pos));
            err
        })?;

        let t = t + 3;

        try_eq_symbol(tokens, t, Symbol::SemiColon).map_err(|mut err| {
            err.max_after(tokens.get(t - 1).map(|token| token.pos));
            err
        })?;

        Ok((t + 1, Let { name, value }))
    }
}
