mod resolver;

use crate::ascm::function::Function;

pub struct Module {
    functions: Vec<Function>,
}
