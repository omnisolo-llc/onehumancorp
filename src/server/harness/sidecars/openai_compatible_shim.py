#!/usr/bin/env python3
"""Run deployment-pinned CLIs behind the scoped OpenAI-compatible facade.

Native option contracts:
https://github.com/Aider-AI/aider/blob/v0.86.0/aider/args.py
https://github.com/aaif-goose/goose/blob/v1.33.1/crates/goose-cli/src/cli.rs
https://github.com/OpenInterpreter/open-interpreter/blob/v0.4.2/interpreter/terminal_interface/start_terminal_interface.py
https://github.com/plandex-ai/plandex/tree/cli/v2.2.1/app/cli
"""

import json
import contextlib
import base64
import hashlib
import http.server
import fcntl
import pty
import re
import termios
import selectors
import signal
import subprocess
import tempfile
import threading
import uuid
import time
from pathlib import Path
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
            if isinstance(value, int) and not isinstance(value, bool) and value >= 0:
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


class ObservationProxy:
    """Forward CLI-originated requests; retain usage from actual provider replies."""

    def __init__(self, token, base_url, model):
        self.token, self.base_url, self.model = token, base_url, model
        self.usage = []
        self.requests = 0
        self.failures = 0
        owner = self

        class Handler(http.server.BaseHTTPRequestHandler):
            def log_message(self, *_):
                pass

            def do_GET(self):
                # Model discovery is local and never makes an inference turn.
                if self.path != "/v1/models":
                    self.send_error(404)
                    return
                raw = json.dumps(
                    {"object": "list", "data": [{"id": model, "object": "model"}]}
                ).encode()
                self.send_response(200)
                self.send_header("Content-Type", "application/json")
                self.send_header("Content-Length", str(len(raw)))
                self.end_headers()
                self.wfile.write(raw)

            def do_POST(self):
                if self.path not in ("/v1/chat/completions", "/v1/responses"):
                    self.send_error(404)
                    return
                if self.headers.get("Authorization") != "Bearer " + token:
                    self.send_error(401)
                    return
                try:
                    length = int(self.headers.get("Content-Length", "0"))
                    if not 0 < length <= MAX_RESPONSE_BYTES:
                        raise ValueError()
                    body = json.loads(self.rfile.read(length))
                    body["model"] = model
                    effort = os.environ.get("OPENAI_REASONING_EFFORT", "").strip()
                    if effort and effort != "none":
                        if self.path.endswith("/responses"):
                            body["reasoning"] = {"effort": effort}
                        else:
                            body["reasoning_effort"] = effort
                    if body.get("stream") and self.path.endswith("/chat/completions"):
                        body["stream_options"] = {"include_usage": True}
                    request = urllib.request.Request(
                        base_url + self.path.removeprefix("/v1"),
                        data=json.dumps(body).encode(),
                        headers={
                            "Authorization": "Bearer " + token,
                            "Content-Type": "application/json",
                        },
                    )
                    with urllib.request.urlopen(request, timeout=30) as response:
                        raw = response.read(MAX_RESPONSE_BYTES + 1)
                        content_type = response.headers.get(
                            "Content-Type", "application/json"
                        )
                        if len(raw) > MAX_RESPONSE_BYTES:
                            raise ValueError()
                    owner.requests += 1
                    if "text/event-stream" in content_type:
                        for line in raw.splitlines():
                            if (
                                line.startswith(b"data:")
                                and line[5:].strip() != b"[DONE]"
                            ):
                                with contextlib.suppress(ValueError):
                                    owner.record_usage(json.loads(line[5:]))
                    else:
                        owner.record_usage(json.loads(raw))
                    self.send_response(200)
                    self.send_header("Content-Type", content_type)
                    self.send_header("Content-Length", str(len(raw)))
                    self.end_headers()
                    self.wfile.write(raw)
                except Exception:
                    owner.failures += 1
                    with contextlib.suppress(OSError):
                        self.send_error(502, "scoped provider request failed")

        self.server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), Handler)
        self.thread = threading.Thread(target=self.server.serve_forever, daemon=True)

    def record_usage(self, response):
        if not isinstance(response, dict):
            return
        if (
            str(response.get("type", "")).startswith("response.")
            and response.get("type") != "response.completed"
        ):
            return
        usage = _usage(response)
        if usage is None and isinstance(response.get("response"), dict):
            usage = _usage(response["response"])
        if usage:
            self.usage.append(usage)

    def __enter__(self):
        self.thread.start()
        return self

    def __exit__(self, *_):
        self.server.shutdown()
        self.server.server_close()
        self.thread.join(timeout=2)

    @property
    def url(self):
        return "http://127.0.0.1:%d/v1" % self.server.server_port


