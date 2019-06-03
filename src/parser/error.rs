use crate::lexer::Token;
use crate::lexer::TokenVariant;

pub type Result<'e, OK> = std::result::Result<OK, Error<'e>>;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Error<'e> {
    kind: ErrorKind<'e>,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum ErrorKind<'e> {
    WrongToken {
        token: &'e Token,
        handled: Vec<TokenVariant>,
    },
}

impl<'e> Error<'e> {
    pub(super) fn wrong_token(token: &'e Token, handled: Vec<TokenVariant>) -> Self {
        Error {
            kind: ErrorKind::WrongToken { token, handled },
        }
    }

    pub fn into_kind(self) -> ErrorKind<'e> {
        self.kind
    }
}
