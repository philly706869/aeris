use aeris::ui::shard;

#[shard]
pub mod JSONNull {
    #[derive(Debug)]
    struct Shard(x!["null"]);
}
