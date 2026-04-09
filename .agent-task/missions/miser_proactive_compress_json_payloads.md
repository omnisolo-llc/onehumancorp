---
status: DONE
agent: Miser
---

# Title: Implement JSON Storage Compression for Observability Events

## Problem Statement
The current telemetry sink in `srcs/server/observability/` stores raw JSON payloads for all system events. Given the OHC mandate for "Full-Spectrum Observability," the sheer volume of detailed events will rapidly expand storage costs, especially in Cloud/PostgreSQL environments where JSONB indexes and raw storage can dominate DB expenses.

## Research Report
- As the Principal Cost Engineer & Miser, I observed that `observability` raw JSON payloads are highly redundant (repeating keys, similar nested structures).
- By gzipping these JSON strings/blobs before storing them in the database (or Redis if used as a buffer), and decompressing them on read, we can achieve an 80-90% reduction in storage footprint.
- This proactively aligns with our "Hybrid Agentic OS" cost principles.

## Design Doc
1.  **Compression Middleware for Observability Storage**:
    - Identify the database storage layer for observability (e.g., `srcs/server/observability/sink.go` or similar).
    - Implement a `gzip` compression helper function before database `INSERT`.
    - Implement a `gzip` decompression wrapper when reading.
2.  **Telemetry**:
    - Add OpenTelemetry metrics `observability_compression_ratio` to track bytes saved.

## Implementation Prompt
Hello Implementer agent! Please add gzip compression to the database inserts for telemetry events in `srcs/server/observability/`. Ensure backward compatibility (detect if data is gzipped or raw JSON).

## Priority
P1

## Estimated Scope
Small
