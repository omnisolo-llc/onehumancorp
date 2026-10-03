"""Run offline native tests with an owned loopback HTTP sink; never forward requests.

PostgreSQL and explicitly loopback HTTP fixtures remain reachable. Any request
sent to the sink makes the gate fail, even when the Rust process exits zero.
"""
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import json
import os
import subprocess
import sys
import threading


def run(command):
    attempts = []

    class Sink(BaseHTTPRequestHandler):
        def setup(self):
            # Count even malformed/unsupported methods without retaining a URL,
            # header, token, body or other private request content.
            attempts.append("connection")
            super().setup()

        def deny(self):
            self.close_connection = True
            self.send_response(502)
            self.send_header("Connection", "close")
            self.send_header("Content-Length", "0")
            self.end_headers()

        do_CONNECT = deny
        do_GET = deny
        do_HEAD = deny
        do_POST = deny
        do_PUT = deny
        do_PATCH = deny
        do_DELETE = deny
        do_OPTIONS = deny
        do_TRACE = deny

        def log_message(self, *_):
            pass

    server = ThreadingHTTPServer(("127.0.0.1", 0), Sink)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    env = os.environ.copy()
    proxy = f"http://127.0.0.1:{server.server_port}"
    for name in ("HTTP_PROXY", "HTTPS_PROXY", "ALL_PROXY", "http_proxy", "https_proxy", "all_proxy"):
        env[name] = proxy
    env["NO_PROXY"] = env["no_proxy"] = "127.0.0.1,localhost,::1"
    env.pop("REDIS_URL", None)
    try:
        result = subprocess.run(command, env=env, check=False)
    finally:
        server.shutdown()
        server.server_close()
        thread.join()
    print(json.dumps({"denied_non_loopback_http_attempts": len(attempts)}), flush=True)
    return result.returncode if result.returncode != 0 else (1 if attempts else 0)


if __name__ == "__main__":
    if len(sys.argv) < 3 or sys.argv[1] != "--":
        raise SystemExit("Usage: deny_http_egress.py -- command [arguments]")
    raise SystemExit(run(sys.argv[2:]))
