# Hybrid MCP Config Sync

## Overview
While OHC supports basic database and file state synchronization via emerging hybrid tools, Enterprise deployments often utilize disparate configuration systems (e.g. centralized Consul vs. local .env / JSON files) in their Cloud versus Standalone environments. A critical gap exists: Agents operating locally in a standalone mode must be able to securely read local configs and selectively sync them back to the multi-tenant Enterprise Vault in the Cloud via an MCP interface, ensuring configuration continuity when moving from local development to cloud production.

## Architecture
The Hybrid Config Sync MCP Tool operates as an interface for configuration propagation:
1. **MCP Exposer:** Exposes `get_config` and `sync_config_to_cloud` operations via the MCP bundle.
2. **Local Adapter:** Reads from standard local files (e.g., `.ohc/config.yaml`) in Standalone mode.
3. **Cloud Adapter:** Interacts with the shared tenant Config DB or Vault in Cloud mode.
4. **Security/Resolution:** Configurations synced to the cloud MUST be verified with SPIFFE/SPIRE identity to prevent privilege escalation or cross-tenant contamination.

### API Contracts
```rust
// Proposed Rust payload shape for a future config-sync MCP integration.
#[derive(serde::Serialize, serde::Deserialize)]
struct ConfigSyncPayload {
    tenant_id: String,
    agent_id: String,
    key: String,
    value: String,
    metadata: std::collections::HashMap<String, String>,
}
```

### DB Schema Changes
Added a new `mcp_config_sync_log` table in PostgreSQL and SQLite to audit configuration changes pushed from standalone agents.
