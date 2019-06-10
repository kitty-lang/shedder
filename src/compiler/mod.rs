use crate::lexer::Ident;
use crate::parser::Module;
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
use compile::CompilerTy;

pub fn compile(main: &Module) -> Result<Compiled> {
    let mut compiler = Compiler::new("main"); // FIXME

    // --- FIXME ---
    compiler.add_external_function(
        Ident::Owned("puts".into()),
        CompilerTy::Ty(Ty::Void).fn_type(&[Ty::Str.into()]),
    );
    // --- FIXME ---

    main.compile(&mut compiler)?;

    Ok(compiler.compiled())
}
