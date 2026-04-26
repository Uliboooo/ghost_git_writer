"""README generation from codebase."""
from __future__ import annotations

from pathlib import Path
from typing import Optional

from ggw.llm import LlmReqInfo, call_llm

_DEFAULT_PROMPT = """You are a helpful assistant that generates professional README.md files.
Please read the following codebase and generate a README.md that includes:
- Project name and brief description
- Key features
- Technologies used
- Installation instructions
- How to run the project
- Example usage (if applicable)
- License section (if available in the code)
- Any relevant badges or links (GitHub repo, docs, etc.)

Here is the project code or file list:"""


def load_codebase(path_list: list[Path]) -> str:
    parts: list[str] = []
    for p in path_list:
        if p.exists() and p.is_file():
            try:
                text = p.read_text(encoding="utf-8", errors="ignore")
                parts.append(f"path: {p}\n\n{text}")
            except OSError:
                pass
    return "\n\n".join(parts)


def gen_readme(
    path_list: list[Path],
    model: LlmReqInfo,
    lang: Optional[str] = None,
    extra: Optional[str] = None,
) -> str:
    lang = lang or "english"
    code_base = load_codebase(path_list)
    extra_str = f" # Additional Instructions: {extra}" if extra else ""
    prompt = f"Please in {lang}.\n{_DEFAULT_PROMPT} {code_base}.\n{extra_str}"
    return call_llm(model, prompt)
