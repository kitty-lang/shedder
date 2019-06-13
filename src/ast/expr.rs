use fnv::FnvHashMap;

use crate::lexer::Ident;
use crate::lexer::Literal;
use crate::lexer::Ty;
use crate::parser::expr::Expr;

use super::error::*;
use super::tree::Tree;

impl<'e> Expr<'e> {
    pub(super) fn ty(&self, vars: &FnvHashMap<Ident<'e>, Ty>, tree: &Tree) -> Result<Ty> {
        match self {
            Expr::Literal(lit) => match lit.lit {
                Literal::String(_) => Ok(Ty::Str),
            },
            Expr::Func(func) => {
                let decl = tree.funcs.get(&func.name).unwrap(); // FIXME

                // TODO: args

                Ok(decl.ret)
            }
            Expr::Var(var) => Ok(*vars.get(var).unwrap()), // FIXME
        }
    }
}
