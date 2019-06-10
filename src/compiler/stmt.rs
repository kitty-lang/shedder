use crate::expr::Expr;
use crate::stmt::Let;
use crate::stmt::Return;
use crate::stmt::Stmt;

use super::compile::Compile;
use super::compile::Compiler;
use super::compile::State;
use super::error::*;

impl<'s> Compile<'s> for Stmt<'s> {
    fn prepare(&self, compiler: &mut Compiler<'s>, state: &mut State<'s>) {
        match self {
            Stmt::Expr(expr) => expr.prepare(compiler, state),
            Stmt::Let(let_) => let_.prepare(compiler, state),
            Stmt::Return(ret) => ret.prepare(compiler, state),
        }
    }

    fn compile(&self, compiler: &mut Compiler<'s>, state: &mut State<'s>) -> Result<()> {
        match self {
            Stmt::Expr(expr) => expr.compile(compiler, state),
            Stmt::Let(let_) => let_.compile(compiler, state),
            Stmt::Return(ret) => ret.compile(compiler, state),
        }
    }
}

impl<'l> Compile<'l> for Let<'l> {
    fn prepare(&self, compiler: &mut Compiler<'l>, state: &mut State<'l>) {
        match &self.value {
            Expr::Literal(lit) => {
                compiler.alias(state, self.name.clone(), lit.name.clone());
                lit.prepare(compiler, state);
            }
            _ => unimplemented!(), // FIXME
        }
    }

    fn compile(&self, compiler: &mut Compiler<'l>, state: &mut State<'l>) -> Result<()> {
        match &self.value {
            Expr::Literal(lit) => lit.compile(compiler, state),
            _ => unimplemented!(), // FIXME
        }
    }
}

impl<'r> Compile<'r> for Return<'r> {
    fn compile(&self, compiler: &mut Compiler<'r>, state: &mut State<'r>) -> Result<()> {
        match &self.0 {
            Expr::Literal(lit) => {
                lit.prepare(compiler, state);
                compiler.ret(
                    state,
                    Some(&compiler.get_var(state, &lit.name).unwrap()), // FIXME
                );
            }
            _ => unimplemented!(), // FIXME
        }

        Ok(())
    }
}
