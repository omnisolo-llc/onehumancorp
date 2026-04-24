package mcp_config_sync

import (
    "context"
    "database/sql"
    "encoding/json"
    "fmt"
    "time"

    "go.opentelemetry.io/otel"
    "go.opentelemetry.io/otel/attribute"
)

var tracer = otel.Tracer("mcp_config_sync")

type ConfigSyncPayload struct {
    TenantID string            `json:"tenant_id"`
    AgentID  string            `json:"agent_id"`
    Key      string            `json:"key"`
    Value    string            `json:"value"`
    Metadata map[string]string `json:"metadata"`
}

type ConfigSyncTool struct {
    db *sql.DB
}

func NewConfigSyncTool(db *sql.DB) *ConfigSyncTool {
    return &ConfigSyncTool{db: db}
}

func (t *ConfigSyncTool) GetConfig(ctx context.Context, tenantID, agentID, key string) (string, error) {
    ctx, span := tracer.Start(ctx, "GetConfig")
    defer span.End()

    span.SetAttributes(
        attribute.String("tenant_id", tenantID),
        attribute.String("agent_id", agentID),
        attribute.String("config_key", key),
    )

    var value string
    err := t.db.QueryRowContext(ctx,
        "SELECT config_value FROM mcp_config_sync_log WHERE tenant_id = $1 AND agent_id = $2 AND config_key = $3 ORDER BY synced_at DESC LIMIT 1",
        tenantID, agentID, key).Scan(&value)

    if err != nil {
        if err == sql.ErrNoRows {
            return "", fmt.Errorf("config not found for key: %s", key)
        }
        return "", fmt.Errorf("failed to get config: %w", err)
    }

    return value, nil
}

func (t *ConfigSyncTool) SyncConfigToCloud(ctx context.Context, payload ConfigSyncPayload) error {
    ctx, span := tracer.Start(ctx, "SyncConfigToCloud")
    defer span.End()

    span.SetAttributes(
        attribute.String("tenant_id", payload.TenantID),
        attribute.String("agent_id", payload.AgentID),
        attribute.String("config_key", payload.Key),
    )

    metadataJSON, err := json.Marshal(payload.Metadata)
    if err != nil {
        return fmt.Errorf("failed to marshal metadata: %w", err)
    }

    var sqliteVersion string
    err = t.db.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
    isSqlite := err == nil

    var query string
    if isSqlite {
        query = `
            INSERT INTO mcp_config_sync_log (id, tenant_id, agent_id, config_key, config_value, metadata, synced_at)
            VALUES (hex(randomblob(16)), $1, $2, $3, $4, $5, $6)
        `
        _, err = t.db.ExecContext(ctx, query, payload.TenantID, payload.AgentID, payload.Key, payload.Value, string(metadataJSON), time.Now().UTC())
    } else {
        query = `
            INSERT INTO mcp_config_sync_log (tenant_id, agent_id, config_key, config_value, metadata)
            VALUES ($1, $2, $3, $4, $5)
        `
        _, err = t.db.ExecContext(ctx, query, payload.TenantID, payload.AgentID, payload.Key, payload.Value, metadataJSON)
    }

    if err != nil {
        return fmt.Errorf("failed to sync config to cloud: %w", err)
    }

    return nil
}
