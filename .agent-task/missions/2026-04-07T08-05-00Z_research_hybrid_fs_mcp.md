---
status: DONE
agent: Researcher
priority: P1
---

# Title: Integrate Hybrid File System MCP Server

## Problem Statement
The OHC Hybrid Architecture requires seamless operations across Cloud (distributed) and Standalone (local) modes. For integrations specifically related to file operations or unstructured data ingestion, agents need a common interface. An existing task for "Hybrid Blob Storage MCP" (S3 vs local) exists, but there is no specific blueprint for an advanced unified "Hybrid File System / FS MCP Proxy" that bridges the model context protocols. This fragmented access slows down autonomous debugging, testing, and retrieval.

## Research Report
- **Market Context**: External tools and default MCP servers (e.g., `filesystem` MCP) assume a single monolithic environment, usually purely local paths. Replit Agent operates purely in the cloud with cloud filesystems. Claude Code operates purely on local files.
- **OHC Requirement**: We need a "Hybrid File System MCP Proxy" that abstracts file system operations. While Blob storage (S3) handles large unstructured binaries and web artifacts, the application also needs a POSIX-like abstraction for configuration files, intermediate code generation scripts, and hybrid workspaces that synchronize between local Standalone mode and K8s persistent volumes (or Ephemeral scratch space) in Cloud-native mode.
- **Tooling Discovery**: A dedicated MCP adapter wrapping an interface like `fs.FS` or a custom `mcp.FileSystemProvider` into standard MCP tools (`read_file`, `write_file`, `list_directory`). This unifies file operations for agents so that Agent tools execute identical code regardless of the underlying target.
- **Security & Multi-Tenancy**: In Cloud mode, access must be chrooted or scoped via `auth.Claims` to tenant-specific virtual directories to prevent cross-tenant access. In standalone mode, access might be broader but still needs directory bounding for safety.

## Design Doc
- **Module Path**: `srcs/server/tools/hybridfsmcp`
- **Architecture**:
  - Implements the Model Context Protocol (MCP) for `list_tools` and `call_tool`.
  - Expose tools: `read_file`, `write_file`, `list_directory`, `search_files`.
  - Backed by an interface `mcp.FileSystemProvider` with methods `ReadFile`, `WriteFile`, `ListDir`.
  - **Local Implementation (`LocalFSProvider`)**: Maps directly to local file system with safety bounds. Binds to a base directory to prevent path traversal (using `strings.HasPrefix` with filepath separator).
  - **Cloud Implementation (`CloudFSProvider`)**: Uses `auth.Claims` from the context to enforce tenant isolation. Paths are scoped by `tenant_id`. Maps to Tenant-scoped Kubernetes Persistent Volumes or a virtualized S3-backed file system interface.
  - Dynamically uses the system's mode to determine the provider (`OHC_MULTITENANT` vs `OHC_STANDALONE`).

## Implementation Prompt
Hello Implementer agent!
1. Create a new directory `srcs/server/tools/hybridfsmcp` and a `BUILD.bazel` file.
2. Abstract the file writing and reading logic behind an interface `mcp.FileSystemProvider` with methods `ReadFile`, `WriteFile`, `ListDir` in `provider.go`.
3. Implement `LocalFSProvider` (with path bounding to a workspace dir) and `CloudFSProvider` (tenant-scoped via `auth.Claims`).
4. Create an MCP server `HybridFSProxy` that uses this provider to expose standard filesystem tools (`read_file`, `write_file`, `list_directory`, `search_files`).
5. Ensure factory logic correctly instantiates the provider based on the `OHC_MULTITENANT` and `OHC_STANDALONE` modes.
6. Add unit tests for both Local and Cloud providers in `mcp_test.go` ensuring >90% coverage.

## Priority
P1

## Estimated Scope
Medium
