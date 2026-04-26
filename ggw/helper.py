"""Shared helper utilities."""
from __future__ import annotations

from datetime import datetime


def get_now() -> str:
    return datetime.now().strftime("%b-%d-%H-%M")
