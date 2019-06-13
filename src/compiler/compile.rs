use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::path::Path;

use fnv::FnvHashMap;
use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Linkage;
use inkwell::module::Module;
use inkwell::targets::CodeModel;
use inkwell::targets::FileType;
use inkwell::targets::RelocMode;
use inkwell::targets::Target;
use inkwell::targets::TargetMachine;
use inkwell::types::BasicTypeEnum;
use inkwell::types::FunctionType;
use inkwell::values::BasicValue;
use inkwell::values::BasicValueEnum;
use inkwell::values::FunctionValue;
use inkwell::values::PointerValue;
use inkwell::AddressSpace;
use inkwell::OptimizationLevel;

use crate::lexer::Ident;
use crate::lexer::Ty;

use super::error::*;

#[derive(Debug)]
pub(super) struct Compiler<'c> {
    pub(super) ctx: Context,
    builder: Builder,
    module: Option<Module>,
    modules: Vec<Module>,
    // TODO: passes
    funcs: FnvHashMap<Ident<'c>, Func<'c>>,
}

#[derive(Debug)]
pub(super) struct State<'s> {
    pub(super) func: Ident<'s>,
    pub(super) block: Ident<'s>,
}

#[derive(Debug)]
struct Func<'f> {
    func: FunctionValue,
    blocks: FnvHashMap<Ident<'f>, Block<'f>>,
}

#[derive(Debug)]
struct Block<'b> {
    block: BasicBlock,
    vars: FnvHashMap<Ident<'b>, Var<'b>>,
}

#[derive(Debug)]
enum Var<'v> {
    Alias(Ident<'v>),
    Global(PointerValue),
}

#[derive(Debug)]
pub(super) enum CompilerTy<'t> {
    Ty(Ty),
    FunctionType {
        args: &'t [CompilerTy<'t>],
        ret: &'t CompilerTy<'t>,
    },
}

#[derive(Debug)]
pub struct Compiled {
    module: Module,
    tm: Option<TargetMachine>,
}

impl<'c> Compiler<'c> {
    pub(super) fn new() -> Compiler<'c> {
        let ctx = Context::create();
        let builder = ctx.create_builder();

        Compiler {
            ctx,
            builder,
            module: None,
            modules: vec![],
            funcs: FnvHashMap::default(),
        }
    }

    fn module(&self) -> &Module {
        self.module.as_ref().unwrap() // FIXME
    }

    pub(super) fn new_module(&mut self, module: &str) {
        if let Some(module) = self.module.take() {
            self.modules.push(module);
        }

        self.module = Some(self.ctx.create_module(module));
    }

    pub(super) fn add_function(&mut self, name: Ident<'c>, ty: CompilerTy) {
        let func = self
            .module()
            .add_function(name.inner(), ty.as_fn_type(&self.ctx, &[]), None);

        self.funcs.insert(
            name,
            Func {
                func,
                blocks: FnvHashMap::default(),
            },
        );
    }

    pub(super) fn add_external_function(&mut self, name: Ident<'c>, ty: CompilerTy) {
        let func = self.module().add_function(
            name.inner(),
            ty.as_fn_type(&self.ctx, &[]),
            Some(Linkage::AvailableExternally),
        );

        self.funcs.insert(
            name,
            Func {
                func,
                blocks: FnvHashMap::default(),
            },
        );
    }

    pub(super) fn append_block(&mut self, func: &Ident<'c>, name: Ident<'c>) {
        let func = self.funcs.get_mut(func).unwrap(); // FIXME
        let block = func.func.append_basic_block(name.inner());

        func.blocks.insert(
            name,
            Block {
                block,
                vars: FnvHashMap::default(),
            },
        );
    }

    pub(super) fn add_global_string(&mut self, state: &State<'c>, name: Ident<'c>, string: &str) {
        let block = self
            .funcs
            .get_mut(&state.func)
            .unwrap() // FIXME
            .blocks
            .get_mut(&state.block)
            .unwrap(); // FIXME
        self.builder.position_at_end(&block.block);

        let var = self
            .builder
            .build_global_string_ptr(string, name.inner())
            .as_pointer_value();
        block.vars.insert(name, Var::Global(var));
    }

    pub(super) fn alias(&mut self, state: &State<'c>, alias: Ident<'c>, var: Ident<'c>) {
        self.funcs
            .get_mut(&state.func)
            .unwrap() // FIXME
            .blocks
            .get_mut(&state.block)
            .unwrap() // FIXME
            .vars
            .insert(alias, Var::Alias(var));
    }

    pub(super) fn get_var(&self, state: &State, name: &Ident) -> Option<BasicValueEnum> {
        match self
            .funcs
            .get(&state.func)?
            .blocks
            .get(&state.block)?
            .vars
            .get(name)?
        {
            Var::Alias(var) => self.get_var(state, var),
            Var::Global(var) => Some((*var).into()),
        }
    }

    pub(super) fn call(
        &self,
        state: &State,
        func: &Ident,
        args: &[BasicValueEnum],
    ) -> Option<BasicValueEnum> {
        let block = self
            .funcs
            .get(&state.func)
            .unwrap() // FIXME
            .blocks
            .get(&state.block)
            .unwrap(); // FIXME
        self.builder.position_at_end(&block.block);

        let name = func;
        let func = self.funcs.get(name).unwrap(); // FIXME

        let call = self.builder.build_call(
            func.func,
            args,
            name.inner(), // FIXME: custom
        );

        call.try_as_basic_value().left()
    }

    pub(super) fn ret(&self, state: &State, value: Option<&dyn BasicValue>) {
        let block = self
            .funcs
            .get(&state.func)
            .unwrap() // FIXME
            .blocks
            .get(&state.block)
            .unwrap(); // FIXME
        self.builder.position_at_end(&block.block);

        self.builder.build_return(value);
    }

    pub(super) fn compiled(self) -> Compiled {
        let module = self.ctx.create_module("main");

        if let Some(module_) = self.module {
            module.link_in_module(module_).unwrap(); // FIXME
        }

        for module_ in self.modules {
            module.link_in_module(module_).unwrap(); // FIXME
        }

        Compiled { module, tm: None }
    }
}

