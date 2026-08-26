use aeris::ui::shard;

#[shard]
pub struct JSONNull {
    text: x!["null"],
}