def _command(harness_id, home, model):
    # Flags verified against the deployment-pinned upstream release sources.
    if harness_id == "aider":
        return [
            "aider",
            "--model",
            "openai/" + model,
            "--message-file",
            "/dev/stdin",
            "--no-stream",
            "--no-pretty",
            "--no-git",
            "--no-auto-commits",
            "--no-check-update",
            "--no-show-release-notes",
            "--no-analytics",
            "--yes-always",
            "--no-show-model-warnings",
            "--chat-mode",
            "ask",
        ]
    if harness_id == "goose":
        return [
            "goose",
            "run",
            "--instructions",
            "-",
            "--no-session",
            "--no-profile",
            "--quiet",
            "--max-turns",
            "1",
            "--provider",
            "openai",
            "--model",
            model,
        ]
    if harness_id == "open-interpreter":
        # --stdin in 0.4.2 reads one line only. The profile preserves the complete
        # multiline prompt through stdin, then exits after a single chat call.
        profile = Path(home) / "omnisolo.py"
        profile.write_text(
            "import sys\nfrom interpreter import interpreter\n"
            "interpreter.offline = True\ninterpreter.auto_run = False\n"
            "interpreter.plain_text_display = True\n"
            "interpreter.chat(sys.stdin.read(), display=True)\nraise SystemExit(0)\n"
        )
        return [
            "interpreter",
            "--profile",
            str(profile),
            "--model",
            "openai/" + model,
            "--plain",
            "--offline",
        ]
    return ["plandex", "tell", "--stop", "--no-build", "--no-exec", "--skip-menu"]


def _run_command(command, prompt, environment, cwd):
    timeout = float(os.environ.get("OMNISOLO_SHIM_TIMEOUT_SECS", "90"))
    if not 0 < timeout <= 300:
        raise ShimError("invalid process timeout")
    terminal = None
    if command[:2] == ["plandex", "tell"]:
        terminal = pty.openpty()
        command = [
            sys.executable,
            str(Path(__file__).resolve()),
            "--exec-with-tty",
            str(terminal[1]),
            *command,
        ]
    try:
        child = subprocess.Popen(
            command,
            pass_fds=(() if terminal is None else (terminal[1],)),
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            env=environment,
            cwd=cwd,
            start_new_session=True,
        )
    except OSError:
        if terminal:
            for fd in terminal:
                os.close(fd)
        raise ShimError(
            "process_spawn: pinned harness command is unavailable"
        ) from None
    # Each prompt owns a process group; cancellation and all exit paths reap it.
    deadline = time.monotonic() + timeout
    output = bytearray()
    total = 0
    try:
        with selectors.DefaultSelector() as selector:
            for stream in (child.stdin, child.stdout, child.stderr):
                os.set_blocking(stream.fileno(), False)
            data = memoryview(prompt.encode())
            selector.register(child.stdin, selectors.EVENT_WRITE)
            selector.register(child.stdout, selectors.EVENT_READ)
            selector.register(child.stderr, selectors.EVENT_READ)
            while selector.get_map():
                if time.monotonic() >= deadline:
                    raise ShimError("process_timeout: harness deadline exceeded")
                for key, _ in selector.select(
                    min(0.1, max(0, deadline - time.monotonic()))
                ):
                    stream = key.fileobj
                    if stream is child.stdin:
                        if data:
                            try:
                                data = data[os.write(stream.fileno(), data[:65536]) :]
                            except BrokenPipeError:
                                data = data[len(data) :]
                        if not data:
                            selector.unregister(stream)
                            stream.close()
                    else:
                        chunk = os.read(stream.fileno(), 65536)
                        if not chunk:
                            selector.unregister(stream)
                            continue
                        total += len(chunk)
                        if total > MAX_RESPONSE_BYTES:
                            raise ShimError(
                                "output_limit: harness output exceeded limit"
                            )
                        if stream is child.stdout:
                            output.extend(chunk)
            code = child.wait(timeout=max(0.01, deadline - time.monotonic()))
            if code != 0:
                raise ShimError("process_exit: harness exited with status %d" % code)
            return output.decode("utf-8", errors="replace").strip()
    finally:
        with contextlib.suppress(ProcessLookupError):
            os.killpg(child.pid, signal.SIGKILL)
        child.wait()
        for stream in (child.stdin, child.stdout, child.stderr):
            stream.close()
        if terminal:
            for fd in terminal:
                os.close(fd)


