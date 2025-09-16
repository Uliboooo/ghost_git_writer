use crate::{helper, llm};

const DEFAULT_PROMT: &str = "**Output 'SemVer filed name' and 'the reason' separated by ‘|’.about version filed name, Only contain semver name(Major or Minor or Patch)** must strictly adhere to this format: 'Minor | Reasons'. in Semantic Versioning, which field version should be incremented? think to reference git diff data:";

pub async fn whichi_sem<T: AsRef<str>>(
    diff: T,
    status: T,
    model: llm::LlmReqInfo,
    lang: Option<T>,
    extra: Option<T>,
) -> Result<(String, Option<String>), llm::Error> {
    let diff = diff.as_ref();
    let st = status.as_ref();
    let extra = helper::init_extra(extra);
    let lang = helper::init_lang(lang);

    let promt =
        format!("Please in {lang}.\n{DEFAULT_PROMT}\ngit status: {st}\ndiff: {diff}\n{extra}");

    let res = llm::call_llm(model, promt).await?;

    let res = res.split('|').collect::<Vec<_>>();
    let ress = if res.len() < 3 {
        (res[0].to_string(), Some(res[1].to_string()))
    } else {
        (res[0].to_string(), None)
    };
    Ok(ress)
}
