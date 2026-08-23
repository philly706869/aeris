use aeris::ui::shard;

#[shard]
pub mod JSONBoolean {
    #[derive(Debug)]
    enum Shard {
        True(JSONTrue),
        False(JSONFalse),
    }
}

#[shard]
pub mod JSONTrue {
    #[derive(Debug)]
    struct Shard {
        text: x!["true"],
    }
}

#[shard]
pub mod JSONFalse {
    #[derive(Debug)]
    struct Shard {
        text: x!["false"],
    }
}
