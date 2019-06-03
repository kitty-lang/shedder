use nom::branch::alt;
use nom::character::complete::char;
use nom::multi::many0;
use nom::IResult;

mod lex;
mod token;

pub use token::*;

use lex::Lex;

pub fn lex(input: &str) -> IResult<&str, Vec<Token>> {
    let (input, _) = many0(alt((char(' '), char('\n'))))(input)?;
    let (input, token) = Token::try_lex(input)?;

    if token == Token::EOF {
        return Ok((input, vec![token]));
    }

    let mut tokens = vec![token];

    match lex(input) {
        Ok((input, mut tokens_)) => {
            tokens.append(&mut tokens_);
            Ok((input, tokens))
        }
        Err(ref err) if err.is_incomplete() => Ok((input, tokens)),
        Err(err) => Err(err),
    }
}
