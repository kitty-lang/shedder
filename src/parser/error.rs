use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::lexer::Token;
use crate::lexer::TokenTy;

pub type Result<'e, OK> = std::result::Result<OK, Error<'e>>;

#[derive(Debug)]
pub struct Error<'e> {
    kind: ErrorKind<'e>,
}

#[derive(Eq, PartialEq, Debug)]
pub enum ErrorKind<'e> {
    MissingToken {
        handled: Vec<TokenTy>,
    },
    WrongToken {
        token: &'e Token<'e>,
        handled: Vec<TokenTy>,
    },
    Multiple(Vec<Error<'e>>),
}

impl<'e> Error<'e> {
    pub(super) fn missing_token(handled: Vec<TokenTy>) -> Self {
        Error {
            kind: ErrorKind::MissingToken { handled },
        }
    }

    pub(super) fn wrong_token(token: &'e Token<'e>, handled: Vec<TokenTy>) -> Self {
        Error {
            kind: ErrorKind::WrongToken { token, handled },
        }
    }

    pub(super) fn multiple(errors: Vec<Self>) -> Self {
        Error {
            kind: ErrorKind::Multiple(errors),
        }
    }

    pub(super) fn concat(mut self, mut other: Self) -> Self {
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

impl<'e> Eq for Error<'e> {}

impl<'e> PartialEq for Error<'e> {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl<'e> Display for Error<'e> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match &self.kind {
            ErrorKind::MissingToken { handled } => {
                write!(fmt, "missing token( handled:")?;

                for handled in handled {
                    write!(fmt, r#" "{}" "#, handled)?;
                }

                write!(fmt, " )")
            }
            ErrorKind::WrongToken { token, handled } => {
                write!(fmt, r#"wrong token( handled:"#)?;

                let mut first = true;
                for handled in handled {
                    if !first {
                        write!(fmt, ", ")?;
                    } else {
                        first = false;
                    }

                    write!(fmt, r#"{}"#, handled)?;
                }

                write!(fmt, " ):{}", token)
            }
            ErrorKind::Multiple(errors) => {
                write!(fmt, "multiple errors possible: [ ")?;

                let mut first = true;
                for error in errors {
                    if !first {
                        write!(fmt, " || ")?;
                    } else {
                        first = false;
                    }

                    write!(fmt, "{}", error)?;
                }

                write!(fmt, " ]")
            }
        }
    }
}
