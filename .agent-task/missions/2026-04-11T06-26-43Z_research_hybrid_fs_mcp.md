---
status: FAILED
agent: Implementer
---

# Title: Implement Hybrid File System (FS) MCP Tool
## Problem Statement
While OHC-HA (One Human Corp Hybrid Architecture) bridges the gap between Cloud (K8s/Postgres) and Local (Standalone Desktop/SQLite), our agents lack a unified mechanism to manage and synchronize actual file artifacts across these boundaries. Currently, when an agent in Standalone Mode generates a large asset (like a dataset, log, or build artifact) locally, there is no standardized, native tool for them to seamlessly synchronize this specific file asset into the Cloud orchestration environment. The current offline-to-cloud sync focuses heavily on *state* (`agent_missions` and `swarm_memory_embeddings`) but neglects raw file payloads, creating a friction point where agents cannot easily pass large raw data between the Desktop and the Cloud.

## Research Report
- Current Market Analysis:
  - Claude Code heavily relies on local OS file system commands (`ls`, `cat`), which completely break in a scaled cloud environment.
  - Replit Agent manages cloud-native files well but has zero awareness of the developer's local desktop environment.
- The Opportunity: OHC can introduce a "Hybrid FS MCP Protocol" (Model Context Protocol). This means providing agents with standardized file system tools (`fs_read`, `fs_write`, `fs_sync`) that transparently detect whether they are running in Standalone (Local) or Cloud-Native mode.
- When in Local Mode, if an agent uses `fs_sync`, the file should be chunked and securely pushed to a centralized Cloud Blob Storage or directly via a new Sync Daemon capability to ensure the multi-tenant K8s pods can immediately access it for further computation.

## Design Doc
1. **New MCP Tools:**
   - `fs_hybrid_read(path string)`: Reads a file. If path is a cloud URI, fetches it; if local, reads local FS.
   - `fs_hybrid_write(path string, data []byte)`: Writes a file.
   - `fs_hybrid_sync(local_path string, cloud_path string)`: Initiates a secure transfer of the file from the SQLite-backed Standalone environment up to the Cloud Postgres/K8s environment.
2. **Architecture Update:**
   - Integrate with the existing `SyncDaemon`. The daemon must be updated to listen for "hybrid_fs" sync requests (perhaps by adding a new status `FILE_SYNC_PENDING` to a new table or extending `agent_missions`).
3. **UI/UX (Visual Excellence Mandate):**
   - The transfer progress MUST be exposed to the user UI using Glassmorphism (20px blur) tooltips and Outfit/Inter typography, ensuring they feel the "Premium" data sovereignty.

## Implementation Prompt
- Implement the Hybrid FS MCP tools in `srcs/server/tools/hybrid_fs.go`.
- Ensure the methods correctly determine the environment (Cloud vs Local) via context.
- Add `fs_hybrid_read`, `fs_hybrid_write`, and `fs_hybrid_sync` to the universal MCP tool registry.
- Update the Standalone Mode `SyncDaemon` to handle file sync queues.
- Ensure all file operations are fully covered by OpenTelemetry traces.
- Provide a robust unit test suite (>= 95% coverage) mocking both local and cloud environments.
- Verify using Playwright that the progress UI is functioning as expected in the dashboard.

## Priority
P1

## Estimated Scope
Medium
