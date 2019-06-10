use crate::decl::Func;
use crate::lexer::Ident;

use super::compile::Compile;
use super::compile::Compiler;
use super::compile::State;
use super::error::*;

impl<'f> Func<'f> {
    pub(super) fn declare(&'f self, compiler: &mut Compiler<'f>) {
        compiler.add_function(
            self.name.as_ref(),
            self.ret.clone().into(),
        );
    }

    pub(super) fn compile(&'f self, compiler: &mut Compiler<'f>) -> Result<()> {
        let entry = Ident::Owned("entry".into());
        compiler.append_block(&self.name, entry.clone());

        let mut state = State {
            func: self.name.as_ref(),
            block: entry,
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
