pub use aeris_std_ui as ui;

use aeris::Context;

pub struct StdContext {
    // TODO
    #[allow(unused)]
    ctx: Context,
}

impl StdContext {
    pub fn new() -> Self {
        let ctx = Context::new();
        Self { ctx }
    }
}
