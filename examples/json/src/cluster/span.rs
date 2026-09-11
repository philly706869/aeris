use xst::shard;

// use crate::cluster::WS;

#[shard]
pub struct Spanned<T> {
    inner: T,
    ws: WS,
}
