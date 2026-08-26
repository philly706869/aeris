use aeris::ui::shard;

// use crate::cluster::WS;

#[shard]
pub struct Spanned {
    inner: T,
    ws: WS,
}
