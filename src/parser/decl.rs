use crate::decl::Decl;
use crate::decl::Func;
use crate::lexer::Keyword;
use crate::lexer::Symbol;
use crate::lexer::Token;
use crate::lexer::TokenTy;
use crate::stmt::Stmt;
use crate::ty::Ty;

use super::error::*;
use super::split;
use super::try_eq_keyword;
use super::try_eq_symbol;
use super::try_get_ident;
use super::try_get_ty;

impl<'d> Decl<'d> {
    pub(super) fn handled() -> Vec<TokenTy> {
        let mut handled = vec![];
        handled.append(&mut Func::handled());
        handled
    }

    pub(super) fn parse(tokens: &'d [Token<'d>]) -> Result<(usize, Self)> {
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

    fn parse(tokens: &'f [Token<'f>]) -> Result<(usize, Self)> {
        try_eq_keyword(tokens, 0, Keyword::Func)?;

        let mut t = 1;
        let name = try_get_ident(tokens, t)
            .map_err(|mut err| {
                err.max_after(tokens.get(t - 1).map(|token| token.pos));
                err
            })?
            .clone();

        t += 1;
        try_eq_symbol(tokens, t, Symbol::LeftParen).map_err(|mut err| {
            err.max_after(tokens.get(t - 1).map(|token| token.pos));
            err
        })?;

        // TODO: args

        t += 1;
        try_eq_symbol(tokens, t, Symbol::RightParen).map_err(|mut err| {
            err.max_after(tokens.get(t - 1).map(|token| token.pos));
            err
        })?;

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

        try_eq_symbol(tokens, t, Symbol::LeftBracket).map_err(|mut err| {
            err.max_after(tokens.get(t - 1).map(|token| token.pos));
            err
        })?;

        let mut func = Func {
            name,
            ret: ty.unwrap_or(Ty::Void),
            stmts: vec![],
        };

        t += 1;
        loop {
            if t >= tokens.len() {
                let mut handled = Stmt::handled();
                handled.push(TokenTy::Symbol(Symbol::RightBracket));
                return Err(Error::missing_token(
                    handled,
                    tokens.get(t - 1).map(|token| token.pos),
                ));
            }

            if tokens[t].eq_symbol(Symbol::RightBracket) {
                t += 1;
                break;
            }

            let (t_, stmt) = Stmt::parse(split(tokens, t)).map_err(|mut err| {
                err.max_after(Some(tokens[t].pos));
                err
            })?;

            func.stmts.push(stmt);
            t += t_;
        }

        Ok((t, func))
    }
}
