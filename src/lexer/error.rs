use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use super::Position;

pub type Result<OK> = std::result::Result<OK, Error>;

#[derive(Eq, PartialEq, Debug)]
pub struct Error {
    kind: ErrorKind,
}

#[derive(Eq, PartialEq, Debug)]
pub enum ErrorKind {
    NotHandled(Position),
}

impl Error {
    pub(super) fn not_handled(pos: Position) -> Error {
        Error {
            kind: ErrorKind::NotHandled(pos),
        }
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::NotHandled(pos) => write!(fmt, "input not handled by token at {}", pos),
        }
    }
}
