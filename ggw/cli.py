"""Click-based CLI for ghost_git_writer (ggw)."""
from __future__ import annotations

import os
import sys
from pathlib import Path
from typing import Optional

import click
from rich.console import Console
from rich.panel import Panel

from ggw import commit_gen, diff_sum_gen, readme_gen, which_sem
from ggw.config import Config, Model
from ggw.git import get_diff, get_git_status, get_user_email, git_commit
from ggw.llm import LlmReqInfo

_console = Console()
_err_console = Console(stderr=True)

# ---------------------------------------------------------------------------
# Environment variable names
# ---------------------------------------------------------------------------
_ANTHROPIC_API = "GGW_ANTHROPIC_API"
_GEMINI_API = "GGW_GEMINI_API"
_GEMINI_API_FALL = "GEMINI_API_KEY"
_OPENAI_API = "GGW_OPENAI_API"
_DEEPSEEK_API = "GGW_DEEPSEEK_API"

# ---------------------------------------------------------------------------
# Shared root options (applied to every subcommand via @_root_options)
# ---------------------------------------------------------------------------
_ROOT_OPTIONS = [
    click.option(
        "-m",
        "--model",
        default=None,
        help="Model spec: `provider/model` or a config alias.",
    ),
    click.option("--temperature", type=float, default=None, help="LLM temperature."),
    click.option("--max-tokens", type=int, default=None, help="Max output tokens."),
    click.option("--base-url", default=None, help="Custom base URL (e.g. for Ollama)."),
    click.option(
        "-p", "--path", "work_path", default=None, help="Git project root path."
    ),
    click.option(
        "-l", "--lang", default=None, help="Output language. e.g. `-l japanese`"
    ),
    click.option(
        "-e",
        "--extra",
        default=None,
        help="Extra prompt appended to the default prompt.",
    ),
    click.option("--config", "config_path", default=None, help="Config file path."),
    click.option(
        "--oneline",
        is_flag=True,
        default=False,
        help="Print only the LLM output (for pipes).",
    ),
    click.option(
        "--stdin",
        is_flag=True,
        default=False,
        help="Read diff content from stdin.",
    ),
]


def _root_options(f):
    """Decorator that attaches all root options to a Click command."""
    for opt in reversed(_ROOT_OPTIONS):
        f = opt(f)
    return f


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def _resolve_config(config_path: Optional[str]) -> Optional[Config]:
    if config_path:
        p = Path(config_path)
        if p.exists():
            return Config.load(p)
        return None

    home = Path.home()
    primary = home / ".config" / "ggw" / "config.toml"
    secondary = home / ".ggw.toml"

    for p in (primary, secondary):
        if p.exists():
            try:
                return Config.load(p)
            except Exception:
                pass
    return None


def _resolve_work_path(work_path: Optional[str]) -> Path:
    p = Path(work_path) if work_path else Path.cwd()
    if not p.exists():
        _err_console.print(
            f"[red]error:[/red] work path does not exist: {p}"
        )
        sys.exit(1)
    return p


def _resolve_model(
    config: Optional[Config],
    model_arg: Optional[str],
    temperature: Optional[float],
    max_tokens: Optional[int],
    base_url: Optional[str],
) -> Model:
    if model_arg:
        if "/" in model_arg:
            provider, model_name = model_arg.split("/", 1)
            return Model(
                provider=provider,
                model=model_name,
                temperature=temperature,
                max_tokens=max_tokens,
                base_url=base_url,
            )
        # alias lookup
        if config is None or config.llms is None:
            _err_console.print(
                "[red]error:[/red] A config file is required to resolve model aliases. "
                "Pass the model as 'provider/model' (e.g. `-m gemini/gemini-2.0-flash`)."
            )
            sys.exit(1)
        m = config.llms.get_model(model_arg)
        if m is None:
            _err_console.print(
                f"[red]error:[/red] Model alias {model_arg!r} not found in config."
            )
            sys.exit(1)
        return m

    # no model arg – use default from config
    if config is None or config.llms is None:
        _err_console.print(
            "[red]error:[/red] No config file found and no model specified. "
            "Create ~/.config/ggw/config.toml or pass `-m provider/model`."
        )
        sys.exit(1)
    m = config.llms.get_default()
    if m is None:
        _err_console.print(
            "[red]error:[/red] No default_model set in config. "
            "Add `default_model` under [llms] or pass `-m provider/model`."
        )
        sys.exit(1)
    return m


