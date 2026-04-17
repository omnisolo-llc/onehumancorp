package mcp

import (
	"context"
	"fmt"
)

type SchemaSyncTool struct {
	proxy *McpSyncProxy
}

func NewSchemaSyncTool(proxy *McpSyncProxy) *SchemaSyncTool {
	return &SchemaSyncTool{proxy: proxy}
}

func (t *SchemaSyncTool) Execute(ctx context.Context, versions []string, direction string) error {
	if direction != "push" && direction != "pull" {
		return fmt.Errorf("invalid direction: %s", direction)
	}

	payload := map[string]interface{}{
		"schema_versions": versions,
		"sync_direction":  direction,
	}

	_, err := t.proxy.BufferIntegrationState(ctx, "hybrid_schema_sync", payload)
	if err != nil {
		return fmt.Errorf("failed to buffer state: %w", err)
	}

	return t.proxy.SyncPendingStates(ctx)
}
