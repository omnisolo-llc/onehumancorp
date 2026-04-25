package mcp_audit_sync

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/src/server/auth"
	"github.com/onehumancorp/mono/src/server/db"
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

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

type AuditTool struct {
	dbProvider db.Provider
}

func NewAuditTool(provider db.Provider) *AuditTool {
	return &AuditTool{
		dbProvider: provider,
	}
}

// ListTools returns the list of available tools.
func (t *AuditTool) ListTools() []Tool {
	return []Tool{
		{
			Name:        "sync_audit_logs_to_cloud",
			Description: "Syncs a batch of audit logs to the cloud database.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"tenant_id": {"type": "string"}, "agent_id": {"type": "string"}, "action": {"type": "string"}, "resource": {"type": "string"}, "status": {"type": "string"}, "metadata": {"type": "string"}, "timestamp": {"type": "integer"}}, "required": ["tenant_id", "agent_id", "action", "resource", "status", "timestamp"]}`),
		},
	}
}

// CallTool executes a tool by name.
func (t *AuditTool) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	switch toolName {
	case "sync_audit_logs_to_cloud":
		tenantID, _ := arguments["tenant_id"].(string)
		agentID, _ := arguments["agent_id"].(string)
		action, _ := arguments["action"].(string)
		resource, _ := arguments["resource"].(string)
		status, _ := arguments["status"].(string)
		metadata, _ := arguments["metadata"].(string)
		var timestamp int64
		switch v := arguments["timestamp"].(type) {
		case float64:
			timestamp = int64(v)
		case int:
			timestamp = int64(v)
		case int64:
			timestamp = v
		}

		if tenantID == "" || agentID == "" || action == "" || resource == "" || status == "" || timestamp == 0 {
			return nil, errors.New("missing or invalid arguments")
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

		err := t.SyncAuditLogsToCloud(ctx, payload)
		if err != nil {
			return nil, err
		}

		return map[string]interface{}{"status": "success"}, nil
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
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
