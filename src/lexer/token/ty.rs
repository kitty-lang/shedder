use crate::ty::Ty;

use super::error::*;
use super::split;
use super::Position;
use super::Token;
use super::TokenVariant;

impl<'t> Ty<'t> {
    pub(super) fn lex(input: &'t str, pos: &mut Position) -> Result<(&'t str, Token<'t>)> {
        let tpos = *pos;
        if input.starts_with("str") {
            pos.col += 3;

            Ok((split(input, 3), Ty::Str.token(tpos)))
        } else if input.starts_with("void") {
            pos.col += 4;

            Ok((split(input, 4), Ty::Void.token(tpos)))
        } else {
            Err(Error::not_handled(tpos))
        }
    }

    fn token(self, pos: Position) -> Token<'t> {
        Token {
            token: TokenVariant::Ty(self),
            pos,
        }
    }
}
