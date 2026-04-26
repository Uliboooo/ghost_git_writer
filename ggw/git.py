"""Git operations for ghost_git_writer."""
from __future__ import annotations

import subprocess
from pathlib import Path
from typing import Optional, Tuple


def _run(args: list[str], cwd: Optional[Path] = None, check: bool = True) -> str:
    result = subprocess.run(
        args,
        cwd=cwd,
        capture_output=True,
        text=True,
        check=check,
    )
    return result.stdout


def get_diff(
    points: Tuple[Optional[str], Optional[str]],
    path: Path,
) -> str:
    """Get git diff between two points or against the index."""
    c1, c2 = points

    if c1 is None and c2 is None:
        return _run(["git", "diff", "--staged"], cwd=path)
    elif c1 is not None and c2 is not None:
        return _run(["git", "diff", c1, c2], cwd=path)
    elif c1 is not None:
        return _run(["git", "diff", c1], cwd=path)
    else:
        return _run(["git", "diff", c2], cwd=path)


def get_git_status(path: Path) -> str:
    """Get short git status."""
    return _run(["git", "status", "--short"], cwd=path)


def get_user_email() -> Tuple[str, str]:
    """Return (name, email) from git config."""
    name = _run(["git", "config", "user.name"]).strip()
    email = _run(["git", "config", "user.email"]).strip()
    return name, email


def git_commit(path: Path, msg: str) -> None:
    """Stage all changes and create a commit."""
    _run(["git", "add", "."], cwd=path)
    _run(["git", "commit", "-m", msg], cwd=path)
