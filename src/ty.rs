use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

use inkwell::context::Context;
use inkwell::types::BasicTypeEnum;
use inkwell::types::FunctionType;
use inkwell::types::IntType;
use inkwell::types::PointerType;
use inkwell::types::VoidType;
use inkwell::AddressSpace;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Ty {
    Str,
    Void,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum RawTy<'t> {
    I8,
    I32,
    Void,
    Ptr(&'t RawTy<'t>),
}

impl Ty {
    pub fn str() -> Ty {
        Ty::Str
    }

    pub fn void() -> Ty {
        Ty::Void
    }

    pub fn raw(&self) -> RawTy {
        match self {
            Ty::Str => RawTy::Ptr(&RawTy::I8),
            Ty::Void => RawTy::Void,
        }
    }
}

impl<'t> RawTy<'t> {
    pub fn int32() -> RawTy<'t> {
        RawTy::I32
    }

    pub fn void() -> RawTy<'t> {
        RawTy::Void
    }

    pub fn to_int(&self, ctx: &Context) -> IntType {
        match self {
            RawTy::I8 => ctx.i8_type(),
            RawTy::I32 => ctx.i32_type(),
            _ => panic!("wrong raw type"),
        }
    }

    pub fn to_void(&self, ctx: &Context) -> VoidType {
        ctx.void_type()
    }

    pub fn to_ptr(&self, ctx: &Context) -> PointerType {
        match self {
            RawTy::Ptr(ty) => match ty {
                RawTy::I8 | RawTy::I32 => ty.to_int(ctx).ptr_type(AddressSpace::Generic), // FIXME
                RawTy::Void => ty.to_void(ctx).ptr_type(AddressSpace::Generic),           // FIXME
                RawTy::Ptr(_) => ty.to_ptr(ctx).ptr_type(AddressSpace::Generic),          // FIXME
            },
            _ => panic!("wrong raw type"),
        }
    }

    pub fn fn_type(
        &self,
        ctx: &Context,
        args_tys: &[BasicTypeEnum],
        is_var_args: bool,
    ) -> FunctionType {
        match self {
            RawTy::I8 | RawTy::I32 => self.to_int(ctx).fn_type(args_tys, is_var_args),
            RawTy::Void => self.to_void(ctx).fn_type(args_tys, is_var_args),
            RawTy::Ptr(_) => self.to_ptr(ctx).fn_type(args_tys, is_var_args),
        }
    }
}

impl Display for Ty {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            Ty::Str => write!(fmt, "ty(str)"),
            Ty::Void => write!(fmt, "ty(void)"),
        }
    }
}

impl<'t> Display for RawTy<'t> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            RawTy::I8 => write!(fmt, "raw(i8)"),
            RawTy::I32 => write!(fmt, "raw(i32)"),
            RawTy::Void => write!(fmt, "raw(voide)"),
            RawTy::Ptr(ty) => write!(fmt, "raw(ptr({}))", ty),
        }
    }
}
