use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use super::error::*;
use super::split;
use super::Position;
use super::Token;
use super::TokenVariant;

#[derive(Eq, PartialEq, Debug)]
pub enum Literal<'l> {
    String(&'l str),
}

impl<'l> Literal<'l> {
    pub(super) fn lex(input: &'l str, pos: &mut Position) -> Result<(&'l str, Token<'l>)> {
        match (input.get(0..1), input.get(1..)) {
            (Some(_), None) => Err(Error::not_handled(*pos)),
            (Some(r#"""#), Some(rest)) => {
                let tpos = *pos;

                let mut i = 0;
                for chr in rest.chars() {
                    match chr {
                        '"' => {
                            pos.col += 1;
                            return Ok((
                                split(rest, i + 1),
                                Literal::String(&rest[0..i]).token(tpos),
                            ));
                        }
                        '\n' => {
                            i += 1;
                            pos.line += 1;
                            pos.col = 0;
                        }
                        _ => {
                            i += 1;
                            pos.col += 1;
                        }
                    }
                }

                let epos = *pos;
                *pos = tpos;

                Err(Error::not_handled(epos))
            }
            _ => Err(Error::not_handled(*pos)),
        }
    }

    fn token(self, pos: Position) -> Token<'l> {
        Token {
            token: TokenVariant::Literal(self),
            pos,
        }
    }
}

impl<'l> Display for Literal<'l> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Literal::String(string) => write!(fmt, "lit::string({:?})", string),
        }
    }
}
