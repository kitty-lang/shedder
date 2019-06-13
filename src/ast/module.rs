use fnv::FnvHashSet;

use crate::lexer::Ident;
use crate::parser;

use super::error::*;
use super::func::Func;
use super::tree::Tree;

#[derive(Debug)]
pub struct Module<'m> {
    pub name: Ident<'m>,
    pub funcs: FnvHashSet<Ident<'m>>,
}

impl<'m> Module<'m> {
    pub(super) fn build(module: &'m parser::Module<'m>, tree: &mut Tree<'m>) -> Module<'m> {
        let funcs = &module.funcs;
        let mut module = Module {
            name: module.name.as_ref(),
            funcs: FnvHashSet::default(),
        };

        for func in funcs {
            let name = func.name.as_ref();
            let func = Func::build(func, tree);
            module.funcs.insert(name.clone());
            tree.funcs.insert(name.clone(), func);
        }

        module
    }

    pub(super) fn verify(&self, tree: &'m Tree) -> Result<()> {
        let mut error = Error::multiple(vec![]);

        for func in &self.funcs {
            let func = tree.funcs.get(func).unwrap();
            if let Err(err) = func.verify(tree) {
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
