---
agent: kairos
title: "KAIROS: Teammate Mesh APIs Architecture"
problem: "The Swarm needs Realtime Teammate Mesh APIs for agent coordination across Redis Pub/Sub in Cloud and memory in Standalone."
priority: P1
scope: Medium
---

# Title
KAIROS: Teammate Mesh APIs Architecture

# Problem Statement
Agents currently operate in isolation. We need a highly available realtime communication layer (Teammate Mesh) for agents to communicate, coordinate, and acquire distributed locks across a production environment.

# Research Report
Based on OHC Hybrid Architecture:
- Realtime Teammate Mesh APIs must allow agents to coordinate via Redis Pub/Sub in Cloud mode or memory channels in Standalone mode.
- Git-Lock Coordination requires distributed Redis locks.

# Design Doc
- **Teammate Mesh API**:
    - An interface defining `Publish`, `Subscribe`, and `AcquireLock`.
    - Cloud Implementation: Uses Redis Pub/Sub and Redis distributed locks.
    - Standalone Implementation: Uses in-memory channels and local mutexes.
- The system must track active coordinate sessions.

# Implementation Prompt
- Implement the interface and underlying database schemas if needed to support the Teammate Mesh (`srcs/server/db/migrations/...`).
- Ensure the designs gracefully fallback when Redis is absent.
- Do NOT implement the actual Go backend logic, only output the SQL schema designs as instructed in the orchestrator role.

# Priority
P1

# Estimated Scope
Medium
