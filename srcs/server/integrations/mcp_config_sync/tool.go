package mcp_config_sync

import (
	"context"
	"encoding/json"
	"errors"
	"os"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter         = otel.Meter("mcp_config_sync")
	syncCounter   metric.Int64Counter
	getConfigCounter metric.Int64Counter
)

func init() {
	var err error
	syncCounter, err = meter.Int64Counter("mcp_config_sync.sync_count", metric.WithDescription("Number of configs synced"))
	if err != nil {
		// handle or ignore, but test checks it
	}
	getConfigCounter, err = meter.Int64Counter("mcp_config_sync.get_count", metric.WithDescription("Number of config gets"))
	if err != nil {
		// handle or ignore
	}
}

// ConfigSyncPayload represents a configuration to sync.
type ConfigSyncPayload struct {
	TenantID string            `json:"tenant_id"`
	AgentID  string            `json:"agent_id"`
	Key      string            `json:"key"`
	Value    string            `json:"value"`
	Metadata map[string]string `json:"metadata"`
}

// ConfigProvider is the interface for getting and syncing configurations.
type ConfigProvider interface {
	GetConfig(ctx context.Context, key string) (*ConfigSyncPayload, error)
	SyncConfigToCloud(ctx context.Context, payload ConfigSyncPayload) error
}

// LocalConfigProvider implements ConfigProvider using local files/SQLite.
type LocalConfigProvider struct {
	provider db.Provider
}

// NewLocalConfigProvider creates a new LocalConfigProvider.
func NewLocalConfigProvider(provider db.Provider) *LocalConfigProvider {
	return &LocalConfigProvider{provider: provider}
}

func (p *LocalConfigProvider) GetConfig(ctx context.Context, key string) (*ConfigSyncPayload, error) {
    if getConfigCounter != nil {
        getConfigCounter.Add(ctx, 1)
    }
	// Simulated local file read or SQLite query
	// In a real implementation this would read from .ohc/config.yaml or a local sqlite db
	return &ConfigSyncPayload{
		Key:   key,
		Value: "local-value-" + key, // Placeholder
	}, nil
}

func (p *LocalConfigProvider) SyncConfigToCloud(ctx context.Context, payload ConfigSyncPayload) error {
	// Not applicable for local provider acting purely locally,
	// but an agent running locally would use the CloudProvider instance to push.
	return errors.New("cannot push to cloud from a purely local config provider interface, use the appropriate MCP client")
}

// CloudConfigProvider implements ConfigProvider using Cloud PostgreSQL.
type CloudConfigProvider struct {
	provider db.Provider
}

// NewCloudConfigProvider creates a new CloudConfigProvider.
func NewCloudConfigProvider(provider db.Provider) *CloudConfigProvider {
	return &CloudConfigProvider{provider: provider}
}

func (p *CloudConfigProvider) GetConfig(ctx context.Context, key string) (*ConfigSyncPayload, error) {
    if getConfigCounter != nil {
        getConfigCounter.Add(ctx, 1)
    }
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return nil, errors.New("unauthorized: missing claims or organization ID")
	}

	// Query Postgres using tenant isolation
	// Assuming a table structure, adjust as needed based on actual schema.
	row := p.provider.QueryRow(ctx, "SELECT value, agent_id, metadata FROM mcp_config_sync_log WHERE tenant_id = $1 AND key = $2 ORDER BY created_at DESC LIMIT 1", claims.OrganizationID, key)

	var val, agentID, metadataStr string
	if err := row.Scan(&val, &agentID, &metadataStr); err != nil {
		return nil, err
	}

	var metadata map[string]string
	if metadataStr != "" {
		if err := json.Unmarshal([]byte(metadataStr), &metadata); err != nil {
			// ignore or log unmarshal error
		}
	}

	return &ConfigSyncPayload{
		TenantID: claims.OrganizationID,
		AgentID:  agentID,
		Key:      key,
		Value:    val,
		Metadata: metadata,
	}, nil
}

func (p *CloudConfigProvider) SyncConfigToCloud(ctx context.Context, payload ConfigSyncPayload) error {
    if syncCounter != nil {
        syncCounter.Add(ctx, 1)
    }
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return errors.New("unauthorized: missing claims or organization ID")
	}

	// Enforce tenant isolation by ignoring payload.TenantID and using claims.
	tenantID := claims.OrganizationID

	metadataBytes, _ := json.Marshal(payload.Metadata)

	_, err := p.provider.Exec(ctx,
		"INSERT INTO mcp_config_sync_log(tenant_id, agent_id, key, value, metadata) VALUES($1, $2, $3, $4, $5)",
		tenantID, payload.AgentID, payload.Key, payload.Value, string(metadataBytes))
	return err
}

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

// ConfigSyncMCP implements the MCP interface for config sync operations.
type ConfigSyncMCP struct {
	provider ConfigProvider
}

// NewConfigSyncMCP creates a new ConfigSyncMCP instance.
func NewConfigSyncMCP(provider ConfigProvider) *ConfigSyncMCP {
	return &ConfigSyncMCP{provider: provider}
}

// ListTools returns the list of available tools.
func (m *ConfigSyncMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "get_config",
			Description: "Retrieves a configuration value by key.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"key": {"type": "string"}}, "required": ["key"]}`),
		},
		{
			Name:        "sync_config_to_cloud",
			Description: "Syncs a local configuration to the cloud vault.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"agent_id": {"type": "string"}, "key": {"type": "string"}, "value": {"type": "string"}, "metadata": {"type": "object"}}, "required": ["key", "value"]}`),
		},
	}
}

// CallTool executes a tool by name.
func (m *ConfigSyncMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	switch toolName {
	case "get_config":
		key, ok := arguments["key"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'key' argument")
		}
		result, err := m.provider.GetConfig(ctx, key)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"config": result}, nil
	case "sync_config_to_cloud":
		key, ok := arguments["key"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'key' argument")
		}
		value, ok := arguments["value"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'value' argument")
		}

		agentID, _ := arguments["agent_id"].(string)

		var metadata map[string]string
		if rawMeta, ok := arguments["metadata"].(map[string]interface{}); ok {
			metadata = make(map[string]string)
			for k, v := range rawMeta {
				if strVal, ok := v.(string); ok {
					metadata[k] = strVal
				}
			}
		}

		payload := ConfigSyncPayload{
			AgentID:  agentID,
			Key:      key,
			Value:    value,
			Metadata: metadata,
		}

		if err := m.provider.SyncConfigToCloud(ctx, payload); err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success"}, nil
	default:
		return nil, errors.New("unknown tool: " + toolName)
	}
}

func envBoolDefault(key string, fallback bool) bool {
	val := os.Getenv(key)
	if val == "" {
		return fallback
	}
	return strings.ToLower(val) == "true" || val == "1"
}

// NewProviderFactory returns a ConfigProvider based on environment configuration.
func NewProviderFactory(provider db.Provider) ConfigProvider {
	if envBoolDefault("OHC_MULTITENANT", true) {
		return NewCloudConfigProvider(provider)
	}
	return NewLocalConfigProvider(provider)
}
