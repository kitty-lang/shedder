use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::lexer::Token;

pub mod decl;

mod error;
mod parse;

pub use decl::Decl;
pub use error::*;

use parse::Parse;

pub struct Entry<'e> {
    pub decls: Vec<Decl<'e>>,
}

pub fn parse(tokens: &[Token]) -> Result<Entry> {
    let mut entry = Entry { decls: vec![] };

    let mut t = 0;

    loop {
        if tokens[t] == Token::EOF {
            return Ok(entry);
        }

        let mut handled = vec![];

        match Decl::parse(&tokens[t..]) {
            Ok((_tokens, decl)) => {
                entry.decls.push(decl);
                t = tokens.len() - _tokens.len();
                continue;
            }
            Err(err) => match err.into_kind() {
                ErrorKind::WrongToken {
                    handled: mut handled_,
                    ..
                } => handled.append(&mut handled_),
            },
        }

        return Err(Error::wrong_token(&tokens[t], handled));
    }
}

impl<'e> Display for Entry<'e> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        if !self.decls.is_empty() {
            writeln!(fmt, "decls:")?;
        }

        for decl in &self.decls {
            write!(fmt, " {}", decl)?;
        }

        Ok(())
    }
}
