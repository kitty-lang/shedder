use crate::expr::Expr;
use crate::lexer::TokenTy;
use crate::stmt::Stmt;

use super::error::*;
use super::parse::Parse;
use super::parse::Tokens;

impl<'s> Stmt<'s> {
    fn handled() -> Vec<TokenTy> {
        let mut handled = vec![];
        handled.append(&mut Expr::handled());
        handled
    }
}

impl<'s> Parse<'s> for Stmt<'s> {
    fn parse(tokens: Tokens<'s>) -> Result<(Tokens<'s>, Self)> {
        if tokens.is_empty() {
            return Err(Error::missing_token(Self::handled()));
        }

        let mut error = Error::multiple(vec![]);

        match Expr::parse(tokens) {
            Ok((tokens, expr)) => return Ok((tokens, Stmt::Expr(expr))),
            Err(err) => {
                error = error.concat(err);
            }
        }

        Err(error)
    }
}
