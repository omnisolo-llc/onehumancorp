# Teammate Mesh & AutoDream Architecture

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

## Overview
The OHC Hybrid Architecture requires robust backend systems for agent coordination. The **Teammate Mesh** provides real-time messaging, and **AutoDream** powers long-term memory consolidation via vector databases.

## 1. Shared Task List
A distributed state machine for decomposing and tracking complex features.
- **Cloud-Native:** PostgreSQL table (`shared_tasks`)
- **Standalone:** SQLite table (`shared_tasks`)

## 2. Teammate Mesh
A highly available real-time communication layer.
- **Protocol:** Redis Pub/Sub (`rueidis`) in Cloud Mode; In-memory channels in Standalone Mode.
- **Channels:** `ohc:mesh:tasks`, `ohc:mesh:agent:{id}`

```mermaid
sequenceDiagram
    participant AgentA
    participant Mesh (Redis)
    participant AgentB

    AgentA->>Mesh: Publish(ohc:mesh:tasks, "Task Created")
    Mesh-->>AgentB: Receive("Task Created")
    AgentB->>Mesh: Publish(ohc:mesh:agent:A, "Acknowledged")
```

## 3. AutoDream Data Pipelines
Background worker that vectorizes agent memories.
- **Trigger:** Completing a task or receiving a periodic tick.
- **Process:** Reads `.agent-task/memory/*.yml` -> `MinimaxClient.GenerateEmbedding` -> Inserts into `agent_memories` table.
- **Vector DB:** pgvector in PostgreSQL, direct blob storage in SQLite.

## Visual Excellence
This architecture adheres to the OHC Premium Feel. Agents orchestrate autonomously while data is persisted gracefully.

</div>
