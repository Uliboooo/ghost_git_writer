"""SemVer field determination."""
from __future__ import annotations

from typing import Optional, Tuple

from ggw.llm import LlmReqInfo, call_llm

_DEFAULT_PROMPT = (
    "**Output 'SemVer field name' and 'the reason' separated by '|'."
    " about version field name, Only contain semver name(Major or Minor or Patch)**"
    " must strictly adhere to this format: 'Minor | Reasons'."
    " in Semantic Versioning, which field version should be incremented?"
    " think to reference git diff data:"
)


def which_sem(
    diff: str,
    status: str,
    model: LlmReqInfo,
    lang: Optional[str] = None,
    extra: Optional[str] = None,
) -> Tuple[str, Optional[str]]:
    lang = lang or "english"
    extra_str = f" # Additional Instructions: {extra}" if extra else ""
    prompt = (
        f"Please in {lang}.\n{_DEFAULT_PROMPT}\n"
        f"git status: {status}\ndiff: {diff}\n{extra_str}"
    )
    res = call_llm(model, prompt)
    parts = res.split("|", 1)
    if len(parts) == 2:
        return parts[0].strip(), parts[1].strip()
    return parts[0].strip(), None
