use fnv::FnvHashMap;

use crate::decl::Decl;
use crate::decl::Func;
use crate::lexer::Ident;
use crate::lexer::TokenTy;

use super::error::*;
use super::parse::Parse;
use super::parse::State;
use super::parse::Tokens;
use super::split;

pub struct Module<'m> {
    pub funcs: FnvHashMap<&'m Ident, Func<'m>>, // TODO: overloading
}

impl<'m> Module<'m> {
    pub(super) fn handled() -> Vec<TokenTy> {
        Decl::handled()
    }
}

impl<'m> Parse<'m> for Module<'m> {
    fn parse(tokens: Tokens<'m>, state: &mut State) -> Result<'m, (Tokens<'m>, Self)> {
        if tokens.is_empty() {
            return Err(Error::missing_token(Self::handled()));
        }

        let mut module = Module {
            funcs: FnvHashMap::default(),
        };

        let mut t = 0;
        loop {
            if t >= tokens.len() {
                return Err(Error::missing_token(vec![TokenTy::EOF])); // FIXME
            }

            if tokens[t].is_eof() {
                // FIXME
                t += 1;
                break;
            }

            match Decl::parse(split(tokens, t), state)? {
                (tokens_, Decl::Func(func)) => {
                    module.funcs.insert(func.name, func);
                    t = tokens.len() - tokens_.len();
                }
            }
        }

        Ok((split(tokens, t), module))
    }
}

// TODO: impl<'m> Display for Module<'m> { ... }
