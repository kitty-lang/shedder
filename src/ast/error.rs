use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::lexer::Ty;

use super::stmt::Stmt;

pub type Result<'r, OK> = std::result::Result<OK, Error<'r>>;

#[derive(Debug)]
pub struct Error<'e> {
    pub(super) kind: ErrorKind<'e>,
}

#[derive(Debug)]
pub enum ErrorKind<'e> {
    WrongTy {
        stmt: &'e Stmt<'e>,
        returned: Ty,
        accepted: Vec<Ty>,
    },
    MissingReturn(Ty),
    Multiple(Vec<Error<'e>>),
}

impl<'e> Error<'e> {
    pub(super) fn wrong_ty(stmt: &'e Stmt<'e>, returned: Ty, accepted: Vec<Ty>) -> Error<'e> {
        Error {
            kind: ErrorKind::WrongTy {
                stmt,
                returned,
                accepted,
            },
        }
    }

    pub(super) fn missing_ty(ty: Ty) -> Error<'e> {
        Error {
            kind: ErrorKind::MissingReturn(ty),
        }
    }

    pub(super) fn multiple(errors: Vec<Error<'e>>) -> Error<'e> {
        Error {
            kind: ErrorKind::Multiple(errors),
        }
    }

    pub(super) fn concat(mut self, mut other: Error<'e>) -> Error<'e> {
        match (&mut self.kind, &mut other.kind) {
            (ErrorKind::Multiple(left), ErrorKind::Multiple(right)) => {
                left.append(right);
                self
            }
            (ErrorKind::Multiple(multi), _) => {
                multi.push(other);
                self
            }
            (_, ErrorKind::Multiple(multi)) => {
                multi.push(self);
                other
            }
            (_, _) => Error::multiple(vec![self, other]),
        }
    }
}

impl<'e> Display for Error<'e> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match &self.kind {
            ErrorKind::WrongTy {
                stmt,
                returned,
                accepted,
            } => {
                write!(
                    fmt,
                    "wrong type(stmt={}, returned={}, accepted=[",
                    stmt, returned
                )?;

                for accepted in accepted {
                    write!(fmt, " {} ", accepted)?;
                }

                write!(fmt, "]")
            }
            ErrorKind::MissingReturn(ty) => write!(fmt, "missing return({})", ty),
            ErrorKind::Multiple(errors) => {
                write!(fmt, "multiple errors: [")?;

                for error in errors {
                    write!(fmt, " {} ", error)?;
                }

                write!(fmt, "]")
            }
        }
    }
}
