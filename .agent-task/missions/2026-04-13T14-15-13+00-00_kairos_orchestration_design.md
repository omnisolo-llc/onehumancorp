---
status: DONE
agent: Researcher
---

# 🚀 KAIROS Orchestration: Shared Task List, Teammate Mesh, and autoDream

## Problem Statement
OHC requires a robust, distributed architecture to orchestrate a vast swarm of AI agents. The current system lacks a centralized task definition schema, realtime coordination mechanisms (Teammate Mesh), and a durable long-term memory consolidation pipeline (autoDream).

## Research Report
Based on the OHC Hybrid Architecture (Cloud-Native Mode, Standalone Desktop Mode, Thin Client Mode) and Universal Core Design Protocols (Claude-Class):

1. **Task Orchestration**: The `shared_tasks` schema must accommodate the full lifecycle of complex autonomous tasks in the database (e.g., PostgreSQL for cloud, SQLite for desktop).
2. **Coordination**: The Teammate Mesh requires distributed Pub/Sub (Redis) for real-time task allocation and coordination messages.
3. **Memory Consolidation**: The autoDream vector pipeline must consolidate findings from agents into a durable vector DB (e.g., pgvector/Pinecone) for long-term intelligence sharing (OHC-SIP).

## Design Doc

### 1. Database Schema: Shared Task List
A centralized relational schema to hold high-level tasks decomposed by KAIROS.

**Tables**:
- `tasks`: Core task definition (ID, title, description, status, priority, created_at, updated_at).
- `task_dependencies`: Relational mapping for task prerequisite graphs.
- `task_assignments`: Tracking agent assignments to tasks.

### 2. Teammate Mesh Architecture
A distributed Pub/Sub mechanism for agent coordination.

**Channels**:
- `mesh:tasks`: For broadcasting new tasks and status updates.
- `mesh:coordination`: For agent-to-agent negotiations and locking requests.

**Flow**:
1. KAIROS decomposing a feature request publishes to `mesh:tasks`.
2. Implementer agents subscribe, lock the task via `distributed Redis locks`, and update status.

### 3. autoDream Vector Pipeline
A background worker pipeline to extract memory and consolidate it into long-term embeddings.

**Process**:
1. Agents write raw findings to `.agent-task/memory/{timestamp}.yml`.
2. `autoDream` worker runs periodically (e.g., cron or background queue).
3. Parses YAML, generates text embeddings via LLM API.
4. Upserts vectors into `consolidated_memory` (pgvector/Pinecone).

### 4. Visual Interfaces (UI)
The KAIROS Dashboard must adhere to the Premium Aesthetic:
- `backdrop-filter: blur(20px) saturate(200%)`
- `background: rgba(255, 255, 255, 0.03)`
- `font-family: 'Outfit', 'Inter', sans-serif`

## Implementation Prompt
Dear Implementer Agent, please execute the following:
1. Initialize the `tasks`, `task_dependencies`, and `task_assignments` database schema migrations.
2. Implement the Redis Pub/Sub integration for `mesh:tasks` and `mesh:coordination` channels.
3. Scaffold the background worker queue (e.g., Temporal or BullMQ equivalent) for the `autoDream` pipeline to process `.agent-task/memory/` files into vector embeddings.
4. Ensure 90%+ test coverage. All new code must be fully tested.

## Metadata
- **Priority**: P0 (Critical)
- **Estimated Scope**: Large
