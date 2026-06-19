use std::{collections::HashSet, fmt::Write, fs, path::PathBuf};

pub trait Shard {
    fn data() -> &'static Data;
}

pub struct Data {
    pub hash: &'static [u8],
    pub serialized: &'static [u8],
    pub dependencies: &'static [&'static Data],
}

pub fn preprocess<S>()
where
    S: Shard,
{
    let mut flattened = Vec::new();
    let mut stack = Vec::new();
    let mut set = HashSet::new();
    let data = S::data();
    set.insert(data.hash);
    stack.push(data);
    while let Some(current) = stack.pop() {
        flattened.push(current);
        for dependency in current.dependencies {
            if set.insert(dependency.hash) {
                stack.push(*dependency);
            }
        }
    }

    let cluster_path = std::env::var("CLUSTER").unwrap();
    let mut cluster_path = PathBuf::from(cluster_path);
    fs::create_dir_all(&cluster_path).unwrap();
    let mut hash = String::with_capacity(64);
    for data in flattened {
        hash.clear();
        for byte in data.hash {
            write!(&mut hash, "{:02x}", byte).unwrap();
        }
        cluster_path.push(&hash);
        fs::write(&cluster_path, data.serialized).unwrap();
        cluster_path.pop();
    }
}
