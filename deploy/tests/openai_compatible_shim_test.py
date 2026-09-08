import json
import os
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
import subprocess
import sys
import threading
import tempfile
import textwrap
import time
import signal
import importlib.util
from unittest.mock import patch
import base64
import io
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

    def do_DELETE(self):
        type(self).requests.append({"path": self.path, "method": "DELETE"})
        self.send_response(204)
        self.end_headers()

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
        if self.path == "/accounts":
            encoded = json.dumps(
                {
                    "userId": "fixture-user",
                    "token": "local-native-token",
                    "email": "fixture@local.invalid",
                    "userName": "Fixture",
                    "isLocalMode": True,
                    "orgs": [{"id": "fixture-org", "name": "Fixture Org"}],
                }
            ).encode()
            self.send_response(200)
            self.send_header("Content-Length", str(len(encoded)))
            self.end_headers()
            self.wfile.write(encoded)
            return
        encoded = json.dumps(
            {
                "id": "shim-python-response",
                "output": [
                    {
                        "type": "message",
                        "content": [{"type": "output_text", "text": "python-shim-ok"}],
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
        self.directory = tempfile.TemporaryDirectory(prefix="shim-cli-fixture-")
        self.bin_dir = Path(self.directory.name)
        for name in ("aider", "goose", "interpreter", "plandex"):
            command = self.bin_dir / name
            command.write_text(
                "#!"
                + sys.executable
                + "\n"
                + textwrap.dedent(
                    """\
                import json, os, pathlib, sys, urllib.request
                prompt = sys.stdin.read()
                if pathlib.Path(sys.argv[0]).name == 'plandex' and sys.argv[1] != 'tell':
                    if sys.argv[1] == 'new':
                        path = pathlib.Path(os.environ['HOME'], '.plandex-home-dev-v2', 'fixture-project')
                        path.mkdir()
                        (path / 'current-plans-v2.json').write_text(json.dumps({'fixture-user':{'id':'00000000-0000-0000-0000-000000000099'}}))
                    if sys.argv[1] == 'convo': print('CLI:plandex:python-shim-ok')
                    sys.exit(0)
                name = pathlib.Path(sys.argv[0]).name
                pathlib.Path(sys.argv[0]).with_suffix('.state').write_text(json.dumps({'pid':os.getpid(), 'home':os.environ['HOME']}))
                assert os.getcwd() != os.environ['HOME']
                assert 'UNRELATED_SECRET' not in os.environ
                assert 'PYTHONPATH' not in os.environ
                assert os.environ['HOME'] != os.environ.get('ORIGINAL_HOME')
                if name == 'plandex':
                    assert pathlib.Path(os.environ['HOME'], '.plandex-home-dev-v2', 'auth.json').exists()
                if prompt == 'fail':
                    print(os.environ['OPENAI_API_KEY'], file=sys.stderr)
                    sys.exit(7)
                if prompt == 'timeout':
                    import time
                    import subprocess
                    descendant = subprocess.Popen([sys.executable, '-c', 'import time; time.sleep(60)'])
                    pathlib.Path(sys.argv[0]).with_suffix('.descendant').write_text(str(descendant.pid))
                    time.sleep(10)
                if prompt == 'oversize':
                    print('x' * 17000000)
                    sys.exit(0)
                if prompt == 'no-provider':
                    print('no provider turn')
                    sys.exit(0)
                base = os.environ['OPENAI_API_BASE_URL']
                body = {'model':os.environ['OPENAI_MODEL'], 'messages':[{'role':'user','content':prompt}], 'stream':False}
                request = urllib.request.Request(base + '/chat/completions', data=json.dumps(body).encode(),
                    headers={'Authorization':'Bearer ' + os.environ['OPENAI_API_KEY'], 'Content-Type':'application/json'})
                response = json.load(urllib.request.urlopen(request))
                if name == 'plandex': print('native turn finished')
                else: print('CLI:' + name + ':' + response['output'][0]['content'][0]['text'])
                print('ARGS:' + json.dumps(sys.argv[1:]))
                """
                )
            )
            command.chmod(0o755)

    def tearDown(self):
        self.stop_sidecar()
        self.server.shutdown()
        self.server.server_close()
        self.thread.join(timeout=3)
        self.directory.cleanup()

    def stop_sidecar(self):
        if self.process is None:
            return
        self.process.terminate()
        try:
            self.process.wait(timeout=10)
        except subprocess.TimeoutExpired:
            self.process.kill()
            self.process.wait(timeout=10)
        for stream in (self.process.stdin, self.process.stdout, self.process.stderr):
            stream.close()
        self.process = None

    def start_sidecar(self, harness_id="aider", base_url=None, timeout=10):
        if base_url is None:
            base_url = "http://127.0.0.1:%d/v1" % self.server.server_port
        environment = {
            "OPENAI_API_KEY": SECRET,
            "OPENAI_API_BASE_URL": base_url,
            "OPENAI_MODEL": "gpt-5.6-luna",
            "OPENAI_REASONING_EFFORT": "max",
            "PLANDEX_API_HOST": "http://127.0.0.1:%d" % self.server.server_port,
            "PATH": str(self.bin_dir) + os.pathsep + os.environ.get("PATH", ""),
            "UNRELATED_SECRET": "must-not-inherit",
            "PYTHONPATH": "/untrusted",
            "OMNISOLO_SHIM_TIMEOUT_SECS": str(timeout),
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

    def test_all_pinned_harnesses_execute_their_command_with_scoped_provider(self):
        for harness_id in ("aider", "goose", "open-interpreter", "plandex"):
            ProviderHandler.requests = []
            self.start_sidecar(harness_id)
            created = self.send("create_session", harness_id)
            self.assertEqual(
                created["payload"]["native_session_id"], "shim:" + SESSION_ID
            )
            executed = self.send(
                "execute",
                harness_id,
                payload={"prompt": "shared prompt"},
            )
            self.assertTrue(executed["ok"], executed)
            self.assertIn(
                "CLI:"
                + {"open-interpreter": "interpreter"}.get(harness_id, harness_id)
                + ":python-shim-ok",
                executed["payload"]["final_text"],
            )
            self.assertTrue(
                any(
                    event["event_type"] == "usage.recorded"
                    for event in executed["payload"]["events"]
                )
            )
            self.assertEqual(executed["payload"]["usage"]["total_tokens"], 5)
            requests = [
                request
                for request in ProviderHandler.requests
                if request["path"].startswith("/v1/")
            ]
            self.assertEqual(len(requests), 1)
            request = requests[0]
            self.assertEqual(request["path"], "/v1/chat/completions")
            self.assertEqual(request["authorization"], "Bearer " + SECRET)
            self.assertEqual(request["body"]["model"], "gpt-5.6-luna")
            self.assertEqual(request["body"]["messages"][0]["content"], "shared prompt")
            if harness_id == "plandex":
                self.assertTrue(
                    any(
                        request["path"] == "/plans/00000000-0000-0000-0000-000000000099"
                        for request in ProviderHandler.requests
                    )
                )
            self.stop_sidecar()

    def test_process_failures_timeouts_and_limits_are_typed_and_redacted(self):
        for prompt, failure in [
            ("fail", "process_exit"),
            ("timeout", "process_timeout"),
            ("oversize", "output_limit"),
            ("no-provider", "provider_not_observed"),
        ]:
            self.start_sidecar(timeout=2 if prompt == "timeout" else 10)
            response = self.send("execute", payload={"prompt": prompt})
            self.assertFalse(response["ok"], response)
            self.assertIn(failure, response["error"])
            self.assertNotIn(SECRET, json.dumps(response))
            self.stop_sidecar()

    def test_termination_kills_active_command_and_cleans_home(self):
        self.start_sidecar()
        frame = {
            "protocol_version": 1,
            "request_id": REQUEST_ID,
            "protocol_kind": "openai_compatible_shim",
            "harness_id": "aider",
            "operation": "execute",
            "session_id": SESSION_ID,
            "payload": {"prompt": "timeout"},
        }
        self.process.stdin.write(json.dumps(frame) + "\n")
        self.process.stdin.flush()
        state = self.bin_dir / "aider.state"
        deadline = time.monotonic() + 15
        while not state.exists() and time.monotonic() < deadline:
            time.sleep(0.02)
        self.assertTrue(state.exists())
        metadata = json.loads(state.read_text())
        self.stop_sidecar()
        self.assertFalse(Path(metadata["home"]).exists())
        with self.assertRaises(ProcessLookupError):
            os.kill(metadata["pid"], 0)

    def test_observer_counts_only_terminal_response_usage(self):
        spec = importlib.util.spec_from_file_location("shim_under_test", SIDECAR)
        shim = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(shim)
        with shim.ObservationProxy(
            SECRET, "http://127.0.0.1:1/v1", "model"
        ) as observer:
            response = {
                "id": "r1",
                "usage": {"input_tokens": 2, "output_tokens": 3, "total_tokens": 5},
            }
            observer.record_usage({"type": "response.created", "response": response})
            observer.record_usage({"type": "response.completed", "response": response})
            self.assertEqual(len(observer.usage), 1)

    def test_native_cleanup_stops_and_deletes_only_the_scoped_plan(self):
        spec = importlib.util.spec_from_file_location("shim_cleanup", SIDECAR)
        shim = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(shim)
        with tempfile.TemporaryDirectory() as home:
            directory = Path(home) / ".plandex-home-dev-v2"
            project = directory / "project"
            project.mkdir(parents=True)
            (directory / "auth.json").write_text(
                json.dumps(
                    {
                        "userId": "owner",
                        "orgId": "org",
                        "orgName": "Scoped Org",
                        "token": "native-token",
                        "host": "http://127.0.0.1:8099",
                    }
                )
            )
            (project / "current-plans-v2.json").write_text(
                json.dumps(
                    {"owner": {"id": SESSION_ID}, "other-user": {"id": REQUEST_ID}}
                )
            )
            requests = []

            def respond(request, **kwargs):
                requests.append(request)
                return io.BytesIO(b"")

            with patch.object(shim.urllib.request, "urlopen", side_effect=respond):
                shim._delete_plandex_plan(home)
            self.assertEqual(
                [request.full_url for request in requests],
                [
                    "http://127.0.0.1:8099/plans/" + SESSION_ID + "/main/stop",
                    "http://127.0.0.1:8099/plans/" + SESSION_ID,
                ],
            )
            self.assertTrue(
                all(request.get_method() == "DELETE" for request in requests)
            )
            auth = json.loads(
                base64.urlsafe_b64decode(requests[0].get_header("Authorization")[7:])
            )
            self.assertEqual(auth["orgId"], "org")
            self.assertEqual(auth["token"], "native-token")

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
        self.assertIn(
            "provider facade must be a loopback HTTP(S) URL", response["error"]
        )
        self.assertEqual(ProviderHandler.requests, [])


if __name__ == "__main__":
    unittest.main()
