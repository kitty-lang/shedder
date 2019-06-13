use crate::parser::expr::Expr;

use super::graph::Dependency;

impl<'e> Expr<'e> {
    pub(super) fn dependencies(&self) -> Vec<Dependency> {
        match self {
            Expr::Literal(_) => vec![],
            Expr::Func(func) => {
                let mut dependencies = vec![Dependency::Func(func.name.as_ref())];

                for arg in func.args.inner() {
                    dependencies.append(&mut arg.dependencies());
                }

                dependencies
            }
            Expr::Var(_) => vec![],
        }
    }
}
