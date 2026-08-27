pub trait Shard {
    const DATA: ();
}

pub const fn build<S>()
where
    S: Shard,
{
}
