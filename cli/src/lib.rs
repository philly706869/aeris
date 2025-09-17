use std::path::{Path, PathBuf};

pub fn find_nearest_path<O, T>(start: O, target: T) -> Option<(PathBuf, PathBuf)>
where
    O: AsRef<Path>,
    T: AsRef<Path>,
{
    let mut current: PathBuf = start.as_ref().to_path_buf();

    loop {
        let candidate = current.join(target.as_ref());
        if candidate.exists() {
            return Some((current, candidate));
        }

        if !current.pop() {
            break;
        }
    }

    None
}
