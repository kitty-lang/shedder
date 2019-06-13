use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::parser::expr::Expr;
use crate::parser::stmt;
use crate::parser::stmt::Let;
use crate::parser::stmt::Return;

use super::tree::Tree;

#[derive(Debug)]
pub enum Stmt<'s> {
    Let {
        let_: Let<'s>,
        next: Option<usize>,
    },
    Return {
        ret: Return<'s>,
        next: Option<usize>,
    },
    Expr {
        expr: Expr<'s>,
        next: Option<usize>,
    },
}

impl<'s> Stmt<'s> {
    pub fn next(&self) -> Option<usize> {
        match self {
            Stmt::Let { next, .. } => *next,
            Stmt::Return { next, .. } => *next,
            Stmt::Expr { next, .. } => *next,
        }
    }

    pub(super) fn next_mut(&mut self) -> &mut Option<usize> {
        match self {
            Stmt::Let { next, .. } => next,
            Stmt::Return { next, .. } => next,
            Stmt::Expr { next, .. } => next,
        }
    }

    pub(super) fn push(&mut self, tree: &mut Tree<'s>, stmt: Stmt<'s>) {
        let next = self.next_mut();
        if let Some(next) = next {
            let mut next_ = tree.stmts[*next].take().unwrap(); // FIXME
            next_.push(tree, stmt);
            tree.stmts[*next] = Some(next_);
        } else {
            *next = Some(tree.stmts.len());
            tree.stmts.push(Some(stmt));
        }
    }
}

impl<'s> From<&'s stmt::Stmt<'s>> for Stmt<'s> {
    fn from(stmt: &'s stmt::Stmt<'s>) -> Stmt<'s> {
        match stmt {
            stmt::Stmt::Let(let_) => Stmt::Let {
                let_: let_.as_ref(),
                next: None,
            },
            stmt::Stmt::Return(ret) => Stmt::Return {
                ret: ret.as_ref(),
                next: None,
            },
            stmt::Stmt::Expr(expr) => Stmt::Expr {
                expr: expr.as_ref(),
                next: None,
            },
        }
    }
}

impl<'s> Display for Stmt<'s> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "stmt::")?;
        match self {
            Stmt::Let { let_, .. } => write!(fmt, "{}", let_),
            Stmt::Return { ret, .. } => write!(fmt, "{}", ret),
            Stmt::Expr { expr, .. } => write!(fmt, "{}", expr),
        }
    }
}
