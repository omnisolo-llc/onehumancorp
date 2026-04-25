# Kairos: Master Design Doc

## Overview
This document outlines the architectural design for the Kairos system, specifically focusing on the Shared Task List and Realtime Teammate Mesh components, as well as the initial framework for AutoDream data pipelines.

## 1. Shared Task List Architecture
The Shared Task List manages the swarm's high-level objectives and granular sub-tasks, ensuring consistency across the Hybrid Architecture (Cloud PostgreSQL and Local SQLite).

### Database Schema
*   **Tables:** `shared_tasks`, `task_dependencies`
*   **Fields:**
    *   `id` (UUID/TEXT)
    *   `tenant_id` (String)
    *   `title` (String)
    *   `description` (String)
    *   `status` (String - PENDING, IN_PROGRESS, COMPLETED, FAILED)
    *   `priority` (String - P0, P1, P2)
    *   `agent_id` (String - assigned agent)
    *   `created_at`, `updated_at` (Timestamps)
*   **Compatibility:**
    *   PostgreSQL utilizes native `UUID` and `TIMESTAMP WITH TIME ZONE`. Row-Level Security (RLS) is enabled using the `tenant_id` column to ensure strict data isolation between tenants.
    *   SQLite adapts with `TEXT` for IDs and `DATETIME` for timestamps.

### API Contracts
*   `POST /api/orchestration/tasks`: Create a new task or sub-task.
*   `GET /api/orchestration/tasks`: Retrieve tasks, optionally filtered by agent, status, or organization.

## 2. Teammate Mesh Architecture
The Teammate Mesh provides a low-latency, real-time communication layer for agent coordination and event broadcasting.

### Transport Layer
*   **Cloud-Native Mode:** Utilizes Redis Pub/Sub (`redis_mesh.go`).
*   **Standalone Desktop Mode:** Employs a local memory-based pub/sub bus (`local_mesh.go`).

### Channels
*   `mesh:tasks`: Used for broadcasting task lifecycle events (creation, assignment, completion).
*   `mesh:coordination`: Used for synchronous worker state transitions and inter-agent capability advertisements.
*   `mesh:presence`: Used for heartbeat and status updates (IDLE, WORKING).

### Core Interfaces (Go)
```go
type TeammateMesh interface {
    Publish(ctx context.Context, topic string, payload []byte) error
    Subscribe(ctx context.Context, topic string, handler func(msg []byte)) (Subscription, error)
    AcquireLock(ctx context.Context, key string, ttl time.Duration) (bool, error)
    ReleaseLock(ctx context.Context, key string) error
    RegisterPresence(ctx context.Context, agentID string, status string) error
    GetActiveAgents(ctx context.Context) ([]AgentPresence, error)
}
```

## 3. AutoDream Data Pipeline Architecture
The AutoDream pipeline handles the consolidation of short-term episodic memory into long-term embedded knowledge.

### Processing Flow
1.  **Short-term Memory:** Agents generate episodic summaries during task execution.
2.  **Episodic Summaries:** These summaries are buffered locally or in the cloud.
3.  **Embedding (LLM):** A background worker extracts insights and generates vector embeddings.
4.  **Vector DB:** The embeddings are stored in `consolidated_memory` using `pgvector` (PostgreSQL) or a local blob/approximate index (SQLite).
