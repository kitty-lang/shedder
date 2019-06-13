use fnv::FnvHashMap;

use crate::ast;
use crate::ast::Tree;
use crate::lexer::Ident;

use super::error::*;
use super::func::Func;
use super::graph::Graph;

#[derive(Debug)]
pub(super) struct Module<'m> {
    pub(super) name: Ident<'m>,
    pub(super) funcs: FnvHashMap<Ident<'m>, Func<'m>>,
}

impl<'m> Module<'m> {
    pub(super) fn build(module: &'m ast::Module, ast: &'m Tree) -> Module<'m> {
        let name = module.name.as_ref();
        let funcs = &module.funcs;
        let mut module = Module {
            name: module.name.as_ref(),
            funcs: FnvHashMap::default(),
        };

        for func in funcs {
            module.funcs.insert(
                func.as_ref(),
                Func::build(
                    ast.funcs.get(func).unwrap(), // FIXME
                    name.clone(),
                    ast,
                ),
            );
        }

        module
    }

    pub(super) fn verify(&self, graph: &Graph) -> Result<()> {
        let mut error = Error::multiple(vec![]);

        for func in self.funcs.values() {
            if let Err(err) = func.verify(graph) {
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
