package mcp_config_sync

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os"


	"github.com/onehumancorp/mono/src/server/auth"
	"github.com/onehumancorp/mono/src/server/db"
	"github.com/onehumancorp/mono/src/server/agents"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"gopkg.in/yaml.v3"
)

var tracer = otel.Tracer("mcp_config_sync")

type ConfigSyncPayload struct {
	TenantID string            `json:"tenant_id"`
	AgentID  string            `json:"agent_id"`
	Key      string            `json:"key"`
	Value    string            `json:"value"`
	Metadata map[string]string `json:"metadata"`
}

type ConfigTool struct {
	dbProvider db.Provider
}

func NewConfigTool(provider db.Provider) *ConfigTool {
	return &ConfigTool{
		dbProvider: provider,
	}
}

func (t *ConfigTool) GetConfig(ctx context.Context, key string) (string, error) {
	ctx, span := tracer.Start(ctx, "GetConfig")
	defer span.End()
	span.SetAttributes(attribute.String("config.key", key))

	// In Standalone mode, read from local file system
	if os.Getenv("OHC_HYBRID_MODE") == "standalone" {
		configPath := ".ohc/config.yaml"
		if path := os.Getenv("OHC_CONFIG_PATH"); path != "" {
			configPath = path
		}

		data, err := os.ReadFile(configPath)
		if err != nil {
			if errors.Is(err, os.ErrNotExist) {
				return "", fmt.Errorf("config file not found at %s", configPath)
			}
			return "", fmt.Errorf("failed to read config file: %w", err)
		}

		var config map[string]interface{}
		if err := yaml.Unmarshal(data, &config); err != nil {
			return "", fmt.Errorf("failed to parse config file: %w", err)
		}

		if val, ok := config[key]; ok {
			return fmt.Sprintf("%v", val), nil
		}
		return "", fmt.Errorf("key %s not found in local config", key)
	}

	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return "", errors.New("unauthorized: missing claims or organization ID")
	}

	var value string
	err := t.dbProvider.QueryRow(ctx, "SELECT value FROM mcp_config_sync_log WHERE tenant_id = $1 AND key = $2 ORDER BY created_at DESC LIMIT 1", claims.OrganizationID, key).Scan(&value)
	if err != nil {
		return "", fmt.Errorf("failed to get config: %w", err)
	}

	return value, nil
}

func (t *ConfigTool) SyncConfigToCloud(ctx context.Context, payload ConfigSyncPayload) error {
	ctx, span := tracer.Start(ctx, "SyncConfigToCloud")
	defer span.End()
	span.SetAttributes(
		attribute.String("config.tenant_id", payload.TenantID),
		attribute.String("config.agent_id", payload.AgentID),
		attribute.String("config.key", payload.Key),
	)

	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return errors.New("unauthorized: missing claims or organization ID")
	}

	if claims.OrganizationID != payload.TenantID {
		return errors.New("unauthorized: tenant ID mismatch")
	}

	metadataJSON, err := json.Marshal(payload.Metadata)
	if err != nil {
		return fmt.Errorf("failed to marshal metadata: %w", err)
	}

	query := `
		INSERT INTO mcp_config_sync_log (tenant_id, agent_id, key, value, metadata)
		VALUES ($1, $2, $3, $4, $5)
	`
	_, err = t.dbProvider.Exec(ctx, query, payload.TenantID, payload.AgentID, payload.Key, payload.Value, string(metadataJSON))
	if err != nil {
		return fmt.Errorf("failed to sync config to cloud: %w", err)
	}

	return nil
}

// GetConfigTool returns the MCP tool definition for getting a config value.
func (t *ConfigTool) GetConfigTool() agents.Tool {
	return agents.Tool{
		Name:        "get_config",
		Description: "Reads a configuration value securely from either the local file system (Standalone mode) or the multi-tenant Enterprise Vault (Cloud mode).",
		Parameters:  json.RawMessage(`{"type":"object","properties":{"key":{"type":"string","description":"The configuration key to read"}},"required":["key"]}`),
		Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
			var input struct {
				Key string `json:"key"`
			}
			if err := json.Unmarshal(args, &input); err != nil {
				return "", fmt.Errorf("invalid arguments: %w", err)
			}
			if input.Key == "" {
				return "", errors.New("key is required")
			}

			val, err := t.GetConfig(ctx, input.Key)
			if err != nil {
				return "", err
			}
			return val, nil
		},
	}
}

// SyncConfigToCloudTool returns the MCP tool definition for syncing a config value to the cloud.
func (t *ConfigTool) SyncConfigToCloudTool() agents.Tool {
	return agents.Tool{
		Name:        "sync_config_to_cloud",
		Description: "Syncs a local configuration value back to the multi-tenant Enterprise Vault in the Cloud via an MCP interface.",
		Parameters:  json.RawMessage(`{"type":"object","properties":{"tenant_id":{"type":"string"},"agent_id":{"type":"string"},"key":{"type":"string"},"value":{"type":"string"},"metadata":{"type":"object","additionalProperties":{"type":"string"}}},"required":["tenant_id","agent_id","key","value"]}`),
		Execute: func(ctx context.Context, args json.RawMessage) (string, error) {
			var input ConfigSyncPayload
			if err := json.Unmarshal(args, &input); err != nil {
				return "", fmt.Errorf("invalid arguments: %w", err)
			}
			if input.TenantID == "" || input.AgentID == "" || input.Key == "" || input.Value == "" {
				return "", errors.New("tenant_id, agent_id, key, and value are required")
			}

			if err := t.SyncConfigToCloud(ctx, input); err != nil {
				return "", err
			}
			return "Successfully synced config to cloud", nil
		},
	}
}
