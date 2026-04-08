<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Hybrid Storage Sync Walkthrough

The **Hybrid MCP RAG Protocol** and Hybrid Storage operations seamlessly synchronize context from local standalone executions (using SQLite) to the global cloud orchestration layer (using PostgreSQL).

## The Sync Architecture

The true power of OHC's Multi-Agent architecture is its ability to degrade gracefully. When working in local mode (e.g. while traveling without an internet connection), the Swarm persists its artifacts to a local `SQLite` database.

When connectivity is restored, the **Sync Daemon** seamlessly synchronizes these local records back into the cloud using Last-Write-Wins (LWW) conflict resolution logic.

### Visualization of the Sync Flow

```mermaid
graph TD
    subgraph Standalone Desktop
        A[Agent Task] -->|Writes to| B(Local SQLite DB)
        B -.->|Monitors Changes| C(Sync Daemon)
    end

    subgraph OHC Cloud Service
        D[API Gateway] -->|Validates| E(Postgres DB)
        E -->|Aggregates Truth| F[AutoDream Vectors]
    end

    C -->|gRPC/REST Sync| D

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,C,D,E,F premium;
```

## Step-by-Step Resolution

1. **Standalone Work**: An agent works offline, updating the local state machine and memory artifacts in SQLite. These changes are marked with `sync_status = 'pending'`.
2. **Connectivity Check**: The Sync Daemon verifies the connection to the OHC Cloud Gateway by pinging `GET /api/health`.
3. **Payload Transmission**: The local database diff is serialized into a payload (see the endpoints defined in the [API Playbook](../api_playbook.md)) and transmitted to `POST /api/missions/sync`.
4. **Cloud Aggregation**: The Gateway upserts the data into PostgreSQL. Any conflicts are resolved using timestamps (`last_sync_timestamp`).
5. **Confirmation**: The local SQLite database updates the `sync_status` to `'synced'`.

</div>
