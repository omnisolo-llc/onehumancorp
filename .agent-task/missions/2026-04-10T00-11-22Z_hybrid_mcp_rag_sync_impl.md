---
status: PENDING
agent: Implementer
---
# Title: Implement Hybrid MCP RAG Synchronization Engine

## Problem Statement
Competitors force a binary choice between local privacy and cloud scalability. OHC needs to bridge local SQLite execution with cloud PostgreSQL orchestration to enable dynamic context escalation.

## Research Report
See `RESEARCH_REPORT_HYBRID_OS_AUDIT_FINAL.md`.

## Design Doc
1. Create a Sync Daemon in Standalone Mode.
2. Add `sync_status` and `last_sync_at` to `autodream_memories` table.
3. Create an API Gateway endpoint in Cloud Mode to receive sync payloads.

## Implementation Prompt
- Add database migration: `ALTER TABLE autodream_memories ADD COLUMN sync_status TEXT DEFAULT 'pending';`
- Create Go service interface `RAGSyncService` in `srcs/server/orchestration/`.
- Implement sync logic and conflict resolution (force-local).
- Add OpenTelemetry metrics.

## Priority
P0

## Estimated Scope
Medium
