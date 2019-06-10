use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use crate::lexer::Position;
use crate::lexer::Token;
use crate::lexer::TokenTy;

pub type Result<'r, OK> = std::result::Result<OK, Error<'r>>;

#[derive(Eq, PartialEq, Debug)]
pub struct Error<'e> {
    kind: ErrorKind<'e>,
}

#[derive(Eq, PartialEq, Debug)]
pub enum ErrorKind<'e> {
    MissingToken {
        handled: Vec<TokenTy>,
        after: Option<Position>,
    },
    WrongToken {
        token: &'e Token<'e>,
        handled: Vec<TokenTy>,
    },
    Multiple(Vec<Error<'e>>),
}

impl<'e> Error<'e> {
    pub(super) fn missing_token(handled: Vec<TokenTy>, after: Option<Position>) -> Self {
        Error {
            kind: ErrorKind::MissingToken { handled, after },
        }
    }

    pub(super) fn wrong_token(token: &'e Token<'e>, handled: Vec<TokenTy>) -> Self {
        Error {
            kind: ErrorKind::WrongToken { token, handled },
        }
    }

    pub(super) fn multiple(errors: Vec<Error<'e>>) -> Self {
        Error {
            kind: ErrorKind::Multiple(errors),
        }
    }

    pub(super) fn is_wrong_token(&self) -> bool {
        if let ErrorKind::WrongToken { .. } = self.kind {
            true
        } else {
            false
        }
    }

    pub(super) fn max_after(&mut self, after: Option<Position>) {
        if let ErrorKind::MissingToken { after: after_, .. } = &mut self.kind {
            match (after_, after) {
                (Some(after_), Some(after)) => *after_ = std::cmp::max(*after_, after),
                (after_, Some(after)) => *after_ = Some(after),
                _ => (),
            }
        }
    }

    pub(super) fn concat(mut self, mut other: Error<'e>) -> Self {
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
            ErrorKind::MissingToken { handled, after } => {
                write!(fmt, "missing token( handled:")?;

                for handled in handled {
                    write!(fmt, r#" "{}" "#, handled)?;
                }

                write!(fmt, " )")?;

                if let Some(after) = after {
                    write!(fmt, " after {}", after)?;
                }

                Ok(())
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
