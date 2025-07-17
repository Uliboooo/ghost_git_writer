use std::{
    fmt::Display,
    fs::{self, OpenOptions},
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

use serde::{Serialize, de::DeserializeOwned};

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Serde(serde_json::Error),
    /// File found, but no contents.
    FileIsZero,
    UnInitialized(Option<io::Error>),
    IsADirectoryError,
    IsFile,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "io error: {e}"),
            Error::Serde(e) => write!(f, "serde error: {e}"),
            Error::FileIsZero => write!(f, "file is zero"),
            Error::UnInitialized(e) => write!(
                f,
                "This directory has not yet been initialized. details: {}",
                e.as_ref()
                    .map(|f| f.to_string())
                    .unwrap_or("other error".to_string())
            ),
            Error::IsADirectoryError => write!(f, "this is a directory. sohuld be file path"),
            Error::IsFile => write!(f, "this path is file, pls dir path."),
        }
    }
}

impl Error {
    pub fn is_file_is_zero(&self) -> bool {
        matches!(self, Error::FileIsZero)
    }

    pub fn is_not_found(&self) -> bool {
        match self {
            Error::Io(error) => error.kind() == io::ErrorKind::NotFound,
            _ => false,
        }
    }
}

fn db_path<P: AsRef<Path>>(dir: P) -> PathBuf {
    dir.as_ref()
        .join(".code_pulse")
        .join("db")
        .with_extension("json")
}

pub trait Storage<P: AsRef<Path>> {
    /// *path: project dir path.
    fn open<T>(dir_path: P) -> Result<T, Error>
    where
        T: Serialize + DeserializeOwned,
    {
        if dir_path.as_ref().is_file() {
            return Err(Error::IsFile);
        }

        let mut f = OpenOptions::new()
            .read(true)
            .create(false)
            .truncate(false)
            .write(true)
            .open(db_path(dir_path))
            .map_err(Error::Io)?;

        let mut con = String::new();
        match f.read_to_string(&mut con).map_err(Error::Io) {
            Ok(_) => serde_json::from_str::<T>(&con).map_err(Error::Serde),
            Err(e) => Err(match e {
                Error::Io(error) => match error.kind() {
                    io::ErrorKind::NotFound | io::ErrorKind::UnexpectedEof => {
                        Error::UnInitialized(Some(error))
                    }
                    _ => Error::Io(error),
                },
                Error::FileIsZero => Error::UnInitialized(None),
                other => other,
            }),
        }
    }

    /// ⚠️ overwrite `src` to `path file`.
    fn save(&self, path: P, create: bool) -> Result<(), Error>
    where
        Self: Serialize + DeserializeOwned,
    {
        let serialized_json = serde_json::to_string(self).map_err(Error::Serde)?;
        let mut f = OpenOptions::new()
            .read(false)
            .write(true)
            .truncate(true)
            .create(create)
            .open(path)
            .map_err(Error::Io)?;

        f.write_all(serialized_json.as_bytes()).map_err(Error::Io)
    }
}

/// Arg's `create` use as `OpenOptions.create()`
pub fn load<T: DeserializeOwned, P: AsRef<Path>>(path: P, create: bool) -> Result<T, Error> {
    if path.as_ref().parent().unwrap().exists() || create {
        fs::create_dir_all(path.as_ref().parent().unwrap()).map_err(Error::Io)?;
    }

    let mut f = OpenOptions::new()
        .read(true)
        .create(create)
        .truncate(false)
        .write(true)
        .open(path)
        .map_err(Error::Io)?;
    let mut con = String::new();
    f.read_to_string(&mut con).map_err(Error::Io)?;

    if con.is_empty() {
        Err(Error::FileIsZero)
    } else {
        serde_json::from_str(&con).map_err(Error::Serde)
    }
}

pub fn save<T: Serialize, P: AsRef<Path>>(src: T, path: P) -> Result<(), Error> {
    let serialized_json = serde_json::to_string(&src).map_err(Error::Serde)?;
    let mut f = OpenOptions::new()
        .read(false)
        .write(true)
        .truncate(true)
        .open(path)
        .map_err(Error::Io)?;

    f.write_all(serialized_json.as_bytes()).map_err(Error::Io)
}