def _resolve_api_key(model: Model) -> Optional[str]:
    provider = model.provider.lower()
    if provider == "ollama":
        return None
    if provider == "gemini":
        key = os.environ.get(_GEMINI_API) or os.environ.get(_GEMINI_API_FALL)
        if not key:
            _err_console.print(
                f"[red]error:[/red] Set {_GEMINI_API} or {_GEMINI_API_FALL} env var."
            )
            sys.exit(1)
        return key
    mapping = {
        "openai": _OPENAI_API,
        "anthropic": _ANTHROPIC_API,
        "deepseek": _DEEPSEEK_API,
    }
    env_var = mapping.get(provider)
    if env_var is None:
        _err_console.print(f"[red]error:[/red] Unknown provider: {model.provider!r}")
        sys.exit(1)
    key = os.environ.get(env_var)
    if not key:
        _err_console.print(
            f"[red]error:[/red] Set the {env_var} environment variable."
        )
        sys.exit(1)
    return key


def _build_llm_info(
    config: Optional[Config],
    model_arg: Optional[str],
    temperature: Optional[float],
    max_tokens: Optional[int],
    base_url: Optional[str],
) -> LlmReqInfo:
    model = _resolve_model(config, model_arg, temperature, max_tokens, base_url)
    api_key = _resolve_api_key(model)
    return LlmReqInfo(
        provider=model.provider,
        model=model.model,
        api_key=api_key,
        temperature=model.temperature if temperature is None else temperature,
        max_tokens=model.max_tokens if max_tokens is None else max_tokens,
        base_url=model.base_url if base_url is None else base_url,
    )


def _resolve_diff_arg(diff_arg: Optional[str]):
    """Parse `-D commit1/commit2` into (c1, c2) tuple."""
    if diff_arg is None:
        return (None, None)
    parts = diff_arg.split("/", 1)
    if len(parts) == 2:
        return (parts[0] or None, parts[1] or None)
    return (parts[0] or None, None)


def _print_boxed(msg: str) -> None:
    _console.print(Panel(msg, expand=False))


# ---------------------------------------------------------------------------
# CLI group
# ---------------------------------------------------------------------------

@click.group()
@click.version_option(package_name="ghost-git-writer")
def cli() -> None:
    """ghost_git_writer (ggw) – generate git commit messages, READMEs and more using LLMs."""


# ---------------------------------------------------------------------------
# commit
# ---------------------------------------------------------------------------

@cli.command("commit")
@_root_options
@click.option(
    "--auto-commit",
    is_flag=True,
    default=False,
    help="Commit automatically without asking.",
)
@click.option(
    "-D",
    "--diff",
    "diff_arg",
    default=None,
    help="Diff reference: commit hash, tag, or 'c1/c2'.",
)
def cmd_commit(
    model: Optional[str],
    temperature: Optional[float],
    max_tokens: Optional[int],
    base_url: Optional[str],
    work_path: Optional[str],
    lang: Optional[str],
    extra: Optional[str],
    config_path: Optional[str],
    oneline: bool,
    stdin: bool,
    auto_commit: bool,
    diff_arg: Optional[str],
) -> None:
    """Generate a git commit message from git diff using an LLM."""
    config = _resolve_config(config_path)
    path = _resolve_work_path(work_path)
    llm_info = _build_llm_info(config, model, temperature, max_tokens, base_url)

    if not sys.stdin.isatty() and stdin:
        diff = sys.stdin.read()
    else:
        diff = get_diff(_resolve_diff_arg(diff_arg), path)

    status = get_git_status(path)
    msg = commit_gen.gen_commit_msg(diff, status, llm_info, lang, extra)

    if oneline:
        click.echo(msg)
        return

    _console.print("Generated msg:")
    _print_boxed(msg)

    proceed = auto_commit or click.confirm("continue?", default=False)
    if not proceed:
        _err_console.print("commit canceled")
        sys.exit(1)

    git_commit(path, msg)


# ---------------------------------------------------------------------------
# readme
# ---------------------------------------------------------------------------

