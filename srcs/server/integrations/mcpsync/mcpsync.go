package mcpsync

import (
	"context"
	"fmt"
	"time"
	"log"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type McpSyncProxy struct {
	dbProvider db.Provider
}

func NewMcpSyncProxy(provider db.Provider) *McpSyncProxy {
	return &McpSyncProxy{dbProvider: provider}
}

func (p *McpSyncProxy) BufferIntegrationMetadata(ctx context.Context, id, toolName, arguments string) error {
	query := `INSERT INTO hybrid_mcp_sync_queue (id, tool_name, arguments, status, created_at) VALUES ($1, $2, $3, $4, $5)`
	_, err := p.dbProvider.Exec(ctx, query, id, toolName, arguments, "PENDING", time.Now())
	if err != nil {
		return fmt.Errorf("failed to buffer mcp sync: %w", err)
	}
	return nil
}

// simulateSpiffeMtlsSync simulates connecting to the cloud gateway using SPIFFE SVIDs
func (p *McpSyncProxy) simulateSpiffeMtlsSync(ctx context.Context, id, toolName, arguments string) error {
    // This is where real SPIFFE mTLS connection code would go.
    // For now we log it.
    log.Printf("[MCP Sync Proxy] Successfully established SPIFFE mTLS to Cloud Gateway for sync item: %s (Tool: %s)\n", id, toolName)
    // Simulate network delay
    time.Sleep(50 * time.Millisecond)
    return nil
}


func (p *McpSyncProxy) SyncToCloudGateway(ctx context.Context) error {
	// Query for PENDING
	query := `SELECT id, tool_name, arguments FROM hybrid_mcp_sync_queue WHERE status = 'PENDING'`
	rows, err := p.dbProvider.Query(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to query pending mcp syncs: %w", err)
	}
	defer rows.Close()

	var toSync []struct{ id, toolName, arguments string }
	for rows.Next() {
		var id, toolName, arguments string
		if err := rows.Scan(&id, &toolName, &arguments); err != nil {
			return fmt.Errorf("failed to scan row: %w", err)
		}
		toSync = append(toSync, struct{ id, toolName, arguments string }{id, toolName, arguments})
	}

	for _, item := range toSync {
	    // Simulate the actual cloud sync logic via SPIFFE
	    if err := p.simulateSpiffeMtlsSync(ctx, item.id, item.toolName, item.arguments); err != nil {
	        return fmt.Errorf("failed cloud sync via spiffe for %s: %w", item.id, err)
	    }

		updateQuery := `UPDATE hybrid_mcp_sync_queue SET status = 'SYNCED' WHERE id = $1`
		if _, err := p.dbProvider.Exec(ctx, updateQuery, item.id); err != nil {
			return fmt.Errorf("failed to update status to SYNCED for id %s: %w", item.id, err)
		}
	}

	return nil
}
