use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use nom::bytes::complete::take_till;
use nom::character::complete::char;
use nom::sequence::tuple;
use nom::Err as NomErr;
use nom::IResult;
use nom::Needed;

use super::lex::Lex;

#[derive(Eq, PartialEq, Debug)]
pub enum Literal<'l> {
    String(&'l str),
}

impl<'l> Literal<'l> {
    pub fn len(&self) -> usize {
        match self {
            Literal::String(string) => string.len() + 2,
        }
    }
}

impl<'l> Lex<'l> for Literal<'l> {
    fn try_lex(input: &'l str) -> IResult<&'l str, Literal<'l>> {
        match tuple((char('"'), take_till(|c| c == '"'), char('"')))(input) {
            Ok((input, (_, string, _))) => return Ok((input, Literal::String(string))),
            Err(NomErr::Failure(err)) => return Err(NomErr::Failure(err)),
            _ => (),
        }

        Err(NomErr::Incomplete(Needed::Unknown))
    }
}

impl<'l> Display for Literal<'l> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Literal::String(string) => write!(fmt, "string:{:?}", string),
        }
    }
}
