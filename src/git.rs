use crate::Error;
use git2::{DiffOptions, IndexAddOption, Repository, Signature};
use std::path::Path;

pub fn get_diff<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let repo = Repository::open(path).map_err(Error::GitE)?;
    let head_commit = repo
        .head()
        .map_err(Error::GitE)?
        .peel_to_commit()
        .map_err(Error::GitE)?;
    let head_tree = head_commit.tree().map_err(Error::GitE)?;
    let diff = repo
        .diff_tree_to_workdir(Some(&head_tree), Some(&mut DiffOptions::new()))
        .map_err(Error::GitE)?;
    let mut pa = String::new();
    diff.print(git2::DiffFormat::Patch, |_, _, line| {
        if let Ok(t) = std::str::from_utf8(line.content()) {
            pa.push_str(t);
        }
        true
    })
    .map_err(Error::GitE)?;
    Ok(pa)
}

pub fn git_commit<P: AsRef<Path>, M: AsRef<str>, T: AsRef<str>>(
    path: P,
    msg: &M,
    name: T,
    email: T,
) -> Result<(), Error> {
    let repo = Repository::open(path).map_err(Error::GitE)?;
    let mut index = repo.index().map_err(Error::GitE)?;
    index
        .add_all(["*"].iter(), IndexAddOption::DEFAULT, None)
        .map_err(Error::GitE)?;
    index.write().map_err(Error::GitE)?;

    let tree_id = index.write_tree().map_err(Error::GitE)?;
    let tree = repo.find_tree(tree_id).map_err(Error::GitE)?;

    let parent_commit = repo
        .head()
        .ok()
        .and_then(|h| h.resolve().ok())
        .and_then(|r| r.peel_to_commit().ok());

    let sig = Signature::now(name.as_ref(), email.as_ref()).map_err(Error::GitE)?;

    let _commit_id = if let Some(pa) = parent_commit {
        repo.commit(Some("HEAD"), &sig, &sig, msg.as_ref(), &tree, &[&pa])
            .map_err(Error::GitE)?
    } else {
        // first commit
        repo.commit(Some("HEAD"), &sig, &sig, msg.as_ref(), &tree, &[])
            .map_err(Error::GitE)?
    };

    Ok(())
}

pub fn get_user_email() -> Result<(String, String), Error> {
    let config = git2::Config::open_default().map_err(Error::GitE)?;

    Ok((
        config.get_string("user.name").map_err(Error::GitE)?,
        config.get_string("user.email").map_err(Error::GitE)?,
    ))
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::git::{self, get_user_email};

    #[test]
    fn test_git_commit() {
        let path = env::current_dir().unwrap();
        let sig = get_user_email().unwrap();
        let res = git::git_commit(path, &"test", sig.0, sig.1);
        assert!(res.is_ok());
    }
}
