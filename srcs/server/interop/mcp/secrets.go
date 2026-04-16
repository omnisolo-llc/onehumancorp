package mcp

import (
	"context"
	"errors"
	"fmt"
)

type SecretsSyncTool struct {
	proxy *McpSyncProxy
}

func NewSecretsSyncTool(proxy *McpSyncProxy) *SecretsSyncTool {
	return &SecretsSyncTool{proxy: proxy}
}

func (s *SecretsSyncTool) Execute(ctx context.Context, keys []string, direction string) error {
	if direction != "pull" && direction != "push" {
		return errors.New("invalid sync direction: must be 'pull' or 'push'")
	}

	payload := map[string]interface{}{
		"secret_keys":    keys,
		"sync_direction": direction,
	}

	_, err := s.proxy.BufferIntegrationState(ctx, "hybrid_secrets_sync", payload)
	if err != nil {
		return fmt.Errorf("failed to buffer integration state: %w", err)
	}

	err = s.proxy.SyncPendingStates(ctx)
	if err != nil {
		return fmt.Errorf("failed to sync pending states: %w", err)
	}

	return nil
}
