package mcp_sync

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type McpSyncProxy struct {
	client   *http.Client
	provider db.Provider
}

func NewMcpSyncProxy(provider db.Provider, client *http.Client) *McpSyncProxy {
	if client == nil {
		client = http.DefaultClient
	}
	return &McpSyncProxy{provider: provider, client: client}
}

func (p *McpSyncProxy) BufferToolExecution(ctx context.Context, toolName, payload string) (string, error) {
	id := uuid.New().String()
	query := "INSERT INTO hybrid_mcp_sync_queue (id, tool_name, payload, status, created_at, updated_at) VALUES ($1, $2, $3, 'PENDING', $4, $4)"
	now := time.Now().UTC()
	_, err := p.provider.Exec(ctx, query, id, toolName, payload, now)
	if err != nil {
		return "", fmt.Errorf("failed to buffer tool execution: %w", err)
	}
	return id, nil
}

func (p *McpSyncProxy) SyncToCloud(ctx context.Context, gatewayURL, spiffeID string) error {
	tx, err := p.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}

	query := "SELECT id, tool_name, payload FROM hybrid_mcp_sync_queue WHERE status = 'PENDING' LIMIT 100"
	if !p.provider.IsSQLite() {
		query += " FOR UPDATE SKIP LOCKED"
	}
	rows, err := tx.Query(ctx, query)
	if err != nil {
		tx.Rollback(ctx)
		return fmt.Errorf("failed to query pending executions: %w", err)
	}

	type syncJob struct {
		ID       string
		ToolName string
		Payload  string
	}
	var jobs []syncJob
	for rows.Next() {
		var job syncJob
		if err := rows.Scan(&job.ID, &job.ToolName, &job.Payload); err != nil {
			rows.Close()
			tx.Rollback(ctx)
			return fmt.Errorf("failed to scan row: %w", err)
		}
		jobs = append(jobs, job)
	}
	rows.Close()

	if len(jobs) == 0 {
		tx.Rollback(ctx)
		return nil
	}

	for _, job := range jobs {
		if _, err := tx.Exec(ctx, "UPDATE hybrid_mcp_sync_queue SET status = 'PROCESSING' WHERE id = $1", job.ID); err != nil {
			tx.Rollback(ctx)
			return fmt.Errorf("failed to update status to PROCESSING: %w", err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit tx: %w", err)
	}

	var syncedIDs []string
	for _, job := range jobs {
		reqBody, _ := json.Marshal(map[string]interface{}{
			"tool_name": job.ToolName,
			"payload":   job.Payload,
			"spiffe_id": spiffeID,
		})
		req, err := http.NewRequestWithContext(ctx, http.MethodPost, gatewayURL, bytes.NewReader(reqBody))
		if err != nil {
			continue
		}
		req.Header.Set("Content-Type", "application/json")
		resp, err := p.client.Do(req)
		if err != nil {
			continue
		}
		io.Copy(io.Discard, resp.Body)
		resp.Body.Close()
		if resp.StatusCode >= 200 && resp.StatusCode < 300 {
			syncedIDs = append(syncedIDs, job.ID)
		}
	}

	for _, id := range syncedIDs {
		updateQuery := "UPDATE hybrid_mcp_sync_queue SET status = 'SYNCED', synced_at = $1, updated_at = $1 WHERE id = $2"
		now := time.Now().UTC()
		if _, err := p.provider.Exec(ctx, updateQuery, now, id); err != nil {
			return fmt.Errorf("failed to update status for %s: %w", id, err)
		}
	}

	return nil
}
