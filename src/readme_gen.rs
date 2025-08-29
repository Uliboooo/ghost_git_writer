use std::{fmt::Display, io, path::Path};

use crate::{helper, llm};

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

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Llm(llm::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            //Error::Io(e) => write!(f, "io error: {}", e),
            Error::Io(e) => write!(f, "helper error: {}", e),
            Error::Llm(e) => write!(f, "llm error: {}", e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<llm::Error> for Error {
    fn from(value: llm::Error) -> Self {
        Self::Llm(value)
    }
}

pub async fn gen_readme<T: AsRef<str>, P: AsRef<Path>>(
    path_list: &Vec<P>,
    model: llm::LlmReqInfo,
    lang: Option<T>,
    extra: Option<T>,
) -> Result<String, Error> {
    let lang = helper::init_lang(lang);
    let code_base = helper::load_codebase(path_list)?;
    let extra = helper::init_extra(extra);

    let prompt = format!("Please in {lang}.\n{DEFAULT_PROMT} {code_base}.\n{extra}");

    Ok(llm::call_llm(model, prompt).await?)
}
