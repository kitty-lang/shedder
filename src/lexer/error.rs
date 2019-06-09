use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

pub type Result<OK> = std::result::Result<OK, Error>;

#[derive(Eq, PartialEq, Debug)]
pub struct Error {
    kind: ErrorKind,
}

#[derive(Eq, PartialEq, Debug)]
pub enum ErrorKind {
    NotHandled,
}

impl Error {
    pub(super) fn not_handled() -> Self {
        Error {
            kind: ErrorKind::NotHandled,
        }
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::NotHandled => write!(fmt, "input not handled by token"),
        }
    }
}
