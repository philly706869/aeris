use aeris::ui::shard;

#[shard]
pub enum JSONBoolean {
    True(JSONTrue),
    False(JSONFalse),
}

#[shard]
pub struct JSONTrue {
    text: x!["true"],
}

#[shard]
pub struct JSONFalse {
    text: x!["false"],
}
