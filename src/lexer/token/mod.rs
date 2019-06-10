use std::cmp::Ordering;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::ty::Ty;

use super::error;
use super::error::*;
use super::split;

mod comment;
mod ident;
mod keyword;
mod literal;
mod symbol;
mod ty;

pub use ident::Ident;
pub use keyword::Keyword;
pub use literal::Literal;
pub use symbol::Symbol;

use comment::Comment;

#[derive(Debug)]
pub struct Token<'t> {
    pub token: TokenVariant<'t>,
    pub pos: Position,
}

#[derive(Debug)]
pub enum TokenVariant<'t> {
    Keyword(Keyword),
    Ty(Ty<'t>),
    Ident(Ident<'t>),
    Literal(Literal<'t>),
    Symbol(Symbol),
    EOF,
}

#[derive(Default, Eq, PartialEq, Copy, Clone, Debug)]
pub struct Position {
    pub line: usize,
    pub col: usize,
}

#[derive(Eq, PartialEq, Debug)]
pub enum TokenTy {
    Keyword(Keyword),
    Ty,
    Ident,
    Literal,
    Symbol(Symbol),
    EOF,
}

impl<'t> Token<'t> {
    pub(super) fn lex(input: &'t str, pos: &mut Position) -> Result<(&'t str, Option<Token<'t>>)> {
        if input.is_empty() {
            return Ok((
                input,
                Some(Token {
                    token: TokenVariant::EOF,
                    pos: *pos,
                }),
            ));
        }

        if let Ok(input) = Comment::lex(input, pos) {
            return Ok((input, None));
        }

        if let Ok((input, token)) = Keyword::lex(input, pos) {
            return Ok((input, Some(token)));
        }

        if let Ok((input, token)) = Ty::lex(input, pos) {
            return Ok((input, token));
        }

        if let Ok((input, token)) = Ident::lex(input, pos) {
            return Ok((input, Some(token)));
        }

        if let Ok((input, token)) = Literal::lex(input, pos) {
            return Ok((input, Some(token)));
        }

        if let Ok((input, token)) = Symbol::lex(input, pos) {
            return Ok((input, Some(token)));
        }

        Err(Error::not_handled(*pos))
    }

    pub fn is_eof(&self) -> bool {
        if let TokenVariant::EOF = self.token {
            true
        } else {
            false
        }
    }

    pub fn eq_keyword(&self, keyword: Keyword) -> bool {
        if let TokenVariant::Keyword(keyword_) = self.token {
            keyword_ == keyword
        } else {
            false
        }
    }

    pub fn eq_symbol(&self, symbol: Symbol) -> bool {
        if let TokenVariant::Symbol(symbol_) = self.token {
            symbol_ == symbol
        } else {
            false
        }
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Position) -> Ordering {
        match (self.line.cmp(&other.line), self.col.cmp(&other.col)) {
            (Ordering::Equal, cmp) => cmp,
            (cmp, _) => cmp,
        }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Position) -> Option<Ordering> {
        match (
            self.line.partial_cmp(&other.line),
            self.col.partial_cmp(&other.col),
        ) {
            (Some(Ordering::Equal), cmp) => cmp,
            (Some(cmp), _) => Some(cmp),
            (_, Some(cmp)) => Some(cmp),
            _ => None,
        }
    }
}

impl<'t> Display for Token<'t> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{}{}", self.token, self.pos)
    }
}

impl<'t> Display for TokenVariant<'t> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            TokenVariant::Keyword(keyword) => write!(fmt, "{}", keyword),
            TokenVariant::Ty(ty) => write!(fmt, "{}", ty),
            TokenVariant::Ident(ident) => write!(fmt, "{}", ident),
            TokenVariant::Literal(literal) => write!(fmt, "{}", literal),
            TokenVariant::Symbol(symbol) => write!(fmt, "{}", symbol),
            TokenVariant::EOF => write!(fmt, "eof"),
        }
    }
}

impl Display for Position {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{{{},{}}}", self.line, self.col)
    }
}

impl Display for TokenTy {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            TokenTy::Keyword(keyword) => write!(fmt, "{}", keyword),
            TokenTy::Ty => write!(fmt, "ty"),
            TokenTy::Ident => write!(fmt, "ident"),
            TokenTy::Literal => write!(fmt, "lit"),
            TokenTy::Symbol(symbol) => write!(fmt, "{}", symbol),
            TokenTy::EOF => write!(fmt, "eof"),
        }
    }
}
