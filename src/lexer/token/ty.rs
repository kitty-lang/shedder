use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use super::error::*;
use super::split;
use super::Position;
use super::Token;
use super::TokenVariant;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Ty {
    I32,
    Str,
    Void,
}

impl<'t> Ty {
    pub(super) fn lex(input: &'t str, pos: &mut Position) -> Result<(&'t str, Token<'t>)> {
        let mut chars = input.chars();
        let tpos = *pos;
        if input.starts_with("i32") {
            let next = chars.nth(3).unwrap(); // FIXME
            if next.is_alphanumeric() || next == '_' {
                return Err(Error::not_handled(tpos));
            }

            pos.col += 3;

            Ok((split(input, 3), Ty::I32.token(tpos)))
        } else if input.starts_with("str") {
            let next = chars.nth(3).unwrap(); // FIXME
            if next.is_alphanumeric() || next == '_' {
                return Err(Error::not_handled(tpos));
            }

            pos.col += 3;

            Ok((split(input, 3), Ty::Str.token(tpos)))
        } else if input.starts_with("void") {
            let next = chars.nth(4).unwrap(); // FIXME
            if next.is_alphanumeric() || next == '_' {
                return Err(Error::not_handled(tpos));
            }

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

impl Display for Ty {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "ty::")?;
        match self {
            Ty::I32 => write!(fmt, "i32"),
            Ty::Str => write!(fmt, "str"),
            Ty::Void => write!(fmt, "void"),
        }
    }
}
