use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use crate::{Error, Model, llm, read_codes::load_codes};

const DEFAULT_PROMT: &str =
    "You are a helpful assistant that generates professional README.md files.
Please read the following codebase and generate a README.md that includes:
- Project name and brief description
- Key features
- Technologies used
- Installation instructions
- How to run the project
- Example usage (if applicable)
- License section (if available in the code)
- Any relevant badges or links (GitHub repo, docs, etc.)

Here is the project code or file list:";

pub fn create_readme<T: AsRef<str>, P: AsRef<Path>, U: AsRef<str>>(
    files: &Vec<P>,
    model: Model,
    api_key: Option<T>,
    lang: Option<U>,
    extra: Option<U>,
) -> Result<String, Error> {
    let lang = lang
        .map(|f| f.as_ref().to_string())
        .unwrap_or("english".to_string());
    let code_base = load_codes(files)?;

    let pmt = format!(
        "Generate the README.md in {lang}. you must not use english {DEFAULT_PROMT} {code_base}.{}",
        format!(
            " # Additional Instructions: {}",
            extra
                .map(|f| f.as_ref().to_string())
                .unwrap_or("".to_string())
        )
    );
    llm::call_llm(
        pmt.to_string(),
        model.provider,
        model.model,
        api_key.map(|f| f.as_ref().to_string()),
        None,
        None,
    )
}

pub fn merge_readme<P: AsRef<Path>, T: AsRef<str>>(
    path: P,
    over_write: bool,
    src: T,
) -> Result<(), Error> {
    if over_write {
        let mut f = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)
            .map_err(Error::IoE)?;
        f.write_all(src.as_ref().as_bytes()).map_err(Error::IoE)
    } else {
        let mut f = OpenOptions::new()
            .append(true)
            .truncate(false)
            .open(path)
            .map_err(Error::IoE)?;
        f.write_all(src.as_ref().as_bytes()).map_err(Error::IoE)
    }
}

pub fn save_new_readme<P: AsRef<Path>, T: AsRef<str>>(
    path: P,
    over_write: bool,
    src: T,
) -> Result<(), Error> {
    let mut f = OpenOptions::new()
        .create(true)
        .truncate(over_write)
        .write(true)
        .open(path)
        .map_err(Error::IoE)?;
    f.write_all(src.as_ref().as_bytes()).map_err(Error::IoE)
}

pub fn find_readme<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    fs::read_dir(path).ok()?.find_map(|et| {
        let et = et.ok()?;
        if et.path().file_name().and_then(|f| f.to_str()) == Some("README.md") {
            Some(et.path())
        } else {
            None
        }
    })
}
