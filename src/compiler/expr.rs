use crate::expr::Expr;
use crate::expr::Func;
use crate::lexer::Literal;
use crate::lexer::TokenVariant;

use super::compile::Compile;
use super::compile::Compiler;
use super::error::*;

impl<'e> Compile for Expr<'e> {
    fn compile(&self, compiler: &Compiler) -> Result<()> {
        match self {
            Expr::Func(func) => func.compile(compiler),
        }
    }
}

impl<'f> Compile for Func<'f> {
    fn compile(&self, compiler: &Compiler) -> Result<()> {
        // --- FIXME ---
        assert_eq!(self.name.inner(), "print");
        assert_eq!(self.args.len(), 1);

        let to_print = if let TokenVariant::Literal(Literal::String(string)) = self.args[0].token {
            compiler.add_global_string_ptr(string, "to_print")
        } else {
            panic!();
        };

        compiler.builder.build_call(
            compiler.get_function("puts").unwrap(), // FIXME
            &[to_print.as_pointer_value().into()],
            "print",
        );
        // --- FIXME ---

        Ok(())
    }
}
