use fnv::FnvHashMap;

use crate::lexer::Ident;
use crate::lexer::Ty;
use crate::parser::decl;
use crate::parser::decl::Arg;

use super::error::*;
use super::stmt::Stmt;
use super::tree::Tree;

#[derive(Debug)]
pub struct Func<'f> {
    pub name: Ident<'f>,
    pub args: &'f [Arg<'f>],
    pub ret: Ty,
    pub variadic: bool,
    pub start: Option<usize>,
}

impl<'f> Func<'f> {
    pub(super) fn build(func: &'f decl::Func<'f>, tree: &mut Tree<'f>) -> Func<'f> {
        let stmts = &func.stmts;
        let mut func = Func {
            name: func.name.as_ref(),
            args: &func.args,
            ret: func.ret,
            variadic: func.variadic,
            start: None,
        };

        for stmt in stmts {
            if let Some(start) = &func.start {
                let mut start_ = tree.stmts[*start].take().unwrap(); // FIXME
                start_.push(tree, stmt.into());
                tree.stmts[*start] = Some(start_);
            } else {
                func.start = Some(tree.stmts.len());
                tree.stmts.push(Some(stmt.into()));
            }
        }

        func
    }

    pub(super) fn verify(&self, tree: &'f Tree) -> Result<()> {
        let mut error = Error::multiple(vec![]);

        let mut vars = FnvHashMap::default();
        let mut returned = false;

        for arg in self.args {
            vars.insert(arg.name.as_ref(), arg.ty);
        }

        let mut next = self.start;
        while let Some(next_) = next {
            let stmt = tree.stmts[next_].as_ref().unwrap(); // FIXME
            match stmt {
                Stmt::Let { let_, next: next_ } => {
                    match let_.value.ty(stmt, &vars, tree) {
                        Ok(ty) => {
                            vars.insert(let_.name.as_ref(), ty);
                        }
                        Err(err) => error = error.concat(err),
                    }

                    next = *next_;
                }
                Stmt::Return { ret, next: next_ } => {
                    match ret.0.ty(stmt, &vars, tree) {
                        Ok(ty) => {
                            if ty != self.ret {
                                error = error.concat(Error::wrong_ty(stmt, ty, vec![self.ret]));
                            }
                        }
                        Err(err) => error = error.concat(err),
                    }

                    returned = true;
                    next = *next_;
                }
                Stmt::Expr { expr, next: next_ } => {
                    expr.ty(stmt, &vars, tree)?;
                    next = *next_;
                }
            }
        }

        if !returned && self.ret != Ty::Void {
            return Err(Error::missing_ty(self.ret));
        }

        if let ErrorKind::Multiple(errors) = &mut error.kind {
            if errors.is_empty() {
                Ok(())
            } else if errors.len() == 1 {
                Err(errors.pop().unwrap())
            } else {
                Err(error)
            }
        } else {
            panic!(); // FIXME
        }
    }
}
