use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use fnv::FnvHashMap;

use crate::lexer::Ident;

use super::func::Func;
use super::module::Module;
use super::stmt::Stmt;

#[derive(Debug)]
pub struct Tree<'t> {
    pub modules: FnvHashMap<Ident<'t>, Module<'t>>,
    pub funcs: FnvHashMap<Ident<'t>, Func<'t>>,
    pub stmts: Vec<Option<Stmt<'t>>>,
}

impl<'t> Display for Tree<'t> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        writeln!(fmt, "ast:")?;

        for (name, module) in &self.modules {
            writeln!(fmt, "  {}:", name.inner())?;

            for name in &module.funcs {
                let func = self.funcs.get(name).unwrap(); // FIXME
                write!(fmt, "    func(name={}, args=[", name.inner())?;

                // TODO: args

                writeln!(fmt, "]):")?;

                let mut next = func.start;
                while let Some(next_) = next {
                    let stmt = self.stmts[next_].as_ref().unwrap(); // FIXME
                    writeln!(fmt, "      {}", stmt)?;
                    next = stmt.next();
                }
            }
        }

        Ok(())
    }
}
