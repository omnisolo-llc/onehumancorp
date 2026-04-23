# Hybrid Config Sync MCP Tool

## Overview
This feature adds an MCP (Model Context Protocol) tool to bridge configurations between Standalone local agents and the OHC Enterprise Cloud Vault. Agents operating locally can read configurations locally and push them to the Cloud securely.

## Components
1. **Tool Definition:** An MCP interface containing `get_config` and `sync_config_to_cloud`.
2. **Local/Cloud Adaptors:**
   - Standalone: Resolves config values.
   - Cloud: Writes/audits to the database.

## API Design
```go
package mcp_config_sync

type ConfigSyncPayload struct {
    TenantID string            `json:"tenant_id"`
    AgentID  string            `json:"agent_id"`
    Key      string            `json:"key"`
    Value    string            `json:"value"`
    Metadata map[string]string `json:"metadata"`
}
```

## Security
- All changes synced to the cloud MUST be verified.
- OpenTelemetry instrumentation MUST be present.

## Schema
`mcp_config_sync_log` table tracking:
- `id` (UUID)
- `tenant_id` (String)
- `agent_id` (String)
- `config_key` (String)
- `config_value` (String)
- `metadata` (JSONB / TEXT)
- `synced_at` (Timestamp)
