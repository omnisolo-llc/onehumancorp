package secretssyncmcp

import (
	"context"
	"errors"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// SecretsSyncProvider abstracts the local-to-cloud secrets synchronization logic.
type SecretsSyncProvider interface {
	SyncSecretsDown(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error)
	SyncSecretsUp(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error)
}

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

// SecretsSyncMCP implements the MCP interface for local-to-cloud secrets sync.
type SecretsSyncMCP struct {
	provider SecretsSyncProvider
	isLocal  bool
}

// NewSecretsSyncMCP creates a new SecretsSyncMCP instance.
func NewSecretsSyncMCP(provider SecretsSyncProvider, isLocal bool) *SecretsSyncMCP {
	return &SecretsSyncMCP{
		provider: provider,
		isLocal:  isLocal,
	}
}

// ListTools returns the list of available tools.
func (m *SecretsSyncMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "sync_secrets_down",
			Description: "Synchronize secrets from the cloud to local.",
			InputSchema: `{"type": "object", "properties": {}}`,
		},
		{
			Name:        "sync_secrets_up",
			Description: "Synchronize local secrets to the cloud.",
			InputSchema: `{"type": "object", "properties": {}}`,
		},
	}
}

// CallTool executes a tool by name.
func (m *SecretsSyncMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	if !m.isLocal {
		return map[string]interface{}{
			"status": "skipped",
			"message": "Not running in Standalone/Local mode. Secrets sync is a no-op.",
		}, nil
	}

	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return nil, errors.New("unauthorized: missing claims")
	}

	switch toolName {
	case "sync_secrets_down":
		return m.provider.SyncSecretsDown(ctx, claims)
	case "sync_secrets_up":
		return m.provider.SyncSecretsUp(ctx, claims)
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
