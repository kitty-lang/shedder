use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use super::error::*;
use super::split;
use super::Position;
use super::Token;
use super::TokenVariant;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Symbol {
    Equal,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    SemiColon,
}

impl Symbol {
    pub(super) fn lex<'i>(input: &'i str, pos: &mut Position) -> Result<(&'i str, Token<'i>)> {
        if input.starts_with('=') {
            Ok((split(input, 1), Symbol::Equal.token(pos)))
        } else if input.starts_with('(') {
            Ok((split(input, 1), Symbol::LeftParen.token(pos)))
        } else if input.starts_with(')') {
            Ok((split(input, 1), Symbol::RightParen.token(pos)))
        } else if input.starts_with('{') {
            Ok((split(input, 1), Symbol::LeftBracket.token(pos)))
        } else if input.starts_with('}') {
            Ok((split(input, 1), Symbol::RightBracket.token(pos)))
        } else if input.starts_with(';') {
            Ok((split(input, 1), Symbol::SemiColon.token(pos)))
        } else {
            Err(Error::not_handled())
        }
    }

    fn token<'t>(self, pos: &mut Position) -> Token<'t> {
        let tpos = *pos;

        match self {
            Symbol::Equal => pos.col += 1,
            Symbol::LeftParen => pos.col += 1,
            Symbol::RightParen => pos.col += 1,
            Symbol::LeftBracket => pos.col += 1,
            Symbol::RightBracket => pos.col += 1,
            Symbol::SemiColon => pos.col += 1,
        }

        Token {
            token: TokenVariant::Symbol(self),
            pos: tpos,
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Symbol::Equal => write!(fmt, r#"symbol("=")"#),
            Symbol::LeftParen => write!(fmt, r#"symbol("(")"#),
            Symbol::RightParen => write!(fmt, r#"symbol(")")"#),
            Symbol::LeftBracket => write!(fmt, r#"symbol("{{")"#),
            Symbol::RightBracket => write!(fmt, r#"symbol("}}")"#),
            Symbol::SemiColon => write!(fmt, r#"symbol(";")"#),
        }
    }
}
