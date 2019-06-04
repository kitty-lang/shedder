use crate::decl::Decl;
use crate::decl::Func;
use crate::lexer::Keyword;
use crate::lexer::Symbol;
use crate::lexer::TokenTy;
use crate::stmt::Stmt;

use super::error::*;
use super::parse::try_eq_keyword;
use super::parse::try_eq_symbol;
use super::parse::try_get_ident;
use super::parse::Parse;
use super::parse::Tokens;
use super::split;
use super::Entry;

impl<'d> Decl<'d> {
    pub(super) fn insert(self, entry: &mut Entry<'d>) {
        match &self {
            Decl::Func(Func { name, .. }) => {
                if let Some(func) = entry.funcs.get_mut(name) {
                    func.push(entry.decls.len());
                } else {
                    entry.funcs.insert(name, vec![entry.decls.len()]);
                }

                entry.decls.push(self);
            }
        }
    }
}

impl<'d> Parse<'d> for Decl<'d> {
    fn parse(tokens: Tokens<'d>) -> Result<(Tokens<'d>, Self)> {
        if tokens.is_empty() {
            return Err(Error::missing_token(Self::handled()));
        }

        let mut error = Error::multiple(vec![]);

        match Func::parse(tokens) {
            Ok((tokens, func)) => return Ok((tokens, Decl::Func(func))),
            Err(err) => {
                error = error.concat(err);
            }
        }

        Err(error)
    }
}

impl<'f> Parse<'f> for Func<'f> {
    fn parse(tokens: Tokens<'f>) -> Result<(Tokens<'f>, Self)> {
        try_eq_keyword(tokens, 0, Keyword::Func)?;

        let name = try_get_ident(tokens, 1)?;

        try_eq_symbol(tokens, 2, Symbol::LeftParen)?;

        // TODO

        try_eq_symbol(tokens, 3, Symbol::RightParen)?;

        // TODO

        try_eq_symbol(tokens, 4, Symbol::LeftBracket)?;

        let mut stmts = vec![];
        let mut t = 5;
        loop {
            if t >= tokens.len() {
                return Err(Error::missing_token(vec![TokenTy::Symbol(
                    Symbol::RightBracket,
                )]));
            }

            if tokens[t].eq_symbol(Symbol::RightBracket) {
                t += 1;
                break;
            }

            match Stmt::parse(split(&tokens, t)) {
                Ok((tokens_, stmt)) => {
                    stmts.push(stmt);
                    t = tokens.len() - tokens_.len();
                }
                Err(err) => return Err(err),
            }
        }

        Ok((split(tokens, t), Func { name, stmts }))
    }
}
