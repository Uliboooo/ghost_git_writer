"""Commit message generation."""
from __future__ import annotations

from typing import Optional

from ggw.llm import LlmReqInfo, call_llm

_GEN_MSG_PROMPT = (
    "You are an assistant that writes Git commit messages."
    " When code changes include modifications to documentation files"
    " (e.g., README.md, docs/), ignore those changes and generate the commit"
    " message based solely on source code changes."
    ' Given a description of code changes, output only a single-line commit'
    ' message in Conventional Commits format (e.g., "feat:", "fix:", "docs:", etc.).'
    " Do not include any extra text, code blocks, or formatting."
    " Only output the commit message."
    " git status info and diff changes:"
)


def gen_commit_msg(
    diff: str,
    status: str,
    model: LlmReqInfo,
    lang: Optional[str] = None,
    extra: Optional[str] = None,
) -> str:
    lang = lang or "english"
    extra_str = f" # Additional Instructions: {extra}" if extra else ""
    prompt = (
        f"Please in {lang}.\n{_GEN_MSG_PROMPT}\n"
        f"status: {status}\ndiff:{diff}.\n{extra_str}"
    )
    return call_llm(model, prompt)
