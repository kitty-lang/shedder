use fnv::FnvHashSet;

use crate::ast;
use crate::ast::Tree;
use crate::lexer::Ident;

use super::error::*;
use super::graph::Dependency;
use super::graph::Graph;

#[derive(Debug)]
pub(super) struct Func<'f> {
    pub(super) name: Ident<'f>,
    pub(super) module: Ident<'f>,
    pub(super) dependencies: FnvHashSet<Dependency<'f>>,
}

impl<'f> Func<'f> {
    pub(super) fn build(func: &'f ast::Func<'f>, module: Ident<'f>, ast: &'f Tree<'f>) -> Func<'f> {
        let mut dependencies = FnvHashSet::default();

        let mut next = func.start;
        while let Some(next_) = next {
            let stmt = ast.stmts[next_].as_ref().unwrap();

            for dependency in stmt.dependencies() {
                dependencies.insert(dependency);
            }

            next = stmt.next();
        }

        Func {
            name: func.name.as_ref(),
            module,
            dependencies,
        }
    }

    pub(super) fn verify(&self, graph: &Graph) -> Result<()> {
        let mut error = Error::multiple(vec![]);

        let module = graph.modules.get(&self.module).unwrap();
        for dependency in &self.dependencies {
            match dependency {
                Dependency::Func(func) => {
                    if !module.funcs.contains_key(func) {
                        error = error.concat(Error::missing_dependency(
                            self.name.as_ref(),
                            dependency.as_ref(),
                        ));
                    }
                }
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