def _prepare_plandex(home, environment, proxy, model, cwd):
    host = os.environ.get("PLANDEX_API_HOST", "http://127.0.0.1:8099").rstrip("/")
    parsed = urllib.parse.urlsplit(host)
    if (
        parsed.scheme != "http"
        or parsed.hostname not in ("127.0.0.1", "localhost")
        or parsed.path
        or parsed.username
        or parsed.password
        or parsed.query
        or parsed.fragment
    ):
        raise ShimError(
            "plandex_configuration: native server must use a loopback HTTP URL"
        )
    environment.update(PLANDEX_ENV="development", PLANDEX_API_HOST=host)
    # Bootstrap an isolated native account through the pinned server's local-mode
    # API. Its auth file is temporary and mode 0600; no host account is reused.
    request = urllib.request.Request(
        host + "/accounts",
        data=json.dumps(
            {
                "email": "omnisolo-" + uuid.uuid4().hex + "@local.invalid",
                "userName": "OmniSolo scoped attempt",
                "pin": "",
            }
        ).encode(),
        headers={"Content-Type": "application/json", "X-Client-Version": "2.2.1"},
    )
    try:
        with urllib.request.urlopen(request, timeout=30) as response:
            account = json.loads(response.read(MAX_FRAME_BYTES))
        if account.get("isLocalMode") is not True or not account.get("token"):
            raise ValueError()
    except Exception:
        raise ShimError(
            "plandex_bootstrap: local native server account creation failed"
        ) from None
    org = (account.get("orgs") or [{}])[0]
    auth = {key: account.get(key) for key in ("userId", "userName", "email", "token")}
    auth.update(
        isCloud=False,
        host=host,
        isLocalMode=True,
        orgId=org.get("id", ""),
        orgName=org.get("name", ""),
        orgIsTrial=False,
        integratedModelsMode=False,
    )
    directory = Path(home) / ".plandex-home-dev-v2"
    directory.mkdir(mode=0o700)
    auth_path = directory / "auth.json"
    auth_path.write_text(json.dumps(auth))
    auth_path.chmod(0o600)
    model_id = "omnisolo-scoped-model"
    roles = (
        "planner",
        "architect",
        "coder",
        "summarizer",
        "builder",
        "wholeFileBuilder",
        "names",
        "commitMessages",
        "autoContinue",
    )
    config = {
        "providers": [
            {
                "name": "omnisolo-scoped",
                "baseUrl": proxy.url,
                "apiKeyEnvVar": "OPENAI_API_KEY",
            }
        ],
        "models": [
            {
                "modelId": model_id,
                "publisher": "omnisolo",
                "description": "Scoped OmniSolo model selection",
                "defaultMaxConvoTokens": 16000,
                "maxTokens": 32768,
                "maxOutputTokens": 8192,
                "reservedOutputTokens": 8192,
                "preferredOutputFormat": "xml",
                "providers": [
                    {
                        "provider": "custom",
                        "customProvider": "omnisolo-scoped",
                        "modelName": model,
                    }
                ],
            }
        ],
        "modelPacks": [
            dict(
                {role: model_id for role in roles},
                name="omnisolo-scoped",
                description="Scoped OmniSolo model for every role",
            )
        ],
    }
    path = Path(home) / "models.json"
    path.write_text(json.dumps(config))
    for command in (
        ["plandex", "models", "custom", "--file", str(path), "--save"],
        ["plandex", "set-model", "default", "omnisolo-scoped"],
        ["plandex", "set-auto", "default", "none"],
        ["plandex", "new", "--name", "omnisolo-scoped", "--no-auto"],
    ):
        _run_command(command, "", environment, cwd)
    return auth["token"]


