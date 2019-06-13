use crate::ast::Module;
use crate::ast::Tree;

use super::compile::Compiler;
use super::error::*;

impl<'m> Module<'m> {
    pub(super) fn compile(&'m self, ast: &'m Tree, compiler: &mut Compiler<'m>) -> Result<()> {
        for func in &self.funcs {
            ast.funcs
                .get(func)
                .unwrap() // FIXME
                .declare(compiler);
        }

        for func in &self.funcs {
            ast.funcs
                .get(func)
                .unwrap() // FIXME
                .compile(ast, compiler)?;
        }

        Ok(())
    }
}
