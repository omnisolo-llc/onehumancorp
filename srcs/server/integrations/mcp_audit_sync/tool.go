package mcp_audit_sync

import (
	"context"
	"database/sql"
	"errors"
	"os"

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

func SyncAuditLogsToCloud(ctx context.Context, db *sql.DB, payload AuditSyncPayload) error {
	var span trace.Span
	if os.Getenv("OHC_TELEMETRY_ENABLED") == "true" {
		tracer := otel.Tracer("mcp_audit_sync")
		ctx, span = tracer.Start(ctx, "SyncAuditLogsToCloud")
		defer span.End()
		span.SetAttributes(
			attribute.String("tenant_id", payload.TenantID),
			attribute.String("agent_id", payload.AgentID),
			attribute.String("action", payload.Action),
		)
	}

	if db == nil {
		if span != nil {
			span.RecordError(errors.New("database connection is nil"))
		}
		return errors.New("database connection is nil")
	}

	query := `
		INSERT INTO mcp_audit_sync_log (tenant_id, agent_id, action, resource, status, metadata, timestamp)
		VALUES ($1, $2, $3, $4, $5, $6, $7)
	`
	_, err := db.ExecContext(ctx, query,
		payload.TenantID,
		payload.AgentID,
		payload.Action,
		payload.Resource,
		payload.Status,
		payload.Metadata,
		payload.Timestamp,
	)

	if err != nil && span != nil {
		span.RecordError(err)
	}

	return err
}
