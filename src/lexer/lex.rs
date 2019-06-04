use nom::bytes::complete::tag;
use nom::Err as NomErr;
use nom::IResult;

pub(super) trait Lex<'l>: Sized {
    fn try_lex(input: &'l str) -> IResult<&'l str, Self>;
}

pub(super) fn is_tag<'i>(input: &'i str, val: &str) -> IResult<&'i str, bool> {
    match tag(val)(input) {
        Ok((input, _)) => Ok((input, true)),
        Err(NomErr::Failure(err)) => Err(NomErr::Failure(err)),
        _ => Ok((input, false)),
    }
}
