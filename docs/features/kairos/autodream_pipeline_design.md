# KAIROS Orchestrator: autoDream Data Pipelines

## Overview
Architecture for OHC's long-term memory consolidation system. AutoDream securely parses and embeds memories.

## Research Notes
- **Cloud-Native**: `pgvector` offers highly efficient cosine similarity searches and supports vast scales.
- **Standalone**: SQLite doesn't natively support vectors without extensions like `sqlite-vss`. Thus, OHC will serialize vector arrays to BLOBs for storage and perform brute-force cosine distance in-memory for the desktop app. This ensures graceful degradation.

## Cloud-Native Schema (PostgreSQL + pgvector)
```sql
CREATE TABLE IF NOT EXISTS autodream_memories (
    id VARCHAR PRIMARY KEY,
    organization_id VARCHAR NOT NULL,
    content TEXT NOT NULL,
    embedding vector(1536),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

## Standalone Schema (SQLite)
```sql
CREATE TABLE IF NOT EXISTS autodream_memories (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    content TEXT NOT NULL,
    embedding BLOB,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```
