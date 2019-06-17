use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use crate::lexer;
use crate::lexer::Ident;
use crate::lexer::Symbol;
use crate::lexer::Token;
use crate::lexer::TokenTy;
use crate::lexer::TokenVariant;

use super::split;
use super::try_eq_symbol;
use super::try_get_ident;

use super::error::*;

static LITERALS: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
pub enum Expr<'e> {
    Literal(Literal<'e>),
    Func(Func<'e>),
    Var(Ident<'e>),
}

#[derive(Debug)]
pub enum Literal<'l> {
    Int {
        name: Ident<'l>,
        int: i32,
    },
    String {
        name: Ident<'l>,
        string: &'l str,
    },
    RefDynString {
        name: Ident<'l>,
        segs: &'l [DynStringSeg<'l>],
    },
    OwnedDynString {
        name: Ident<'l>,
        segs: Vec<DynStringSeg<'l>>,
    },
}

#[derive(Debug)]
pub enum DynStringSeg<'s> {
    String(&'s str),
    Expr(Expr<'s>),
}

#[derive(Debug)]
pub struct Func<'f> {
    pub name: Ident<'f>,
    pub args: Args<'f>,
}

#[derive(Debug)]
pub enum Args<'a> {
    Ref(&'a [Expr<'a>]),
    Owned(Vec<Expr<'a>>),
}

impl<'e> Expr<'e> {
    pub fn as_ref(&'e self) -> Expr<'e> {
        match self {
            Expr::Literal(lit) => Expr::Literal(lit.as_ref()),
            Expr::Func(Func { name, args }) => Expr::Func(Func {
                name: name.as_ref(),
                args: args.as_ref(),
            }),
            Expr::Var(var) => Expr::Var(var.as_ref()),
        }
    }

    pub(super) fn handled() -> Vec<TokenTy> {
        let mut handled = vec![];
        handled.push(TokenTy::Literal);
        handled.append(&mut Func::handled());
        handled.push(TokenTy::Ident);
        handled
    }

    pub(super) fn parse(tokens: &'e [Token<'e>]) -> Result<(usize, Expr<'e>)> {
        if tokens.is_empty() {
            return Err(Error::missing_token(Self::handled(), None));
        }

        let mut error = Error::multiple(vec![]);

        if let TokenVariant::Literal(lit) = &tokens[0].token {
            match lit {
                lexer::Literal::Int(int) => {
                    return Ok((
                        1,
                        Expr::Literal(Literal::Int {
                            name: Ident::Owned(format!(
                                "lit{}",
                                LITERALS.fetch_add(1, Ordering::SeqCst)
                            )),
                            int: *int,
                        }),
                    ))
                }
                lexer::Literal::String(string) => {
                    return Ok((
                        1,
                        Expr::Literal(Literal::String {
                            name: Ident::Owned(format!(
                                "lit{}",
                                LITERALS.fetch_add(1, Ordering::SeqCst)
                            )),
                            string,
                        }),
                    ))
                }
                lexer::Literal::DynString(segs_) => {
                    let mut segs = vec![];
                    for seg in segs_ {
                        match seg {
                            lexer::DynStringSeg::String(string) => {
                                segs.push(DynStringSeg::String(string));
                            }
                            lexer::DynStringSeg::Insert(tokens) => {
                                let (i, expr) = Expr::parse(tokens)?;
                                assert_eq!(i, tokens.len()); // FIXME
                                segs.push(DynStringSeg::Expr(expr));
                            }
                        }
                    }

                    return Ok((
                        1,
                        Expr::Literal(Literal::OwnedDynString {
                            name: Ident::Owned(format!(
                                "lit{}",
                                LITERALS.fetch_add(1, Ordering::SeqCst)
                            )),
                            segs,
                        }),
                    ));
                }
            }
        } else {
            error = error.concat(Error::wrong_token(&tokens[0], vec![TokenTy::Literal]));
        }

        match Func::parse(tokens) {
            Ok((t, func)) => return Ok((t, Expr::Func(func))),
            Err(mut err) => {
                error = error.concat({
                    err.max_after(tokens.get(1).map(|token| token.pos));
                    err
                })
            }
        }

        if let TokenVariant::Ident(var) = &tokens[0].token {
            return Ok((1, Expr::Var(var.as_ref())));
        } else {
            error = error.concat(Error::wrong_token(&tokens[0], vec![TokenTy::Ident]));
        }

        Err(error)
    }
}

