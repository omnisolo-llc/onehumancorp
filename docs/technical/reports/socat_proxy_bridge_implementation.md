# Socat Network Bridge Proxy Implementation

## Overview
This document records the architectural details and learnings from the implementation of the `NetworkBridgeProxy` for the OHC Hybrid Agent Harness. The proxy establishes a secure, bounded network environment for sub-agents executing within `bwrap` sandboxes.

## Architecture
To securely allowlist internet access from within an isolated `bwrap --unshare-net` namespace, we implemented a hybrid `socat` + Rust TCP proxy solution:

1. **Host-Side Rust Proxy**: A `tokio::net::TcpListener` binds to an ephemeral loopback port (`127.0.0.1:0`). It reads the first line of incoming HTTP(S) requests.
2. **Domain Allowlisting**:
   - For `CONNECT` requests (HTTPS), it parses the target domain and port directly from the request URI.
   - For regular HTTP requests, it parses the target domain from the `Host` header.
   - It validates the parsed target domain against a strict `allowed_hosts` vector. Unauthorized requests are immediately closed with `403 Forbidden`.
   - **Crucial Security Fix:** The proxy extracts the actual target host from the request line, mitigating SSRF (Server-Side Request Forgery) attacks where a malicious agent might spoof the `Host` header but direct the TCP connection elsewhere.
3. **Bridge to Sandbox**:
   - A host `socat` process is spawned, listening on a unique, ephemeral UNIX socket (`/tmp/ohc-agent-http-<uuid>.sock`) and forwarding traffic to the local Rust proxy port.
   - The UNIX socket is bind-mounted into the `bwrap` sandbox.
   - A startup script inside `bwrap` uses a second `socat` to listen on an internal TCP port (`3128`) and forward to the UNIX socket.
   - Environment variables (`HTTP_PROXY`, `HTTPS_PROXY`, `ALL_PROXY`) are pointed to this internal port.

## Key Learnings & Improvements
- **Race Conditions in Bwrap Setup**: Backgrounding `socat` inside the `bwrap` bash command (`socat ... &`) creates a race condition where the primary agent command might execute before `socat` has bound to the local port. We mitigated this by injecting a lightweight polling loop: `while ! nc -z 127.0.0.1 3128 2>/dev/null; do sleep 0.1; done;`.
- **SSRF Prevention**: A naive implementation relying solely on the HTTP `Host` header for validation is vulnerable to SSRF. `CONNECT` tunnels must be parsed and their specific target domain validated, as the TCP connection is established to the domain in the `CONNECT` line, regardless of the `Host` header.
- **Resource Management**: The `NetworkBridgeProxy` implements the `Drop` trait to cleanly terminate the host-side `socat` child process, abort the asynchronous proxy task, and unlink the UNIX socket file when the harness backend finishes execution.
