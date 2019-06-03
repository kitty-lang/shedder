use cranelift_faerie::FaerieBackend;
use cranelift_module::Module;

use crate::parser::Decl;

use super::compile::Compile;
use super::error::*;

impl<'d> Compile for Decl<'d> {
    fn compile(&self, module: &mut Module<FaerieBackend>) -> Result<()> {
        match self {
            Decl::Func(func) => func.compile(module),
        }
    }
}
