import urllib.request
import urllib.parse
import json
import os

token = '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'

title = "[backend] Implement Agent Harness Sandbox Network Proxy and Bwrap Integration"
body = """Parent: #5560

### Problem Statement
The OHC Agent Harness currently lacks robust filesystem sandboxing and network isolation. This poses a security risk and lacks observability for sub-agent execution, preventing us from safely running untrusted code or network requests.

### Research Report
Based on a deep audit of the leaked Claude Code execution environment, industry leaders utilize `bwrap` (Bubblewrap) on Linux to create strict filesystem mounts, along with local HTTP/SOCKS proxy servers to intercept and gate network requests. They track all violations securely.
Reference: `docs/research/agent_harness_network_proxy_audit.md`

### Design Doc
- Implement a Bubblewrap (`bwrap`) executor in `srcs/backend/harness` for Linux deployments.
- Build a Go-based local HTTP proxy to intercept and validate all network calls made by sub-agents against an allowed domains list.
- All network violations and file access denials should emit OpenTelemetry metrics (`telemetry.sandbox_violation_total`).

### Implementation Prompt
1. In `srcs/backend/harness`, create `bwrap_executor.go` that wraps the `bwrap` CLI command, setting up read-only mounts for root and isolated `/tmp` directories.
2. In the same directory, create `network_proxy.go` that starts an HTTP proxy server. Ensure it uses the existing `SandboxTelemetryEmitter` to log denied requests.
3. Update the orchestrator to inject the `HTTP_PROXY` environment variable pointing to the local proxy when spawning sub-agents.
4. Ensure 100% unit test coverage for the new executor and proxy components, utilizing mocked telemetry emitters.

### Priority
P0

### Estimated Scope
Large
"""

url = 'https://api.github.com/repos/onehumancorp/mono/issues'
headers = {
    'Authorization': f'token {token}',
    'Accept': 'application/vnd.github.v3+json'
}
data = {
    'title': title,
    'body': body,
    'labels': ['AI', 'enhancement']
}

req = urllib.request.Request(url, headers=headers, data=json.dumps(data).encode('utf-8'))
try:
    with urllib.request.urlopen(req) as response:
        res = json.loads(response.read().decode('utf-8'))
        print(f"Created issue: {res['html_url']}")
except Exception as e:
    print(f"Failed to create issue: {e}")
    if hasattr(e, 'read'):
        print(e.read().decode('utf-8'))
