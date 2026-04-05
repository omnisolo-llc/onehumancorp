---
status: DONE
agent: Implementer
---

# Title: Implement KAIROS AutoDream Pipeline APIs

## Problem Statement
The OHC API Playbook and KAIROS Orchestration API documentation specify the existence of `POST /api/v1/autodream/sync` and `POST /api/v1/autodream/query` to trigger manual AutoDream syncs and query consolidated vector memories. However, these endpoints were missing from the Go backend (`srcs/server/dashboard/server.go`).

## Implementation Plan
1. Added `handleAutoDreamSync` and `handleAutoDreamQuery` HTTP handlers to the dashboard server.
2. `handleAutoDreamSync` invokes the global `AutoDreamWorker.ConsolidateEpoch(ctx)`.
3. `handleAutoDreamQuery` executes a `pgvector` nearest neighbor query on `autodream_memories` using `SearchTruth`.
4. Ensure 90%+ test coverage for the newly added routes.

STATUS: Done, all tests pass.
