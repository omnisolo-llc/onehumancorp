---
status: PENDING
agent: "KAIROS Orchestrator"
priority: P0
estimated_scope: Large
title: "KAIROS Orchestration: Shared Task List, Teammate Mesh, and AutoDream Architectural Consolidation"
---

# Problem Statement
The OHC swarm currently lacks a centralized, distributed system for agents to securely coordinate, decompose, and track task execution across the hybrid architecture. Without a "Shared Task List" leveraging our Postgres backend and Redis Pub/Sub (Teammate Mesh), agents cannot effectively orchestrate complex, multi-step workflows.

# Research Report
Based on the `CLAUDE_OHC.md` and the hybrid architecture model, we need a Shared Task List (PostgreSQL 'FOR UPDATE SKIP LOCKED' or SQLite transaction locks), a Teammate Mesh (Redis Pub/Sub), and an AutoDream long-term memory system (pgvector embeddings).
All architectural concepts mentioned in the KAIROS Triad (Shared Tasks via Postgres/SQLite locks, Teammate Mesh via Centrifuge/Redis/Memory, AutoDream pgvector memories) are required.

# Design Doc
See the comprehensive master design document at `docs/features/kairos_orchestration.md` for full technical details, schemas, and API contracts.

# Implementation Prompt
You are an Implementer agent. Your mission is to implement the KAIROS Triad (Shared Task List, Teammate Mesh, AutoDream) in `srcs/server/orchestration`. Use 'FOR UPDATE SKIP LOCKED' for Postgres claiming. Run `bazelisk test //...` to verify. Ensure the implementation degrades gracefully to SQLite in Standalone Mode.
