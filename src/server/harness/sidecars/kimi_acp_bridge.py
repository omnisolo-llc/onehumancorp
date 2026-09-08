#!/usr/bin/env python3
"""Launch Kimi ACP with an explicitly configured OpenAI-compatible provider."""

from __future__ import annotations

import asyncio
import json
import os
import tempfile
from contextvars import ContextVar
from pathlib import Path
from typing import MutableMapping
from urllib.parse import urlsplit

_turn_usage: ContextVar[list | None] = ContextVar("omnisolo_kimi_turn_usage", default=None)


class UsageObservedMessage:
    """Forward the native stream and capture its final provider usage."""
    def __init__(self, source, collector):
        self.source = source
        self.collector = collector
        self.recorded = False

    def __getattr__(self, name):
        return getattr(self.source, name)

    async def __aiter__(self):
        async for part in self.source:
            yield part
        if not self.recorded and self.source.usage is not None:
            self.collector.append(self.source.usage)
            self.recorded = True


def acp_usage(records):
    if not records:
        return None
    counts = {"inputTokens": 0, "outputTokens": 0, "cachedReadTokens": 0, "cachedWriteTokens": 0}
    for usage in records:
        for field, attribute in (("inputTokens", "input"), ("outputTokens", "output"),
                                 ("cachedReadTokens", "input_cache_read"),
                                 ("cachedWriteTokens", "input_cache_creation")):
            value = getattr(usage, attribute)
            if type(value) is not int or value < 0:
                raise ValueError("invalid native provider token usage")
            counts[field] += value
    counts["totalTokens"] = counts["inputTokens"] + counts["outputTokens"]
    return counts


def _valid_base_url(value: str) -> bool:
    if any(character.isspace() or ord(character) < 32 or ord(character) == 127 for character in value):
        return False
    try:
        parsed = urlsplit(value)
        port = parsed.port
    except ValueError:
        return False
    return (
        parsed.scheme in {"http", "https"}
        and bool(parsed.netloc)
        and parsed.hostname is not None
        and parsed.username is None
        and parsed.password is None
        and parsed.query == ""
        and parsed.fragment == ""
        and (port is None or 0 < port <= 65_535)
    )


def prepare_static_provider(environment: MutableMapping[str, str]) -> bool:
    """Materialize non-secret Kimi configuration for OmniSolo's provider route."""
    api_key = environment.get("OPENAI_API_KEY", "").strip()
    base_url = environment.get("OPENAI_API_BASE_URL", "").strip()
    model = environment.get("OPENAI_MODEL", "").strip()
    if (
        not api_key
        or not _valid_base_url(base_url)
        or not model
        or any(ord(character) < 32 or ord(character) == 127 for character in model)
    ):
        return False

    environment["OPENAI_BASE_URL"] = base_url
    share_dir_value = environment.get("KIMI_SHARE_DIR", "").strip()
    if share_dir_value:
        share_dir = Path(share_dir_value)
    else:
        temporary_root = Path(environment.get("TMPDIR", tempfile.gettempdir()))
        share_dir = temporary_root / f"omnisolo-kimi-{os.getpid()}"
        environment["KIMI_SHARE_DIR"] = str(share_dir)
    share_dir.mkdir(mode=0o700, parents=True, exist_ok=True)
    share_dir.chmod(0o700)

    config = {
        "default_model": model,
        "default_thinking": False,
        "telemetry": False,
        "providers": {
            "omnisolo": {
                "type": "openai_responses",
                "base_url": base_url,
                "api_key": "",
            }
        },
        "models": {
            model: {
                "provider": "omnisolo",
                "model": model,
                "max_context_size": 262_144,
                "capabilities": ["thinking"],
            }
        },
    }
    config_path = share_dir / "config.json"
    descriptor = os.open(
        config_path,
        os.O_WRONLY | os.O_CREAT | os.O_TRUNC,
        0o600,
    )
    with os.fdopen(descriptor, "w", encoding="utf-8") as config_file:
        json.dump(config, config_file, separators=(",", ":"), sort_keys=True)
        config_file.write("\n")
    config_path.chmod(0o600)
    return True


def bind_requested_reasoning(effort: str | None, provider_type=None) -> None:
    """Pin the wire effort despite Kimi's narrower native thinking selector.

    The pinned OpenAI SDK's typed Reasoning model does not admit `max` yet.
    Its public extra_body hook overrides that field at request serialization.
    Each generation uses a provider copy; native session state stays unchanged.
    """
    if effort is None:
        return
    if effort not in {"none", "minimal", "low", "medium", "high", "max"}:
        raise ValueError("unsupported configured reasoning effort")
    if provider_type is None:
        from kosong.contrib.chat_provider.openai_responses import OpenAIResponses
        provider_type = OpenAIResponses
    original_generate = provider_type.generate

    async def generate(self, *args, **kwargs):
        configured = self.with_generation_kwargs(
            extra_body={"reasoning": {"effort": effort, "summary": "auto"}}
        )
        message = await original_generate(configured, *args, **kwargs)
        collector = _turn_usage.get()
        return message if collector is None else UsageObservedMessage(message, collector)

    provider_type.generate = generate


def _server(static_provider_configured: bool):
    from kimi_cli.acp.server import ACPServer

    class OmniSoloKimiACPServer(ACPServer):
        async def prompt(self, prompt, session_id, **kwargs):
            from acp.schema import Usage
            records = []
            token = _turn_usage.set(records)
            try:
                response = await super().prompt(prompt, session_id, **kwargs)
                usage = acp_usage(records)
                if usage is not None:
                    response = response.model_copy(update={"usage": Usage.model_validate(usage)})
                return response
            finally:
                _turn_usage.reset(token)

        def _check_auth(self) -> None:
            if static_provider_configured:
                return
            super()._check_auth()

    return OmniSoloKimiACPServer()


def main() -> None:
    import acp
    from kimi_cli.app import enable_logging
    from kimi_cli.utils.logging import logger

    configured = prepare_static_provider(os.environ)
    if configured:
        bind_requested_reasoning(os.environ.get("OPENAI_REASONING_EFFORT"))
    enable_logging()
    logger.info("Starting OmniSolo Kimi ACP bridge on stdio")
    asyncio.run(acp.run_agent(_server(configured), use_unstable_protocol=True))


if __name__ == "__main__":
    main()
