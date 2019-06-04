use crate::stmt::Stmt;

use super::compile::Compile;
use super::compile::Compiler;
use super::error::*;

impl<'s> Compile for Stmt<'s> {
    fn compile(&self, compiler: &Compiler) -> Result<()> {
        match self {
            Stmt::Expr(expr) => expr.compile(compiler),
        }
    }
}
