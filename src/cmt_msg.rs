use crate::{Error, Model, llm};

const GEN_MSG_PMT: &str = "You are an assistant that writes Git commit messages.\
When code changes include modifications to documentation files (e.g., README.md, docs/), ignore those changes and generate the commit message based solely on source code changes.\
Given a description of code changes, output only a single-line commit message in Conventional Commits format (e.g., \"feat:\", \"fix:\", \"docs:\", etc.).\
Do not include any extra text, code blocks, or formatting. Only output the commit message.\
Changes:\n";

pub fn create_cmt_msg<T: AsRef<str>, U: AsRef<str>>(
    diff: T,
    model: Model,
    api_key: Option<T>,
    lang: Option<U>,
    extra: Option<U>,
) -> Result<String, Error> {
    let lang = lang
        .map(|f| f.as_ref().to_string())
        .unwrap_or("english".to_string());
    let pmt = format!(
        "Generate the commit message in {lang}. {GEN_MSG_PMT} {}.{}",
        diff.as_ref(),
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
    .map_err(Error::Llm)
}

// #[cfg(test)]
// mod tests {
//     use std::env;

//     use crate::Model;
//     use crate::cmt_msg::create_cmt_msg;

//     #[test]
//     fn test_cmt_msg() {
//         let pj_root = env::current_dir().unwrap();
//         let test_diff_path = pj_root.join("test_diff.txt");

//         let diff = std::fs::read_to_string(test_diff_path).unwrap();
//         println!("start");
//         let res = create_cmt_msg(
//             diff,
//             Model::new("gemini", "gemini-2.0-flash", None, None),
//             Some(env::var("GEMINI_API_KEY").unwrap()),
//             Some("japanese"),
//         );
//         println!(":{res:?}");
//         assert!(res.is_ok());
//     }
// }
