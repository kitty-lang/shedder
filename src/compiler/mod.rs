use crate::lexer::Ident;
use crate::parser::Module;
use crate::ty::RawTy;
use crate::ty::Ty;

mod compile;
mod decl;
mod error;
mod expr;
mod module;
mod stmt;

pub use compile::Compiled;
pub use error::*;

use compile::Compiler;

pub fn compile(main: &Module) -> Result<Compiled> {
    let mut compiler = Compiler::new("main"); // FIXME

    // --- FIXME ---
    let puts = Ident::new("puts".into());
    let puts_ty = RawTy::int32().fn_type(
        &compiler.ctx,
        &[Ty::str().raw().to_ptr(&compiler.ctx).into()],
        false,
    );

    compiler.add_external_function(puts, puts_ty);
    // --- FIXME ---

    main.compile(&mut compiler)?;

    Ok(compiler.compiled())
}
