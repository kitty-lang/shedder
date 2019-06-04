use crate::expr::Expr;
use crate::expr::Func;
use crate::lexer::Symbol;
use crate::lexer::TokenTy;

use super::error::*;
use super::parse::try_eq_symbol;
use super::parse::try_get_ident;
use super::parse::Parse;
use super::parse::Tokens;
use super::split;

impl<'e> Parse<'e> for Expr<'e> {
    fn parse(tokens: Tokens<'e>) -> Result<(Tokens<'e>, Self)> {
        if tokens.is_empty() {
            return Err(Error::missing_token(Self::handled()));
        }

        let mut error = Error::multiple(vec![]);

        match Func::parse(tokens) {
            Ok((tokens, func)) => return Ok((tokens, Expr::Func(func))),
            Err(err) => {
                error = error.concat(err);
            }
        }

        Err(error)
    }
}

impl<'f> Parse<'f> for Func<'f> {
    fn parse(tokens: Tokens<'f>) -> Result<(Tokens<'f>, Self)> {
        let name = try_get_ident(tokens, 0)?;

        try_eq_symbol(tokens, 1, Symbol::LeftParen)?;

        if tokens.len() < 3 {
            let mut handled = Expr::handled();
            handled.push(TokenTy::Symbol(Symbol::RightParen));

            return Err(Error::missing_token(handled));
        }

        let mut args = vec![];
        let mut can_close = true;
        for token in split(&tokens, 2) {
            if can_close && token.eq_symbol(Symbol::RightParen) {
                break;
            }

            if token.eq_symbol(Symbol::RightParen) {
                can_close = false;
            }

            args.push(token);
        }

        try_eq_symbol(tokens, 3 + args.len(), Symbol::SemiColon)?;

        Ok((split(tokens, 4 + args.len()), Func { name, args }))
    }
}
