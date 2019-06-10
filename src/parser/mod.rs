use crate::lexer::Ident;
use crate::lexer::Keyword;
use crate::lexer::Symbol;
use crate::lexer::Token;
use crate::lexer::TokenTy;
use crate::lexer::TokenVariant;
use crate::ty::Ty;

mod decl;
mod error;
mod expr;
mod module;
mod stmt;

pub use error::*;
pub use module::Module;

pub fn parse<'t>(tokens: &'t [Token<'t>]) -> Result<Module> {
    let mut module = Module::new(Ident::Owned("main".into())); // FIXME
    let t = module.parse(tokens)?;

    if t < tokens.len() - 1 || (t < tokens.len() && !tokens[t].is_eof()) {
        Err(Error::missing_token(Module::handled(), Some(tokens[t].pos)))
    } else {
        Ok(module)
    }
}

fn split<'t>(tokens: &'t [Token<'t>], at: usize) -> &'t [Token<'t>] {
    if at >= tokens.len() {
        &[]
    } else {
        &tokens[at..]
    }
}

fn try_get_ty<'t>(tokens: &'t [Token<'t>], at: usize) -> Result<Ty<'t>> {
    match tokens.get(at).map(|token| (token, &token.token)) {
        Some((_, TokenVariant::Ty(ty))) => Ok(ty.clone()),
        Some((_, TokenVariant::Ident(ident))) => Ok(Ty::User(ident.clone())),
        Some((token, _)) => Err(Error::wrong_token(token, vec![TokenTy::Ty, TokenTy::Ident])),
        None => Err(Error::missing_token(vec![TokenTy::Ty, TokenTy::Ident], None)),
    }
}

fn try_get_ident<'t>(tokens: &'t [Token<'t>], at: usize) -> Result<&'t Ident<'t>> {
    match tokens.get(at).map(|token| (token, &token.token)) {
        Some((_, TokenVariant::Ident(ident))) => Ok(ident),
        Some((token, _)) => Err(Error::wrong_token(token, vec![TokenTy::Ident])),
        None => Err(Error::missing_token(vec![TokenTy::Ident], None)),
    }
}

fn try_eq_keyword<'t>(tokens: &'t [Token<'t>], at: usize, keyword: Keyword) -> Result<'t, ()> {
    match tokens.get(at) {
        Some(token) if token.eq_keyword(keyword) => Ok(()),
        Some(token) => Err(Error::wrong_token(token, vec![TokenTy::Keyword(keyword)])),
        None => Err(Error::missing_token(vec![TokenTy::Keyword(keyword)], None)),
    }
}

fn try_eq_symbol<'t>(tokens: &'t [Token<'t>], at: usize, symbol: Symbol) -> Result<'t, ()> {
    match tokens.get(at) {
        Some(token) if token.eq_symbol(symbol) => Ok(()),
        Some(token) => Err(Error::wrong_token(token, vec![TokenTy::Symbol(symbol)])),
        None => Err(Error::missing_token(vec![TokenTy::Symbol(symbol)], None)),
    }
}
