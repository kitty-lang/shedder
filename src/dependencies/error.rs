use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::lexer::Ident;

use super::graph::Dependency;

pub type Result<'r, OK> = std::result::Result<OK, Error<'r>>;

#[derive(Debug)]
pub struct Error<'e> {
    pub(super) kind: ErrorKind<'e>,
}

#[derive(Debug)]
pub enum ErrorKind<'e> {
    MissingDependency {
        func: Ident<'e>,
        dependency: Dependency<'e>,
    },
    Multiple(Vec<Error<'e>>),
}

impl<'e> Error<'e> {
    pub(super) fn missing_dependency(func: Ident<'e>, dependency: Dependency<'e>) -> Error<'e> {
        Error {
            kind: ErrorKind::MissingDependency { func, dependency },
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
            ErrorKind::MissingDependency { func, dependency } => write!(
                fmt,
                "missing dependency(in={}, {})",
                func.inner(),
                dependency
            ),
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
