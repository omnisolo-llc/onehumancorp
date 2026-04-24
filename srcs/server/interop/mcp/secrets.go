package mcp

import (
	"context"
	"fmt"
)

type SecretsSyncTool struct {
	proxy *McpSyncProxy
}

func NewSecretsSyncTool(proxy *McpSyncProxy) *SecretsSyncTool {
	return &SecretsSyncTool{proxy: proxy}
}

func (t *SecretsSyncTool) Execute(ctx context.Context, keys []string, direction string) error {
	if direction != "push" && direction != "pull" {
		return fmt.Errorf("invalid direction: %s", direction)
	}

	payload := map[string]interface{}{
		"secret_keys": keys,
		"sync_direction": direction,
	}

	_, err := t.proxy.BufferIntegrationState(ctx, "hybrid_secrets_sync", payload)
	if err != nil {
		return fmt.Errorf("failed to buffer state: %w", err)
	}

	return t.proxy.SyncPendingStates(ctx)
}
