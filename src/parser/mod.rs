use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use fnv::FnvHashMap;

use crate::decl::Decl;
use crate::lexer::Ident;
use crate::lexer::Token;

mod decl;
mod error;
mod expr;
mod parse;
mod stmt;

pub use error::*;

use parse::Parse;

pub struct Entry<'e> {
    pub funcs: FnvHashMap<&'e Ident, Vec<usize>>, // FIXME: duplicates?
    pub decls: Vec<Decl<'e>>,
}

pub fn parse<'t>(tokens: &'t [Token<'t>]) -> Result<Entry<'t>> {
    let mut entry = Entry {
        funcs: FnvHashMap::default(),
        decls: vec![],
    };

    let mut t = 0;

    loop {
        if tokens.len() > t && tokens[t].is_eof() {
            return Ok(entry);
        }

        let mut error = Error::multiple(vec![]);

        match Decl::parse(split(tokens, t)) {
            Ok((_tokens, decl)) => {
                decl.insert(&mut entry);
                t = tokens.len() - _tokens.len();
                continue;
            }
            Err(err) => {
                error = error.concat(err);
            }
        }

        return Err(error);
    }
}

fn split<'t>(tokens: &'t [Token<'t>], at: usize) -> &'t [Token<'t>] {
    if at >= tokens.len() {
        println!("done");
        &[]
    } else {
        &tokens[at..]
    }
}

impl<'e> Display for Entry<'e> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        if !self.funcs.is_empty() {
            writeln!(fmt, "funcs:")?;

            for name in self.funcs.keys() {
                writeln!(fmt, "  {}", name.inner())?;
            }
        }

        if !self.decls.is_empty() {
            writeln!(fmt, "decls:")?;

            for decl in &self.decls {
                writeln!(fmt, "  {}", decl)?;
            }
        }

        Ok(())
    }
}
