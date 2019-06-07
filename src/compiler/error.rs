use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use inkwell::support::LLVMString;

pub type Result<OK> = std::result::Result<OK, Error>;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Error {
    kind: ErrorKind,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum ErrorKind {
    LLVM(String),
    MissingTargetMachine,
}

impl Error {
    pub(super) fn llvm(string: LLVMString) -> Self {
        Error {
            kind: ErrorKind::LLVM(string.to_string()),
        }
    }

    pub(super) fn missing_target_machine() -> Self {
        Error {
            kind: ErrorKind::MissingTargetMachine,
        }
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match &self.kind {
            ErrorKind::LLVM(err) => write!(fmt, "LLVM error: {}", err),
            ErrorKind::MissingTargetMachine => write!(fmt, "missing target machine"),
        }
    }
}