impl<'l> Literal<'l> {
    pub fn as_ref(&'l self) -> Literal<'l> {
        match self {
            Literal::Int { name, int } => Literal::Int {
                name: name.as_ref(),
                int: *int,
            },
            Literal::String { name, string } => Literal::String {
                name: name.as_ref(),
                string,
            },
            Literal::RefDynString { name, segs } => Literal::RefDynString {
                name: name.as_ref(),
                segs,
            },
            Literal::OwnedDynString { name, segs } => Literal::RefDynString {
                name: name.as_ref(),
                segs,
            },
        }
    }

    pub fn name(&'l self) -> Ident<'l> {
        match self {
            Literal::Int { name, .. } => name.as_ref(),
            Literal::String { name, .. } => name.as_ref(),
            Literal::RefDynString { name, .. } => name.as_ref(),
            Literal::OwnedDynString { name, .. } => name.as_ref(),
        }
    }
}

impl<'f> Func<'f> {
    fn handled() -> Vec<TokenTy> {
        vec![TokenTy::Ident]
    }

    fn parse(tokens: &'f [Token<'f>]) -> Result<(usize, Func<'f>)> {
        let name = try_get_ident(tokens, 0)?.as_ref();

        let mut t = 1;
        try_eq_symbol(tokens, t, Symbol::LeftParen).map_err(|mut err| {
            err.max_after(tokens.get(t - 1).map(|token| token.pos));
            err
        })?;

        let mut args = vec![];

        t += 1;
        loop {
            println!("{:?}", split(tokens, t));
            if t >= tokens.len() {
                let mut handled = Expr::handled();
                handled.push(TokenTy::Symbol(Symbol::RightParen));
                return Err(Error::missing_token(handled, Some(tokens[t - 1].pos)));
            }

            if tokens[t].eq_symbol(Symbol::RightParen) {
                t += 1;
                break;
            }

            if !args.is_empty() {
                try_eq_symbol(tokens, t, Symbol::Comma).map_err(|mut err| {
                    err.max_after(tokens.get(t - 1).map(|token| token.pos));
                    err
                })?;

                t += 1;
            }

            let (t_, expr) = Expr::parse(split(tokens, t))?;

            args.push(expr);
            t += t_;
        }

        Ok((
            t,
            Func {
                name,
                args: Args::Owned(args),
            },
        ))
    }
}

impl<'a> Args<'a> {
    pub fn len(&self) -> usize {
        self.inner().len()
    }

    pub fn inner(&self) -> &[Expr] {
        match self {
            Args::Ref(args) => args,
            Args::Owned(args) => args,
        }
    }

    pub fn as_ref(&'a self) -> Args<'a> {
        match self {
            Args::Ref(args) => Args::Ref(args),
            Args::Owned(args) => Args::Ref(args),
        }
    }
}

impl<'e> Display for Expr<'e> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "expr::")?;
        match self {
            Expr::Literal(lit) => write!(fmt, "{}", lit),
            Expr::Func(func) => write!(fmt, "{}", func),
            Expr::Var(var) => write!(fmt, "var({})", var.inner()),
        }
    }
}

impl<'l> Display for Literal<'l> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Literal::Int { name, int } => {
                write!(fmt, "lit(name={}, value=int({}))", name.inner(), int)
            }
            Literal::String { name, string } => {
                write!(fmt, "lit(name={}, value=string({}))", name.inner(), string)
            }
            Literal::RefDynString { name, segs } => {
                write!(fmt, "lit(name={}, value=dyn_string[", name.inner())?;

                for seg in *segs {
                    write!(fmt, " {} ", seg)?;
                }

                write!(fmt, "]")
            }
            Literal::OwnedDynString { name, segs } => {
                write!(fmt, "lit(name={}, value=dyn_string[", name.inner())?;

                for seg in segs {
                    write!(fmt, " {} ", seg)?;
                }

                write!(fmt, "]")
            }
        }
    }
}

impl<'s> Display for DynStringSeg<'s> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            DynStringSeg::String(string) => write!(fmt, "string({:?}", string),
            DynStringSeg::Expr(expr) => write!(fmt, "{}", expr),
        }
    }
}

impl<'f> Display for Func<'f> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "func(name={}, args=[", self.name.inner())?;

        for arg in self.args.inner() {
            write!(fmt, " {} ", arg)?;
        }

        write!(fmt, "])")
    }
}
