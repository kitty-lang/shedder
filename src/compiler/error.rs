pub type Result<OK> = std::result::Result<OK, Error>;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Error {
    kind: ErrorKind,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum ErrorKind {}
