use aeris::ui::shard;

#[shard]
pub struct WS {
    space: x![{' ' '\t' '\n' '\r'}*],
}
