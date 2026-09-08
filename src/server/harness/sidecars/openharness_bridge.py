#!/usr/bin/env python3
"""JSONL sidecar for the pinned AgentBoardTT Harness Python SDK."""

from __future__ import annotations

import asyncio
import inspect
import json
import math
import os
import sys
import uuid
from dataclasses import dataclass, replace
from copy import copy
from types import SimpleNamespace
from pathlib import Path
from typing import Any
from urllib.parse import urlsplit

harness: Any = None
SteeringChannel: Any = None
OpenAIProvider: Any = None


SDK_VERSION = "0.6.0"
PROTOCOL_VERSION = 1
MAX_FRAME_BYTES = 1024 * 1024
REDACTED = "[REDACTED]"
SENSITIVE_KEYS = {
    "apikey",
    "authorization",
    "credential",
    "credentials",
    "password",
    "secret",
    "token",
    "accesstoken",
    "refreshtoken",
}


class MalformedSdkEvent(ValueError):
    """Raised when harness-agent emits an incompatible SDK object."""


class FrameTooLarge(ValueError):
    """Raised when a JSONL frame exceeds the protocol allocation bound."""


class CommandError(ValueError):
    def __init__(self, code: str, message: str) -> None:
        super().__init__(message)
        self.code = code


def _safe_session_id(value: str) -> bool:
    return (
        1 <= len(value.encode("utf-8")) <= 128
        and value[0].isascii()
        and value[0].isalnum()
        and all(
            character.isascii() and (character.isalnum() or character in "_.-")
            for character in value
        )
    )


def _validate_sdk_contract() -> str | None:
    """Pin the private provider injection seam used by harness-agent 0.6.0."""
    try:
        run_parameter = inspect.signature(harness.run).parameters.get("_provider")
        if (
            run_parameter is None
            or run_parameter.kind is not inspect.Parameter.KEYWORD_ONLY
            or run_parameter.default is not None
        ):
            return "harness.run must expose keyword-only _provider=None"
        provider_parameters = inspect.signature(OpenAIProvider).parameters
        for name in ("api_key", "model", "base_url"):
            parameter = provider_parameters.get(name)
            if parameter is None or parameter.kind not in (
                inspect.Parameter.POSITIONAL_OR_KEYWORD,
                inspect.Parameter.KEYWORD_ONLY,
            ):
                return f"OpenAIProvider must accept {name}"
    except (TypeError, ValueError):
        return "harness-agent provider injection signatures are not inspectable"
    return None


def _load_pinned_sdk(session_home: Path) -> None:
    """Import the pinned SDK only after isolating dotenv/config discovery."""
    global OpenAIProvider, SteeringChannel, harness
    os.environ["HOME"] = str(session_home)
    os.environ["XDG_CONFIG_HOME"] = str(session_home / ".config")
    os.environ["XDG_CACHE_HOME"] = str(session_home / ".cache")
    os.environ["XDG_DATA_HOME"] = str(session_home / ".local" / "share")
    os.chdir(session_home)
    import importlib

    harness = importlib.import_module("harness")
    SteeringChannel = importlib.import_module(
        "harness.core.steering"
    ).SteeringChannel
    OpenAIProvider = usage_complete_provider(importlib.import_module(
        "harness.providers.openai"
    ).OpenAIProvider)
    engine = importlib.import_module("harness.core.engine")
    engine.load_toml_config = lambda _cwd=None: {}


