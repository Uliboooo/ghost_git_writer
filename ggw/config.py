"""TOML config loading for ghost_git_writer."""
from __future__ import annotations

import tomllib
from dataclasses import dataclass, field
from pathlib import Path
from typing import Optional


@dataclass
class Model:
    provider: str
    model: str
    temperature: Optional[float] = None
    max_tokens: Optional[int] = None
    base_url: Optional[str] = None


@dataclass
class OllamaConfig:
    base_url: str = "http://localhost:11434"


@dataclass
class Llm:
    default_model: Optional[Model] = None
    models: dict[str, Model] = field(default_factory=dict)
    ollama: Optional[OllamaConfig] = None

    def get_default(self) -> Optional[Model]:
        return self.default_model

    def get_model(self, name: str) -> Optional[Model]:
        return self.models.get(name)


@dataclass
class Config:
    llms: Optional[Llm] = None

    @classmethod
    def load(cls, path: Path) -> "Config":
        with open(path, "rb") as f:
            data = tomllib.load(f)
        return cls._from_dict(data)

    @classmethod
    def _from_dict(cls, data: dict) -> "Config":
        llms_data = data.get("llms")
        llms = None
        if llms_data:
            default_model = None
            dm_data = llms_data.get("default_model")
            if dm_data:
                default_model = Model(
                    provider=dm_data["provider"],
                    model=dm_data["model"],
                    temperature=dm_data.get("temperature"),
                    max_tokens=dm_data.get("max_tokens"),
                    base_url=dm_data.get("base_url"),
                )

            models: dict[str, Model] = {}
            for name, m_data in llms_data.get("models", {}).items():
                models[name] = Model(
                    provider=m_data["provider"],
                    model=m_data["model"],
                    temperature=m_data.get("temperature"),
                    max_tokens=m_data.get("max_tokens"),
                    base_url=m_data.get("base_url"),
                )

            ollama_data = llms_data.get("ollama")
            ollama = OllamaConfig(**ollama_data) if ollama_data else None

            llms = Llm(default_model=default_model, models=models, ollama=ollama)

        return cls(llms=llms)
