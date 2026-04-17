<div markdown="1" style="font-family: 'Outfit', sans-serif; background: rgba(255, 255, 255, 0.1); backdrop-filter: blur(20px); border-radius: 12px; padding: 24px; color: #333;">

# Design Document: Cross-Mode MCP Vector Embeddings Sync Tool

## 1. Overview
This document outlines the architecture for the `mcp_vector_sync` tool. It aims to synchronize locally generated vector embeddings (SQLite) with the Cloud-Native Postgres (pgvector) instance, ensuring continuous context preservation across OHC Hybrid modes.

## 2. Architecture
The tool acts as a bridge invoked by agents via the Model Context Protocol (MCP).
- **Local State**: SQLite database storing locally generated vector embeddings.
- **Cloud State**: PostgreSQL with `pgvector` for multi-tenant centralized storage.
- **Sync Mechanism**: Timestamp-based watermarking to identify delta changes.

## 3. Data Schema Changes
- **Local SQLite (`vector_embeddings` table)**:
  - Add `last_synced_at` (TIMESTAMP).
- **Cloud PostgreSQL**:
  - Support upsert operations mapping local IDs to Cloud IDs with conflict resolution based on timestamps.

## 4. API Contract
- `POST /api/v1/sync/embeddings`
  - **Payload**: `[{ "id": "uuid", "vector": [0.1, 0.2, ...], "metadata": { ... } }]`
  - **Response**: `{ "synced": count, "failed": count }`

## 5. Security & Isolation
- All sync operations must be authenticated via SPIFFE/SPIRE.
- Multi-tenant boundary checks must strictly enforce that an agent can only sync to its assigned tenant profile.

</div>
