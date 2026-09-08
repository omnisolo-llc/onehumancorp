"""Opt-in installed pinned CLI probe; uses a local fixture, never a real provider.

Run: python3 deploy/tests/pinned_shim_cli_probe.py aider|goose|open-interpreter|plandex
The named pinned executable must be installed. Plandex also needs the pinned local server.
"""

import importlib.util, json, os, sys, threading
from pathlib import Path
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

spec = importlib.util.spec_from_file_location(
    "shim",
    Path(__file__).resolve().parents[2]
    / "src/server/harness/sidecars/openai_compatible_shim.py",
)
shim = importlib.util.module_from_spec(spec)
spec.loader.exec_module(shim)


class Handler(BaseHTTPRequestHandler):
    def log_message(self, *args):
        pass

    def do_POST(self):
        body = json.loads(self.rfile.read(int(self.headers["Content-Length"])))
        print(
            "REQUEST",
            self.path,
            body.get("model"),
            body.get("reasoning_effort", body.get("reasoning")),
            body.get("stream"),
            file=sys.stderr,
        )
        data = {
            "id": "chatcmpl-fixture",
            "object": "chat.completion",
            "created": 1,
            "model": "gpt-5.6-luna",
            "choices": [
                {
                    "index": 0,
                    "message": {"role": "assistant", "content": "PINNED_CLI_OK"},
                    "finish_reason": "stop",
                }
            ],
            "usage": {"prompt_tokens": 10, "completion_tokens": 5, "total_tokens": 15},
        }
        if self.path.endswith("/responses"):
            data = {
                "id": "resp-fixture",
                "object": "response",
                "created_at": 1,
                "status": "completed",
                "model": "gpt-5.6-luna",
                "output": [
                    {
                        "id": "msg-fixture",
                        "type": "message",
                        "role": "assistant",
                        "status": "completed",
                        "content": [
                            {
                                "type": "output_text",
                                "text": "PINNED_CLI_OK",
                                "annotations": [],
                            }
                        ],
                    }
                ],
                "usage": {"input_tokens": 10, "output_tokens": 5, "total_tokens": 15},
            }
        if body.get("stream") and self.path.endswith("/responses"):
            events = [
                {
                    "type": "response.created",
                    "response": dict(data, status="in_progress", output=[]),
                },
                {
                    "type": "response.output_item.added",
                    "output_index": 0,
                    "item": dict(data["output"][0], status="in_progress", content=[]),
                },
                {
                    "type": "response.content_part.added",
                    "item_id": "msg-fixture",
                    "output_index": 0,
                    "content_index": 0,
                    "part": {"type": "output_text", "text": "", "annotations": []},
                },
                {
                    "type": "response.output_text.delta",
                    "item_id": "msg-fixture",
                    "output_index": 0,
                    "content_index": 0,
                    "delta": "PINNED_CLI_OK",
                },
                {
                    "type": "response.output_text.done",
                    "item_id": "msg-fixture",
                    "output_index": 0,
                    "content_index": 0,
                    "text": "PINNED_CLI_OK",
                },
                {
                    "type": "response.output_item.done",
                    "output_index": 0,
                    "item": data["output"][0],
                },
                {"type": "response.completed", "response": data},
            ]
            raw = "".join(
                "event: "
                + v["type"]
                + "\ndata: "
                + json.dumps(dict(v, sequence_number=i))
                + "\n\n"
                for i, v in enumerate(events)
            ).encode()
            ctype = "text/event-stream"
        elif body.get("stream"):
            chunks = [
                {
                    "id": "chatcmpl-fixture",
                    "object": "chat.completion.chunk",
                    "created": 1,
                    "model": "gpt-5.6-luna",
                    "choices": [
                        {
                            "index": 0,
                            "delta": {"role": "assistant", "content": "PINNED_CLI_OK"},
                            "finish_reason": None,
                        }
                    ],
                },
                {
                    "id": "chatcmpl-fixture",
                    "object": "chat.completion.chunk",
                    "created": 1,
                    "model": "gpt-5.6-luna",
                    "choices": [{"index": 0, "delta": {}, "finish_reason": "stop"}],
                    "usage": data["usage"],
                },
            ]
            raw = (
                "".join("data: " + json.dumps(v) + "\n\n" for v in chunks)
                + "data: [DONE]\n\n"
            ).encode()
            ctype = "text/event-stream"
        else:
            raw = json.dumps(data).encode()
            ctype = "application/json"
        self.send_response(200)
        self.send_header("Content-Type", ctype)
        self.send_header("Content-Length", str(len(raw)))
        self.end_headers()
        self.wfile.write(raw)


server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
threading.Thread(target=server.serve_forever, daemon=True).start()
os.environ.update(
    OPENAI_API_KEY="fixture-only-token",
    OPENAI_API_BASE_URL=f"http://127.0.0.1:{server.server_port}/v1",
    OPENAI_MODEL="gpt-5.6-luna",
    OPENAI_REASONING_EFFORT="max",
    OMNISOLO_SHIM_TIMEOUT_SECS="30",
)
try:
    result = shim._execution_payload(
        sys.argv[1], "Reply with PINNED_CLI_OK. Do not execute code."
    )
    assert "PINNED_CLI_OK" in result["final_text"], result
    assert result["usage"]["total_tokens"] >= 15, result
    print(json.dumps(result))
finally:
    server.shutdown()
