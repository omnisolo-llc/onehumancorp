---
status: PENDING
agent: Implementer
priority: P1
estimated_scope: Large
---

# Title: Implement Elastic Swarm Bursting Protocol

## Problem Statement
Current OHC Standalone Mode executes tasks strictly against the local SQLite SIPDB. When local compute (e.g., Apple Silicon M2) is saturated by massively parallel sub-agent reasoning, task queues stall. The system needs a "bursting" capability to offload heavy `agent_missions` to the multi-tenant OHC Cloud-Native API without losing task context or violating zero-trust boundaries.

## Research Report
- **Market Gap**: Claude Code and Replit Agent are bound to their respective host environments (CLI vs Cloud). OpenClaw scales in K8s but lacks a lightweight client.
- **OHC's Unfair Advantage**: The Hybrid Architecture allows seamless context migration. OHC can perform Local-Private RAG, and when generic compute scales up, offload the workload to the Cloud.
- **Technical Requirements**: A new `status = 'BURSTING'` state in the `agent_missions` table. A daemon that identifies bursting candidates, authenticates via SPIFFE/SPIRE, and proxies the payload to a designated remote OHC K8s endpoint.

## Design Doc
- **Target**: `srcs/server/orchestration/sip.go` and `srcs/server/orchestration/sync_daemon.go`
- **Architecture**:
    1. Introduce `status = 'BURSTING'` constraint to the `agent_missions` schema.
    2. Enhance `SyncMissions` to detect tasks marked for bursting (e.g., via a specialized payload flag or specific role).
    3. The payload is scrubbed using `telemetry.RedactInterfacePII` before bursting to the remote endpoint.
    4. Provide fallback logic if the remote API is unavailable.

## Implementation Prompt
Hello Implementer!
1. Please add `status = 'BURSTING'` as a valid enum/string state for `agent_missions` tracking in `srcs/server/orchestration/sip.go`.
2. Locate the synchronization logic in `srcs/server/orchestration/sip.go` (`SyncMissions`) or `srcs/server/orchestration/sync_daemon.go`.
3. Add logic so that when local compute load is detected as high (mock this with a simple env flag `OHC_ENABLE_BURSTING=true`), PENDING missions can be upgraded to BURSTING and specifically pushed to the `remoteEndpoint`.
4. Ensure the `telemetry.RedactInterfacePII` runs BEFORE bursting the payload over the wire to ensure data privacy.
5. Achieve >90% test coverage in `sip_test.go` for the bursting logic.

## Priority
P1

## Estimated Scope
Large
