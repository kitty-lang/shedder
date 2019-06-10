use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use super::error::*;
use super::split;
use super::Position;
use super::Token;
use super::TokenVariant;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Keyword {
    Func,
    Let,
    Return,
}

impl Keyword {
    pub(super) fn lex<'i>(input: &'i str, pos: &mut Position) -> Result<(&'i str, Token<'i>)> {
        let tpos = *pos;
        if input.starts_with("func") {
            pos.col += 4;

            Ok((split(input, 4), Keyword::Func.token(tpos)))
        } else if input.starts_with("let") {
            pos.col += 3;

            Ok((split(input, 3), Keyword::Let.token(tpos)))
        } else if input.starts_with("return") {
            pos.col += 6;

            Ok((split(input, 6), Keyword::Return.token(tpos)))
        } else {
            Err(Error::not_handled(tpos))
        }
    }

    fn token<'t>(self, pos: Position) -> Token<'t> {
        Token {
            token: TokenVariant::Keyword(self),
            pos,
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Keyword::Func => write!(fmt, "keyword::func"),
            Keyword::Let => write!(fmt, "keyword::let"),
            Keyword::Return => write!(fmt, "keyword::return"),
        }
    }
}
