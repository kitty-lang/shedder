use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use nom::Err as NomErr;
use nom::IResult;
use nom::Needed;

use super::lex::is_tag;
use super::lex::Lex;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Keyword {
    Func,
    Let,
}

impl Keyword {
    pub fn len(self) -> usize {
        match self {
            Keyword::Func => 4,
            Keyword::Let => 3,
        }
    }
}

impl<'l> Lex<'l> for Keyword {
    fn try_lex(input: &'l str) -> IResult<&'l str, Keyword> {
        if let (input, true) = is_tag(input, "func")? {
            return Ok((input, Keyword::Func));
        }

        if let (input, true) = is_tag(input, "let")? {
            return Ok((input, Keyword::Let));
        }

        Err(NomErr::Incomplete(Needed::Unknown))
    }
}

impl Display for Keyword {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "keyword:")?;
        match self {
            Keyword::Func => write!(fmt, "func"),
            Keyword::Let => write!(fmt, "let"),
        }
    }
}
