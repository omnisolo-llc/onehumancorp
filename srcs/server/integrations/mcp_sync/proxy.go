package mcp_sync

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type McpSyncProxy struct {
	provider db.Provider
}

func NewMcpSyncProxy(provider db.Provider) *McpSyncProxy {
	return &McpSyncProxy{provider: provider}
}

func (p *McpSyncProxy) EnqueueSync(ctx context.Context, id, toolName string, arguments map[string]interface{}) error {
	argsJSON, err := json.Marshal(arguments)
	if err != nil {
		return fmt.Errorf("failed to marshal arguments: %w", err)
	}

	query := `
		INSERT INTO hybrid_mcp_sync_queue (id, tool_name, arguments, status, created_at)
		VALUES ($1, $2, $3, 'PENDING', CURRENT_TIMESTAMP)
	`
	_, err = p.provider.Exec(ctx, query, id, toolName, string(argsJSON))
	if err != nil {
		return fmt.Errorf("failed to enqueue sync: %w", err)
	}

	return nil
}
