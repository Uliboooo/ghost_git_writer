"""Diff summarization."""
from __future__ import annotations

from typing import Optional

from ggw.llm import LlmReqInfo, call_llm

_DEFAULT_PROMPT = (
    "summarize the git diff changes.\n"
    "List the key modifications, what was added, removed, or modified,"
    " and briefly explain their purpose or impact if possible.\n"
    "about only changes. must not write about project."
    " you don't readme writer, you summarize diff changes."
)


def sum_diff(
    diff: str,
    status: str,
    model: LlmReqInfo,
    lang: Optional[str] = None,
    extra: Optional[str] = None,
) -> str:
    lang = lang or "english"
    extra_str = f" # Additional Instructions: {extra}" if extra else ""
    prompt = (
        f"Please in {lang}.\n{_DEFAULT_PROMPT}\n"
        f"status: {status}\ndiff: {diff}.\n{extra_str}"
    )
    return call_llm(model, prompt)
