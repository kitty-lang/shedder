use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;

use super::error::*;
use super::split;
use super::Position;
use super::Token;
use super::TokenVariant;

#[derive(Clone, Debug)]
pub enum Ident<'i> {
    Ref(&'i str),
    Owned(String),
}

impl<'i> Ident<'i> {
    pub fn inner(&self) -> &str {
        match self {
            Ident::Ref(ident) => ident,
            Ident::Owned(ident) => ident,
        }
    }

    pub fn as_ref(&'i self) -> Ident<'i> {
        match self {
            Ident::Ref(ident) => Ident::Ref(ident),
            Ident::Owned(ident) => Ident::Ref(ident),
        }
    }

    pub(super) fn lex(input: &'i str, pos: &mut Position) -> Result<(&'i str, Token<'i>)> {
        let mut i = 0;
        for chr in input.chars() {
            if chr.is_alphabetic() || chr == '_' || (chr.is_alphanumeric() && i > 0) {
                i += 1;
            } else {
                break;
            }
        }

        if i > 0 {
            let tpos = *pos;
            pos.col += i;

            Ok((
                split(input, i),
                Ident::Ref(input.get(0..i).unwrap()).token(tpos),
            ))
        } else {
            Err(Error::not_handled(*pos))
        }
    }

    fn token(self, pos: Position) -> Token<'i> {
        Token {
            token: TokenVariant::Ident(self),
            pos,
        }
    }
}

impl<'i> Hash for Ident<'i> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner().hash(state);
    }
}

impl<'i> Eq for Ident<'i> {}

impl<'i> PartialEq for Ident<'i> {
    fn eq(&self, other: &Ident<'i>) -> bool {
        self.inner() == other.inner()
    }
}

impl<'i> Display for Ident<'i> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Ident::Ref(ident) => write!(fmt, "ident({})", ident),
            Ident::Owned(ident) => write!(fmt, "ident({})", ident),
        }
    }
}
