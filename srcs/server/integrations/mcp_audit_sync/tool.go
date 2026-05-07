package mcp_audit_sync

import (
	"context"
	"encoding/json"
	"fmt"
	"github.com/google/uuid"


	"time"
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

type AuditSyncTool struct {
	DB        interface{ Exec(context.Context, string, ...interface{}) (interface{}, error) }
	Telemetry interface{ IncrementCounter(string, int64, map[string]string) }
}

func NewAuditSyncTool(db interface{ Exec(context.Context, string, ...interface{}) (interface{}, error) }, tele interface{ IncrementCounter(string, int64, map[string]string) }) *AuditSyncTool {
	return &AuditSyncTool{
		DB:        db,
		Telemetry: tele,
	}
}

func (t *AuditSyncTool) SyncAuditLogsToCloud(ctx context.Context, payloadStr string) error {
	var payload AuditSyncPayload
	if err := json.Unmarshal([]byte(payloadStr), &payload); err != nil {
		return fmt.Errorf("failed to unmarshal payload: %w", err)
	}

	if payload.TenantID == "" || payload.AgentID == "" || payload.Action == "" || payload.Resource == "" || payload.Status == "" {
		return fmt.Errorf("invalid payload: missing required fields")
	}

	id := uuid.New().String()
	query := `
		INSERT INTO mcp_audit_sync_log (id, tenant_id, agent_id, action, resource, status, metadata, timestamp)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
	`

	_, err := t.DB.Exec(ctx, query, id, payload.TenantID, payload.AgentID, payload.Action, payload.Resource, payload.Status, payload.Metadata, time.Unix(payload.Timestamp, 0))
	if err != nil {
		return fmt.Errorf("failed to insert audit log: %w", err)
	}

	if t.Telemetry != nil {
		t.Telemetry.IncrementCounter("ohc.audit_sync.count", 1, map[string]string{
			"tenant_id": payload.TenantID,
			"agent_id":  payload.AgentID,
			"action":    payload.Action,
			"status":    payload.Status,
		})
	}

	return nil
}
