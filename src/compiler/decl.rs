use crate::decl::Decl;
use crate::decl::Func;
use crate::ty::Ty;

use super::compile::Compile;
use super::compile::Compiler;
use super::error::*;

impl<'d> Compile for Decl<'d> {
    fn compile(&self, compiler: &Compiler) -> Result<()> {
        match self {
            Decl::Func(func) => func.compile(compiler),
        }
    }
}

impl<'f> Compile for Func<'f> {
    fn compile(&self, compiler: &Compiler) -> Result<()> {
        let func_ty = Ty::void().raw().fn_type(&compiler.ctx, &[], false);

        let func = compiler.add_function(&self.name.inner(), func_ty);

        let entry = func.append_basic_block("entry"); // FIXME
        compiler.builder.position_at_end(&entry);

        for stmt in &self.stmts {
            stmt.compile(compiler)?;
        }

        compiler.builder.build_return(None); // FIXME

        assert!(func.verify(true));

        Ok(())
    }
}
