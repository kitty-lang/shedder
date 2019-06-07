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
use inkwell::types::FunctionType;
use inkwell::values::BasicValue;
use inkwell::values::BasicValueEnum;
use inkwell::values::FunctionValue;
use inkwell::values::PointerValue;
use inkwell::OptimizationLevel;

use crate::lexer::Ident;

use super::error::*;

#[derive(Debug)]
pub(super) struct Compiler {
    pub(super) ctx: Context,
    builder: Builder,
    module: Module,
    // TODO: passes
    funcs: FnvHashMap<Ident, Func>,
}

#[derive(Debug)]
pub(super) struct State<'s> {
    pub(super) func: &'s Ident,
    pub(super) block: &'s str,
}

#[derive(Debug)]
struct Func {
    func: FunctionValue,
    blocks: FnvHashMap<String, Block>,
}

#[derive(Debug)]
struct Block {
    block: BasicBlock,
    vars: FnvHashMap<Ident, Var>,
}

#[derive(Debug)]
enum Var {
    Alias(Ident),
    Global(PointerValue),
}

#[derive(Debug)]
pub struct Compiled {
    module: Module,
    tm: Option<TargetMachine>,
}

pub(super) trait Compile<'c> {
    #[allow(unused)]
    fn prepare(&self, compiler: &mut Compiler, state: &mut State<'c>) {}

    #[allow(unused)]
    fn compile(&self, compiler: &mut Compiler, state: &mut State<'c>) -> Result<()> {
        Ok(())
    }
}

impl Compiler {
    pub(super) fn new(module: &str) -> Self {
        let ctx = Context::create();
        let builder = ctx.create_builder();
        let module = ctx.create_module(module);

        Compiler {
            ctx,
            builder,
            module,
            funcs: FnvHashMap::default(),
        }
    }

    pub(super) fn add_function(&mut self, name: Ident, ty: FunctionType) {
        let func = self.module.add_function(name.inner(), ty, None);

        self.funcs.insert(
            name,
            Func {
                func,
                blocks: FnvHashMap::default(),
            },
        );
    }

    pub(super) fn add_external_function(&mut self, name: Ident, ty: FunctionType) {
        let func = self
            .module
            .add_function(name.inner(), ty, Some(Linkage::AvailableExternally));

        self.funcs.insert(
            name,
            Func {
                func,
                blocks: FnvHashMap::default(),
            },
        );
    }

    pub(super) fn append_block(&mut self, func: &Ident, name: String) {
        let func = self.funcs.get_mut(func).unwrap(); // FIXME
        let block = func.func.append_basic_block(&name);

        func.blocks.insert(
            name,
            Block {
                block,
                vars: FnvHashMap::default(),
            },
        );
    }

    pub(super) fn add_global_string(&mut self, state: &State, name: Ident, string: &str) {
        let block = self
            .funcs
            .get_mut(state.func)
            .unwrap() // FIXME
            .blocks
            .get_mut(state.block)
            .unwrap(); // FIXME
        self.builder.position_at_end(&block.block);

        let var = self
            .builder
            .build_global_string_ptr(string, name.inner())
            .as_pointer_value();
        block.vars.insert(name, Var::Global(var));
    }

    pub(super) fn alias(&mut self, state: &State, alias: Ident, var: Ident) {
        self.funcs
            .get_mut(state.func)
            .unwrap() // FIXME
            .blocks
            .get_mut(state.block)
            .unwrap() // FIXME
            .vars
            .insert(alias, Var::Alias(var));
    }

    pub(super) fn get_var(&self, state: &State, name: &Ident) -> Option<BasicValueEnum> {
        match self
            .funcs
            .get(&state.func)?
            .blocks
            .get(state.block)?
            .vars
            .get(name)?
        {
            Var::Alias(var) => self.get_var(state, var),
            Var::Global(var) => Some((*var).into()),
        }
    }

    pub(super) fn call(&self, state: &State, func: &Ident, args: &[BasicValueEnum]) {
        let block = self
            .funcs
            .get(&state.func)
            .unwrap() // FIXME
            .blocks
            .get(state.block)
            .unwrap(); // FIXME
        self.builder.position_at_end(&block.block);

        let name = func;
        let func = self.funcs.get(name).unwrap(); // FIXME

        self.builder.build_call(
            func.func,
            args,
            name.inner(), // FIXME: custom
        );
    }

    pub(super) fn ret(&self, state: &State, value: Option<&dyn BasicValue>) {
        let block = self
            .funcs
            .get(&state.func)
            .unwrap() // FIXME
            .blocks
            .get(state.block)
            .unwrap(); // FIXME
        self.builder.position_at_end(&block.block);

        self.builder.build_return(value);
    }

    pub(super) fn compiled(self) -> Compiled {
        Compiled {
            module: self.module,
            tm: None,
        }
    }
}

impl Compiled {
    pub fn create_target_machine(&mut self) {
        let opt = OptimizationLevel::None; // FIXME
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

impl Display for Compiler {
    fn fmt(&self, _: &mut Formatter) -> fmt::Result {
        // TODO: ctx
        // TODO: builder
        self.module.print_to_stderr(); // FIXME
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
