use std::{
    fmt::Write,
    fs,
    path::{Path, PathBuf},
};

use rustc_hash::FxHashSet;

pub trait Shard {
    fn meta() -> &'static dyn Meta;
}

pub trait Meta {
    fn hash(&self) -> &'static [u8];
    fn data(&self) -> &'static [u8];
    fn deps(&self) -> &'static [&'static dyn Meta];
}

pub fn preprocess<S, P>(path: P)
where
    S: Shard,
    P: AsRef<Path>,
{
    let mut flattened = Vec::new();
    let mut stack = Vec::new();
    let mut set = FxHashSet::default();
    let meta = S::meta();
    set.insert(meta.hash());
    stack.push(meta);
    while let Some(current) = stack.pop() {
        flattened.push(current);
        for dep in current.deps() {
            if set.insert(dep.hash()) {
                stack.push(*dep);
            }
        }
    }

    let mut cluster_path = PathBuf::from(path.as_ref());
    fs::create_dir_all(&cluster_path).unwrap();
    let mut hash = String::with_capacity(64);
    for meta in flattened {
        hash.clear();
        for byte in meta.hash() {
            write!(&mut hash, "{:02x}", byte).unwrap();
        }
        cluster_path.push(&hash);
        fs::write(&cluster_path, meta.data()).unwrap();
        cluster_path.pop();
    }
}
