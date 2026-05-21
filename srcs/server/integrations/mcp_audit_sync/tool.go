package mcp_audit_sync

import (
    "context"
    "database/sql"
    "fmt"
    "os"

    "time"

    "go.opentelemetry.io/otel"
    "go.opentelemetry.io/otel/attribute"
    "go.opentelemetry.io/otel/trace"
)

type AuditSyncPayload struct {
    TenantID  string `json:"tenant_id"`
    AgentID   string `json:"agent_id"`
    Action    string `json:"action"`
    Resource  string `json:"resource"`
    Status    string `json:"status"`
    Metadata  string `json:"metadata"`
    Timestamp int64  `json:"timestamp"`
}

type AuditLogger struct {
    db     *sql.DB
    tracer trace.Tracer
}

func NewAuditLogger(db *sql.DB) *AuditLogger {
    tracer := otel.Tracer("mcp_audit_sync")
    return &AuditLogger{
        db:     db,
        tracer: tracer,
    }
}

func (l *AuditLogger) SyncAuditLogsToCloud(ctx context.Context, payload AuditSyncPayload, spiffeID string) error {
    var span trace.Span
    if os.Getenv("OHC_TELEMETRY_ENABLED") == "true" {
        ctx, span = l.tracer.Start(ctx, "SyncAuditLogsToCloud")
        defer span.End()

        span.SetAttributes(
            attribute.String("tenant_id", payload.TenantID),
            attribute.String("agent_id", payload.AgentID),
            attribute.String("action", payload.Action),
            attribute.String("resource", payload.Resource),
            attribute.String("status", payload.Status),
        )
    }

    if payload.TenantID == "" || payload.AgentID == "" || payload.Action == "" {
        err := fmt.Errorf("missing required fields in payload")
        if span != nil {
            span.RecordError(err)
        }
        return err
    }

    // SPIFFE/SPIRE authorization check
    expectedSpiffePrefix := fmt.Sprintf("spiffe://onehumancorp.io/tenant/%s/agent/%s", payload.TenantID, payload.AgentID)
    if spiffeID != expectedSpiffePrefix && spiffeID != "spiffe://onehumancorp.io/admin" {
        err := fmt.Errorf("unauthorized: SPIFFE ID %s does not match expected prefix %s", spiffeID, expectedSpiffePrefix)
        if span != nil {
            span.RecordError(err)
        }
        return err
    }

    query := `
        INSERT INTO mcp_audit_sync_log (tenant_id, agent_id, action, resource, status, metadata, timestamp)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
    `

    // Fallback to time.Now() if timestamp is missing
    if payload.Timestamp == 0 {
        payload.Timestamp = time.Now().Unix()
    }

    _, err := l.db.ExecContext(ctx, query,
        payload.TenantID,
        payload.AgentID,
        payload.Action,
        payload.Resource,
        payload.Status,
        payload.Metadata,
        payload.Timestamp,
    )

    if err != nil {
        err = fmt.Errorf("failed to insert audit log: %w", err)
        if span != nil {
            span.RecordError(err)
        }
        return err
    }

    return nil
}


// MCP Tool Interface Implementation

func (l *AuditLogger) Name() string {
    return "sync_audit_logs_to_cloud"
}

func (l *AuditLogger) Description() string {
    return "Synchronizes local agent audit logs to the Enterprise Cloud database."
}

func (l *AuditLogger) Execute(ctx context.Context, params map[string]interface{}, spiffeID string) (interface{}, error) {
    tenantID, _ := params["tenant_id"].(string)
    agentID, _ := params["agent_id"].(string)
    action, _ := params["action"].(string)
    resource, _ := params["resource"].(string)
    status, _ := params["status"].(string)
    metadata, _ := params["metadata"].(string)
    var timestamp int64
    if t, ok := params["timestamp"].(float64); ok {
        timestamp = int64(t)
    }

    payload := AuditSyncPayload{
        TenantID:  tenantID,
        AgentID:   agentID,
        Action:    action,
        Resource:  resource,
        Status:    status,
        Metadata:  metadata,
        Timestamp: timestamp,
    }

    err := l.SyncAuditLogsToCloud(ctx, payload, spiffeID)
    if err != nil {
        return nil, err
    }

    return map[string]string{"status": "success"}, nil
}
