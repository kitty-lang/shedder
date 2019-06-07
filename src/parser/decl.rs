use crate::decl::Decl;
use crate::decl::Func;
use crate::expr::Expr;
use crate::lexer::Keyword;
use crate::lexer::Symbol;
use crate::lexer::TokenTy;
use crate::stmt::Stmt;

use super::error::*;
use super::parse::try_eq_keyword;
use super::parse::try_eq_symbol;
use super::parse::try_get_ident;
use super::parse::Parse;
use super::parse::State;
use super::parse::Tokens;
use super::split;

impl<'d> Decl<'d> {
    pub(super) fn handled() -> Vec<TokenTy> {
        vec![TokenTy::Keyword(Keyword::Func)]
    }
}

impl<'e> Expr<'e> {
    pub(super) fn handled() -> Vec<TokenTy> {
        vec![TokenTy::Literal]
    }
}

impl<'d> Parse<'d> for Decl<'d> {
    fn parse(tokens: Tokens<'d>, state: &mut State) -> Result<'d, (Tokens<'d>, Self)> {
        if tokens.is_empty() {
            return Err(Error::missing_token(Self::handled()));
        }

        let mut error = Error::multiple(vec![]);

        match Func::parse(tokens, state) {
            Ok((tokens, func)) => return Ok((tokens, Decl::Func(func))),
            Err(err) => {
                error = error.concat(err);
            }
        }

        Err(error)
    }
}

impl<'f> Parse<'f> for Func<'f> {
    fn parse(tokens: Tokens<'f>, state: &mut State) -> Result<'f, (Tokens<'f>, Self)> {
        try_eq_keyword(tokens, 0, Keyword::Func)?;

        let name = try_get_ident(tokens, 1)?;

        try_eq_symbol(tokens, 2, Symbol::LeftParen)?;

        // TODO: args

        try_eq_symbol(tokens, 3, Symbol::RightParen)?;

        // TODO: ret

        try_eq_symbol(tokens, 4, Symbol::LeftBracket)?;

        let mut func = Func {
            name,
            stmts: vec![],
        };

        let mut t = 5;
        loop {
            if t >= tokens.len() {
                let mut handled = Stmt::handled();
                handled.push(TokenTy::Symbol(Symbol::RightBracket));

                return Err(Error::missing_token(handled));
            }

            if tokens[t].eq_symbol(Symbol::RightBracket) {
                t += 1;
                break;
            }

            let (tokens_, stmt) = Stmt::parse(split(&tokens, t), state)?;
            func.stmts.push(stmt);
            t = tokens.len() - tokens_.len();
        }

        Ok((split(tokens, t), func))
    }
}
