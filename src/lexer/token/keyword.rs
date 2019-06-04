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
}

impl Keyword {
    pub fn len(self) -> usize {
        match self {
            Keyword::Func => 4,
        }
    }
}

impl<'l> Lex<'l> for Keyword {
    fn try_lex(input: &'l str) -> IResult<&'l str, Keyword> {
        if let (input, true) = is_tag(input, "func")? {
            return Ok((input, Keyword::Func));
        }

        Err(NomErr::Incomplete(Needed::Unknown))
    }
}

impl Display for Keyword {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "keyword:")?;
        match self {
            Keyword::Func => write!(fmt, "func"),
        }
    }
}
