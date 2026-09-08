use xst::shard;

// use crate::cluster::WS;

#[shard]
pub struct Spanned {
    inner: T,
    ws: WS,
}
