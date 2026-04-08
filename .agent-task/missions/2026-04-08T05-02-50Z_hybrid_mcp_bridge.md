---
status: PENDING
agent: Researcher
---

# Title: Implement Hybrid Universal Local-to-Cloud MCP Proxy

## Problem Statement
Competitors like Claude Code, OpenClaw, and Replit Agent suffer from a binary architecture: they are either strictly local (lacking distributed memory and scaling) or strictly cloud-native (failing to support offline, standalone desktop execution). OHC has a dual-database architecture (SQLite locally, Postgres in the cloud), but currently lacks a seamless, high-speed proxy to synchronize local agent state (checkpoints, episodic memory, and embeddings) up to the multitenant cloud cluster for swarm delegation.

## Research Report
The market audit (see `docs/research/2026-04-08-hybrid-os-benchmark.md`) proves that the single biggest feature gap in the Agentic OS ecosystem is hybrid mobility. A developer should be able to work offline, build context locally in SQLite, and upon reconnection, delegate the complex multi-agent execution to the KAIROS cloud swarm.

**Competitive Data:**
- **Claude Code:** No distributed memory offloading.
- **OpenClaw:** Missing seamless SQLite-to-Postgres hybridity.
- **Replit Agent:** Zero offline capabilities.

By implementing an MCP-based state proxy, OHC will instantly disrupt these platforms by offering true "write local, scale global" agentic execution.

## Design Doc
**Architecture:**
A new Go worker service (`srcs/server/orchestration/hybrid_mcp_proxy.go`) that implements the `UniversalAdapter` interface. It will poll the local SQLite `SIPDB` for un-synced checkpoints.
- Uses `go-spiffe/v2` for Zero Trust mTLS when establishing the cloud connection.
- Batches memory payloads (JSON) and executes a Postgres `UPSERT` via the OHC Cloud API.
- Listens to OHC-SIP Redis Pub/Sub channels (if available) or falls back to standard HTTP polling for sync confirmation.

**API Contracts:**
```go
package orchestration

import "context"

type HybridSyncProxy interface {
    SyncCheckpoints(ctx context.Context, localDb *SIPDB, cloudEndpoint string) error
    PushVectorState(ctx context.Context, payload []byte) error
}
```

**UI Wireframes:**
The Flutter standalone app will feature a glassmorphism "Syncing to Cloud" indicator in the navigation bar using the CSS tokens defined in the benchmark report.

## Implementation Prompt
**Role:** Implementer Agent
**Task:** Build the `Hybrid Universal Local-to-Cloud MCP Proxy` in Go.

**Instructions:**
1.  **File to create:** `srcs/server/orchestration/hybrid_mcp_proxy.go`.
2.  **Logic:** Implement the `HybridSyncProxy` interface. Write a function `StartProxyDaemon(ctx context.Context, db *SIPDB, endpoint string)` that uses a `time.Ticker` (e.g., every 5 seconds) to check for unsynced agent checkpoints in the local SQLite DB.
3.  **Security:** Ensure you use SPIFFE/SPIRE for the mTLS connection to the `cloudEndpoint`. Parse SPIFFE IDs to strictly validate the path segments (e.g., must contain `onehumancorp.io` or `ohc.local`).
4.  **Error Handling:** Never `panic()`. All constructors and sync functions must return `error`. Use the `log/slog` package and `telemetry.LogAgentExecution` to trace the sync cycles.
5.  **Concurrency:** Use a `sync.Pool` for the payload JSON buffers to minimize GC overhead during high-frequency syncs.
6.  **Tests:** Create `srcs/server/orchestration/hybrid_mcp_proxy_test.go`. Write table-driven unit tests covering the Happy Path (successful sync) and Edge Cases (network timeout, invalid SPIFFE ID). Set `integrations.AllowLocalIPsForTesting = true` for local mock server tests. Use Bazel (`bazelisk test //...`) to verify coverage is >95%.

## Priority
`P0` (Critical - Disrupts Market)

## Estimated Scope
Medium
