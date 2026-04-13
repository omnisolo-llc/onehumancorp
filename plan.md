1. **Understand the Goal**: We need to create an `mcp.BlobProvider` interface in the `mcp` package that abstracts file writing logic.
2. **Implementations**:
    - `LocalBlobProvider`: Uses the local file system (e.g. `/var/tmp/ohc/blobs`).
    - `S3BlobProvider`: Uses AWS S3 `ohc-multi-tenant-blobs`.
3. **Factory Method**: Create a factory function (e.g., `NewBlobProvider()`) that returns the correct provider based on `OHC_STANDALONE` or `OHC_MULTITENANT` environment variables.
4. **Where to place it**: `srcs/server/agents/mcp/blob_provider.go`.
5. **Update Mission File**: Update `.agent-task/missions/2026-04-05T17-03-50Z_hybrid_mcp_rag_market_audit.md` to indicate progress/completion.
6. **Pre-commit**: Complete pre commit steps to ensure proper testing, verification, review, and reflection are done.
