package secretssyncmcp

import (
	"context"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string
	Description string
	InputSchema map[string]interface{}
}

// SecretsSyncProvider defines the interface for syncing secrets.
type SecretsSyncProvider interface {
	SyncSecretsDown(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error)
	SyncSecretsUp(ctx context.Context, claims *auth.Claims) (map[string]interface{}, error)
}

// SecretsSyncMCP implements the MCP interface for secrets sync.
type SecretsSyncMCP struct {
	provider SecretsSyncProvider
	isLocal  bool
}

// NewSecretsSyncMCP creates a new SecretsSyncMCP.
func NewSecretsSyncMCP(provider SecretsSyncProvider, isLocal bool) *SecretsSyncMCP {
	return &SecretsSyncMCP{
		provider: provider,
		isLocal:  isLocal,
	}
}

// ListTools returns the available tools.
func (m *SecretsSyncMCP) ListTools() []Tool {
	if !m.isLocal {
		return []Tool{}
	}
	return []Tool{
		{
			Name:        "secrets_sync_down",
			Description: "Pull latest cloud secrets down to the local standalone database.",
			InputSchema: map[string]interface{}{
				"type":       "object",
				"properties": map[string]interface{}{},
			},
		},
		{
			Name:        "secrets_sync_up",
			Description: "Push local unsynced secrets up to the cloud database.",
			InputSchema: map[string]interface{}{
				"type":       "object",
				"properties": map[string]interface{}{},
			},
		},
	}
}

// CallTool executes a specific tool.
func (m *SecretsSyncMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	if !m.isLocal {
		return nil, fmt.Errorf("secret sync tools are only available in standalone mode")
	}

	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		claims = &auth.Claims{}
	}

	switch toolName {
	case "secrets_sync_down":
		return m.provider.SyncSecretsDown(ctx, claims)
	case "secrets_sync_up":
		return m.provider.SyncSecretsUp(ctx, claims)
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
