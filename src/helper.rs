use std::{
    fs, io, path::{Path, PathBuf}
};

// #[derive(Debug)]
// pub enum Error {
//     Io(std::io::Error),
// }
//
// impl Display for Error {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Error::Io(e) => write!(f, "io error: {}", e),
//         }
//     }
// }
//
// impl From<std::io::Error> for Error {
//     fn from(value: std::io::Error) -> Self {
//         Self::Io(value)
//     }
// }

pub fn init_lang<T: AsRef<str>>(lang: Option<T>) -> String {
    lang.map(|f| f.as_ref().to_string())
        .unwrap_or("english".to_string())
}

pub fn init_extra<T: AsRef<str>>(extra: Option<T>) -> String {
    format!(
        " # Additional Instructions: {}",
        extra
            .map(|f| f.as_ref().to_string())
            .unwrap_or("".to_string())
    )
}

pub fn load_codebase<P: AsRef<Path>>(path_list: &Vec<P>) -> Result<String, io::Error> {
    let mut file_contes = Vec::new();

    for f in path_list {
        let p = f.as_ref();
        if p.exists()
            && let Ok(s) = fs::read_to_string(p)
        {
            file_contes.push(format!("path: {}\n\n{s}", f.as_ref().to_string_lossy()));
        }
    }

    Ok(file_contes.into_iter().collect::<String>())
}

pub fn find_readme<T: AsRef<Path>>(work_path: T) -> Option<PathBuf> {
    fs::read_dir(work_path)
        .ok()?
        .filter_map(|et| et.ok())
        .find_map(|et| {
            et.path()
                .file_name()
                .and_then(|n| n.to_str())
                .filter(|s| s.eq_ignore_ascii_case("readme.md"))
                .map(|_| et.path())
        })
}

pub fn get_now() -> String {
    chrono::Local::now().format("%b-%d-%H-%M").to_string()
}
