use cranelift_faerie::FaerieBackend;
use cranelift_module::Module;

use super::error::Result;

pub trait Compile {
    fn compile(&self, module: &mut Module<FaerieBackend>) -> Result<()>;
}
