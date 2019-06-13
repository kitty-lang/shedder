use fnv::FnvHashMap;

use crate::parser;

mod error;
mod expr;
mod func;
mod module;
mod stmt;
mod tree;

pub use error::*;
pub use func::Func;
pub use module::Module;
pub use stmt::Stmt;
pub use tree::Tree;

impl<'t> Tree<'t> {
    pub fn build(modules: &[&'t parser::Module<'t>]) -> Tree<'t> {
        let mut tree = Tree {
            modules: FnvHashMap::default(),
            funcs: FnvHashMap::default(),
            stmts: vec![],
        };

        for module in modules {
            let name = module.name.as_ref();
            let module = Module::build(module, &mut tree);
            tree.modules.insert(name, module);
        }

        tree
    }

    pub fn verify(&self) -> Result<()> {
        let mut error = Error::multiple(vec![]);

        for module in self.modules.values() {
            if let Err(err) = module.verify(self) {
                error = error.concat(err);
            }
        }

        if let ErrorKind::Multiple(errors) = &mut error.kind {
            if errors.is_empty() {
                Ok(())
            } else if errors.len() == 1 {
                Err(errors.pop().unwrap())
            } else {
                Err(error)
            }
        } else {
            panic!(); // FIXME
        }
    }
}
