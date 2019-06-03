use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while1;
use nom::character::complete::digit1;
use nom::multi::many0;
use nom::sequence::pair;
use nom::Err as NomErr;
use nom::IResult;
use nom::Needed;

use super::token::*;

pub(super) trait Lex: Sized {
    fn try_lex(input: &str) -> IResult<&str, Self>;
}

impl Lex for Token {
    fn try_lex(input: &str) -> IResult<&str, Token> {
        if input.is_empty() {
            return Ok((input, Token::EOF));
        }

        match Symbol::try_lex(input) {
            Ok((input, symbol)) => return Ok((input, Token::Symbol(symbol))),
            Err(NomErr::Failure(err)) => return Err(NomErr::Failure(err)),
            _ => (),
        }

        match Keyword::try_lex(input) {
            Ok((input, keyword)) => return Ok((input, Token::Keyword(keyword))),
            Err(NomErr::Failure(err)) => return Err(NomErr::Failure(err)),
            _ => (),
        }

        match Ident::try_lex(input) {
            Ok((input, ident)) => return Ok((input, Token::Ident(ident))),
            Err(NomErr::Failure(err)) => return Err(NomErr::Failure(err)),
            _ => (),
        }

        Err(NomErr::Incomplete(Needed::Unknown))
    }
}

impl Lex for Symbol {
    fn try_lex(input: &str) -> IResult<&str, Symbol> {
        match tag("(")(input) {
            Ok((input, _)) => return Ok((input, Symbol::LeftParen)),
            Err(NomErr::Failure(err)) => return Err(NomErr::Failure(err)),
            _ => (),
        }

        match tag(")")(input) {
            Ok((input, _)) => return Ok((input, Symbol::RightParen)),
            Err(NomErr::Failure(err)) => return Err(NomErr::Failure(err)),
            _ => (),
        }

        match tag("{")(input) {
            Ok((input, _)) => return Ok((input, Symbol::LeftBracket)),
            Err(NomErr::Failure(err)) => return Err(NomErr::Failure(err)),
            _ => (),
        }

        match tag("}")(input) {
            Ok((input, _)) => return Ok((input, Symbol::RightBracket)),
            Err(NomErr::Failure(err)) => return Err(NomErr::Failure(err)),
            _ => (),
        }

        Err(NomErr::Incomplete(Needed::Unknown))
    }
}

impl Lex for Keyword {
    fn try_lex(input: &str) -> IResult<&str, Keyword> {
        match tag("func")(input) {
            Ok((input, _)) => Ok((input, Keyword::Func)),
            Err(err) => Err(err),
        }
    }
}

impl Lex for Ident {
    fn try_lex(input: &str) -> IResult<&str, Ident> {
        match pair(
            alt((digit1, lowercase1)),
            many0(alt((digit1, lowercase1, is_a("_")))),
        )(input)
        {
            Ok((input, (first, next))) => {
                let mut ident = first.to_string();
                for next in next {
                    ident.push_str(next);
                }
                Ok((input, Ident(ident)))
            }
            Err(err) => Err(err),
        }
    }
}

fn lowercase1(input: &str) -> IResult<&str, &str> {
    take_while1(|chr: char| {
        let chr = chr as u8;
        chr >= 0x61 && chr <= 0x7a
    })(input)
}
