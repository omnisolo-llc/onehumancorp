package mcp_sync

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"net/http"
	"os"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type SyncQueueItem struct {
	ID        string    `json:"id"`
	ToolName  string    `json:"tool_name"`
	Arguments string    `json:"arguments"`
	Status    string    `json:"status"`
	CreatedAt time.Time `json:"created_at"`
}

type McpSyncProxy struct {
	dbProvider db.Provider
	gatewayURL string
	client     *http.Client
}

func NewMcpSyncProxy(dbProvider db.Provider, gatewayURL string) *McpSyncProxy {
	return &McpSyncProxy{
		dbProvider: dbProvider,
		gatewayURL: gatewayURL,
		client:     &http.Client{Timeout: 10 * time.Second},
	}
}

// BufferIntegrationMetadata buffers MCP metadata into the database
func (p *McpSyncProxy) BufferIntegrationMetadata(ctx context.Context, toolName string, arguments interface{}) error {
	id := uuid.New().String()
	argsBytes, err := json.Marshal(arguments)
	if err != nil {
		return fmt.Errorf("failed to marshal arguments: %w", err)
	}

	argsStr := string(argsBytes)
	if p.dbProvider.IsSQLite() {
		query := `INSERT INTO hybrid_mcp_sync_queue (id, tool_name, arguments, status, created_at) VALUES (?, ?, ?, ?, ?)`
		_, err = p.dbProvider.Exec(ctx, query, id, toolName, argsStr, "PENDING", time.Now().UTC())
	} else {
		queryPG := `INSERT INTO hybrid_mcp_sync_queue (id, tool_name, arguments, status, created_at) VALUES ($1, $2, $3, $4, $5)`
		_, err = p.dbProvider.Exec(ctx, queryPG, id, toolName, argsStr, "PENDING", time.Now().UTC())
	}

	if err != nil {
		return fmt.Errorf("failed to buffer metadata: %v", err)
	}

	return nil
}

// SyncToCloudGateway syncs pending items to the cloud gateway
func (p *McpSyncProxy) SyncToCloudGateway(ctx context.Context) (int, error) {
	// Fetch PENDING items, locking them to prevent multiple workers from syncing the same items
	var query string
	if p.dbProvider.IsSQLite() {
		// SQLite doesn't have SKIP LOCKED, but usually operates sequentially or locks the whole DB file
		// So we just select and then update below
		query = `SELECT id, tool_name, arguments, status, created_at FROM hybrid_mcp_sync_queue WHERE status = 'PENDING'`
	} else {
		query = `SELECT id, tool_name, arguments, status, created_at FROM hybrid_mcp_sync_queue WHERE status = 'PENDING' FOR UPDATE SKIP LOCKED`
	}

	tx, err := p.dbProvider.Begin(ctx)
	if err != nil {
		return 0, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	rows, err := tx.Query(ctx, query)
	if err != nil {
		return 0, fmt.Errorf("failed to query pending items: %w", err)
	}
	defer rows.Close()

	var items []SyncQueueItem
	for rows.Next() {
		var item SyncQueueItem
		if err := rows.Scan(&item.ID, &item.ToolName, &item.Arguments, &item.Status, &item.CreatedAt); err != nil {
			return 0, fmt.Errorf("failed to scan item: %w", err)
		}
		items = append(items, item)
	}

	if len(items) == 0 {
		return 0, nil
	}

	syncedCount := 0
	for _, item := range items {
		// Sync individual item
		if err := p.syncItem(ctx, item); err != nil {
			slog.Error("failed to sync item", "id", item.ID, "error", err)
			continue
		}

		// Mark as SYNCED
		var updateErr error
		if p.dbProvider.IsSQLite() {
			updateQuery := `UPDATE hybrid_mcp_sync_queue SET status = 'SYNCED' WHERE id = ?`
			_, updateErr = tx.Exec(ctx, updateQuery, item.ID)
		} else {
			updateQueryPG := `UPDATE hybrid_mcp_sync_queue SET status = 'SYNCED' WHERE id = $1`
			_, updateErr = tx.Exec(ctx, updateQueryPG, item.ID)
		}

		if updateErr != nil {
			slog.Error("failed to update item status", "id", item.ID, "error", updateErr)
		} else {
			syncedCount++
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return 0, fmt.Errorf("failed to commit transaction: %w", err)
	}

	return syncedCount, nil
}

func (p *McpSyncProxy) syncItem(ctx context.Context, item SyncQueueItem) error {
	payloadBytes, err := json.Marshal(item)
	if err != nil {
		return err
	}

	req, err := http.NewRequestWithContext(ctx, http.MethodPost, p.gatewayURL+"/sync", bytes.NewReader(payloadBytes))
	if err != nil {
		return err
	}
	req.Header.Set("Content-Type", "application/json")

	// Apply SPIFFE/SPIRE SVID
	if spiffeToken := os.Getenv("SPIFFE_IDENTITY_TOKEN"); spiffeToken != "" {
		req.Header.Set("X-Spiffe-Token", spiffeToken)
	}

	resp, err := p.client.Do(req)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	if resp.StatusCode >= 400 {
		return fmt.Errorf("gateway returned status: %d", resp.StatusCode)
	}

	return nil
}
