# KAIROS Orchestrator: Shared Task List Database Schema

## Overview
This document outlines the database schema required for the OHC Swarm to manage deep-deliberation cycles and distributed tasks.

## Sequence Diagram: Deep Deliberation Cycle
```mermaid
sequenceDiagram
    participant UI as KAIROS Dashboard
    participant Orchestrator as KAIROS Orchestrator
    participant DB as Hybrid DB (Pg/SQLite)
    participant Implementer as Agent Team

    UI->>Orchestrator: Create Mission
    Orchestrator->>DB: INSERT INTO shared_tasks
    DB-->>Orchestrator: ACK
    Orchestrator->>Implementer: Broadcast mesh:tasks
    Implementer->>Orchestrator: Claim Task (Redis Lock)
    Orchestrator->>DB: UPDATE shared_tasks SET status='IN_PROGRESS'
```

## Cloud-Native Mode (PostgreSQL)
```sql
CREATE TABLE IF NOT EXISTS shared_tasks (
    id VARCHAR PRIMARY KEY,
    organization_id VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    status VARCHAR NOT NULL DEFAULT 'PENDING',
    dependencies JSONB NOT NULL DEFAULT '[]',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

## Standalone Mode (SQLite Graceful Degradation)
```sql
CREATE TABLE IF NOT EXISTS shared_tasks (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    title TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'PENDING',
    dependencies TEXT NOT NULL DEFAULT '[]',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```
