use aeris::ui::shard;

// use crate::cluster::WS;

#[shard]
pub mod Spanned {
    #[derive(Debug)]
    struct Shard<Child> {
        child: Child,
        ws: WS,
    }
}