def usage_complete_provider(provider_type):
    """Keep the native provider, but await its trailing OpenAI usage chunk.

    SDK 0.6.0 emits message_end at finish_reason, before OpenAI's final
    choices=[] usage chunk. Delay only that event until the stream drains.
    """
    class UsageCompleteProvider(provider_type):
        async def chat_completion_stream(self, *args, **kwargs):
            usage = None
            create = self._client.chat.completions.create

            async def observed_create(*create_args, **create_kwargs):
                source = await create(*create_args, **create_kwargs)

                async def observed_chunks():
                    nonlocal usage
                    async for chunk in source:
                        raw = getattr(chunk, "usage", None)
                        if raw is not None:
                            counts = [getattr(raw, name, None) for name in
                                      ("prompt_tokens", "completion_tokens")]
                            if all(type(count) is int and count >= 0 for count in counts):
                                usage = dict(zip(("input_tokens", "output_tokens"), counts))
                        yield chunk
                return observed_chunks()

            scoped = copy(self)
            scoped._client = SimpleNamespace(chat=SimpleNamespace(
                completions=SimpleNamespace(create=observed_create)))
            terminal = None
            async for event in provider_type.chat_completion_stream(scoped, *args, **kwargs):
                if event.type == "message_end":
                    terminal = event
                else:
                    yield event
            if terminal is not None:
                yield replace(terminal, usage=usage if usage is not None else terminal.usage)
    return UsageCompleteProvider


def _normalized_key(key: object) -> str:
    return "".join(character for character in str(key).lower() if character.isalnum())


def _sensitive_key(key: object) -> bool:
    normalized = _normalized_key(key)
    return normalized in SENSITIVE_KEYS or normalized.endswith(
        ("apikey", "authorization", "credential", "password", "secret")
    )


def redact(value: Any, secret: str) -> Any:
    """Recursively remove credential fields and occurrences of the active key."""
    if isinstance(value, dict):
        return {
            str(key): REDACTED
            if _sensitive_key(key)
            else redact(item, secret)
            for key, item in value.items()
        }
    if isinstance(value, (list, tuple)):
        return [redact(item, secret) for item in value]
    if isinstance(value, str) and secret:
        return value.replace(secret, REDACTED)
    return value


def _write_frame(frame: dict[str, Any], secret: str) -> None:
    wire = json.dumps(
        redact(frame, secret),
        separators=(",", ":"),
        ensure_ascii=False,
        allow_nan=False,
    )
    if len(wire.encode("utf-8")) + 1 > MAX_FRAME_BYTES:
        raise FrameTooLarge("OpenHarness JSONL frame exceeds the limit")
    sys.stdout.write(wire + "\n")
    sys.stdout.flush()


def _required_string(command: dict[str, Any], field: str) -> str:
    value = command.get(field)
    if not isinstance(value, str) or not value.strip():
        raise ValueError(f"{field} must be a non-empty string")
    return value


def _required_number(event: object, field: str, *, integer: bool = False) -> int | float:
    value = getattr(event, field, None)
    expected = int if integer else (int, float)
    if (
        isinstance(value, bool)
        or not isinstance(value, expected)
        or (isinstance(value, float) and not math.isfinite(value))
        or value < 0
    ):
        raise MalformedSdkEvent(
            f"Result.{field} must be a finite non-negative number"
        )
    return value


def _provider_error_code(error: BaseException) -> str:
    name = type(error).__name__.lower()
    message = str(error).lower()
    status = getattr(error, "status_code", None)
    if isinstance(error, (asyncio.TimeoutError, TimeoutError)) or "timeout" in name:
        return "timeout"
    if status in (401, 403) or "authentication" in name or "unauthorized" in message:
        return "auth"
    if status == 429 or "ratelimit" in name or "rate limit" in message:
        return "rate_limit"
    if any(
        marker in message
        for marker in (
            "context length",
            "context window",
            "maximum context",
            "too many tokens",
        )
    ):
        return "context"
    if isinstance(error, KeyError) or "unknown model" in message or "modelnotfound" in name:
        return "unknown_model"
    if "unsupportedreasoning" in name or (
        "reasoning" in message and "unsupported" in message
    ):
        return "unsupported_reasoning"
    if isinstance(status, int) and 400 <= status < 500:
        return "provider_rejection"
    return "uncertain"


