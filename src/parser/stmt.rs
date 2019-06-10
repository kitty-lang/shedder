use crate::expr::Expr;
use crate::lexer::Keyword;
use crate::lexer::Symbol;
use crate::lexer::Token;
use crate::lexer::TokenTy;
use crate::stmt::Let;
use crate::stmt::Return;
use crate::stmt::Stmt;

use super::error::*;
use super::split;
use super::try_eq_keyword;
use super::try_eq_symbol;
use super::try_get_ident;

impl<'s> Stmt<'s> {
    pub(super) fn handled() -> Vec<TokenTy> {
        let mut handled = vec![];
        handled.append(&mut Let::handled());
        handled.append(&mut Return::handled());
        handled.append(&mut Expr::handled());
        handled
    }

    pub(super) fn parse(tokens: &'s [Token<'s>]) -> Result<(usize, Self)> {
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
    fn handled() -> Vec<TokenTy> {
        vec![TokenTy::Keyword(Keyword::Let)]
    }

    fn parse(tokens: &'l [Token<'l>]) -> Result<(usize, Self)> {
        try_eq_keyword(tokens, 0, Keyword::Let)?;

        let mut t = 1;
        let name = try_get_ident(tokens, t)
            .map_err(|mut err| {
                err.max_after(tokens.get(t - 1).map(|token| token.pos));
                err
            })?
            .clone();

        t += 1;
        try_eq_symbol(tokens, t, Symbol::Equal).map_err(|mut err| {
            err.max_after(tokens.get(t - 1).map(|token| token.pos));
            err
        })?;

        t += 1;
        let (t_, value) = Expr::parse(split(tokens, t)).map_err(|mut err| {
            err.max_after(tokens.get(t).map(|token| token.pos));
            err
        })?;

        Ok((t + t_, Let { name, value }))
    }
}

impl<'r> Return<'r> {
    fn handled() -> Vec<TokenTy> {
        vec![TokenTy::Keyword(Keyword::Return)]
    }

    fn parse(tokens: &'r [Token<'r>]) -> Result<(usize, Self)> {
        try_eq_keyword(tokens, 0, Keyword::Return)?;

        let (t, expr) = Expr::parse(split(tokens, 1)).map_err(|mut err| {
            err.max_after(tokens.get(0).map(|token| token.pos));
            err
        })?;

        Ok((t + 1, Return(expr)))
    }
}
