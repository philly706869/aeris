//! AERIS Code Model

mod block;
mod function;
mod global;
mod instruction;
mod module;
mod uid;

pub use block::Block;
pub use function::Function;
pub use global::Global;
pub use instruction::Instruction;
pub use module::Module;

use std::collections::HashSet;

pub struct AERISCodeModel {
    modules: HashSet<Module>,
}