@dataclass
class ActiveRun:
    request_id: str
    session_id: str
    steering: SteeringChannel
    task: asyncio.Task[None] | None = None


@dataclass(frozen=True)
class ResolvedModel:
    provider: str
    model_id: str
    base_url: str
    reasoning_effort: str
    api_dialect: str
    context_window_tokens: int
    max_output_tokens: int
    capabilities: tuple[str, ...]
    binding_revision: str
    binding_digest: str


def _required_positive_integer(value: object, field: str) -> int:
    if isinstance(value, bool) or not isinstance(value, int) or value <= 0:
        raise ValueError(f"resolved_model.{field} must be a positive integer")
    return value


def _validate_base_url(value: str) -> str:
    parsed = urlsplit(value)
    if (
        parsed.scheme not in ("http", "https")
        or not parsed.hostname
        or parsed.username is not None
        or parsed.password is not None
        or parsed.query
        or parsed.fragment
        or any(character.isspace() or ord(character) < 32 for character in value)
    ):
        raise ValueError("resolved_model.base_url must be a credential-free HTTP(S) URL")
    return value


def _resolved_model(command: dict[str, Any]) -> ResolvedModel:
    value = command.get("resolved_model")
    if not isinstance(value, dict):
        raise ValueError("resolved_model must be an object")

    def required(field: str) -> str:
        item = value.get(field)
        if not isinstance(item, str) or not item.strip():
            raise ValueError(f"resolved_model.{field} must be a non-empty string")
        return item

    provider = required("provider")
    if provider != "openai":
        raise ValueError("resolved_model.provider must be openai")
    reasoning_effort = required("reasoning_effort")
    if reasoning_effort != "max":
        raise ValueError("resolved_model.reasoning_effort must be max")
    api_dialect = required("api_dialect")
    if api_dialect not in ("openai_chat_completions", "openai_responses"):
        raise ValueError("resolved_model.api_dialect is unsupported")
    capabilities = value.get("capabilities")
    if (
        not isinstance(capabilities, list)
        or not capabilities
        or any(not isinstance(item, str) or not item.strip() for item in capabilities)
    ):
        raise ValueError("resolved_model.capabilities must be a non-empty string array")
    return ResolvedModel(
        provider=provider,
        model_id=required("model_id"),
        base_url=_validate_base_url(required("base_url")),
        reasoning_effort=reasoning_effort,
        api_dialect=api_dialect,
        context_window_tokens=_required_positive_integer(
            value.get("context_window_tokens"), "context_window_tokens"
        ),
        max_output_tokens=_required_positive_integer(
            value.get("max_output_tokens"), "max_output_tokens"
        ),
        capabilities=tuple(capabilities),
        binding_revision=required("binding_revision"),
        binding_digest=required("binding_digest"),
    )


