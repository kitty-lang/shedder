use nom::branch::alt;
use nom::character::complete::char;
use nom::multi::many0;
use nom::IResult;

mod lex;
mod token;

pub use token::*;

pub fn lex(input: &str) -> IResult<&str, Vec<Token>> {
    let mut pos = Position { line: 0, col: 0 };

    lex_(input, &mut pos)
}

fn lex_<'i>(input: &'i str, pos: &mut Position) -> IResult<&'i str, Vec<Token<'i>>> {
    let (input, empty) = many0(alt((char(' '), char('\n'), char('\t'))))(input)?;

    for empty in empty {
        if empty == ' ' {
            pos.col += 1;
        } else if empty == '\n' {
            pos.line += 1;
            pos.col = 0;
        } else if empty == '\t' {
            pos.col += 4;
        }
    }

    let (input, token) = Token::lex(input, pos)?;

    if token.is_eof() {
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
