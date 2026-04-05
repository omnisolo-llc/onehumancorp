---
status: DONE
agent: Miser
---
# Title: Implement HTTP API GZIP Compression for Cost Efficiency

## Problem Statement
The OHC (One Human Corp) dashboard and API responses are currently uncompressed. Large JSON payloads, such as those returned by the `GET /api/v1/telemetry`, `GET /api/v1/agents`, and `GET /api/v1/missions` endpoints, consume significant cloud egress bandwidth. As the platform scales, this unoptimized egress will exponentially increase network infrastructure costs.

## Research Report
- Current HTTP responses use raw JSON encoding without `Content-Encoding: gzip`.
- Implementing an HTTP middleware that intercepts responses and compresses them with `gzip` when the client requests it (via `Accept-Encoding: gzip`) can reduce JSON payload sizes by up to 80-90%.
- This fulfills the Miser Cost Engineer mandate of optimizing cloud resource management and bandwidth efficiency.

## Design Doc
1. **Gzip Middleware**:
   - Create `srcs/server/utils/gzip_middleware.go` providing `GzipMiddleware(next http.Handler) http.Handler`.
   - The middleware will check for the `Accept-Encoding: gzip` header.
   - If present, it will wrap the `http.ResponseWriter` with a `gzip.Writer` to compress the payload on the fly.
2. **Integration**:
   - Apply the `GzipMiddleware` to the API routes in `srcs/server/dashboard/server.go`.

## Implementation Prompt
- Implement `GzipMiddleware`.
- Wrap the main mux or API routes in `dashboard/server.go` with this middleware.
- Ensure all tests pass.

## Priority
P2

## Estimated Scope
Small
