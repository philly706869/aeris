use aeris::ui::shard;

#[shard]
pub mod JSONBoolean {
    #[derive(Debug)]
    enum Shard {
        True(x!["true"]),
        False(x!["false"]),
    }
}
