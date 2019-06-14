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
    LeftBrace,
    RightBrace,
    Comma,
    Colon,
    SemiColon,
}

impl Symbol {
    pub(super) fn lex<'i>(input: &'i str, pos: &mut Position) -> Result<(&'i str, Token<'i>)> {
        let tpos = *pos;
        if input.starts_with('=') {
            pos.col += 1;

            Ok((split(input, 1), Symbol::Equal.token(tpos)))
        } else if input.starts_with('(') {
            pos.col += 1;

            Ok((split(input, 1), Symbol::LeftParen.token(tpos)))
        } else if input.starts_with(')') {
            pos.col += 1;

            Ok((split(input, 1), Symbol::RightParen.token(tpos)))
        } else if input.starts_with('{') {
            pos.col += 1;

            Ok((split(input, 1), Symbol::LeftBrace.token(tpos)))
        } else if input.starts_with('}') {
            pos.col += 1;

            Ok((split(input, 1), Symbol::RightBrace.token(tpos)))
        } else if input.starts_with(',') {
            pos.col += 1;

            Ok((split(input, 1), Symbol::Comma.token(tpos)))
        } else if input.starts_with(':') {
            pos.col += 1;

            Ok((split(input, 1), Symbol::Colon.token(tpos)))
        } else if input.starts_with(';') {
            pos.col += 1;

            Ok((split(input, 1), Symbol::SemiColon.token(tpos)))
        } else {
            Err(Error::not_handled(tpos))
        }
    }

    fn token<'t>(self, pos: Position) -> Token<'t> {
        Token {
            token: TokenVariant::Symbol(self),
            pos,
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Symbol::Equal => write!(fmt, r#"symbol("=")"#),
            Symbol::LeftParen => write!(fmt, r#"symbol("(")"#),
            Symbol::RightParen => write!(fmt, r#"symbol(")")"#),
            Symbol::LeftBrace => write!(fmt, r#"symbol("{{")"#),
            Symbol::RightBrace => write!(fmt, r#"symbol("}}")"#),
            Symbol::Comma => write!(fmt, r#"symbol(",")"#),
            Symbol::Colon => write!(fmt, r#"symbol(":")"#),
            Symbol::SemiColon => write!(fmt, r#"symbol(";")"#),
        }
    }
}
