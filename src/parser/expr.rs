use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use crate::expr::Expr;
use crate::expr::Func;
use crate::expr::Literal;
use crate::lexer::Ident;
use crate::lexer::Symbol;
use crate::lexer::Token;
use crate::lexer::TokenTy;
use crate::lexer::TokenVariant;

use super::split;
use super::try_eq_symbol;
use super::try_get_ident;

use super::error::*;

static LITERALS: AtomicUsize = AtomicUsize::new(0);

impl<'e> Expr<'e> {
    pub(super) fn handled() -> Vec<TokenTy> {
        let mut handled = vec![];
        handled.append(&mut Func::handled());
        handled.push(TokenTy::Literal);
        // handled.push(TokenTy::Ident); // NOTE: already in `Func::handled`
        handled
    }

    pub(super) fn parse(tokens: &'e [Token<'e>]) -> Result<(usize, Self)> {
        if tokens.is_empty() {
            return Err(Error::missing_token(Self::handled(), None));
        }

        let mut error = Error::multiple(vec![]);

        if let TokenVariant::Literal(lit) = &tokens[0].token {
            return Ok((
                1,
                Expr::Literal(Literal {
                    name: Ident::Owned(format!("lit{}", LITERALS.fetch_add(1, Ordering::SeqCst))),
                    lit,
                }),
            ));
        } else {
            error = error.concat(Error::wrong_token(
                &tokens[0],
                vec![TokenTy::Literal],
            ));
        }

        match Func::parse(tokens) {
            Ok((t, func)) => return Ok((t, Expr::Func(func))),
            Err(mut err) => {
                error = error.concat({
                    err.max_after(tokens.get(1).map(|token| token.pos));
                    err
                })
            }
        }

        if let TokenVariant::Ident(var) = &tokens[0].token {
            return Ok((1, Expr::Var(var.clone())));
        } else {
            error = error.concat(Error::wrong_token(
                &tokens[0],
                vec![TokenTy::Ident],
            ));
        }

        Err(error)
    }
}

impl<'f> Func<'f> {
    fn handled() -> Vec<TokenTy> {
        vec![TokenTy::Ident]
    }

    fn parse(tokens: &'f [Token<'f>]) -> Result<(usize, Self)> {
        let name = try_get_ident(tokens, 0)?.clone();

        let mut t = 1;
        try_eq_symbol(tokens, t, Symbol::LeftParen).map_err(|mut err| {
            err.max_after(tokens.get(t - 1).map(|token| token.pos));
            err
        })?;

        let mut func = Func { name, args: vec![] };

        t += 1;
        loop {
            if t >= tokens.len() {
                let mut handled = Expr::handled();
                handled.push(TokenTy::Symbol(Symbol::RightParen));
                return Err(Error::missing_token(handled, Some(tokens[t - 1].pos)));
            }

            if tokens[t].eq_symbol(Symbol::RightParen) {
                t += 1;
                break;
            }

            if !func.args.is_empty() {
                try_eq_symbol(tokens, t, Symbol::Comma).map_err(|mut err| {
                    err.max_after(tokens.get(t - 1).map(|token| token.pos));
                    err
                })?;
            }

            let (t_, expr) = Expr::parse(split(tokens, t))?;

            func.args.push(expr);
            t += t_;
        }

        Ok((t, func))
    }
}
