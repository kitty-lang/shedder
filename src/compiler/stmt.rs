use crate::ast::Stmt;
use crate::parser::expr::Expr;
use crate::parser::stmt::Let;
use crate::parser::stmt::Return;

use super::compile::Compiler;
use super::compile::State;
use super::error::*;

impl<'s> Stmt<'s> {
    pub(super) fn prepare(&'s self, compiler: &mut Compiler<'s>, state: &mut State<'s>) {
        match self {
            Stmt::Let { let_, .. } => let_.prepare(compiler, state),
            Stmt::Return { .. } => (),
            Stmt::Expr { expr, .. } => expr.prepare(compiler, state),
        }
    }

    pub(super) fn compile(
        &'s self,
        compiler: &mut Compiler<'s>,
        state: &mut State<'s>,
    ) -> Result<()> {
        match self {
            Stmt::Let { let_, .. } => let_.compile(compiler, state),
            Stmt::Return { ret, .. } => ret.compile(compiler, state),
            Stmt::Expr { expr, .. } => expr.compile(compiler, state),
        }
    }
}

impl<'l> Let<'l> {
    pub(super) fn prepare(&'l self, compiler: &mut Compiler<'l>, state: &mut State<'l>) {
        match &self.value {
            Expr::Literal(lit) => {
                compiler.alias(state, self.name.as_ref(), lit.name());
                lit.prepare(compiler, state);
            }
            Expr::Func(func) => {
                compiler.register_var(
                    state,
                    self.name.as_ref(),
                    func.call(compiler, state).unwrap(), // FIXME
                );
            }
            _ => unimplemented!(), // FIXME
        }
    }

    pub(super) fn compile(&self, _: &mut Compiler<'l>, _: &mut State<'l>) -> Result<()> {
        println!("{}", self.value);
        match &self.value {
            Expr::Literal(_) => Ok(()),
            Expr::Func(_) => Ok(()),
            _ => unimplemented!(), // FIXME
        }
    }
}

impl<'r> Return<'r> {
    pub(super) fn compile(
        &'r self,
        compiler: &mut Compiler<'r>,
        state: &mut State<'r>,
    ) -> Result<()> {
        match &self.0 {
            Expr::Literal(lit) => {
                lit.prepare(compiler, state);
                compiler.ret(
                    state,
                    Some(&compiler.get_var(state, &lit.name()).unwrap()), // FIXME
                );
            }
            _ => unimplemented!(), // FIXME
        }

        Ok(())
    }
}
