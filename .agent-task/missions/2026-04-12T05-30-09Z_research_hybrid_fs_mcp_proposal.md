---
status: STUCK
agent: Implementer
priority: P1
---

# Title: Integrate Multi-Tenant File System Hybrid MCP

## Problem Statement
OHC-HA currently lacks a unified file system abstraction that functions seamlessly across both Standalone Desktop mode (local disk) and Cloud-Native mode (S3-compatible blob storage). Agents need the ability to read, write, and list files without knowing the underlying storage backend. When transitioning from single-user desktop to multi-tenant cloud, file paths must be strictly isolated to prevent cross-tenant data leakage.

## Research Report
- **Market Context**: Agents often rely on `fs` tools to process local data. In multi-tenant cloud environments, direct local file system access is a security risk and lacks persistence across pods.
- **OHC Requirement**: A "Hybrid FS Store MCP".
- **Tooling Discovery**: An adapter exposing `fs_read`, `fs_write`, and `fs_list`.
  - **Standalone Mode (`OHC_STANDALONE=true`)**: Backed by a constrained local directory (`~/.ohc-local-data/fs`).
  - **Cloud Mode**: Backed by S3/MinIO.
- **Security & Multi-Tenancy**: In Cloud mode, all paths must be prefixed with `tenant/{organization_id}/` (derived from `auth.Claims`) to guarantee isolation. Path traversal attacks (`../`) must be strictly blocked in both modes.

## Design Doc
- **Module Path**: `srcs/server/tools/fsmcp`
- **Architecture**:
  - Exposes tools: `fs_read`, `fs_write`, `fs_list`.
  - Fallback logic: Detect `OHC_STANDALONE` or if S3 config is present.
  - Multi-tenancy: S3 keys MUST be `tenant/{org_id}/fs/{path}`.
  - Security: Implement strict `filepath.Clean` and `strings.HasPrefix` checks to prevent path traversal outside the designated root.

## Implementation Prompt
Hello Implementer agent!
1. Create a new directory `srcs/server/tools/fsmcp`.
2. Implement the `fs_read`, `fs_write`, and `fs_list` tools under the MCP Tool interface.
3. Use `auth.Claims` to enforce multi-tenancy.
4. If in Standalone mode, perform safe local file I/O within a designated base directory. If in Cloud mode, integrate with an S3 client (e.g., MinIO go client).
5. Ensure strict path traversal prevention.
6. Achieve >90% test coverage for the `fsmcp` package, including tests for path traversal attempts.

## Priority
P1

## Estimated Scope
Medium
