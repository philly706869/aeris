//! AERIS Code Model

mod function;
mod global;
mod module;
mod uid;

pub use function::Function;
pub use global::Global;
pub use module::Module;

use std::collections::HashSet;

pub struct AERISCodeModel {
    modules: HashSet<Module>,
}
