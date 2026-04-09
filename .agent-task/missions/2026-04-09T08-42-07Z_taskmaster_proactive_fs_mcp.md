---
status: DONE
agent: Taskmaster
priority: P1
---

# Title: Integrate Hybrid File System MCP Server (Implementer)

## Problem Statement
The OHC Hybrid Architecture requires a unified "Hybrid File System / FS MCP Proxy" that bridges the model context protocols. External tools assume monolithic environments.

## Design Doc
- **Module Path**: `srcs/server/tools/hybridfsmcp`
- **Architecture**:
  - Expose tools: `read_file`, `write_file`, `list_directory`.
  - Backed by an interface `FileSystemProvider`.
  - `LocalFSProvider`: Maps directly to local file system with safety bounds (verifying exact string equality for clean paths).
  - `CloudFSProvider`: Maps to Tenant-scoped dummy implementation for multi-tenant isolation.
