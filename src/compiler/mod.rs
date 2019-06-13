use crate::ast::Tree;
use crate::lexer::Ident;
use crate::lexer::Ty;

mod compile;
mod decl;
mod error;
mod expr;
mod module;
mod stmt;

pub use compile::Compiled;
pub use error::*;

use compile::Compiler;
use compile::CompilerTy;

pub fn compile(ast: &Tree) -> Result<Compiled> {
    let mut compiler = Compiler::new();

    // --- FIXME ---
    compiler.new_module("_");
    compiler.add_external_function(
        Ident::Owned("puts".into()),
        CompilerTy::Ty(Ty::Void).fn_type(&[Ty::Str.into()]),
    );
    // --- FIXME ---

    for module in ast.modules.values() {
        compiler.new_module(module.name.inner());
        module.compile(ast, &mut compiler)?;
    }

    Ok(compiler.compiled())
}
