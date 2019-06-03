use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::lexer::Ident;
use crate::lexer::Keyword;
use crate::lexer::Symbol;
use crate::lexer::Token;
use crate::lexer::TokenVariant;

use super::error::*;
use super::parse::Parse;

#[derive(Debug)]
pub enum Decl<'d> {
    Func(Func<'d>),
}

#[derive(Debug)]
pub struct Func<'f> {
    pub name: &'f Ident,
    // TODO: params
    // TODO: ret
    // TODO: exprs
}

impl<'d> Parse<'d> for Decl<'d> {
    fn parse(tokens: &'d [Token]) -> Result<(&'d [Token], Self)> {
        if tokens[0].is_keyword(Keyword::Func) {
            let (tokens, func) = Func::parse(&tokens[1..])?;

            Ok((tokens, Decl::Func(func)))
        } else {
            Err(Error::wrong_token(
                &tokens[0],
                vec![TokenVariant::Keyword(Keyword::Func)],
            ))
        }
    }
}

impl<'f> Parse<'f> for Func<'f> {
    fn parse(tokens: &'f [Token]) -> Result<(&'f [Token], Self)> {
        let name = tokens[0].ident()?;

        tokens[1].symbol(Symbol::LeftParen)?;
        tokens[2].symbol(Symbol::RightParen)?;
        tokens[3].symbol(Symbol::LeftBracket)?;
        tokens[4].symbol(Symbol::RightBracket)?;

        Ok((&tokens[5..], Func { name }))
    }
}

impl<'d> Display for Decl<'d> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "decl ")?;
        match self {
            Decl::Func(func) => writeln!(fmt, "{}", func),
        }
    }
}

impl<'f> Display for Func<'f> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "func ")?;
        write!(fmt, "{}", self.name)
    }
}
