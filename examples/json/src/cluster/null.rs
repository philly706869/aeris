use xst::shard;

#[shard]
pub struct JSONNull {
    text: x!["null"],
}
