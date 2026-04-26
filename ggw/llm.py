"""LLM API calls for ghost_git_writer."""
from __future__ import annotations

from dataclasses import dataclass
from typing import Optional

import requests
from rich.console import Console
from rich.progress import Progress, SpinnerColumn, TextColumn

_console = Console(stderr=True)

SUPPORTED_PROVIDERS = {"ollama", "openai", "gemini", "anthropic", "deepseek"}


@dataclass
class LlmReqInfo:
    provider: str
    model: str
    api_key: Optional[str]
    temperature: Optional[float] = None
    max_tokens: Optional[int] = None
    base_url: Optional[str] = None


def call_llm(llm_info: LlmReqInfo, prompt: str) -> str:
    """Call the LLM and return the response text, showing a spinner while waiting."""
    with Progress(
        SpinnerColumn(),
        TextColumn("[progress.description]{task.description}"),
        transient=True,
        console=_console,
    ) as progress:
        progress.add_task("LLM call...", total=None)
        result = _dispatch(llm_info, prompt)
    _console.print("finished.")
    return result


def _dispatch(llm_info: LlmReqInfo, prompt: str) -> str:
    provider = llm_info.provider.lower()
    if provider == "gemini":
        return _call_gemini(llm_info, prompt)
    if provider == "openai":
        return _call_openai_compat(llm_info, prompt, "https://api.openai.com/v1")
    if provider == "anthropic":
        return _call_anthropic(llm_info, prompt)
    if provider == "deepseek":
        return _call_openai_compat(llm_info, prompt, "https://api.deepseek.com/v1")
    if provider == "ollama":
        return _call_ollama(llm_info, prompt)
    raise ValueError(f"Unsupported provider: {llm_info.provider!r}")


def _call_gemini(llm_info: LlmReqInfo, prompt: str) -> str:
    url = (
        "https://generativelanguage.googleapis.com/v1beta/models/"
        f"{llm_info.model}:generateContent"
    )
    body = {"contents": [{"parts": [{"text": prompt}]}]}
    resp = requests.post(
        url,
        json=body,
        headers={
            "x-goog-api-key": llm_info.api_key or "",
            "Content-Type": "application/json",
        },
        timeout=120,
    )
    resp.raise_for_status()
    data = resp.json()
    return data["candidates"][0]["content"]["parts"][0]["text"]


def _call_openai_compat(llm_info: LlmReqInfo, prompt: str, default_base: str) -> str:
    base_url = llm_info.base_url or default_base
    url = f"{base_url}/chat/completions"
    body: dict = {
        "model": llm_info.model,
        "messages": [{"role": "user", "content": prompt}],
    }
    if llm_info.temperature is not None:
        body["temperature"] = llm_info.temperature
    if llm_info.max_tokens is not None:
        body["max_tokens"] = llm_info.max_tokens
    resp = requests.post(
        url,
        json=body,
        headers={
            "Authorization": f"Bearer {llm_info.api_key or ''}",
            "Content-Type": "application/json",
        },
        timeout=120,
    )
    resp.raise_for_status()
    data = resp.json()
    return data["choices"][0]["message"]["content"]


def _call_anthropic(llm_info: LlmReqInfo, prompt: str) -> str:
    url = "https://api.anthropic.com/v1/messages"
    body: dict = {
        "model": llm_info.model,
        "max_tokens": llm_info.max_tokens or 1024,
        "messages": [{"role": "user", "content": prompt}],
    }
    if llm_info.temperature is not None:
        body["temperature"] = llm_info.temperature
    resp = requests.post(
        url,
        json=body,
        headers={
            "x-api-key": llm_info.api_key or "",
            "anthropic-version": "2023-06-01",
            "Content-Type": "application/json",
        },
        timeout=120,
    )
    resp.raise_for_status()
    data = resp.json()
    return data["content"][0]["text"]


def _call_ollama(llm_info: LlmReqInfo, prompt: str) -> str:
    base_url = llm_info.base_url or "http://localhost:11434"
    url = f"{base_url}/api/generate"
    body = {"model": llm_info.model, "prompt": prompt, "stream": False}
    resp = requests.post(url, json=body, timeout=300)
    resp.raise_for_status()
    data = resp.json()
    return data["response"]
