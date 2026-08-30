use aeris::ui::shard;

#[shard]
pub struct Punctuated<T, P> {
    inner: Option<(T, Vec<(P, T)>)>,
}
