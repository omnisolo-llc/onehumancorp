package sync

import (
	"bytes"
	"context"
	"fmt"
	"log/slog"
	"net/http"
	"os"
	"time"

	"github.com/onehumancorp/mono/src/server/orchestration"
	"github.com/onehumancorp/mono/src/server/telemetry"
	"go.opentelemetry.io/otel"
)

// SyncEscalator monitors local_mcp_rag_tasks and escalates them to the cloud swarm.
type SyncEscalator struct {
	hub *orchestration.Hub
}

// NewSyncEscalator creates a new SyncEscalator.
func NewSyncEscalator(hub *orchestration.Hub) *SyncEscalator {
	return &SyncEscalator{
		hub: hub,
	}
}

// Start runs the daemon in the background.
func (s *SyncEscalator) Start(ctx context.Context, interval time.Duration) {
	go func() {
		ticker := time.NewTicker(interval)
		defer ticker.Stop()

		for {
			select {
			case <-ctx.Done():
				return
			case <-ticker.C:
				if err := s.processEscalations(ctx); err != nil {
					slog.Error("failed to process escalations", "error", err)
				}
			}
		}
	}()
}

func (s *SyncEscalator) processEscalations(ctx context.Context) error {
	ctx, span := otel.Tracer("github.com/onehumancorp/mono/src/server/services/sync").Start(ctx, "processEscalations")
	defer span.End()

	db := s.hub.SIPDB()
	if db == nil {
		return fmt.Errorf("SIPDB is nil")
	}

	provider := db.Provider()
	if provider == nil {
		return fmt.Errorf("DB provider is nil")
	}

	query := `SELECT id, tenant_id, payload FROM local_mcp_rag_tasks WHERE escalation_status = 'local'`
	rows, err := provider.Query(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to query tasks: %w", err)
	}
	defer rows.Close()

	type taskPayload struct {
		ID       string
		TenantID string
		Payload  string
	}

	var tasks []taskPayload
	for rows.Next() {
		var t taskPayload
		if err := rows.Scan(&t.ID, &t.TenantID, &t.Payload); err != nil {
			slog.Error("failed to scan task row", "error", err)
			continue
		}
		tasks = append(tasks, t)
	}

	if err := rows.Err(); err != nil {
		return fmt.Errorf("row iteration error: %w", err)
	}

	for _, t := range tasks {
		payloadStr := fmt.Sprintf(`{"id": "%s", "tenant_id": "%s", "data": "%s"}`, t.ID, t.TenantID, t.Payload)
		payloadBytes := []byte(payloadStr)
		req, err := http.NewRequestWithContext(ctx, http.MethodPost, "https://cloud.onehumancorp.com/api/v1/orchestration/escalate", bytes.NewBuffer(payloadBytes))
		if err != nil {
			slog.Error("failed to create request", "error", err)
			continue
		}
		req.Header.Set("Content-Type", "application/json")

		if spiffeToken := os.Getenv("SPIFFE_IDENTITY_TOKEN"); spiffeToken != "" {
			req.Header.Set("Authorization", "Bearer "+spiffeToken)
		}

		client := http.DefaultClient
		resp, err := client.Do(req)
		if err != nil {
			slog.Error("failed to send escalation request", "error", err)
			continue
		}
		resp.Body.Close()

		if resp.StatusCode == http.StatusOK {
			updateQuery := `UPDATE local_mcp_rag_tasks SET escalation_status = 'cloud' WHERE id = $1 AND tenant_id = $2`
			if provider.IsSQLite() {
				updateQuery = `UPDATE local_mcp_rag_tasks SET escalation_status = 'cloud' WHERE id = ? AND tenant_id = ?`
			}
			_, err := provider.Exec(ctx, updateQuery, t.ID, t.TenantID)
			if err != nil {
				slog.Error("failed to update task status", "error", err)
			} else {
				if telemetry.TasksEscalatedTotal != nil {
					telemetry.TasksEscalatedTotal.Add(ctx, 1)
				}
			}
		} else {
			slog.Error("escalation failed with status", "status", resp.StatusCode)
		}
	}

	return nil
}
