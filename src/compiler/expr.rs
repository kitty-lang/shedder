use inkwell::values::BasicValueEnum;

use crate::lexer;
use crate::parser::expr::Expr;
use crate::parser::expr::Func;
use crate::parser::expr::Literal;

use super::compile::Compiler;
use super::compile::State;
use super::error::*;

impl<'f> Func<'f> {
    pub(super) fn call(&self, compiler: &Compiler, state: &State) -> Option<BasicValueEnum> {
        let mut args = vec![];

        for arg in self.args.inner() {
            match arg {
                Expr::Literal(Literal { name, .. }) => {
                    args.push(compiler.get_var(&state, name).unwrap()); // FIXME
                }
                Expr::Func(func) => {
                    args.push(func.call(compiler, state).unwrap()); // FIXME
                }
                Expr::Var(var) => args.push(compiler.get_var(&state, var).unwrap()), // FIXME
            }
        }

        compiler.call(state, &self.name, &args)
    }
}

impl<'e> Expr<'e> {
    pub(super) fn prepare(&'e self, compiler: &mut Compiler<'e>, state: &mut State<'e>) {
        match self {
            Expr::Literal(lit) => lit.prepare(compiler, state),
            Expr::Func(func) => func.prepare(compiler, state),
            Expr::Var(_) => (),
        }
    }

    pub(super) fn compile(
        &'e self,
        compiler: &mut Compiler<'e>,
        state: &mut State<'e>,
    ) -> Result<()> {
        match self {
            Expr::Literal(_) => Ok(()),
            Expr::Func(func) => func.compile(compiler, state),
            Expr::Var(_) => Ok(()),
        }
    }
}

impl<'l> Literal<'l> {
    pub(super) fn prepare(&'l self, compiler: &mut Compiler<'l>, state: &mut State<'l>) {
        match self.lit {
            lexer::Literal::String(string) => {
                compiler.add_global_string(state, self.name.as_ref(), string);
            }
        }
    }
}

impl<'f> Func<'f> {
    pub(super) fn prepare(&'f self, compiler: &mut Compiler<'f>, state: &mut State<'f>) {
        for arg in self.args.inner() {
            match arg {
                Expr::Literal(lit) => lit.prepare(compiler, state),
                Expr::Func(func) => func.prepare(compiler, state),
                Expr::Var(_) => (),
            }
        }
    }

    pub(super) fn compile(&self, compiler: &mut Compiler<'f>, state: &mut State<'f>) -> Result<()> {
        self.call(compiler, state);
        Ok(())
    }
}
