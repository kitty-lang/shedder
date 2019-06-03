use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Token {
    Keyword(Keyword),
    Ident(Ident),
    Symbol(Symbol),
    EOF,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum TokenVariant {
    Keyword(Keyword),
    Ident,
    Symbol(Symbol),
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Symbol {
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Keyword {
    Func,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Ident(pub String);

impl Display for Token {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Token::Keyword(keyword) => write!(fmt, "{}", keyword),
            Token::Ident(ident) => write!(fmt, "{}", ident),
            Token::Symbol(symbol) => write!(fmt, "{}", symbol),
            Token::EOF => write!(fmt, "eof"),
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Symbol::LeftParen => write!(fmt, "("),
            Symbol::RightParen => write!(fmt, ")"),
            Symbol::LeftBracket => write!(fmt, "{{"),
            Symbol::RightBracket => write!(fmt, "}}"),
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Keyword::Func => write!(fmt, "func"),
        }
    }
}

impl Display for Ident {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{:?}", self.0)
    }
}
