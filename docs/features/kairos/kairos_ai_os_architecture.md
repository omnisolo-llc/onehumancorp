# OHC KAIROS: Hybrid AI OS Architecture

## Premium Visual Design Mandate
All OHC interfaces derived from this architecture MUST adhere to:
*   \`backdrop-filter: blur(20px) saturate(200%)\`
*   \`background: rgba(255, 255, 255, 0.03)\`
*   \`font-family: 'Outfit', 'Inter', sans-serif\`

---

## Phase 1: UltraPlan/Decomposition (Shared Task List)

The Shared Task List is the backbone of the OHC Swarm. It tracks complex feature decomposition into actionable, sequenced \`shared_tasks\`.

### Sequence Diagram
```mermaid
sequenceDiagram
    participant KAIROS
    participant TaskDB as PostgreSQL (TaskDB)
    participant Implementer

    KAIROS->>TaskDB: INSERT INTO shared_tasks (status='PENDING', priority='P0')
    KAIROS->>TaskDB: INSERT INTO task_dependencies (task_id, depends_on)
    Note right of KAIROS: Task is now pending and waiting for its DAG dependencies.
    Implementer->>TaskDB: SELECT id FROM shared_tasks WHERE status='PENDING' FOR UPDATE SKIP LOCKED
    TaskDB-->>Implementer: Return task row
    Implementer->>TaskDB: UPDATE shared_tasks SET status='IN_PROGRESS' WHERE id=?
    Implementer->>KAIROS: Publish TASK_CLAIMED event via Mesh
```

### Database Schema (PostgreSQL)
```sql
CREATE TABLE shared_tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    description TEXT,
    status VARCHAR NOT NULL DEFAULT 'PENDING',
    agent_id VARCHAR,
    priority VARCHAR NOT NULL DEFAULT 'P2',
    payload JSONB,
    parent_plan_id TEXT,
    locked_until TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE task_dependencies (
    task_id UUID NOT NULL REFERENCES shared_tasks(id) ON DELETE CASCADE,
    depends_on_task_id UUID NOT NULL REFERENCES shared_tasks(id) ON DELETE CASCADE,
    PRIMARY KEY (task_id, depends_on_task_id)
);
```

---

## Phase 2: Orchestration (Teammate Mesh Architecture)

Realtime communication between agents is critical for the "Zero Friction" swarm experience.

### Realtime API Contracts
- **Transport**: WebSockets / gRPC locally, backed by Redis Pub/Sub for horizontal scaling in Cloud-Native Mode.
- **Event Bus Channels**:
  - \`mesh:tasks\` - Task transitions (CREATE, CLAIM, COMPLETE)
  - \`mesh:presence\` - Agent health/heartbeats.
- **Message Format (JSON)**:
  ```json
  {
    "event_type": "TASK_CLAIMED",
    "agent_id": "Implementer-1",
    "payload": {
      "task_id": "123e4567-e89b-12d3-a456-426614174000",
      "timestamp": "2026-04-05T22:45:00Z"
    }
  }
  ```

---

## Phase 3: autoDream (Memory Consolidation Pipeline)

The long-term memory system. Agents document their findings locally, and the autoDream background pipeline asynchronously vectorizes these findings into a durable pgvector store.

### Data Pipeline Architecture
1. **Source**: Local \`.agent-task/memory/\` YAML files.
2. **Ingestion Agent**: Reads files, generates chunked text.
3. **Embedding Generation**: Calls LLM provider (e.g., Anthropic/OpenAI/Minimax) to produce vectors.
4. **Storage (pgvector)**:
```sql
CREATE EXTENSION IF NOT EXISTS vector;
CREATE TABLE autodream_memories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    topic TEXT NOT NULL,
    content TEXT NOT NULL,
    embedding vector(1536),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```
