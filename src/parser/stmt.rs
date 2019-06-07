use crate::expr::Expr;
use crate::lexer::Keyword;
use crate::lexer::Symbol;
use crate::lexer::TokenTy;
use crate::stmt::Let;
use crate::stmt::Stmt;

use super::error::*;
use super::parse::try_eq_keyword;
use super::parse::try_eq_symbol;
use super::parse::try_get_ident;
use super::parse::Parse;
use super::parse::State;
use super::parse::Tokens;
use super::split;

impl<'s> Stmt<'s> {
    pub(super) fn handled() -> Vec<TokenTy> {
        let mut handled = vec![];
        handled.append(&mut Expr::handled());
        handled
    }
}

impl<'s> Parse<'s> for Stmt<'s> {
    fn parse(tokens: Tokens<'s>, state: &mut State) -> Result<'s, (Tokens<'s>, Self)> {
        if tokens.is_empty() {
            return Err(Error::missing_token(Self::handled()));
        }

        let mut error = Error::multiple(vec![]);

        match Expr::parse(tokens, state) {
            Ok((tokens, expr)) => return Ok((tokens, Stmt::Expr(expr))),
            Err(err) => {
                error = error.concat(err);
            }
        }

        match Let::parse(tokens, state) {
            Ok((tokens, let_)) => return Ok((tokens, Stmt::Let(let_))),
            Err(err) => {
                error = error.concat(err);
            }
        }

        Err(error)
    }
}

impl<'l> Parse<'l> for Let<'l> {
    fn parse(tokens: Tokens<'l>, state: &mut State) -> Result<'l, (Tokens<'l>, Self)> {
        try_eq_keyword(tokens, 0, Keyword::Let)?;

        let name = try_get_ident(tokens, 1)?;

        try_eq_symbol(tokens, 2, Symbol::Equal)?;

        let (tokens, value) = Expr::parse(split(tokens, 3), state)?;

        try_eq_symbol(tokens, 0, Symbol::SemiColon)?;

        Ok((split(tokens, 1), Let { name, value }))
    }
}
