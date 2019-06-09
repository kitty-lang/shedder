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
            (Some(_), None) => Err(Error::not_handled()),
            (Some(r#"""#), Some(rest)) => {
                let tpos = *pos;

                let mut i = 0;
                for chr in rest.chars() {
                    match chr {
                        '"' => {
                            pos.col += 1;
                            return Ok((
                                split(rest, i + 1),
                                Token {
                                    token: TokenVariant::Literal(Literal::String(
                                        rest.get(0..i).unwrap(),
                                    )),
                                    pos: tpos,
                                },
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

                Err(Error::not_handled())
            }
            _ => Err(Error::not_handled()),
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
