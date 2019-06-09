use crate::decl::Decl;
use crate::decl::Func;
use crate::lexer::Keyword;
use crate::lexer::Symbol;
use crate::lexer::Token;
use crate::lexer::TokenTy;
use crate::stmt::Stmt;

use super::error::*;
use super::split;
use super::try_eq_keyword;
use super::try_eq_symbol;
use super::try_get_ident;

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

        let name = try_get_ident(tokens, 1)
            .map_err(|mut err| {
                err.max_after(tokens.get(0).map(|token| token.pos));
                err
            })?
            .clone();

        try_eq_symbol(tokens, 2, Symbol::LeftParen).map_err(|mut err| {
            err.max_after(tokens.get(1).map(|token| token.pos));
            err
        })?;

        // TODO: args

        try_eq_symbol(tokens, 3, Symbol::RightParen).map_err(|mut err| {
            err.max_after(tokens.get(2).map(|token| token.pos));
            err
        })?;

        // TODO: ret

        try_eq_symbol(tokens, 4, Symbol::LeftBracket).map_err(|mut err| {
            err.max_after(tokens.get(3).map(|token| token.pos));
            err
        })?;

        let mut func = Func {
            name,
            stmts: vec![],
        };

        let mut t = 5;
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
