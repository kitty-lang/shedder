use crate::ast::Stmt;

use super::graph::Dependency;

impl<'s> Stmt<'s> {
    pub(super) fn dependencies(&self) -> Vec<Dependency> {
        match self {
            Stmt::Let { let_, .. } => let_.value.dependencies(),
            Stmt::Return { ret, .. } => ret.0.dependencies(),
            Stmt::Expr { expr, .. } => expr.dependencies(),
        }
    }
}
