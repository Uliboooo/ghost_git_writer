use std::path::Path;

use git2::{DiffOptions, Repository};

use crate::Error;

pub fn get_diff<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let repo = Repository::open(path).unwrap();
    let head_commit = repo
        .head()
        .map_err(Error::Git2)?
        .peel_to_commit()
        .map_err(Error::Git2)?;
    let head_tree = head_commit.tree().map_err(Error::Git2)?;
    let diff = repo
        .diff_tree_to_workdir(Some(&head_tree), Some(&mut DiffOptions::new()))
        .map_err(Error::Git2)?;
    let mut pa = String::new();
    diff.print(git2::DiffFormat::Patch, |_, _, line| {
        if let Ok(t) = std::str::from_utf8(line.content()) {
            pa.push_str(t);
        }
        true
    })
    .map_err(Error::Git2)?;
    Ok(pa)
}
