use crate::fs::read_images_from_dir;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub(crate) struct Pair {
    pub title: String,
    pub left: PathBuf,
    pub right: PathBuf,
}

impl Pair {
    pub fn new(title: String, left: PathBuf, right: PathBuf) -> Self {
        Pair { title, left, right }
    }
}

pub(crate) fn pairs_from_paths(left_path: &Path, right_path: &Path) -> anyhow::Result<Vec<Pair>> {
    if !left_path.is_dir() {
        anyhow::bail!("Left path ('{}') is not a directory", left_path.display());
    }
    if !right_path.is_dir() {
        anyhow::bail!("Right path ('{}') is not a directory", left_path.display());
    }
    let mut names = read_images_from_dir(&left_path)?;
    names.append(&mut read_images_from_dir(&right_path)?);
    names.sort_unstable();
    names.dedup();
    Ok(names
        .into_iter()
        .map(|name| {
            let left = left_path.join(&name);
            let right = right_path.join(&name);
            Pair::new(
                name.into_string()
                    .unwrap_or_else(|name| name.to_string_lossy().into_owned()),
                left,
                right,
            )
        })
        .collect())
}
