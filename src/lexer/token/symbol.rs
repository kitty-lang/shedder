use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use nom::Err as NomErr;
use nom::IResult;
use nom::Needed;

use super::lex::is_tag;
use super::lex::Lex;

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
    pub fn len(self) -> usize {
        match self {
            Symbol::Equal => 1,
            Symbol::LeftParen => 1,
            Symbol::RightParen => 1,
            Symbol::LeftBracket => 1,
            Symbol::RightBracket => 1,
            Symbol::SemiColon => 1,
        }
    }
}

impl<'l> Lex<'l> for Symbol {
    fn try_lex(input: &'l str) -> IResult<&'l str, Symbol> {
        if let (input, true) = is_tag(input, "=")? {
            return Ok((input, Symbol::Equal));
        }

        if let (input, true) = is_tag(input, "(")? {
            return Ok((input, Symbol::LeftParen));
        }

        if let (input, true) = is_tag(input, ")")? {
            return Ok((input, Symbol::RightParen));
        }

        if let (input, true) = is_tag(input, "{")? {
            return Ok((input, Symbol::LeftBracket));
        }

        if let (input, true) = is_tag(input, "}")? {
            return Ok((input, Symbol::RightBracket));
        }

        if let (input, true) = is_tag(input, ";")? {
            return Ok((input, Symbol::SemiColon));
        }

        Err(NomErr::Incomplete(Needed::Unknown))
    }
}

impl Display for Symbol {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "symbol:")?;
        match self {
            Symbol::Equal => write!(fmt, "="),
            Symbol::LeftParen => write!(fmt, "("),
            Symbol::RightParen => write!(fmt, ")"),
            Symbol::LeftBracket => write!(fmt, "{{"),
            Symbol::RightBracket => write!(fmt, "}}"),
            Symbol::SemiColon => write!(fmt, ";"),
        }
    }
}
