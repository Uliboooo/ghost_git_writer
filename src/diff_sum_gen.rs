use crate::{helper, llm};

const DEFAULT_PROMPT: &str = "summarize the git diff changes.
List the key modifications, what was added, removed, or modified, and briefly explain their purpose or impact if possible.
about only changes. must not write about project. you don't readme writer, you summarize diff changes.";

pub async fn sum_diff<T: AsRef<str>>(
    diff: T,
    status: T,
    model: llm::LlmReqInfo,
    lang: Option<T>,
    extra: Option<T>,
) -> Result<String, llm::Error> {
    let lang = helper::init_lang(lang);
    let st = status.as_ref();
    let extra = helper::init_extra(extra);
    let diff = diff.as_ref();
    let prompt =
        format!("Please in {lang}.\n{DEFAULT_PROMPT}\nstatus: {st}\ndiff: {diff}.\n{extra}");

    llm::call_llm(model, prompt).await
}
