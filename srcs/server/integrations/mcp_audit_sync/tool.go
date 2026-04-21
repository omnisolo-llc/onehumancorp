package mcp_audit_sync

import (
	"context"
	"errors"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
)

var tracer = otel.Tracer("mcp_audit_sync")

type AuditSyncPayload struct {
	TenantID  string `json:"tenant_id"`
	AgentID   string `json:"agent_id"`
	Action    string `json:"action"`
	Resource  string `json:"resource"`
	Status    string `json:"status"`
	Metadata  string `json:"metadata"`
	Timestamp int64  `json:"timestamp"`
}

type AuditTool struct {
	dbProvider db.Provider
}

func NewAuditTool(provider db.Provider) *AuditTool {
	return &AuditTool{
		dbProvider: provider,
	}
}

func (t *AuditTool) SyncAuditLogsToCloud(ctx context.Context, payload AuditSyncPayload) error {
	ctx, span := tracer.Start(ctx, "SyncAuditLogsToCloud")
	defer span.End()
	span.SetAttributes(
		attribute.String("audit.tenant_id", payload.TenantID),
		attribute.String("audit.agent_id", payload.AgentID),
		attribute.String("audit.action", payload.Action),
	)

	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return errors.New("unauthorized: missing claims or organization ID")
	}

	if claims.OrganizationID != payload.TenantID {
		return errors.New("unauthorized: tenant ID mismatch")
	}

	query := `
		INSERT INTO mcp_audit_sync_log (tenant_id, agent_id, action, resource, status, metadata, timestamp, created_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
	`
	_, err := t.dbProvider.Exec(ctx, query, payload.TenantID, payload.AgentID, payload.Action, payload.Resource, payload.Status, payload.Metadata, payload.Timestamp, time.Now().Unix())
	if err != nil {
		return fmt.Errorf("failed to sync audit logs to cloud: %w", err)
	}

	return nil
}
