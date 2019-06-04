use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::path::Path;

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
use inkwell::values::FunctionValue;
use inkwell::values::GlobalValue;
use inkwell::OptimizationLevel;

use super::error::*;

#[derive(Debug)]
pub(super) struct Compiler {
    pub(super) ctx: Context,
    pub(super) builder: Builder,
    // TODO: passes
    module: Module,
}

#[derive(Debug)]
pub struct Compiled {
    module: Module,
    tm: Option<TargetMachine>,
}

pub(super) trait Compile {
    fn compile(&self, compiler: &Compiler) -> Result<()>;
}

impl Compiler {
    pub(super) fn new() -> Self {
        let ctx = Context::create();
        let builder = ctx.create_builder();
        let module = ctx.create_module("main"); // FIXME

        Compiler {
            ctx,
            builder,
            module,
        }
    }

    pub(super) fn get_function(&self, name: &str) -> Option<FunctionValue> {
        self.module.get_function(name)
    }

    pub(super) fn add_global_string_ptr(&self, string: &str, name: &str) -> GlobalValue {
        self.builder.build_global_string_ptr(string, name)
    }

    pub(super) fn add_function(&self, name: &str, ty: FunctionType) -> FunctionValue {
        self.module.add_function(name, ty, None)
    }

    pub(super) fn add_external_function(&self, name: &str, ty: FunctionType) -> FunctionValue {
        self.module
            .add_function(name, ty, Some(Linkage::AvailableExternally))
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