class Bridge:
    def __init__(self, api_key: str, session_home: Path) -> None:
        self.api_key = api_key
        self.session_directory = session_home / ".harness" / "sessions"
        self.session_directory.mkdir(parents=True, exist_ok=True, mode=0o700)
        self.session_directory.chmod(0o700)
        self.sessions: set[str] = set()
        self.active_runs: dict[str, ActiveRun] = {}
        self.output_lock = asyncio.Lock()
        self.stopping = False

    def session_path(self, session_id: str) -> Path:
        if not _safe_session_id(session_id):
            raise CommandError("invalid_command", "session_id is unsafe")
        return self.session_directory / f"{session_id}.jsonl"

    def require_session(self, session_id: str) -> Path:
        path = self.session_path(session_id)
        if path.is_symlink() or not path.is_file():
            raise CommandError("session_not_found", "native session state does not exist")
        return path

    async def emit(self, frame: dict[str, Any]) -> None:
        async with self.output_lock:
            _write_frame(frame, self.api_key)

    async def emit_error(
        self,
        request_id: str,
        code: str,
        message: str,
        *,
        session_id: str | None = None,
        terminal: bool,
    ) -> None:
        message = message[:8192]
        await self.emit(
            {
                "type": "error",
                "request_id": request_id,
                "session_id": session_id,
                "code": code,
                "message": message,
                "terminal": terminal,
            }
        )

    async def handle(self, command: dict[str, Any]) -> bool:
        request_id = _required_string(command, "id")
        command_type = _required_string(command, "type")
        if command_type == "create_session":
            session_id = str(uuid.uuid4())
            path = self.session_path(session_id)
            descriptor = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
            os.close(descriptor)
            self.sessions.add(session_id)
            await self.emit(
                {
                    "type": "session",
                    "id": request_id,
                    "command": command_type,
                    "session_id": session_id,
                }
            )
            return False
        if command_type == "resume_session":
            session_id = _required_string(command, "session_id")
            self.require_session(session_id)
            self.sessions.add(session_id)
            await self.emit(
                {
                    "type": "session",
                    "id": request_id,
                    "command": command_type,
                    "session_id": session_id,
                }
            )
            return False
        if command_type == "delete_session":
            session_id = _required_string(command, "session_id")
            path = self.require_session(session_id)
            if session_id in self.active_runs:
                raise CommandError("session_active", "cannot delete an active session")
            path.unlink()
            self.sessions.discard(session_id)
            await self.emit(
                {
                    "type": "session_deleted",
                    "id": request_id,
                    "command": command_type,
                    "session_id": session_id,
                }
            )
            return False
        if command_type == "prompt":
            await self.start_prompt(request_id, command)
            return False
        if command_type == "steer":
            await self.steer(request_id, command)
            return False
        if command_type == "cancel":
            await self.cancel(request_id, command)
            return False
        if command_type == "shutdown":
            await self.stop()
            await self.emit({"type": "ack", "id": request_id, "command": command_type})
            return True
        raise ValueError(f"unsupported command type: {command_type}")

    async def start_prompt(self, request_id: str, command: dict[str, Any]) -> None:
        session_id = _required_string(command, "session_id")
        self.require_session(session_id)
        if session_id not in self.sessions:
            raise ValueError("prompt references an unknown session_id")
        if self.active_runs:
            raise ValueError("sidecar already has an active prompt")
        prompt = _required_string(command, "prompt")
        resolved_model = _resolved_model(command)
        permission_mode = _required_string(command, "permission_mode")
        if permission_mode not in ("default", "accept_edits", "plan", "bypass"):
            raise ValueError("permission_mode is unsupported")
        cwd = _required_string(command, "cwd")
        steering = SteeringChannel()
        active = ActiveRun(request_id, session_id, steering)
        self.active_runs[session_id] = active
        await self.emit(
            {
                "type": "downgrade",
                "request_id": request_id,
                "session_id": session_id,
                "field": "reasoning_effort",
                "requested": resolved_model.reasoning_effort,
                "applied": "provider_default",
                "reason": "harness-agent 0.6.0 OpenAIProvider has no reasoning-effort parameter",
                "binding_revision": resolved_model.binding_revision,
                "binding_digest": resolved_model.binding_digest,
            }
        )
        if resolved_model.api_dialect == "openai_responses":
            await self.emit(
                {
                    "type": "downgrade",
                    "request_id": request_id,
                    "session_id": session_id,
                    "field": "api_dialect",
                    "requested": resolved_model.api_dialect,
                    "applied": "openai_chat_completions",
                    "reason": "harness-agent 0.6.0 OpenAIProvider uses Chat Completions",
                    "binding_revision": resolved_model.binding_revision,
                    "binding_digest": resolved_model.binding_digest,
                }
            )
        await self.emit({"type": "ack", "id": request_id, "command": "prompt"})
        active.task = asyncio.create_task(
            self.run_prompt(
                active,
                prompt=prompt,
                resolved_model=resolved_model,
                permission_mode=permission_mode,
                cwd=cwd,
            )
        )

    async def run_prompt(
        self,
        active: ActiveRun,
        *,
        prompt: str,
        resolved_model: ResolvedModel,
        permission_mode: str,
        cwd: str,
    ) -> None:
        terminal_seen = False
        registry_models: dict[str, Any] | None = None
        previous_model: Any = None
        model_was_registered = False
        try:
            provider_adapter = OpenAIProvider(
                api_key=self.api_key,
                model=resolved_model.model_id,
                base_url=resolved_model.base_url,
            )
            os.environ.pop("OPENAI_API_KEY", None)
            os.environ.pop("HARNESS_PROVIDER", None)
            os.environ.pop("HARNESS_MODEL", None)
            try:
                from harness.providers.registry import MODELS
                from harness.types.providers import ModelInfo

                registry_models = MODELS
                model_was_registered = resolved_model.model_id in MODELS
                previous_model = MODELS.get(resolved_model.model_id)
                MODELS[resolved_model.model_id] = ModelInfo(
                    id=resolved_model.model_id,
                    provider="openai",
                    display_name=resolved_model.model_id,
                    context_window=resolved_model.context_window_tokens,
                    max_output_tokens=resolved_model.max_output_tokens,
                    supports_tools="tools" in resolved_model.capabilities,
                    supports_streaming="streaming" in resolved_model.capabilities,
                    supports_vision="vision" in resolved_model.capabilities,
                )
            except (ImportError, AttributeError, TypeError):
                registry_models = None
            stream = harness.run(
                prompt,
                provider="openai",
                model=resolved_model.model_id,
                base_url=resolved_model.base_url,
                permission_mode=permission_mode,
                session_id=active.session_id,
                max_tokens=resolved_model.max_output_tokens,
                cwd=cwd,
                steering=active.steering,
                _provider=provider_adapter,
            )
            async for sdk_event in stream:
                frame = self.convert_event(active, sdk_event)
                await self.emit(frame)
                if frame["type"] == "result":
                    result_session_id = frame["session_id"]
                    self.sessions.add(result_session_id)
                    terminal_seen = True
            if not terminal_seen:
                raise MalformedSdkEvent("SDK stream ended without a Result event")
        except asyncio.CancelledError:
            await self.emit(
                {
                    "type": "cancelled",
                    "request_id": active.request_id,
                    "session_id": active.session_id,
                }
            )
            raise
        except MalformedSdkEvent as error:
            await self.emit_error(
                active.request_id,
                "malformed_sdk_event",
                str(error),
                session_id=active.session_id,
                terminal=True,
            )
        except FrameTooLarge as error:
            await self.emit_error(
                active.request_id,
                "frame_too_large",
                str(error),
                session_id=active.session_id,
                terminal=True,
            )
        except Exception as error:  # SDK/provider exceptions are protocol data.
            await self.emit_error(
                active.request_id,
                _provider_error_code(error),
                str(error),
                session_id=active.session_id,
                terminal=True,
            )
        finally:
            if registry_models is not None:
                if model_was_registered:
                    registry_models[resolved_model.model_id] = previous_model
                else:
                    registry_models.pop(resolved_model.model_id, None)
            if self.active_runs.get(active.session_id) is active:
                self.active_runs.pop(active.session_id, None)
            await active.steering.close()

    def convert_event(self, active: ActiveRun, event: object) -> dict[str, Any]:
        common = {
            "request_id": active.request_id,
            "session_id": active.session_id,
        }
        if isinstance(event, harness.TextMessage):
            if not isinstance(event.text, str) or not isinstance(event.is_partial, bool):
                raise MalformedSdkEvent("TextMessage fields have incompatible types")
            return {
                "type": "text",
                **common,
                "text": event.text,
                "is_partial": event.is_partial,
            }
        if isinstance(event, harness.SystemEvent):
            if (
                not isinstance(event.type, str)
                or not event.type.strip()
                or not isinstance(event.data, dict)
            ):
                raise MalformedSdkEvent("SystemEvent fields have incompatible types")
            return {
                "type": "system",
                **common,
                "event": event.type,
                "data": event.data,
            }
        if isinstance(event, harness.CompactionEvent):
            if not isinstance(event.summary, str):
                raise MalformedSdkEvent("CompactionEvent.summary must be a string")
            return {
                "type": "compaction",
                **common,
                "tokens_before": _required_number(
                    event, "tokens_before", integer=True
                ),
                "tokens_after": _required_number(event, "tokens_after", integer=True),
                "summary": event.summary,
            }
        if isinstance(event, harness.ToolUse):
            if (
                not isinstance(event.id, str)
                or not event.id
                or not isinstance(event.name, str)
                or not event.name
                or not isinstance(event.args, dict)
            ):
                raise MalformedSdkEvent("ToolUse fields have incompatible types")
            return {
                "type": "tool_use",
                **common,
                "tool_use_id": event.id,
                "name": event.name,
                "args": event.args,
            }
        if isinstance(event, harness.ToolResult):
            if (
                not isinstance(event.tool_use_id, str)
                or not event.tool_use_id
                or not isinstance(event.content, str)
                or not isinstance(event.is_error, bool)
                or (event.display is not None and not isinstance(event.display, str))
            ):
                raise MalformedSdkEvent("ToolResult fields have incompatible types")
            return {
                "type": "tool_result",
                **common,
                "tool_use_id": event.tool_use_id,
                "content": event.content,
                "is_error": event.is_error,
                "display": event.display,
            }
        if isinstance(event, harness.Result):
            if (
                not isinstance(event.text, str)
                or not isinstance(event.session_id, str)
                or not _safe_session_id(event.session_id)
                or event.session_id != active.session_id
                or not isinstance(event.stop_reason, str)
                or not event.stop_reason
            ):
                raise MalformedSdkEvent("Result string fields have incompatible types")
            return {
                "type": "result",
                "request_id": active.request_id,
                "session_id": event.session_id,
                "text": event.text,
                "turns": _required_number(event, "turns", integer=True),
                "tool_calls": _required_number(event, "tool_calls", integer=True),
                "total_tokens": _required_number(event, "total_tokens", integer=True),
                "total_cost": _required_number(event, "total_cost"),
                "stop_reason": event.stop_reason,
            }
        raise MalformedSdkEvent(f"unsupported SDK event type: {type(event).__name__}")

    async def steer(self, request_id: str, command: dict[str, Any]) -> None:
        session_id = _required_string(command, "session_id")
        message = _required_string(command, "message")
        active = self.active_runs.get(session_id)
        if active is None:
            raise ValueError("cannot steer a session without an active prompt")
        await active.steering.send(message)
        await self.emit({"type": "ack", "id": request_id, "command": "steer"})

    async def cancel(self, request_id: str, command: dict[str, Any]) -> None:
        session_id = _required_string(command, "session_id")
        active = self.active_runs.get(session_id)
        if active is None or active.task is None:
            raise ValueError("cannot cancel a session without an active prompt")
        await self.emit({"type": "ack", "id": request_id, "command": "cancel"})
        active.task.cancel()

    async def stop(self) -> None:
        if self.stopping:
            return
        self.stopping = True
        tasks = [active.task for active in self.active_runs.values() if active.task is not None]
        for task in tasks:
            task.cancel()
        if tasks:
            await asyncio.gather(*tasks, return_exceptions=True)


