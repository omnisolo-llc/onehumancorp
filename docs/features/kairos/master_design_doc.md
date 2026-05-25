# Architect Hybrid Task List & Mesh Orchestration

## Problem Statement
The OHC swarm requires a robust shared task list, realtime Teammate Mesh APIs, and an AutoDream data pipeline architecture. These are essential for true autonomous orchestration in both cloud-native and local-first deployments.

## Research Report
1. **Shared Task List**: A robust backend database design (e.g., PostgreSQL for cloud, SQLite for local) holds high-level tasks that can be decomposed and assigned to agents.
2. **Teammate Mesh**: A highly available realtime communication layer. In cloud mode, this implies Redis Pub/Sub APIs or NATS (`mesh:tasks`, `mesh:coordination`), and in local mode, SQLite/In-process IPC.
3. **AutoDream Data Pipelines**: An architecture for long-term memory consolidation, moving episodic context into embedded vector truth via pgvector.

## Design Doc
1. **Shared Task List Architecture**:
   - DB Schema: Tasks, Sub-tasks, Agent Assignments, Statuses (implemented as `shared_tasks_v4`).
   - Implementation: `SharedTaskOrchestrator` in Rust supporting PostgreSQL and SQLite for hybrid OS needs.
2. **Teammate Mesh Architecture**:
   - Channels: `mesh:tasks`, `mesh:coordination`.
   - APIs: Publish, Subscribe, Acknowledge (implemented via `TeammateMesh` Rust trait).
   - Transports: Redis, NATS, PostgreSQL LISTEN/NOTIFY, SQLite IPC, In-Process.
3. **AutoDream Data Pipeline Architecture**:
   - Flow: Short-term memory -> Agent Episodic Summaries -> Embedding (LLM) -> Vector DB (pgvector/Pinecone/local blob).

## Decisions
- Used Rust instead of Go for the primary backend language to align with OHC Mono standards.
- Used `sqlx` to provide hybrid PostgreSQL and SQLite compatible queries.
- Created `CentrifugeNode` and `TeammateMesh` traits to abstract realtime Pub/Sub.
