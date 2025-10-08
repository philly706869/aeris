//! AERIS Code Model

mod block;
mod function;
mod global;
mod instruction;
mod module;
mod type_;
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
        let context = inkwell::context::Context::create();
        for (module_id, module) in &self.modules {
            let llvm_module = context.create_module("");
            for function in &module.functions {
                // llvm_module.add_function("", ty, linkage);
            }
        }
    }
}
