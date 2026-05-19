# Miser: Implement HTTP API GZIP Compression for Cost Efficiency

## Blocker Notes
- During the implementation of the HTTP GZIP compression middleware `src/server/utils/gzip_middleware.rs` using the `axum` web framework, we found that true streaming compression (e.g., via `tower-http`'s `CompressionLayer` or `async-compression`) was restricted since those ecosystem crates are omitted from the existing repository `Cargo.toml`.
- While an initial approach utilized an in-memory buffer (`axum::body::to_bytes(body, usize::MAX).await`), feedback highlighted that this introduces an Out-of-Memory (OOM) / Denial of Service (DoS) vulnerability and breaks streaming endpoints like Server-Sent Events (SSE).
- To provide a functional and safe workaround within the existing project constraints, the implementation strictly applies the 50MB in-memory buffering limit _only_ to `application/json` responses, explicitly ignoring other content types and bypassing streams (SSE, binary objects) to prevent hangs or resource exhaustion on real-time routes.
- This workaround successfully solves the user request for compressing JSON payloads on API endpoints (e.g., `/api/v1/telemetry`, `agents`, `missions`) while preventing the systemic regressions identified in the previous code review.
- A spurious side-effect modifying `.bazelrc` to disable remote caches/buildbuddy to bypass transient unauthenticated Docker pull rate limits has been fully reverted.
