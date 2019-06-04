use crate::parser::Entry;
use crate::ty::RawTy;
use crate::ty::Ty;

mod compile;
mod decl;
mod error;
mod expr;
mod stmt;

pub use compile::Compiled;
pub use error::*;

use compile::Compile;
use compile::Compiler;

pub fn compile(entry: &Entry) -> Result<Compiled> {
    let compiler = Compiler::new();

    // --- FIXME ---
    let puts_ty = RawTy::int32().fn_type(
        &compiler.ctx,
        &[Ty::str().raw().to_ptr(&compiler.ctx).into()],
        false,
    );

    compiler.add_external_function("puts", puts_ty);
    // --- FIXME ---

    for decl in &entry.decls {
        decl.compile(&compiler)?;
    }

    Ok(compiler.compiled())
}
