use crate::{Error, Model, llm};

const GEN_MSG_PMT: &str = "You are a helpful assistant that writes concise and clear Git commit messages.\n\nGiven the following diff or description of code changes, write a Git commit message that follows these rules:\n- Use **imperative mood** (e.g., \"Add feature\", not \"Added\" or \"Adds\")\n- Keep the subject line under **50 characters**\n- Provide an optional body (if necessary), wrapped at **72 characters**\n- Focus on **what** was changed and **why**, not **how**\n\nCode changes:\n```\n[Paste your diff or a short description of the change here]\n```\n\nGenerate the Git commit message below:";

pub fn create_cmt_msg<T: AsRef<str>>(
    diff: T,
    model: Model,
    api_key: Option<T>,
) -> Result<String, Error> {
    let pmt = format!("{GEN_MSG_PMT} {}", diff.as_ref());
    llm::call_llm(
        pmt.to_string(),
        model.provider,
        model.model_name,
        api_key.map(|f| f.as_ref().to_string()),
        None,
        None,
    )
    .map_err(Error::Llm)
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::Model;
    use crate::cmt_msg::create_cmt_msg;

    #[test]
    fn test_cmt_msg() {
        let pj_root = env::current_dir().unwrap();
        let test_diff_path = pj_root.join("test_diff.txt");

        let diff = std::fs::read_to_string(test_diff_path).unwrap();
        println!("start");
        let res = create_cmt_msg(
            diff,
            Model::new("gemini", "gemini-2.0-flash"),
            Some(env::var("GEMINI_API_KEY").unwrap()),
        );
        println!(":{res:?}");
        assert!(res.is_ok());
    }
}
