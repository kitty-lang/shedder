use crate::expr::Expr;
use crate::expr::Func;
use crate::expr::Literal;
use crate::lexer::Ident;
use crate::lexer::Symbol;
use crate::lexer::TokenTy;
use crate::lexer::TokenVariant;

use super::error::*;
use super::parse::try_eq_symbol;
use super::parse::try_get_ident;
use super::parse::Parse;
use super::parse::State;
use super::parse::Tokens;
use super::split;

impl<'e> Parse<'e> for Expr<'e> {
    fn parse(tokens: Tokens<'e>, state: &mut State) -> Result<'e, (Tokens<'e>, Self)> {
        if tokens.is_empty() {
            return Err(Error::missing_token(Self::handled()));
        }

        let mut error = Error::multiple(vec![]);

        if let TokenVariant::Literal(lit) = &tokens[0].token {
            let name = Ident::new(format!("lit{}", state.literals));
            state.literals += 1;

            return Ok((split(tokens, 1), Expr::Literal(Literal { name, lit })));
        }

        match Func::parse(tokens, state) {
            Ok((tokens, func)) => return Ok((tokens, Expr::Func(func))),
            Err(err) => {
                error = error.concat(err);
            }
        }

        if let TokenVariant::Ident(var) = &tokens[0].token {
            return Ok((split(tokens, 1), Expr::Var(var)));
        }

        Err(error)
    }
}

impl<'f> Parse<'f> for Func<'f> {
    fn parse(tokens: Tokens<'f>, state: &mut State) -> Result<'f, (Tokens<'f>, Self)> {
        let name = try_get_ident(tokens, 0)?;

        try_eq_symbol(tokens, 1, Symbol::LeftParen)?;

        if tokens.len() < 3 {
            let mut handled = Expr::handled();
            handled.push(TokenTy::Symbol(Symbol::RightParen));

            return Err(Error::missing_token(handled));
        }

        let mut func = Func { name, args: vec![] };

        let mut t = 2;
        loop {
            if t >= tokens.len() {
                let mut handled = Expr::handled();
                handled.push(TokenTy::Symbol(Symbol::RightParen));

                return Err(Error::missing_token(handled));
            }

            if tokens[t].eq_symbol(Symbol::RightParen) {
                t += 1;
                break;
            }

            let (tokens_, expr) = Expr::parse(split(&tokens, t), state)?;
            func.args.push(expr);
            t = tokens.len() - tokens_.len();
        }

        try_eq_symbol(tokens, t, Symbol::SemiColon)?;
        t += 1;

        Ok((split(tokens, t), func))
    }
}
