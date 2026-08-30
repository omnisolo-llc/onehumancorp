#!/usr/bin/env python3
"""Small JSONL bridge for harnesses that expose an OpenAI-compatible model API."""

import json
import os
import sys
import urllib.error
import urllib.parse
import urllib.request


MAX_FRAME_BYTES = 1_048_576
MAX_RESPONSE_BYTES = 16 * 1_024 * 1_024
NIL_REQUEST_ID = "00000000-0000-0000-0000-000000000000"
HARNESS_ENTRYPOINTS = {
    "aider": "aider",
    "goose": "goose",
    "open-interpreter": "interpreter",
    "plandex": "plandex",
}


class ShimError(Exception):
    pass


def _provider_configuration():
    token = os.environ.get("OPENAI_API_KEY", "").strip()
    base_url = os.environ.get("OPENAI_API_BASE_URL", "").strip().rstrip("/")
    model = os.environ.get("OPENAI_MODEL", "").strip()
    if not token or not base_url or not model:
        raise ShimError("provider configuration is incomplete")
    parsed = urllib.parse.urlsplit(base_url)
    if (
        parsed.scheme not in ("http", "https")
        or not parsed.hostname
        or parsed.username
        or parsed.password
        or parsed.query
        or parsed.fragment
        or parsed.hostname not in ("127.0.0.1", "localhost")
    ):
        raise ShimError("provider facade must be a loopback HTTP(S) URL")
    return token, base_url, model


def _provider_response(token, base_url, model, prompt):
    body = {
        "model": model,
        "input": prompt,
        "stream": False,
    }
    reasoning_effort = os.environ.get("OPENAI_REASONING_EFFORT", "").strip().lower()
    if reasoning_effort and reasoning_effort != "none":
        body["reasoning"] = {"effort": reasoning_effort}
    request = urllib.request.Request(
        base_url + "/responses",
        data=json.dumps(body, separators=(",", ":")).encode("utf-8"),
        headers={
            "Authorization": "Bearer " + token,
            "Content-Type": "application/json",
            "Accept": "application/json",
        },
        method="POST",
    )
    try:
        with urllib.request.urlopen(request, timeout=30) as response:
            raw = response.read(MAX_RESPONSE_BYTES + 1)
            status = response.status
    except urllib.error.HTTPError as error:
        raise ShimError("provider request failed with status %d" % error.code) from None
    except (urllib.error.URLError, OSError, TimeoutError, ValueError):
        raise ShimError("provider request failed") from None
    if status < 200 or status >= 300 or len(raw) > MAX_RESPONSE_BYTES:
        raise ShimError("provider returned an invalid response")
    try:
        parsed = json.loads(raw.decode("utf-8"))
    except (UnicodeDecodeError, json.JSONDecodeError):
        raise ShimError("provider returned invalid JSON") from None
    if not isinstance(parsed, dict):
        raise ShimError("provider returned an invalid response")
    return parsed


def _response_text(response):
    output_text = response.get("output_text")
    if isinstance(output_text, str):
        return output_text
    for item in response.get("output", []):
        if not isinstance(item, dict) or item.get("type") != "message":
            continue
        for content in item.get("content", []):
            if not isinstance(content, dict):
                continue
            if content.get("type") in ("output_text", "text") and isinstance(
                content.get("text"), str
            ):
                return content["text"]
    for choice in response.get("choices", []):
        if not isinstance(choice, dict):
            continue
        message = choice.get("message")
        if isinstance(message, dict) and isinstance(message.get("content"), str):
            return message["content"]
    raise ShimError("provider returned no assistant text")


def _usage(response):
    raw = response.get("usage")
    if not isinstance(raw, dict):
        return None
    usage = {}
    for name, aliases in (
        ("input_tokens", ("input_tokens", "prompt_tokens")),
        ("output_tokens", ("output_tokens", "completion_tokens")),
        ("total_tokens", ("total_tokens",)),
        ("cached_tokens", ("cached_tokens",)),
    ):
        for alias in aliases:
            value = raw.get(alias)
            if isinstance(value, int) and value >= 0:
                usage[name] = value
                break
    return usage or None


