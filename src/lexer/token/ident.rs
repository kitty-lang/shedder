use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::character::complete::alpha1;
use nom::character::complete::alphanumeric1;
use nom::multi::many0;
use nom::sequence::pair;
use nom::IResult;

use super::lex::Lex;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Ident(String);

impl Ident {
    pub fn inner(&self) -> &str {
        &self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'l> Lex<'l> for Ident {
    fn try_lex(input: &'l str) -> IResult<&'l str, Ident> {
        match pair(
            alt((alpha1, is_a("_"))),
            many0(alt((alphanumeric1, is_a("_")))),
        )(input)
        {
            Ok((input, (first, next))) => {
                let mut ident = first.to_string();
                for next in next {
                    ident.push_str(next);
                }
                Ok((input, Ident(ident)))
            }
            Err(err) => Err(err),
        }
    }
}

impl Display for Ident {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "ident:{:?}", self.inner())
    }
}
