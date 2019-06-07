use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use nom::Err as NomErr;
use nom::IResult;
use nom::Needed;

use super::lex;
use super::lex::Lex;

mod ident;
mod keyword;
mod literal;
mod symbol;

pub use ident::Ident;
pub use keyword::Keyword;
pub use literal::Literal;
pub use symbol::Symbol;

#[derive(Eq, PartialEq, Debug)]
pub struct Token<'t> {
    pub token: TokenVariant<'t>,
    pub pos: Position,
}

#[derive(Eq, PartialEq, Debug)]
pub enum TokenVariant<'t> {
    Keyword(Keyword),
    Ident(Ident),
    Literal(Literal<'t>),
    Symbol(Symbol),
    EOF,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum TokenTy {
    Keyword(Keyword),
    Ident,
    #[allow(unused)] // FIXME
    Literal,
    Symbol(Symbol),
    EOF,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Position {
    pub line: usize,
    pub col: usize,
}

impl<'t> Token<'t> {
    fn keyword(keyword: Keyword, pos: Position) -> Self {
        Token {
            token: TokenVariant::Keyword(keyword),
            pos,
        }
    }

    fn ident(ident: Ident, pos: Position) -> Self {
        Token {
            token: TokenVariant::Ident(ident),
            pos,
        }
    }

    fn literal(lit: Literal<'t>, pos: Position) -> Self {
        Token {
            token: TokenVariant::Literal(lit),
            pos,
        }
    }

    fn symbol(symbol: Symbol, pos: Position) -> Self {
        Token {
            token: TokenVariant::Symbol(symbol),
            pos,
        }
    }

    fn eof(pos: Position) -> Self {
        Token {
            token: TokenVariant::EOF,
            pos,
        }
    }

    pub fn len(&self) -> usize {
        match &self.token {
            TokenVariant::Keyword(keyword) => keyword.len(),
            TokenVariant::Ident(ident) => ident.len(),
            TokenVariant::Literal(lit) => lit.len(),
            TokenVariant::Symbol(symbol) => symbol.len(),
            TokenVariant::EOF => 0,
        }
    }

    pub fn is_eof(&self) -> bool {
        self.token == TokenVariant::EOF
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

    pub(super) fn lex(input: &'t str, pos: &mut Position) -> IResult<&'t str, Self> {
        if input.is_empty() {
            return Ok((input, Token::eof(*pos)));
        }

        match Symbol::try_lex(input) {
            Ok((input, symbol)) => {
                let token = Token::symbol(symbol, *pos);
                pos.col += token.len();
                return Ok((input, token));
            }
            Err(NomErr::Failure(err)) => return Err(NomErr::Failure(err)),
            _ => (),
        }

        match Keyword::try_lex(input) {
            Ok((input, keyword)) => {
                let token = Token::keyword(keyword, *pos);
                pos.col += token.len();
                return Ok((input, token));
            }
            Err(NomErr::Failure(err)) => return Err(NomErr::Failure(err)),
            _ => (),
        }

        match Ident::try_lex(input) {
            Ok((input, ident)) => {
                let token = Token::ident(ident, *pos);
                pos.col += token.len();
                return Ok((input, token));
            }
            Err(NomErr::Failure(err)) => return Err(NomErr::Failure(err)),
            _ => (),
        }

        match Literal::try_lex(input) {
            Ok((input, literal)) => {
                let token = Token::literal(literal, *pos);
                pos.col += token.len();
                return Ok((input, token));
            }
            Err(NomErr::Failure(err)) => return Err(NomErr::Failure(err)),
            _ => (),
        }

        Err(NomErr::Incomplete(Needed::Unknown))
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
            TokenVariant::Ident(ident) => write!(fmt, "{}", ident),
            TokenVariant::Literal(literal) => write!(fmt, "{}", literal),
            TokenVariant::Symbol(symbol) => write!(fmt, "{}", symbol),
            TokenVariant::EOF => write!(fmt, "eof"),
        }
    }
}

impl Display for TokenTy {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            TokenTy::Keyword(keyword) => write!(fmt, "{}", keyword),
            TokenTy::Ident => write!(fmt, "Ident"),
            TokenTy::Literal => write!(fmt, "Literal"),
            TokenTy::Symbol(symbol) => write!(fmt, "{}", symbol),
            TokenTy::EOF => write!(fmt, "eof"),
        }
    }
}

impl Display for Position {
    fn fmt(&self, _: &mut Formatter) -> fmt::Result {
        // write!(fmt, "({},{})", self.line, self.col)
        Ok(()) // FIXME: optional
    }
}
