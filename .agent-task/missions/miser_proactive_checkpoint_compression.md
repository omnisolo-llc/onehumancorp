---
status: IN_PROGRESS
agent: Miser
---

# Title: Implement Storage Compression for LangGraph Checkpoints

## Problem Statement
The `PGCheckpointer` (in `srcs/server/checkpointer/checkpointer.go`) currently stores the full `state` JSON blob uncompressed into the `checkpoints` table. Since agent workflows generate extensive histories and tool definitions in their state, these JSON blobs can grow very large. Uncompressed state storage dramatically increases PostgreSQL database costs and local SQLite disk utilization, violating the cost-optimization principles of the "Hybrid Agentic OS".

## Research Report
- Current `PGCheckpointer` marshals `state` to JSON and stores it natively as `[]byte` mapping to JSONB/JSON depending on the database.
- By compressing the JSON payload using `gzip` and storing it as `base64` or raw `[]byte` before inserting, we can achieve up to 80-90% reduction in storage size for repetitive text and schema-heavy payloads.
- We must provide a backward-compatibility layer so that old, uncompressed checkpoints can still be read successfully (`decompress` falling back to raw data if not gzip-encoded).

## Design Doc
1. **Checkpoint Compression**:
   - Create helper methods `compressData` and `decompressData` inside `checkpointer.go` (similar to what exists in `cached_minimax_client.go`, maybe copy them or implement fresh using standard `compress/gzip` and `encoding/base64`).
   - In `SaveCheckpoint`, compress the `stateBytes` before upserting into the database.
   - In `LoadCheckpoint`, decompress the `stateBytes` after fetching from the database before decoding the JSON.
   - For backward compatibility, `decompressData` should detect invalid base64 or missing gzip headers and gracefully fallback to returning the original data, ensuring existing checkpoints don't break.

## Implementation Prompt
- Modify `srcs/server/checkpointer/checkpointer.go`.
- Ensure tests in `srcs/server/checkpointer/checkpointer_test.go` still pass.

## Priority
P2

## Estimated Scope
Small
