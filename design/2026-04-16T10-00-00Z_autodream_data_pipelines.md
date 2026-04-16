# [research] Architect AutoDream Data Pipelines for OHC VectorDB

## Problem Statement
The One Human Corp (OHC) Swarm requires an architectural blueprint for ingesting and consolidating swarm memory state into the production Vector DB (pgvector). Currently, agents lack a structured mechanism to record their architectural discoveries and state changes, leading to amnesia across sub-agent executions in both Cloud-Native and Standalone modes.

## Research Report
Based on OHC architecture guidelines:
1. **Durable State Storage**: The Vector DB (pgvector) must store "AutoDream" architectural consolidation findings from all agents.
2. **Hybrid Deployment Parity**: The pipeline must support K8s-orchestrated Postgres (Cloud-native) and local SQLite with pgvector extensions (Standalone mode).
3. **Data Ingestion Constraints**: Must adhere to strict tenant isolation in Cloud-Native mode and low resource consumption in Standalone Mode.

## Design Doc
1. **Database Schema**: Define `autodream_memories` table with `id`, `agent_id`, `context_vector` (vector embedding), `metadata` (JSONB for Postgres, TEXT for SQLite), and `created_at`.
2. **Ingestion Pipeline**: Architect a background worker `AutoDreamIngestor` that polls the `swarm_memory_embeddings` queue and batch inserts into the database.
3. **API Contract**: Define `RecordMemory` and `QueryMemories` gRPC/HTTP endpoints for agents to interact with the VectorDB.
4. **Telemetry**: Add OpenTelemetry metrics for ingestion rate, latency, and vector search performance.

## Implementation Prompt
You are an Implementer agent. Your mission is to implement the AutoDream Data Pipelines for the OHC VectorDB.
1. Add the `AutoDreamMemory` struct in the domain layer.
2. Create database migration scripts for the `autodream_memories` table (handle both Postgres pgvector and SQLite vector extensions gracefully).
3. Implement the `AutoDreamIngestor` background worker with proper queue polling and transaction management.
4. Implement the `RecordMemory` and `QueryMemories` API endpoints.
5. Add relevant telemetry metrics to `srcs/server/telemetry/telemetry.go`.
6. Write comprehensive tests and ensure `bazel test //srcs/server/autodream/...` passes with >90% coverage.

## Priority
P1

## Estimated Scope
Large
