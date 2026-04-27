package mcp_config_sync

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os"

	local "github.com/onehumancorp/mono/src/server/agents/local"
	"github.com/onehumancorp/mono/src/server/auth"
	"github.com/onehumancorp/mono/src/server/db"
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

type mcpConfigTool struct {
    def local.ToolDefinition
    execute func(ctx context.Context, workDir string, input map[string]interface{}) (string, error)
}

func (t *mcpConfigTool) Definition() local.ToolDefinition {
    return t.def
}

func (t *mcpConfigTool) Execute(ctx context.Context, workDir string, input map[string]interface{}) (string, error) {
    return t.execute(ctx, workDir, input)
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
func (t *ConfigTool) GetConfigTool() local.Tool {
	return &mcpConfigTool{
		def: local.ToolDefinition{
			Name:        "get_config",
			Description: "Reads a configuration value securely from either the local file system (Standalone mode) or the multi-tenant Enterprise Vault (Cloud mode).",
			InputSchema: map[string]interface{}{"type": "object", "properties": map[string]interface{}{"key": map[string]interface{}{"type": "string", "description": "The configuration key to retrieve"}}, "required": []interface{}{"key"}},
		},
		execute: func(ctx context.Context, workDir string, input map[string]interface{}) (string, error) {
			keyVal, ok := input["key"].(string)
			if !ok {
				return "", fmt.Errorf("invalid arguments: missing key")
			}
			if keyVal == "" {
				return "", errors.New("key is required")
			}

			val, err := t.GetConfig(ctx, keyVal)
			if err != nil {
				return "", err
			}
			return val, nil
		},
	}
}

func (t *ConfigTool) SyncConfigToCloudTool() local.Tool {
	return &mcpConfigTool{
		def: local.ToolDefinition{
			Name:        "sync_config_to_cloud",
			Description: "Syncs a local configuration value back to the multi-tenant Enterprise Vault in the Cloud via an MCP interface.",
			InputSchema: map[string]interface{}{"type": "object", "properties": map[string]interface{}{"tenant_id": map[string]interface{}{"type": "string"},"agent_id": map[string]interface{}{"type": "string"},"key": map[string]interface{}{"type": "string"},"value": map[string]interface{}{"type": "string"},"metadata": map[string]interface{}{"type": "object"}}, "required": []interface{}{"tenant_id","agent_id","key","value"}},
		},
		execute: func(ctx context.Context, workDir string, input map[string]interface{}) (string, error) {
			tenantID, _ := input["tenant_id"].(string)
			agentID, _ := input["agent_id"].(string)
			keyVal, _ := input["key"].(string)
			val, _ := input["value"].(string)
            metaMap := make(map[string]string)
            if m, ok := input["metadata"].(map[string]interface{}); ok {
                for k, v := range m {
                    if vs, ok := v.(string); ok {
                        metaMap[k] = vs
                    }
                }
            }

            payload := ConfigSyncPayload{
                TenantID: tenantID,
                AgentID:  agentID,
                Key:      keyVal,
                Value:    val,
                Metadata: metaMap,
            }

			if err := t.SyncConfigToCloud(ctx, payload); err != nil {
				return "", err
			}
			return "Successfully synced config to cloud", nil
		},
	}
}