@contextlib.contextmanager
def _plandex_cleanup(home):
    failed = False
    try:
        yield
    except BaseException:
        failed = True
        raise
    finally:
        try:
            _delete_plandex_plan(home)
        except ShimError:
            if not failed:
                raise


def _delete_plandex_plan(home):
    directory = Path(home) / ".plandex-home-dev-v2"
    auth_path = directory / "auth.json"
    if not auth_path.exists():
        return
    try:
        auth = json.loads(auth_path.read_text())
        org_hash = hashlib.sha256(
            (
                auth.get("orgName", "")
                + "||"
                + str(auth.get("orgIsTrial", False)).lower()
                + "||"
                + str(auth.get("integratedModelsMode", False)).lower()
            ).encode()
        ).hexdigest()
        bearer = base64.urlsafe_b64encode(
            json.dumps(
                {
                    "token": auth["token"],
                    "orgId": auth.get("orgId", ""),
                    "hash": org_hash,
                }
            ).encode()
        ).decode()
        for path in directory.glob("*/current-plans-v2.json"):
            settings = json.loads(path.read_text()).get(auth["userId"], {})
            plan_id = str(uuid.UUID(settings["id"]))
            for suffix in ("/main/stop", ""):
                request = urllib.request.Request(
                    auth["host"] + "/plans/" + plan_id + suffix,
                    method="DELETE",
                    headers={
                        "Authorization": "Bearer " + bearer,
                        "X-Client-Version": "2.2.1",
                    },
                )
                try:
                    with urllib.request.urlopen(request, timeout=1) as response:
                        response.read(MAX_FRAME_BYTES)
                except urllib.error.HTTPError as error:
                    if error.code != 404:
                        raise
    except Exception:
        raise ShimError(
            "plandex_cleanup: native plan could not be stopped and deleted"
        ) from None


