use cranelift::codegen::ir::ExternalName;
use cranelift::codegen::ir::Function;
use cranelift::codegen::isa::CallConv;
use cranelift::frontend::FunctionBuilder;
use cranelift::prelude::*;
use cranelift_faerie::FaerieBackend;
use cranelift_module::Linkage;
use cranelift_module::Module;

use crate::parser::decl::Func;

use super::compile::Compile;
use super::error::*;

impl<'f> Compile for Func<'f> {
    fn compile(&self, module: &mut Module<FaerieBackend>) -> Result<()> {
        let sig = Signature::new(CallConv::SystemV);
        // TODO: params
        // TODO: return

        let mut ctx = FunctionBuilderContext::new();
        let mut func = Function::with_name_signature(
            ExternalName::user(0, 0), // FIXME
            sig,
        );

        let mut builder = FunctionBuilder::new(&mut func, &mut ctx);

        let entry = builder.create_ebb();

        builder.append_ebb_params_for_function_params(entry);
        builder.switch_to_block(entry);
        builder.seal_block(entry);

        builder.ins().nop();

        builder.ins().return_(&[]);

        // TODO: exprs

        builder.finalize();

        println!("{:?}", func); // FIXME

        let mut ctx = module.make_context();
        ctx.func = func;

        let id = module
            .declare_function(
                &self.name.0,
                Linkage::Export, // FIXME
                &ctx.func.signature,
            )
            .unwrap();

        module.define_function(id, &mut ctx).unwrap();

        module.finalize_definitions();

        Ok(())
    }
}
