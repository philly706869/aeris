use std::{fmt::Write, fs, path::PathBuf};

pub trait Shard {}

pub trait Preprocess {
    fn hash() -> &'static [u8];
    fn data() -> &'static [u8];
}

pub fn preprocess<P>()
where
    P: Preprocess,
{
    let hash = P::hash();
    let data = P::data();
    let mut hash_string = String::new();
    for byte in hash {
        write!(&mut hash_string, "{:02x}", byte).unwrap();
    }
    let cluster_path = std::env::var("CLUSTER").unwrap();
    let mut cluster_path = PathBuf::from(cluster_path);
    fs::create_dir_all(&cluster_path).unwrap();
    cluster_path.push(&hash_string);
    fs::write(&cluster_path, data).unwrap();
}
