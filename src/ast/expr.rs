use fnv::FnvHashMap;

use crate::lexer::Ident;
use crate::lexer::Ty;
use crate::parser::expr::Expr;
use crate::parser::expr::Literal;

use super::error::*;
use super::stmt::Stmt;
use super::tree::Tree;

impl<'e> Expr<'e> {
    pub(super) fn ty(
        &self,
        stmt: &'e Stmt,
        vars: &FnvHashMap<Ident<'e>, Ty>,
        tree: &Tree,
    ) -> Result<Ty> {
        match self {
            Expr::Literal(lit) => match lit {
                Literal::Int { .. } => Ok(Ty::I32),
                Literal::String { .. } => Ok(Ty::Str),
                Literal::RefDynString { .. } => Ok(Ty::Str),
                Literal::OwnedDynString { .. } => Ok(Ty::Str),
            },
            Expr::Func(func) => {
                let decl = tree.funcs.get(&func.name).unwrap(); // FIXME

                if func.args.len() < decl.args.len() {
                    return Err(Error::wrong_ty(
                        stmt,
                        Ty::Void,
                        decl.args[func.args.len()..]
                            .iter()
                            .map(|arg| arg.ty)
                            .collect(),
                    ));
                } else if func.args.len() > decl.args.len() && !decl.variadic {
                    return Err(Error::wrong_ty(
                        stmt,
                        func.args.inner()[decl.args.len()].ty(stmt, vars, tree)?,
                        vec![],
                    ));
                }

                for (a, arg) in func.args.inner().iter().enumerate() {
                    if a >= decl.args.len() {
                        break;
                    }

                    if arg.ty(stmt, vars, tree)? != decl.args[a].ty {
                        return Err(Error::wrong_ty(
                            stmt,
                            arg.ty(stmt, vars, tree)?,
                            vec![decl.args[a].ty],
                        ));
                    }
                }

                Ok(decl.ret)
            }
            Expr::Var(var) => Ok(*vars.get(var).unwrap()), // FIXME
        }
    }
}
