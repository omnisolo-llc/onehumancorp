package mcp

import (
	"context"
	"crypto/tls"
	"encoding/json"
	"fmt"
	"time"
	"bytes"
	"net/http"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type SyncStatus string

const (
	StatusPending SyncStatus = "PENDING"
	StatusSynced  SyncStatus = "SYNCED"
	StatusFailed  SyncStatus = "FAILED"
)

type McpSyncProxy struct {
	dbProvider db.Provider
	tlsConfig  *tls.Config // Used for SPIFFE mTLS
	cloudEndpoint string
	httpClient *http.Client
}

func NewMcpSyncProxy(dbProvider db.Provider, tlsConfig *tls.Config, cloudEndpoint string) *McpSyncProxy {
	client := &http.Client{
		Timeout: 10 * time.Second,
	}
	if tlsConfig != nil {
		client.Transport = &http.Transport{
			TLSClientConfig: tlsConfig,
		}
	}

	return &McpSyncProxy{
		dbProvider: dbProvider,
		tlsConfig:  tlsConfig,
		cloudEndpoint: cloudEndpoint,
		httpClient: client,
	}
}

func (p *McpSyncProxy) BufferIntegrationState(ctx context.Context, toolName string, payload map[string]interface{}) (string, error) {
	payloadBytes, err := json.Marshal(payload)
	if err != nil {
		return "", fmt.Errorf("failed to marshal payload: %w", err)
	}

	id := uuid.New().String()
	query := `
		INSERT INTO hybrid_mcp_sync_queue (id, tool_name, payload, status)
		VALUES ($1, $2, $3, $4)
	`

	_, err = p.dbProvider.Exec(ctx, query, id, toolName, string(payloadBytes), StatusPending)
	if err != nil {
		return "", fmt.Errorf("failed to insert into hybrid_mcp_sync_queue: %w", err)
	}

	return id, nil
}

func (p *McpSyncProxy) SyncPendingStates(ctx context.Context) error {
	query := `
		SELECT id, tool_name, payload
		FROM hybrid_mcp_sync_queue
		WHERE status = $1
	`
	rows, err := p.dbProvider.Query(ctx, query, StatusPending)
	if err != nil {
		return fmt.Errorf("failed to query pending states: %w", err)
	}
	defer rows.Close()

	for rows.Next() {
		var id, toolName, payload string
		if err := rows.Scan(&id, &toolName, &payload); err != nil {
			return fmt.Errorf("failed to scan row: %w", err)
		}

		// Attempt network transmission of state
		reqPayload := map[string]string{
			"id": id,
			"tool_name": toolName,
			"payload": payload,
		}
		reqBytes, _ := json.Marshal(reqPayload)
		req, err := http.NewRequestWithContext(ctx, "POST", p.cloudEndpoint+"/api/mcp/sync", bytes.NewBuffer(reqBytes))
		if err != nil {
			return fmt.Errorf("failed to create request: %w", err)
		}
		req.Header.Set("Content-Type", "application/json")

		resp, err := p.httpClient.Do(req)

		statusToSet := StatusSynced
		if err != nil || resp.StatusCode >= 400 {
			// In a real implementation we would retry, for now mark as failed or leave pending
			statusToSet = StatusFailed
		}
		if resp != nil {
			resp.Body.Close()
		}

		updateQuery := `
			UPDATE hybrid_mcp_sync_queue
			SET status = $1, synced_at = $2
			WHERE id = $3
		`
		if _, err := p.dbProvider.Exec(ctx, updateQuery, statusToSet, time.Now(), id); err != nil {
			return fmt.Errorf("failed to update status for id %s: %w", id, err)
		}
	}

    return nil
}
