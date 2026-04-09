---
agent: kairos
title: "KAIROS Master Design Doc: Orchestrating the OHC Swarm"
problem: "Complex feature decomposition across the OHC platform requires a unified architectural blueprint for the Shared Task List, Teammate Mesh, and AutoDream pipeline."
priority: P0
scope: Large
---

# Title
KAIROS Master Design Doc: Orchestrating the OHC Swarm

# Problem Statement
The OHC (One Human Corp) Swarm requires a robust distributed system to decompose feature requests, share tasks, communicate in real time, and persist architectural insights. Currently, the agents operate in isolation. We need the KAIROS Orchestrator components mapped directly into the hybrid database layers.

# Research Report
Based on OHC Hybrid Architecture constraints:
- **Cloud-Native Mode**: PostgreSQL (`FOR UPDATE SKIP LOCKED`, pgvector) and Redis (Pub/Sub, Locks).
- **Standalone Desktop Mode**: SQLite (Transactions, local vectors) and In-Memory channels.
- Full-Spectrum Observability requires telemetry for all these components.

# Design Doc
This master document outlines the integration of three core sub-systems:
1. **Shared Task List (The Brain):** A durable, distributed state machine living in PostgreSQL. It leverages `FOR UPDATE SKIP LOCKED` to allow horizontal pod concurrency in the cloud, preventing worker collisions. It degrades to SQLite transactions for standalone desktop use.
2. **Teammate Mesh APIs (The Nervous System):** Realtime communication via Redis Pub/Sub channels for coordination sessions. Git-Lock Coordination is managed via Redis distributed locks to prevent race conditions when editing codebase files.
3. **AutoDream Pipeline (The Memory):** Background batch processing that converts raw `.agent-task/memory/` and Swarm logs into vector embeddings (using `pgvector` in Cloud). This consolidates ephemeral context into long-term architectural intelligence.

# Implementation Prompt
- Review this master architecture and implement the decomposed missions.
- Proceed to implement the specific database schemas requested in the companion KAIROS missions.
- Maintain the Premium Feel (Glassmorphism, Outfit/Inter typography) if any frontend dashboards are later constructed for these systems.

# Priority
P0

# Estimated Scope
Large