def _execution_payload(harness_id, prompt):
    token, base_url, model = _provider_configuration()
    with tempfile.TemporaryDirectory(
        prefix="omnisolo-shim-"
    ) as directory, ObservationProxy(
        token, base_url, model
    ) as proxy, contextlib.ExitStack() as cleanup:
        home, cwd = str(Path(directory) / "home"), str(Path(directory) / "workspace")
        Path(home).mkdir(mode=0o700)
        Path(cwd).mkdir(mode=0o700)
        environment = {
            "PATH": os.environ.get("PATH", "/usr/local/bin:/usr/bin:/bin"),
            "HOME": home,
            "XDG_CONFIG_HOME": home + "/config",
            "XDG_DATA_HOME": home + "/data",
            "XDG_CACHE_HOME": home + "/cache",
            "TMPDIR": home,
            "TERM": "dumb",
            "NO_COLOR": "1",
            "PYTHONUNBUFFERED": "1",
            "OPENAI_API_KEY": token,
            "OPENAI_API_BASE_URL": proxy.url,
            "OPENAI_BASE_URL": proxy.url,
            "OPENAI_API_BASE": proxy.url,
            "OPENAI_MODEL": model,
            "AIDER_OPENAI_API_BASE": proxy.url,
            "GOOSE_PROVIDER": "openai",
            "GOOSE_MODEL": model,
            "OPENAI_HOST": proxy.url.removesuffix("/v1"),
            "OPENAI_BASE_PATH": "v1/chat/completions",
            "GOOSE_MODE": "chat",
            "GOOSE_DISABLE_KEYRING": "1",
            "LITELLM_TELEMETRY": "False",
            "DO_NOT_TRACK": "1",
        }
        native_token = None
        if harness_id == "plandex":
            cleanup.enter_context(_plandex_cleanup(home))
            native_token = _prepare_plandex(home, environment, proxy, model, cwd)
        text = _run_command(_command(harness_id, home, model), prompt, environment, cwd)
        if harness_id == "plandex":
            text = _run_command(
                ["plandex", "convo", "2", "--plain"], "", environment, cwd
            )
        text = re.sub(r"\x1b\[[0-?]*[ -/]*[@-~]", "", text).replace(token, "[REDACTED]")
        if native_token:
            text = text.replace(native_token, "[REDACTED]")
        if proxy.failures:
            raise ShimError("provider_failure: harness provider request failed")
        if not proxy.requests:
            raise ShimError(
                "provider_not_observed: harness made no scoped provider turn"
            )
        if not text:
            raise ShimError("process_output: harness returned no text")
        usage = {}
        for observed in proxy.usage:
            for key, value in observed.items():
                usage[key] = usage.get(key, 0) + value
        events = [
            {
                "event_type": "assistant.final",
                "durable": True,
                "payload": {
                    "text": text,
                    "harness_id": harness_id,
                    "source": "pinned_cli_stdout",
                },
                "native_cursor": None,
            }
        ]
        if usage:
            events.append(
                {
                    "event_type": "usage.recorded",
                    "durable": True,
                    "payload": {"usage": usage, "source": "observed_provider_response"},
                    "native_cursor": None,
                }
            )
        else:
            events.append(
                {
                    "event_type": "capability.downgraded",
                    "durable": True,
                    "payload": {
                        "capability": "usage",
                        "requested": "token_counts",
                        "effective": "unavailable",
                        "reason": "Provider returned no token usage",
                    },
                    "native_cursor": None,
                }
            )
        events.append(
            {
                "event_type": "turn.completed",
                "durable": True,
                "payload": {"exit_code": 0},
                "native_cursor": None,
            }
        )
        return {"events": events, "final_text": text, "usage": usage or None}


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
        return _execution_payload(harness_id, payload["prompt"])
    if operation in ("cancel", "quiesce", "reconcile"):
        return {
            "events": [
                {
                    "event_type": (
                        "turn.cancelled" if operation == "cancel" else "turn.reconciled"
                    ),
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
            "checkpoint_ref": "openai-shim:"
            + str(attempt_id or frame.get("session_id")),
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
    def terminate(signum, _frame):
        raise SystemExit(128 + signum)

    signal.signal(signal.SIGTERM, terminate)
    signal.signal(signal.SIGHUP, terminate)
    signal.signal(signal.SIGINT, terminate)
    while True:
        raw_line = sys.stdin.buffer.readline(MAX_FRAME_BYTES + 1)
        if not raw_line:
            break
        if len(raw_line) > MAX_FRAME_BYTES:
            _write(
                {
                    "request_id": NIL_REQUEST_ID,
                    "ok": False,
                    "error": "request frame is too large",
                }
            )
            return
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
    if len(sys.argv) > 1 and sys.argv[1] == "--exec-with-tty":
        # Plandex 2.2.1 requires a controlling tty even with a piped prompt.
        # Retain piped stdin/stdout; the native UI only uses the tty for input.
        descriptor = int(sys.argv[2])
        fcntl.ioctl(descriptor, termios.TIOCSCTTY, 0)
        os.execvpe(sys.argv[3], sys.argv[3:], os.environ)
    main()
