use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use super::error::*;
use super::split;
use super::Position;
use super::Token;
use super::TokenVariant;

#[derive(Debug)]
pub enum Literal<'l> {
    Int(i32),
    String(&'l str),
    DynString(Vec<DynStringSeg<'l>>),
}

#[derive(Debug)]
pub enum DynStringSeg<'s> {
    String(&'s str),
    Insert(Vec<Token<'s>>),
}

impl<'l> Literal<'l> {
    pub(super) fn lex(input: &'l str, pos: &mut Position) -> Result<(&'l str, Token<'l>)> {
        match (input.get(0..1), input.get(1..)) {
            (Some(_), None) => Err(Error::not_handled(*pos)),
            (Some(r#"""#), Some(rest)) => {
                let tpos = *pos;

                let mut esc = false;
                let mut i = 0;
                for chr in rest.chars() {
                    match chr {
                        '\\' => {
                            esc = true;
                            i += 1;
                            pos.col += 1;
                            continue;
                        }
                        '"' => {
                            i += 1;
                            pos.col += 1;

                            if !esc {
                                return Ok((
                                    split(rest, i),
                                    Literal::String(&rest[0..i - 1]).token(tpos),
                                ));
                            }
                        }
                        '\n' => {
                            i += 1;
                            pos.line += 1;
                            pos.col = 0;
                        }
                        _ => {
                            i += 1;
                            pos.col += 1;
                        }
                    }

                    esc = false;
                }

                let epos = *pos;
                *pos = tpos;

                Err(Error::not_handled(epos))
            }
            (Some("`"), Some(rest)) => {
                let tpos = *pos;

                let mut esc = false;
                let mut segs = vec![];
                let mut last = 0;
                let mut i = 0;

                'main: loop {
                    if rest.len() <= i {
                        break;
                    }

                    match &rest[i..=i] {
                        "\\" => {
                            esc = true;
                            pos.col += 1;
                            continue;
                        }
                        "`" => {
                            i += 1;
                            pos.col += 1;

                            if !esc {
                                segs.push(DynStringSeg::String(&rest[last..i - 1]));

                                return Ok((split(rest, i), Literal::DynString(segs).token(tpos)));
                            }
                        }
                        "$" => {
                            i += 1;
                            pos.col += 1;

                            if &rest[i..=i] == "{" {
                                segs.push(DynStringSeg::String(&rest[last..i - 1]));
                                i += 1;
                                pos.col += 1;

                                let mut tokens = vec![];
                                loop {
                                    if &rest[i..=i] == "}" {
                                        i += 1;
                                        pos.col += 1;

                                        break;
                                    } else if rest.len() <= i {
                                        break 'main;
                                    }

                                    match Token::lex(split(rest, i), pos)? {
                                        (rest_, Some(token)) => {
                                            tokens.push(token);
                                            i = rest.len() - rest_.len();
                                        }
                                        (rest_, None) => i = rest.len() - rest_.len(),
                                    }
                                }

                                last = i;
                                segs.push(DynStringSeg::Insert(tokens));
                            }
                        }
                        "\n" => {
                            i += 1;
                            pos.line += 1;
                            pos.col = 0;
                        }
                        _ => {
                            i += 1;
                            pos.col += 1;
                        }
                    }

                    esc = false;
                }

                let epos = *pos;
                *pos = tpos;

                Err(Error::not_handled(epos))
            }
            (Some(_), Some(_)) => {
                let tpos = *pos;

                let mut i = 0;
                for chr in input.chars() {
                    if chr.is_numeric() {
                        i += 1;
                        pos.col += 1;
                    } else if i > 0 {
                        return Ok((
                            split(input, i),
                            Literal::Int(
                                i32::from_str_radix(&input[0..i], 10).unwrap(), // FIXME
                            )
                            .token(tpos),
                        ));
                    } else {
                        break;
                    }
                }

                let epos = *pos;
                *pos = tpos;

                Err(Error::not_handled(epos))
            }
            _ => Err(Error::not_handled(*pos)),
        }
    }

    fn token(self, pos: Position) -> Token<'l> {
        Token {
            token: TokenVariant::Literal(self),
            pos,
        }
    }
}

impl<'l> Display for Literal<'l> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Literal::Int(int) => write!(fmt, "lit::int({})", int),
            Literal::String(string) => write!(fmt, "lit::string({:?})", string),
            Literal::DynString(string) => {
                write!(fmt, "lit::dyn_string([")?;

                for seg in string {
                    write!(fmt, " {} ", seg)?;
                }

                write!(fmt, "])")
            }
        }
    }
}

impl<'s> Display for DynStringSeg<'s> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            DynStringSeg::String(string) => write!(fmt, "seg::string({:?})", string),
            DynStringSeg::Insert(insert) => {
                write!(fmt, "seg::insert([")?;

                for token in insert {
                    write!(fmt, " {} ", token)?;
                }

                write!(fmt, "])")
            }
        }
    }
}
