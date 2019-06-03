use crate::lexer::*;

use super::error::*;

pub trait Parse<'p>: Sized {
    fn parse(tokens: &'p [Token]) -> Result<(&'p [Token], Self)>;
}

impl Token {
    pub fn is_keyword(&self, keyword: Keyword) -> bool {
        if let Token::Keyword(keyword_) = self {
            keyword_ == &keyword
        } else {
            false
        }
    }

    pub fn ident(&self) -> Result<&Ident> {
        match self {
            Token::Ident(ident) => Ok(ident),
            _ => Err(Error::wrong_token(self, vec![TokenVariant::Ident])),
        }
    }

    pub fn symbol(&self, symbol: Symbol) -> Result<()> {
        match self {
            Token::Symbol(_) => Ok(()),
            _ => Err(Error::wrong_token(self, vec![TokenVariant::Symbol(symbol)])),
        }
    }
}
