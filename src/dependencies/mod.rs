use fnv::FnvHashMap;

use crate::ast::Tree;

mod error;
mod expr;
mod func;
mod graph;
mod module;
mod stmt;

pub use error::*;
pub use graph::Dependency;
pub use graph::Graph;

use module::Module;

impl<'g> Graph<'g> {
    pub fn build(ast: &'g Tree) -> Graph<'g> {
        let mut graph = Graph {
            modules: FnvHashMap::default(),
        };

        for module in ast.modules.values() {
            graph
                .modules
                .insert(module.name.as_ref(), Module::build(module, ast));
        }

        graph
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
