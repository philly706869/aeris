//! AERIS Code Model

mod block;
mod function;
mod global;
mod instruction;
mod layout;
mod module;
mod uid;

pub use block::Block;
pub use function::Function;
pub use global::Global;
pub use instruction::Instruction;
pub use module::Module;

use std::collections::HashMap;

use inkwell::types::{BasicMetadataTypeEnum, BasicType};

pub struct AERISCodeModel<'m, 'f, 'g> {
    modules: HashMap<usize, &'m Module<'f, 'g>>,
}

impl<'m, 'f, 'g> AERISCodeModel<'m, 'f, 'g> {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    fn build_llvm(&self) {
        let ctx = inkwell::context::Context::create();
        for (module_id, module) in &self.modules {
            let llvm_module = ctx.create_module("");
            for (function_id, function) in &module.functions {
                let return_type = function.return_layout().to_llvm_basic_type_enum(&ctx);
                let param_types: Vec<BasicMetadataTypeEnum> = function
                    .param_layout()
                    .iter()
                    .map(|ty| ty.to_llvm_basic_type_enum(&ctx).into())
                    .collect();
                let fn_type = return_type.fn_type(&param_types, function.variadic());
                let llvm_function = llvm_module.add_function("", fn_type, None);
            }
        }
    }
}
