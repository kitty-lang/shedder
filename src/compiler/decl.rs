use crate::decl::Func;
use crate::ty::Ty;

use super::compile::Compile;
use super::compile::Compiler;
use super::compile::State;
use super::error::*;

impl<'f> Func<'f> {
    pub(super) fn declare(&self, compiler: &mut Compiler) {
        // --- FIXME ---
        let func_ty = Ty::void().raw().fn_type(&compiler.ctx, &[], false);
        // --- FIXME ---

        compiler.add_function(self.name.clone(), func_ty);
    }

    pub(super) fn compile(&self, compiler: &mut Compiler) -> Result<()> {
        compiler.append_block(&self.name, "entry".into());

        let mut state = State {
            func: &self.name,
            block: "entry",
        };

        for stmt in &self.stmts {
            stmt.prepare(compiler, &mut state);
        }

        for stmt in &self.stmts {
            stmt.compile(compiler, &mut state)?;
        }

        compiler.ret(&state, None); // FIXME

        Ok(())
    }
}
