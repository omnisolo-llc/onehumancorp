---
status: DONE
agent: Jules
priority: P1
---

# Title: Integrate Hybrid Blob Storage MCP Server

## Problem Statement
The OHC Hybrid Architecture (OHC-HA) operates across multi-tenant Cloud and single-user Standalone modes. In Cloud mode, blobs (images, memory artifacts, video files) are stored in S3-compatible object storage. In Standalone mode, they are stored on the local filesystem to minimize dependencies. AI agents currently lack a unified way to read, list, and verify blobs (such as user-uploaded files or agent-generated visual artifacts) regardless of the underlying storage backend. This fragmented access slows down autonomous debugging, testing, and retrieval.

## Research Report
- **Market Context**: External tools and default MCP servers (e.g. `filesystem` MCP) assume a single monolithic environment (local paths only).
- **OHC Requirement**: We need a "Hybrid Blob Storage MCP Server" that dynamically binds to either an S3 client (e.g. `minio-go` or `aws-sdk-go`) or the local filesystem (`os`, `io/fs`) depending on the `OHC_STANDALONE` environment variable or the active `orchestration.Hub`.
- **Tooling Discovery**: A dedicated MCP adapter that wraps OHC's internal blob storage provider interface (similar to how `db.Provider` works) into standard MCP tools (`mcp.ListTools`, `mcp.CallTool`) ensures that agents only need to know a single logical path or bucket concept, while the engine handles translation to Cloud/Local.
- **Security & Multi-Tenancy**: In Cloud mode, access must be strictly scoped to the tenant's bucket namespace. The MCP implementation must use `auth.Claims` to restrict directory listing and object retrieval to the authenticated user's organization scope.

## Design Doc
- **Module Path**: `srcs/server/tools/blobinspector`
- **Architecture**:
  - Implements the Model Context Protocol (MCP) for `list_tools` and `call_tool`.
  - Exposes `list_blobs`, `read_blob_metadata`, and `get_blob_url` (or presigned URL generation) tools.
  - Dynamically uses the system's `StorageProvider` (to be injected via `hub.Storage()`).
  - For Cloud (S3): Prepends `tenant_id` to object keys to enforce multi-tenant isolation.
  - For Standalone (Local FS): Translates logical bucket paths to absolute local file system paths (e.g., `~/.ohc/data/blobs/`).
- **Security**: Strict READ-ONLY mode by default. Write tools (`upload_blob`, `delete_blob`) must require explicit context elevation.

## Implementation Prompt
1. Create a new directory `srcs/server/tools/blobinspector`.
2. Implement the MCP server conforming to the project's internal tool registry interfaces (`srcs/server/tools/tools.go`).
3. Implement `ListTools` returning definitions for `list_blobs`, `read_blob_metadata`, and `get_blob_url`.
4. Implement `CallTool`:
   - Inject `auth.Claims` from the context.
   - Detect Storage mode: `if hub.Storage().IsLocal() { ... } else { ... }`.
   - Ensure Cloud queries enforce tenant scoping by prefixing paths with `claim.OrganizationID`.
   - Return clear errors if blobs are not found or access is denied.
5. Add unit tests for both S3 (using mock S3 client) and Local FS (using a temporary directory) proving cross-mode functionality.
6. Achieve >90% test coverage for the `blobinspector` package.

## Priority
P1

## Estimated Scope
Medium
