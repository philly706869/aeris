use xst::shard;

use crate::WS;

#[shard]
pub struct Spanned<T> {
    inner: T,
    ws: WS,
}
