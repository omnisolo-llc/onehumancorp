import json
import os
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
import subprocess
import sys
import threading
import unittest


REPO_ROOT = Path(__file__).resolve().parents[2]
SIDECAR = REPO_ROOT / "src/server/harness/sidecars/openai_compatible_shim.py"
SECRET = "shim-python-secret-canary"
SESSION_ID = "00000000-0000-0000-0000-000000000001"
REQUEST_ID = "00000000-0000-0000-0000-000000000002"


class ProviderHandler(BaseHTTPRequestHandler):
    requests = []

    def log_message(self, *_):
        pass

    def do_POST(self):
        size = int(self.headers.get("content-length", "0"))
        body = json.loads(self.rfile.read(size))
        type(self).requests.append(
            {
                "path": self.path,
                "authorization": self.headers.get("authorization"),
                "body": body,
            }
        )
        encoded = json.dumps(
            {
                "id": "shim-python-response",
                "output": [
                    {
                        "type": "message",
                        "content": [
                            {"type": "output_text", "text": "python-shim-ok"}
                        ],
                    }
                ],
                "usage": {
                    "input_tokens": 2,
                    "output_tokens": 3,
                    "total_tokens": 5,
                },
            },
            separators=(",", ":"),
        ).encode()
        self.send_response(200)
        self.send_header("content-type", "application/json")
        self.send_header("content-length", str(len(encoded)))
        self.end_headers()
        self.wfile.write(encoded)


class OpenAiCompatibleShimTest(unittest.TestCase):
    def setUp(self):
        ProviderHandler.requests = []
        self.server = ThreadingHTTPServer(("127.0.0.1", 0), ProviderHandler)
        self.thread = threading.Thread(target=self.server.serve_forever, daemon=True)
        self.thread.start()
        self.process = None

    def tearDown(self):
        self.stop_sidecar()
        self.server.shutdown()
        self.server.server_close()
        self.thread.join(timeout=3)

    def stop_sidecar(self):
        if self.process is None:
            return
        self.process.terminate()
        try:
            self.process.wait(timeout=3)
        except subprocess.TimeoutExpired:
            self.process.kill()
            self.process.wait(timeout=3)
        for stream in (self.process.stdin, self.process.stdout, self.process.stderr):
            stream.close()
        self.process = None

    def start_sidecar(self, harness_id="aider", base_url=None):
        if base_url is None:
            base_url = "http://127.0.0.1:%d/v1" % self.server.server_port
        environment = {
            "OPENAI_API_KEY": SECRET,
            "OPENAI_API_BASE_URL": base_url,
            "OPENAI_MODEL": "gpt-5.6-luna",
            "OPENAI_REASONING_EFFORT": "max",
            "PATH": os.environ.get("PATH", ""),
        }
        self.process = subprocess.Popen(
            [sys.executable, str(SIDECAR)],
            cwd=str(REPO_ROOT),
            env=environment,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )
        return harness_id

    def send(self, operation, harness_id="aider", **extra):
        frame = {
            "protocol_version": 1,
            "request_id": REQUEST_ID,
            "protocol_kind": "openai_compatible_shim",
            "harness_id": harness_id,
            "operation": operation,
            "session_id": SESSION_ID,
        }
        frame.update(extra)
        self.process.stdin.write(json.dumps(frame) + "\n")
        self.process.stdin.flush()
        return json.loads(self.process.stdout.readline())

    def test_all_pinned_harnesses_share_the_provider_translation(self):
        for harness_id in ("aider", "goose", "open-interpreter", "plandex"):
            ProviderHandler.requests = []
            self.start_sidecar(harness_id)
            created = self.send("create_session", harness_id)
            self.assertEqual(created["payload"]["native_session_id"], "shim:" + SESSION_ID)
            executed = self.send(
                "execute",
                harness_id,
                payload={"prompt": "shared prompt"},
            )
            self.assertTrue(executed["ok"])
            self.assertEqual(executed["payload"]["final_text"], "python-shim-ok")
            self.assertEqual(executed["payload"]["usage"]["total_tokens"], 5)
            self.assertEqual(len(ProviderHandler.requests), 1)
            request = ProviderHandler.requests[0]
            self.assertEqual(request["path"], "/v1/responses")
            self.assertEqual(request["authorization"], "Bearer " + SECRET)
            self.assertEqual(request["body"]["model"], "gpt-5.6-luna")
            self.assertEqual(request["body"]["input"], "shared prompt")
            self.stop_sidecar()

    def test_unknown_harness_and_unallowlisted_arguments_fail_closed(self):
        self.start_sidecar()
        unknown = self.send("execute", "unknown", payload={"prompt": "nope"})
        self.assertFalse(unknown["ok"])
        self.assertIn("unsupported OpenAI-compatible shim harness", unknown["error"])
        self.assertNotIn(SECRET, json.dumps(unknown))
        unsafe = self.send(
            "execute",
            "aider",
            harness_args=["--shell=$(cat /run/secrets/key)"],
            payload={"prompt": "nope"},
        )
        self.assertFalse(unsafe["ok"])
        self.assertIn("shim arguments are not allowlisted", unsafe["error"])
        self.assertEqual(ProviderHandler.requests, [])

    def test_non_loopback_provider_url_is_rejected_without_network_access(self):
        self.start_sidecar(base_url="https://provider.example/v1")
        response = self.send("execute", payload={"prompt": "nope"})
        self.assertFalse(response["ok"])
        self.assertIn("provider facade must be a loopback HTTP(S) URL", response["error"])
        self.assertEqual(ProviderHandler.requests, [])


if __name__ == "__main__":
    unittest.main()