def _session_payload(session_id):
    if not isinstance(session_id, str) or not session_id.strip():
        raise ShimError("session id is required")
    return {
        "native_session_id": "shim:" + session_id,
        "native_cursor": "0",
    }


def _execution_payload(prompt):
    token, base_url, model = _provider_configuration()
    response = _provider_response(token, base_url, model, prompt)
    text = _response_text(response)
    events = [
        {
            "event_type": "assistant.text_chunk",
            "durable": False,
            "payload": {"content": text},
            "native_cursor": None,
        },
        {
            "event_type": "assistant.final",
            "durable": True,
            "payload": {"text": text},
            "native_cursor": None,
        },
    ]
    return {
        "events": events,
        "final_text": text,
        "usage": _usage(response),
    }


def _handle(frame):
    if not isinstance(frame, dict):
        raise ShimError("request must be an object")
    if frame.get("protocol_version") != 1:
        raise ShimError("unsupported protocol version")
    if frame.get("protocol_kind") != "openai_compatible_shim":
        raise ShimError("unsupported shim protocol")
    harness_id = frame.get("harness_id")
    if harness_id not in HARNESS_ENTRYPOINTS:
        raise ShimError("unsupported OpenAI-compatible shim harness")
    arguments = frame.get("harness_args", [])
    if arguments:
        raise ShimError("shim arguments are not allowlisted")
    operation = frame.get("operation")
    if not isinstance(operation, str):
        raise ShimError("operation is required")
    if operation in (
        "create_session",
        "fork_session",
        "import_session",
        "resume_session",
    ):
        return _session_payload(frame.get("session_id"))
    if operation in ("execute", "start", "resume", "steer"):
        payload = frame.get("payload")
        if not isinstance(payload, dict) or not isinstance(payload.get("prompt"), str):
            raise ShimError("prompt is required")
        return _execution_payload(payload["prompt"])
    if operation in ("cancel", "quiesce", "reconcile"):
        return {
            "events": [
                {
                    "event_type": "turn.cancelled"
                    if operation == "cancel"
                    else "turn.reconciled",
                    "durable": True,
                    "payload": {},
                    "native_cursor": "0",
                }
            ],
            "final_text": None,
            "usage": None,
        }
    if operation == "checkpoint":
        payload = frame.get("payload")
        attempt_id = payload.get("attempt_id") if isinstance(payload, dict) else None
        return {
            "checkpoint_ref": "openai-shim:" + str(attempt_id or frame.get("session_id")),
            "native_cursor": "0",
        }
    if operation in ("close_session", "delete_session"):
        return {}
    if operation == "exchange":
        return {"accepted": True}
    raise ShimError("unsupported shim operation")


def _write(frame):
    sys.stdout.write(json.dumps(frame, separators=(",", ":"), ensure_ascii=True) + "\n")
    sys.stdout.flush()


def main():
    for raw_line in sys.stdin.buffer:
        if len(raw_line) > MAX_FRAME_BYTES:
            _write(
                {
                    "request_id": NIL_REQUEST_ID,
                    "ok": False,
                    "error": "request frame is too large",
                }
            )
            continue
        try:
            frame = json.loads(raw_line)
            request_id = (
                frame.get("request_id", NIL_REQUEST_ID)
                if isinstance(frame, dict)
                else NIL_REQUEST_ID
            )
            payload = _handle(frame)
            _write({"request_id": request_id, "ok": True, "payload": payload})
        except ShimError as error:
            request_id = (
                frame.get("request_id", NIL_REQUEST_ID)
                if isinstance(frame, dict)
                else NIL_REQUEST_ID
            )
            _write({"request_id": request_id, "ok": False, "error": str(error)})
        except (json.JSONDecodeError, UnicodeDecodeError):
            _write(
                {
                    "request_id": NIL_REQUEST_ID,
                    "ok": False,
                    "error": "request must be valid JSON",
                }
            )
        except Exception:
            _write(
                {
                    "request_id": NIL_REQUEST_ID,
                    "ok": False,
                    "error": "shim request failed",
                }
            )


if __name__ == "__main__":
    main()
