use std::{fs, path::Path};

use crate::Error;

pub fn load_codes<P: AsRef<Path>>(path_list: &Vec<P>) -> Result<String, Error> {
    let mut file_contents = Vec::new();

    for f in path_list {
        let p = f.as_ref();
        if p.exists() {
            if let Ok(s) = fs::read_to_string(p).map_err(Error::IoE) {
                file_contents.push(format!("path: {}\ncontents:\n{s}", p.to_string_lossy()));
            }
        }
    }

    Ok(file_contents.into_iter().collect::<String>())
}
