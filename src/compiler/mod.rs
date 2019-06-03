use std::str::FromStr;

use cranelift::prelude::*;
use cranelift_faerie::FaerieBackend;
use cranelift_faerie::FaerieBuilder;
use cranelift_faerie::FaerieProduct;
use cranelift_faerie::FaerieTrapCollection;
use cranelift_module::Module;
use target_lexicon::triple;

use crate::parser::Entry;

mod compile;
mod decl;
mod error;
mod func;

pub use error::*;

use compile::Compile;

pub fn compile(entry: &Entry) -> Result<FaerieProduct> {
    let mut flags = settings::builder();
    flags.set("opt_level", "fastest").unwrap(); // FIXME: on debug only
    flags.enable("is_pic").unwrap();

    let isa = isa::lookup(
        triple!("x86_64-unknown-unknown-elf"), // FIXME
    )
    .unwrap()
    .finish(settings::Flags::new(flags));

    let builder = FaerieBuilder::new(
        isa,
        "empty-main".into(), // FIXME
        FaerieTrapCollection::Disabled,
        FaerieBuilder::default_libcall_names(),
    )
    .unwrap();

    let mut module = Module::<FaerieBackend>::new(builder);

    for decl in &entry.decls {
        decl.compile(&mut module)?;
    }

    Ok(module.finish())
}
