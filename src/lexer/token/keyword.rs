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
}

impl Keyword {
    pub(super) fn lex<'i>(input: &'i str, pos: &mut Position) -> Result<(&'i str, Token<'i>)> {
        if input.starts_with("func") {
            Ok((split(input, 4), Keyword::Func.token(pos)))
        } else if input.starts_with("let") {
            Ok((split(input, 3), Keyword::Let.token(pos)))
        } else {
            Err(Error::not_handled())
        }
    }

    fn token<'t>(self, pos: &mut Position) -> Token<'t> {
        let tpos = *pos;

        match self {
            Keyword::Func => pos.col += 4,
            Keyword::Let => pos.col += 3,
        }

        Token {
            token: TokenVariant::Keyword(self),
            pos: tpos,
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Keyword::Func => write!(fmt, "keyword::func"),
            Keyword::Let => write!(fmt, "keyword::let"),
        }
    }
}
