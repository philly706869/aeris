use aeris::ui::shard;

#[shard]
pub mod WS {
    #[derive(Debug)]
    struct Shard(x![{' ' '\t' '\n' '\r'}*]);
}