@cli.command("readme")
@_root_options
@click.option(
    "-s",
    "--sources",
    "source_path",
    default=None,
    help="Comma-separated list of source file paths.",
)
@click.option(
    "-d",
    "--directory",
    "source_dir",
    default=None,
    help="Source folder – all files in the directory are used.",
)
@click.option(
    "--merge-readme",
    "allow_merge",
    is_flag=True,
    default=False,
    help="Append generated content to existing README.md.",
)
def cmd_readme(
    model: Optional[str],
    temperature: Optional[float],
    max_tokens: Optional[int],
    base_url: Optional[str],
    work_path: Optional[str],
    lang: Optional[str],
    extra: Optional[str],
    config_path: Optional[str],
    oneline: bool,
    stdin: bool,
    source_path: Optional[str],
    source_dir: Optional[str],
    allow_merge: bool,
) -> None:
    """Generate a README.md from the codebase using an LLM."""
    config = _resolve_config(config_path)
    path = _resolve_work_path(work_path)
    llm_info = _build_llm_info(config, model, temperature, max_tokens, base_url)

    file_list: list[Path] = []

    if source_path:
        file_list.extend(Path(p) for p in source_path.split(","))

    if source_dir:
        src_dir = Path(source_dir)
        if src_dir.is_dir():
            file_list.extend(f for f in src_dir.iterdir() if f.is_file())

    if not file_list:
        src = path / "src"
        if not src.is_dir():
            _err_console.print(
                "[red]error:[/red] No source specified and no 'src/' directory found. "
                "Use -s or -d to specify sources."
            )
            sys.exit(1)
        if not click.confirm(
            "No source specified. Process the 'src/' directory?", default=True
        ):
            _err_console.print("No source specified.")
            sys.exit(1)
        file_list.extend(f for f in src.iterdir() if f.is_file())

    content = readme_gen.gen_readme(file_list, llm_info, lang, extra)

    if oneline:
        click.echo(content)
        return

    _console.print("Generated README:\n")
    _console.print(content)

    existing_readme = _find_readme(path)
    if existing_readme:
        if allow_merge or click.confirm("Merge into README.md?", default=False):
            with open(existing_readme, "a", encoding="utf-8") as f:
                f.write("\n" + content)
            return
    # save to timestamped file
    from ggw.helper import get_now
    out_path = path / f"{get_now()}.md"
    if click.confirm(f"Save to {out_path.name}?", default=True):
        out_path.write_text(content, encoding="utf-8")
    else:
        _err_console.print("canceled")
        sys.exit(1)


def _find_readme(work_path: Path) -> Optional[Path]:
    for p in work_path.iterdir():
        if p.is_file() and p.name.lower() == "readme.md":
            return p
    return None


# ---------------------------------------------------------------------------
# sumdiff
# ---------------------------------------------------------------------------

@cli.command("sumdiff")
@_root_options
@click.option(
    "-D",
    "--diff",
    "diff_arg",
    default=None,
    help="Diff reference: commit hash, tag, or 'c1/c2'.",
)
def cmd_sumdiff(
    model: Optional[str],
    temperature: Optional[float],
    max_tokens: Optional[int],
    base_url: Optional[str],
    work_path: Optional[str],
    lang: Optional[str],
    extra: Optional[str],
    config_path: Optional[str],
    oneline: bool,
    stdin: bool,
    diff_arg: Optional[str],
) -> None:
    """Summarize git diff changes using an LLM."""
    config = _resolve_config(config_path)
    path = _resolve_work_path(work_path)
    llm_info = _build_llm_info(config, model, temperature, max_tokens, base_url)

    if not sys.stdin.isatty() and stdin:
        diff = sys.stdin.read()
    else:
        diff = get_diff(_resolve_diff_arg(diff_arg), path)

    status = get_git_status(path)
    result = diff_sum_gen.sum_diff(diff, status, llm_info, lang, extra)

    if oneline:
        click.echo(result)
    else:
        _console.print(f"diff summarize:\n{result}")


# ---------------------------------------------------------------------------
# which-sem
# ---------------------------------------------------------------------------

@cli.command("which-sem")
@_root_options
@click.option(
    "-D",
    "--diff",
    "diff_arg",
    default=None,
    help="Diff reference: commit hash, tag, or 'c1/c2'.",
)
def cmd_which_sem(
    model: Optional[str],
    temperature: Optional[float],
    max_tokens: Optional[int],
    base_url: Optional[str],
    work_path: Optional[str],
    lang: Optional[str],
    extra: Optional[str],
    config_path: Optional[str],
    oneline: bool,
    stdin: bool,
    diff_arg: Optional[str],
) -> None:
    """Determine which SemVer field to bump based on git diff."""
    config = _resolve_config(config_path)
    path = _resolve_work_path(work_path)
    llm_info = _build_llm_info(config, model, temperature, max_tokens, base_url)

    if not sys.stdin.isatty() and stdin:
        diff = sys.stdin.read()
    else:
        diff = get_diff(_resolve_diff_arg(diff_arg), path)

    status = get_git_status(path)
    part, reason = which_sem.which_sem(diff, status, llm_info, lang, extra)

    if oneline:
        click.echo(part)
        return

    _console.print(f"should increase at\n{_semver_selector(part)}")
    if reason:
        _console.print(f"reasons:\n{reason.strip()}")


def _semver_selector(part: str) -> str:
    """Return a simple text representation of which SemVer part is selected."""
    part_upper = part.strip().upper()
    parts = ["MAJOR", "MINOR", "PATCH"]
    segments: list[str] = []
    for p in parts:
        if p == part_upper:
            segments.append(f"[bold green][ {p} ][/bold green]")
        else:
            segments.append(f"  {p}  ")
    return " ".join(segments)
