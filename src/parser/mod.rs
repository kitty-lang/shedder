mod decl;
mod error;
mod expr;
mod module;
mod parse;
mod stmt;

pub use error::*;
pub use module::Module;

use parse::Parse;
use parse::State;
use parse::Tokens;

pub fn parse(tokens: Tokens) -> Result<Module> {
    // --- FIXME ---
    let mut state = State { literals: 0 };
    let (tokens, module) = Module::parse(tokens, &mut state)?;

    if tokens.len() > 1 || (tokens.len() == 1 && !tokens[0].is_eof()) {
        Err(Error::missing_token(Module::handled()))
    } else {
        Ok(module)
    }
    // --- FIXME ---
}

fn split(tokens: Tokens, at: usize) -> Tokens {
    if at >= tokens.len() {
        &[]
    } else {
        &tokens[at..]
    }
}
