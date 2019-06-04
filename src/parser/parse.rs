use crate::lexer::Ident;
use crate::lexer::Keyword;
use crate::lexer::Symbol;
use crate::lexer::Token;
use crate::lexer::TokenTy;
use crate::lexer::TokenVariant;

use super::error::*;

pub(super) type Tokens<'t> = &'t [Token<'t>];

pub(super) trait Parse<'t>: Sized {
    fn parse(tokens: Tokens<'t>) -> Result<(Tokens<'t>, Self)>;
}

pub(super) fn try_get_ident<'t>(tokens: Tokens<'t>, at: usize) -> Result<&'t Ident> {
    let token = if let Some(token) = tokens.get(at) {
        token
    } else {
        return Err(Error::missing_token(vec![TokenTy::Ident]));
    };

    if let TokenVariant::Ident(ident) = &token.token {
        Ok(ident)
    } else {
        Err(Error::wrong_token(&token, vec![TokenTy::Ident]))
    }
}

pub(super) fn try_eq_keyword(tokens: Tokens, at: usize, keyword: Keyword) -> Result<()> {
    let token = if let Some(token) = tokens.get(at) {
        token
    } else {
        return Err(Error::missing_token(vec![TokenTy::Keyword(keyword)]));
    };

    if token.eq_keyword(keyword) {
        Ok(())
    } else {
        Err(Error::wrong_token(&token, vec![TokenTy::Keyword(keyword)]))
    }
}

pub(super) fn try_eq_symbol(tokens: Tokens, at: usize, symbol: Symbol) -> Result<()> {
    let token = if let Some(token) = tokens.get(at) {
        token
    } else {
        return Err(Error::missing_token(vec![TokenTy::Symbol(symbol)]));
    };

    if token.eq_symbol(symbol) {
        Ok(())
    } else {
        Err(Error::wrong_token(&token, vec![TokenTy::Symbol(symbol)]))
    }
}
