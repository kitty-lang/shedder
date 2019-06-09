use crate::lexer::Ident;
use crate::parser::Module;

mod compile;
mod decl;
mod error;
mod expr;
mod module;
mod stmt;

pub use compile::Compiled;
pub use error::*;

use compile::Compiler;
use compile::Ty;

pub fn compile(main: &Module) -> Result<Compiled> {
    let mut compiler = Compiler::new("main"); // FIXME

    // --- FIXME ---
    compiler.add_external_function(Ident::Owned("puts".into()), Ty::Void.fn_type(&[Ty::Str]));
    // --- FIXME ---

    main.compile(&mut compiler)?;

    Ok(compiler.compiled())
}
