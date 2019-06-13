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
pub struct Literal<'l> {
    pub name: Ident<'l>,
    pub lit: &'l lexer::Literal<'l>,
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
            Expr::Literal(Literal { name, lit }) => Expr::Literal(Literal {
                name: name.as_ref(),
                lit,
            }),
            Expr::Func(Func { name, args }) => Expr::Func(Func {
                name: name.as_ref(),
                args: args.as_ref(),
            }),
            Expr::Var(var) => Expr::Var(var.as_ref()),
        }
    }

    pub(super) fn handled() -> Vec<TokenTy> {
        let mut handled = vec![];
        handled.append(&mut Func::handled());
        handled.push(TokenTy::Literal);
        // handled.push(TokenTy::Ident); // NOTE: already in `Func::handled`
        handled
    }

    pub(super) fn parse(tokens: &'e [Token<'e>]) -> Result<(usize, Expr<'e>)> {
        if tokens.is_empty() {
            return Err(Error::missing_token(Self::handled(), None));
        }

        let mut error = Error::multiple(vec![]);

        if let TokenVariant::Literal(lit) = &tokens[0].token {
            return Ok((
                1,
                Expr::Literal(Literal {
                    name: Ident::Owned(format!("lit{}", LITERALS.fetch_add(1, Ordering::SeqCst))),
                    lit,
                }),
            ));
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
            Expr::Var(var) => write!(fmt, "{}", var),
        }
    }
}

impl<'l> Display for Literal<'l> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "lit(name={}, value={})", self.name.inner(), self.lit)
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
