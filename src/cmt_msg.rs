use crate::{Error, Model, llm};

pub fn create_cmt_msg<T: AsRef<str>>(
    diff: T,
    model: Model,
    api_key: Option<T>,
) -> Result<String, Error> {
    let pmt = format!("create git commit message for diff: {}", diff.as_ref());
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
    fn cc() {
        let diff =
            std::fs::read_to_string("/Users/yuki/Develop/ghost_git_writer/test_diff.txt").unwrap();
        println!("start");
        let res = create_cmt_msg(
            diff,
            Model::new("gemini", "gemini-2.0-flash"),
            Some(env::var("GEMINI_API_KEY").unwrap()),
        );
        println!(":{res:?}");
    }
}