async def _main() -> int:
    api_key = os.environ.get("OPENAI_API_KEY", "")
    if not api_key:
        _write_frame(
            {
                "type": "error",
                "request_id": "__startup__",
                "session_id": None,
                "code": "missing_api_key",
                "message": "OPENAI_API_KEY is required",
                "terminal": True,
            },
            "",
        )
        return 78
    session_home = Path(os.environ.get("OPENHARNESS_SESSION_HOME", ""))
    try:
        if not session_home.is_absolute():
            raise ValueError("OPENHARNESS_SESSION_HOME must be absolute")
        if session_home.is_symlink():
            raise ValueError("OPENHARNESS_SESSION_HOME must not be a symlink")
        session_home = session_home.resolve(strict=True)
        temporary_root = Path("/tmp").resolve(strict=True)
        if session_home != temporary_root and temporary_root not in session_home.parents:
            raise ValueError("OPENHARNESS_SESSION_HOME must be private under /tmp")
        if not session_home.is_dir():
            raise ValueError("OPENHARNESS_SESSION_HOME must be a real directory")
        probe = session_home / f".openharness-write-probe-{uuid.uuid4().hex}"
        descriptor = os.open(probe, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
        os.close(descriptor)
        probe.unlink()
    except (OSError, ValueError) as error:
        _write_frame(
            {
                "type": "error",
                "request_id": "__startup__",
                "session_id": None,
                "code": "invalid_session_home",
                "message": str(error),
                "terminal": True,
            },
            api_key,
        )
        return 78
    try:
        _load_pinned_sdk(session_home)
    except Exception as error:
        _write_frame(
            {
                "type": "error",
                "request_id": "__startup__",
                "session_id": None,
                "code": "sdk_import_error",
                "message": str(error),
                "terminal": True,
            },
            api_key,
        )
        return 78
    if getattr(harness, "__version__", None) != SDK_VERSION:
        _write_frame(
            {
                "type": "error",
                "request_id": "__startup__",
                "session_id": None,
                "code": "sdk_version_mismatch",
                "message": f"harness-agent {SDK_VERSION} is required",
                "terminal": True,
            },
            api_key,
        )
        return 78
    contract_error = _validate_sdk_contract()
    if contract_error is not None:
        _write_frame(
            {
                "type": "error",
                "request_id": "__startup__",
                "session_id": None,
                "code": "sdk_contract_mismatch",
                "message": contract_error,
                "terminal": True,
            },
            api_key,
        )
        return 78
    bridge = Bridge(api_key, session_home)
    await bridge.emit(
        {
            "type": "ready",
            "protocol_version": PROTOCOL_VERSION,
            "sdk_version": SDK_VERSION,
        }
    )
    while not bridge.stopping:
        raw_bytes = await asyncio.to_thread(
            sys.stdin.buffer.readline, MAX_FRAME_BYTES + 1
        )
        if raw_bytes == b"":
            await bridge.stop()
            break
        if len(raw_bytes) > MAX_FRAME_BYTES:
            while raw_bytes and not raw_bytes.endswith(b"\n"):
                raw_bytes = await asyncio.to_thread(
                    sys.stdin.buffer.readline, MAX_FRAME_BYTES + 1
                )
            await bridge.emit_error(
                "__protocol__",
                "frame_too_large",
                "OpenHarness command frame exceeds the JSONL limit",
                terminal=False,
            )
            continue
        try:
            raw_line = raw_bytes.decode("utf-8")
        except UnicodeDecodeError:
            await bridge.emit_error(
                "__protocol__",
                "invalid_command",
                "command frame is not UTF-8",
                terminal=False,
            )
            continue
        if not raw_line.strip():
            continue
        request_id = "__protocol__"
        try:
            command = json.loads(raw_line)
            if not isinstance(command, dict):
                raise ValueError("command frame must be a JSON object")
            if isinstance(command.get("id"), str) and command["id"].strip():
                request_id = command["id"]
            should_exit = await bridge.handle(command)
            if should_exit:
                break
        except (json.JSONDecodeError, ValueError) as error:
            await bridge.emit_error(
                request_id,
                error.code if isinstance(error, CommandError) else "invalid_command",
                str(error),
                terminal=False,
            )
    return 0


if __name__ == "__main__":
    raise SystemExit(asyncio.run(_main()))
