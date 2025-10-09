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
                let param_types: Vec<_> = function
                    .param_layout()
                    .iter()
                    .map(|ty| ty.map_to_llvm_type(&ctx))
                    .collect();
                // let fn_type = function
                //     .return_type()
                //     .map_to_llvm_type(&ctx)
                //     .fn_type(&param_types, function.variadic());
                // let llvm_function = llvm_module.add_function("", ty, linkage);
            }
        }
    }
}
