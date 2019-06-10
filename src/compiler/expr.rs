use inkwell::values::BasicValueEnum;

use crate::expr::Expr;
use crate::expr::Func;
use crate::expr::Literal;
use crate::lexer::Literal as LexLiteral;

use super::compile::Compile;
use super::compile::Compiler;
use super::compile::State;
use super::error::*;

impl<'f> Func<'f> {
    fn call(&self, compiler: &Compiler, state: &State) -> Option<BasicValueEnum> {
        let mut args = vec![];

        for arg in &self.args {
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

impl<'e> Compile<'e> for Expr<'e> {
    fn prepare(&self, compiler: &mut Compiler<'e>, state: &mut State<'e>) {
        match self {
            Expr::Literal(lit) => lit.prepare(compiler, state),
            Expr::Func(func) => func.prepare(compiler, state),
            Expr::Var(_) => (),
        }
    }

    fn compile(&self, compiler: &mut Compiler<'e>, state: &mut State<'e>) -> Result<()> {
        match self {
            Expr::Literal(lit) => lit.compile(compiler, state),
            Expr::Func(func) => func.compile(compiler, state),
            Expr::Var(_) => Ok(()),
        }
    }
}

impl<'l> Compile<'l> for Literal<'l> {
    fn prepare(&self, compiler: &mut Compiler<'l>, state: &mut State<'l>) {
        match self.lit {
            LexLiteral::String(string) => {
                compiler.add_global_string(state, self.name.clone(), string);
            }
        }
    }
}

impl<'f> Compile<'f> for Func<'f> {
    fn prepare(&self, compiler: &mut Compiler<'f>, state: &mut State<'f>) {
        for arg in &self.args {
            match arg {
                Expr::Literal(lit) => lit.prepare(compiler, state),
                Expr::Func(func) => func.prepare(compiler, state),
                Expr::Var(_) => (),
            }
        }
    }

    fn compile(&self, compiler: &mut Compiler<'f>, state: &mut State<'f>) -> Result<()> {
        self.call(compiler, state);
        Ok(())
    }
}
