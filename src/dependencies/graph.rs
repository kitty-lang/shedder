use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use fnv::FnvHashMap;

use crate::lexer::Ident;

use super::module::Module;

#[derive(Debug)]
pub struct Graph<'g> {
    pub(super) modules: FnvHashMap<Ident<'g>, Module<'g>>,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Dependency<'d> {
    Func(Ident<'d>),
}

impl<'d> Dependency<'d> {
    pub fn as_ref(&'d self) -> Dependency<'d> {
        match self {
            Dependency::Func(func) => Dependency::Func(func.as_ref()),
        }
    }
}

impl<'g> Display for Graph<'g> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        writeln!(fmt, "dependency graph:")?;

        for (name, module) in &self.modules {
            writeln!(fmt, "  {}:", name.inner())?;

            writeln!(fmt, "  - funcs:")?;
            for func in module.funcs.values() {
                writeln!(fmt, "    {}:", func.name.inner())?;

                writeln!(fmt, "    - dependencies:")?;
                for dependency in &func.dependencies {
                    writeln!(fmt, "      {}", dependency)?;
                }
            }
        }

        Ok(())
    }
}

impl<'d> Display for Dependency<'d> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Dependency::Func(func) => write!(fmt, "func({})", func.inner()),
        }
    }
}
