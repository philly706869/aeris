use crate::acm::{Block, layout::Layout};

#[derive(Debug)]
pub struct Function {
    return_layout: Layout,
    param_layout: Vec<Layout>,
    variadic: bool,
    blocks: Vec<Block>,
}

impl Function {
    pub fn return_layout(&self) -> &Layout {
        &self.return_layout
    }

    pub fn param_layout(&self) -> &Vec<Layout> {
        &self.param_layout
    }

    pub fn variadic(&self) -> bool {
        self.variadic
    }

    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }
}
