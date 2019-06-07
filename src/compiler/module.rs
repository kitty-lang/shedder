use crate::parser::Module;

use super::compile::Compiler;
use super::error::*;

impl<'m> Module<'m> {
    pub(super) fn compile(&self, compiler: &mut Compiler) -> Result<()> {
        for func in self.funcs.values() {
            func.declare(compiler);
        }

        for func in self.funcs.values() {
            func.compile(compiler)?;
        }

        Ok(())
    }
}
