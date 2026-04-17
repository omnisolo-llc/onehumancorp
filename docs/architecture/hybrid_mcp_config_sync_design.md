<div markdown="1" style="font-family: 'Outfit', sans-serif; background: rgba(255, 255, 255, 0.1); backdrop-filter: blur(20px); border-radius: 12px; padding: 24px; color: #333;">

# Design Document: Enterprise Local-to-Cloud Config Sync

## 1. Overview
The `mcp_config_sync` tool enables seamless bidirectional synchronization of agent and user configurations between Standalone Desktop (SQLite/JSON) and Cloud-Native Mode (PostgreSQL).

## 2. Architecture
- **MCP Tool Registration**: Available as a standard tool within the OHC ecosystem.
- **State Hashing**: Utilizes SHA-256 hashes of config states to detect drifts quickly without large payload transfers.

## 3. API Contract
- `GET /api/v1/sync/config/hash` -> returns current cloud hash.
- `PUT /api/v1/sync/config` -> accepts new configuration JSON.

## 4. Security
- SPIFFE/SPIRE authentication.
- Explicit masking or exclusion of local-only secrets (e.g., local proxy passwords).

</div>