impl<'t> CompilerTy<'t> {
    pub(super) fn fn_type(&'t self, args: &'t [CompilerTy<'t>]) -> CompilerTy<'t> {
        CompilerTy::FunctionType { args, ret: &self }
    }

    fn as_basic_type(&self, ctx: &Context) -> BasicTypeEnum {
        match self {
            CompilerTy::Ty(ty) => match ty {
                Ty::Str => {
                    ctx.i8_type()
                        .ptr_type(AddressSpace::Generic) // TODO: choose address space
                        .into()
                }
                Ty::Void => panic!(), // FIXME
            },
            CompilerTy::FunctionType { .. } => panic!(), // FIXME
        }
    }

    fn as_fn_type(&self, ctx: &Context, args: &[BasicTypeEnum]) -> FunctionType {
        match self {
            CompilerTy::Ty(ty) => match ty {
                Ty::Str => {
                    ctx.i8_type()
                        .ptr_type(AddressSpace::Generic) // TODO: choose address space
                        .fn_type(args, false) // TODO: support variadic functions
                }
                Ty::Void => ctx.void_type().fn_type(&[], false), // TODO: support variadic functions
            },
            CompilerTy::FunctionType { args: args_, ret } => {
                assert!(args.is_empty()); // FIXME

                let mut args = vec![];
                for arg in *args_ {
                    args.push(arg.as_basic_type(ctx));
                }

                ret.as_fn_type(ctx, &args)
            }
        }
    }
}

impl Compiled {
    pub fn create_target_machine(&mut self) {
        let opt = OptimizationLevel::None; // TODO: custom opt leve
        let reloc = RelocMode::Default;
        let model = CodeModel::Default;

        let target = Target::from_name("x86-64").unwrap();
        self.tm = Some(
            target
                .create_target_machine("x86_64-pc-linux-gnu", "x86-64", "", opt, reloc, model)
                .unwrap(),
        );
    }

    pub fn write_to_file(&self, path: &Path) -> Result<()> {
        let tm = if let Some(tm) = &self.tm {
            tm
        } else {
            return Err(Error::missing_target_machine());
        };

        match tm.write_to_file(
            &self.module,
            FileType::Object, // FIXME
            path,
        ) {
            Ok(()) => Ok(()),
            Err(err) => Err(Error::llvm(err)),
        }
    }
}

impl<'t> From<Ty> for CompilerTy<'t> {
    fn from(ty: Ty) -> CompilerTy<'t> {
        CompilerTy::Ty(ty)
    }
}

impl<'c> Display for Compiler<'c> {
    fn fmt(&self, _: &mut Formatter) -> fmt::Result {
        // TODO: ctx
        // TODO: builder

        if let Some(module) = &self.module {
            module.print_to_stderr(); // FIXME
        }

        for module in &self.modules {
            module.print_to_stderr(); // FIXME
        }

        // TODO: funcs

        Ok(())
    }
}

impl Display for Compiled {
    fn fmt(&self, _: &mut Formatter) -> fmt::Result {
        // TODO: tm
        self.module.print_to_stderr(); // FIXME
        Ok(())
    }
}
