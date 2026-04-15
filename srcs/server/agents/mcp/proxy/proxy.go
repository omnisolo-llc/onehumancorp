package proxy

import (
	"bytes"
	"context"
	"crypto/tls"
	"encoding/json"
	"fmt"
	"net/http"
	"os"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/google/uuid"
)

type McpSyncProxy struct {
	dbProvider   db.Provider
	cloudGateway string
	httpClient   *http.Client
}

func NewMcpSyncProxy(dbProvider db.Provider, cloudGateway string) *McpSyncProxy {
	// For SPIFFE mTLS we would typically use a SPIFFE workload API client to get the tls.Config.
	// We configure a basic TLS client here as placeholder/simulation of mTLS if not fully fleshed out.
	tr := &http.Transport{
		TLSClientConfig: &tls.Config{
			// In production, configure mTLS certificates
		},
	}
	return &McpSyncProxy{
		dbProvider:   dbProvider,
		cloudGateway: cloudGateway,
		httpClient:   &http.Client{Timeout: 10 * time.Second, Transport: tr},
	}
}

// Buffer buffers a tool execution request into the local SQLite database.
func (p *McpSyncProxy) Buffer(ctx context.Context, toolName string, arguments map[string]interface{}) (string, error) {
	if !p.dbProvider.IsSQLite() {
		// If we are in the cloud (Postgres), we might just directly push to the gateway or queue,
		// but the prompt mentions "buffer integration metadata locally in SQLite during Standalone mode"
		// and the schema works for both, so let's just buffer it anyway.
	}

	id := uuid.New().String()
	argsBytes, err := json.Marshal(arguments)
	if err != nil {
		return "", fmt.Errorf("failed to marshal arguments: %w", err)
	}

	_, err = p.dbProvider.Exec(ctx,
		"INSERT INTO hybrid_mcp_sync_queue (id, tool_name, arguments, status, created_at) VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP)",
		id, toolName, string(argsBytes), "PENDING")
	if err != nil {
		return "", fmt.Errorf("failed to buffer to db: %w", err)
	}

	return id, nil
}

// Sync periodically syncs buffered tool executions to the cloud gateway.
func (p *McpSyncProxy) Sync(ctx context.Context) (int, error) {
	rows, err := p.dbProvider.Query(ctx, "SELECT id, tool_name, arguments FROM hybrid_mcp_sync_queue WHERE status = 'PENDING' LIMIT 50")
	if err != nil {
		return 0, fmt.Errorf("failed to query queue: %w", err)
	}
	defer rows.Close()

	type queueItem struct {
		ID        string
		ToolName  string
		Arguments string
	}
	var items []queueItem

	for rows.Next() {
		var item queueItem
		if err := rows.Scan(&item.ID, &item.ToolName, &item.Arguments); err != nil {
			continue
		}
		items = append(items, item)
	}

	if len(items) == 0 {
		return 0, nil
	}

	syncedCount := 0

	for _, item := range items {
		var argsMap map[string]interface{}
		if err := json.Unmarshal([]byte(item.Arguments), &argsMap); err != nil {
			// Mark as failed if arguments are invalid
			p.dbProvider.Exec(ctx, "UPDATE hybrid_mcp_sync_queue SET status = 'FAILED' WHERE id = $1", item.ID)
			continue
		}

		payload := map[string]interface{}{
			"tool_name": item.ToolName,
			"arguments": argsMap,
		}

		payloadBytes, err := json.Marshal(payload)
		if err != nil {
			continue
		}

		url := fmt.Sprintf("%s/mcp/sync", p.cloudGateway)
		req, err := http.NewRequestWithContext(ctx, "POST", url, bytes.NewReader(payloadBytes))
		if err != nil {
			continue
		}

		req.Header.Set("Content-Type", "application/json")
		if spiffeToken := os.Getenv("SPIFFE_IDENTITY_TOKEN"); spiffeToken != "" {
			req.Header.Set("Authorization", "Bearer "+spiffeToken)
		}

		resp, err := p.httpClient.Do(req)
		if err != nil {
			continue
		}

		if resp.StatusCode >= 200 && resp.StatusCode < 300 {
			p.dbProvider.Exec(ctx, "UPDATE hybrid_mcp_sync_queue SET status = 'SYNCED' WHERE id = $1", item.ID)
			syncedCount++
		}
		resp.Body.Close()
	}

	return syncedCount, nil
}
