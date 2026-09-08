use xst::shard;

#[shard]
pub struct WS {
    space: x![{' ' '\t' '\n' '\r'}*],
}
