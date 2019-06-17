use inkwell::types::AnyTypeEnum;
use inkwell::types::BasicTypeEnum;
use inkwell::values::BasicValueEnum;

use crate::parser::expr::DynStringSeg;
use crate::parser::expr::Expr;
use crate::parser::expr::Func;
use crate::parser::expr::Literal;

use super::compile::Compiler;
use super::compile::State;
use super::error::*;

impl<'e> Expr<'e> {
    pub(super) fn prepare(&'e self, compiler: &mut Compiler<'e>, state: &mut State<'e>) {
        match self {
            Expr::Literal(lit) => lit.prepare(compiler, state),
            Expr::Func(func) => func.prepare(compiler, state),
            Expr::Var(_) => (),
        }
    }

    pub(super) fn compile(
        &'e self,
        compiler: &mut Compiler<'e>,
        state: &mut State<'e>,
    ) -> Result<()> {
        match self {
            Expr::Literal(_) => Ok(()),
            Expr::Func(func) => func.compile(compiler, state),
            Expr::Var(_) => Ok(()),
        }
    }

    fn as_string(&self, compiler: &Compiler, state: &State) -> String {
        match self {
            Expr::Literal(lit) => lit.as_string(compiler, state),
            Expr::Func(_) => unimplemented!(),
            Expr::Var(var) => {
                let mut ty = match compiler.get_var(state, var).unwrap()[0] // FIXME
                    .get_type()
                {
                    BasicTypeEnum::ArrayType(_) => unimplemented!(), // FIXME
                    BasicTypeEnum::IntType(_) => return "%i".into(),
                    BasicTypeEnum::FloatType(_) => unimplemented!(), // FIXME
                    BasicTypeEnum::PointerType(ptr) => {
                        // --- FIXME ---
                        // (better detection)
                        let ty = ptr.get_element_type();
                        if let AnyTypeEnum::IntType(int) = ty {
                            if int.get_bit_width() == 8 {
                                return "%s".into();
                            }
                        }
                        // --- FIXME ---

                        ty
                    }
                    BasicTypeEnum::StructType(_) => unimplemented!(), // FIXME
                    BasicTypeEnum::VectorType(_) => unimplemented!(), // FIXME
                };

                loop {
                    match ty {
                        AnyTypeEnum::ArrayType(_) => unimplemented!(), // FIXME
                        AnyTypeEnum::FloatType(_) => unimplemented!(), // FIXME
                        AnyTypeEnum::FunctionType(_) => unimplemented!(), // FIXME
                        AnyTypeEnum::IntType(_) => return "%i".into(),
                        AnyTypeEnum::PointerType(ptr) => ty = ptr.get_element_type(),
                        AnyTypeEnum::StructType(_) => unimplemented!(), // FIXME
                        AnyTypeEnum::VectorType(_) => unimplemented!(), // FIXME
                        AnyTypeEnum::VoidType(_) => unimplemented!(),   // FIXME
                    }
                }
            }
        }
    }
}

impl<'l> Literal<'l> {
    pub(super) fn prepare(&'l self, compiler: &mut Compiler<'l>, state: &mut State<'l>) {
        match self {
            Literal::Int { name, int } => {
                compiler.const_int(state, name.as_ref(), *int as u64);
            }
            Literal::String { name, string } => {
                compiler.add_global_string(state, name.as_ref(), &string.replace("\\\"", "\""));
            }
            Literal::RefDynString { name, segs } => {
                let mut string = String::new();
                for seg in *segs {
                    string.push_str(&seg.as_string(compiler, state));
                }

                compiler.add_global_string(state, name.as_ref(), &string);
            }
            Literal::OwnedDynString { name, segs } => {
                let mut string = String::new();
                for seg in segs {
                    string.push_str(&seg.as_string(compiler, state));
                }

                compiler.add_global_string(state, name.as_ref(), &string);
            }
        }
    }

    fn as_string(&self, compiler: &Compiler, state: &State) -> String {
        match self {
            Literal::Int { .. } => "%i".into(),
            Literal::String { .. } => "%s".into(),
            Literal::RefDynString { segs, .. } => {
                let mut string = String::new();

                for seg in *segs {
                    match seg {
                        DynStringSeg::String(string_) => string.push_str(string_),
                        DynStringSeg::Expr(expr) => {
                            string.push_str(&expr.as_string(compiler, state))
                        }
                    }
                }

                string
            }
            Literal::OwnedDynString { segs, .. } => {
                let mut string = String::new();

                for seg in segs {
                    match seg {
                        DynStringSeg::String(string_) => string.push_str(string_),
                        DynStringSeg::Expr(expr) => {
                            string.push_str(&expr.as_string(compiler, state))
                        }
                    }
                }

                string
            }
        }
    }
}

impl<'s> DynStringSeg<'s> {
    fn as_string(&self, compiler: &Compiler, state: &State) -> String {
        // FIXME: borrow
        match self {
            DynStringSeg::String(string) => string.to_string(), // FIXME: borrow
            DynStringSeg::Expr(expr) => expr.as_string(compiler, state),
        }
    }
}

impl<'f> Func<'f> {
    pub(super) fn prepare(&'f self, compiler: &mut Compiler<'f>, state: &mut State<'f>) {
        for arg in self.args.inner() {
            match arg {
                Expr::Literal(lit) => lit.prepare(compiler, state),
                Expr::Func(func) => func.prepare(compiler, state),
                Expr::Var(_) => (),
            }
        }
    }

    pub(super) fn compile(&self, compiler: &mut Compiler<'f>, state: &mut State<'f>) -> Result<()> {
        self.call(compiler, state);
        Ok(())
    }

    pub(super) fn call(&self, compiler: &Compiler, state: &State) -> Option<BasicValueEnum> {
        let mut args = vec![];

        for arg in self.args.inner() {
            match arg {
                Expr::Literal(lit) => {
                    for var in compiler.get_var(&state, &lit.name()).unwrap() {
                        // FIXME
                        args.push(var);
                    }
                }
                Expr::Func(func) => {
                    args.push(func.call(compiler, state).unwrap()); // FIXME
                }
                Expr::Var(var) => {
                    for var in compiler.get_var(&state, var).unwrap() {
                        // FIXME
                        args.push(var);
                    }
                }
            }
        }

        compiler.call(state, &self.name, &args)
    }
}
