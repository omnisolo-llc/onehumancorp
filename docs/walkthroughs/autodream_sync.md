<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif;">

# AutoDream Sync Daemon Walkthrough

Welcome to the AutoDream Sync Daemon guide. This walkthrough explains how the OHC Hybrid Architecture synchronizes local Standalone intelligence (SQLite) up to the multitenant Cloud (PostgreSQL).

## 1. The Sync Lifecycle

The Hybrid AutoDream Synchronization enables "Infinite Scaling" while retaining local privacy. Local intelligence vectors are periodically batch-synced.

```mermaid
sequenceDiagram
    participant Worker as Standalone AutoDreamWorker
    participant SQLite as Local SQLite DB
    participant Sync as Sync Daemon
    participant API as Cloud API Gateway
    participant Postgres as Cloud PostgreSQL

    Worker->>SQLite: 1. Generate & Insert Vector (sync_status='pending')
    Sync->>SQLite: 2. Query Pending Vectors
    SQLite-->>Sync: 3. Return Batched Vectors
    Sync->>API: 4. Push over mTLS (SPIFFE Identity)
    API->>Postgres: 5. Upsert to Global autodream_memories
    API-->>Sync: 6. Acknowledge Success
    Sync->>SQLite: 7. Update sync_status='synced'
```

## 2. API Endpoints for Sync

The sync daemon relies on internal APIs to safely transmit vectorized data.

- **Endpoint:** `POST /api/v1/autodream/sync`
- **Purpose:** Transmit batch updates from local nodes to the cloud hub.

For full API specifications, please refer to the [API Playbook](../api/playbook.md).

</div>
