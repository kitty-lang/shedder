use crate::ast::Func;
use crate::ast::Tree;
use crate::lexer::Ident;

use super::compile::Compiler;
use super::compile::State;
use super::error::*;

impl<'f> Func<'f> {
    pub(super) fn declare(&'f self, compiler: &mut Compiler<'f>) {
        compiler.add_function(self.name.as_ref(), &self.args, self.ret.into());
    }

    pub(super) fn compile(&'f self, ast: &'f Tree, compiler: &mut Compiler<'f>) -> Result<()> {
        let entry = Ident::Owned("entry".into());
        compiler.append_block(&self.name, entry.clone());

        let mut state = State {
            func: self.name.as_ref(),
            block: entry,
        };

        let mut next = self.start;
        while let Some(next_) = next {
            let stmt = ast.stmts[next_].as_ref().unwrap(); // FIXME
            stmt.prepare(compiler, &mut state);
            next = stmt.next();
        }

        let mut next = self.start;
        while let Some(next_) = next {
            let stmt = ast.stmts[next_].as_ref().unwrap(); // FIXME
            stmt.compile(compiler, &mut state)?;
            next = stmt.next();
        }

        compiler.ret(&state, None); // FIXME

        Ok(())
    }
}
