use std::path::Path;

use git2::{DiffOptions, IndexAddOption, Repository, Signature};

// #[derive(Debug)]
// pub enum Error {
//     Git(git2::Error),
// }
//
// impl Display for Error {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Error::Git(e) => write!(f, "git error: {}", e),
//         }
//     }
// }
//
// impl From<git2::Error> for Error {
//     fn from(value: git2::Error) -> Self {
//         Self::Git(value)
//     }
// }

pub fn get_diff<T: AsRef<str>, P: AsRef<Path>>(
    points: (Option<T>, Option<T>),
    path: P,
) -> Result<String, git2::Error> {
    let repo = Repository::open(path)?;

    let points = (
        points.0.map(|f| f.as_ref().to_string()),
        points.1.map(|f| f.as_ref().to_string()),
    );

    let diff = match points {
        (None, None) => repo.diff_index_to_workdir(None, Some(&mut DiffOptions::new())),
        (None, Some(c2)) => {
            let com1 = repo.find_commit(repo.revparse_single(c2.as_str())?.id())?;
            let tree = com1.tree()?;
            repo.diff_tree_to_workdir(Some(&tree), Some(&mut DiffOptions::new()))
        }
        (Some(c1), None) => {
            let com1 = repo.find_commit(repo.revparse_single(c1.as_str())?.id())?;
            let tree = com1.tree()?;
            repo.diff_tree_to_workdir(Some(&tree), Some(&mut DiffOptions::new()))
        }
        (Some(c1), Some(c2)) => {
            let com1 = repo.find_commit(repo.revparse_single(c1.as_str())?.id())?;
            let com2 = repo.find_commit(repo.revparse_single(c2.as_str())?.id())?;

            let tree1 = com1.tree()?;
            let tree2 = com2.tree()?;
            repo.diff_tree_to_tree(Some(&tree1), Some(&tree2), Some(&mut DiffOptions::new()))
        }
    }?;

    let mut pa = String::new();
    diff.print(git2::DiffFormat::Patch, |_, _, line| {
        if let Ok(t) = std::str::from_utf8(line.content()) {
            pa.push_str(t);
        }
        true
    })?;
    Ok(pa)
}

pub fn git_commit<P: AsRef<Path>, M: AsRef<str>, T: AsRef<str>>(
    path: P,
    msg: &M,
    name: T,
    email: T,
) -> Result<(), git2::Error> {
    let repo = Repository::open(path)?;
    let mut index = repo.index()?;
    index.add_all(["*"].iter(), IndexAddOption::CHECK_PATHSPEC, None)?;
    index.write()?;

    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    let parent_commit = repo
        .head()
        .ok()
        .and_then(|h| h.resolve().ok())
        .and_then(|r| r.peel_to_commit().ok());

            // Error::PortIsNotNumber => write!(f, "failed parse port to number"),
    let sig = Signature::now(name.as_ref(), email.as_ref())?;

    let _commit_id = if let Some(pa) = parent_commit {
        repo.commit(Some("HEAD"), &sig, &sig, msg.as_ref(), &tree, &[&pa])?
    } else {
        // first commit
        repo.commit(Some("HEAD"), &sig, &sig, msg.as_ref(), &tree, &[])?
    };

    Ok(())
}

pub fn get_user_email() -> Result<(String, String), git2::Error> {
    let config = git2::Config::open_default()?;

    Ok((
        config.get_string("user.name")?,
        config.get_string("user.email")?,
    ))
}
