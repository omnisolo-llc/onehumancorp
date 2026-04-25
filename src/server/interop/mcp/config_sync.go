package mcp

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"github.com/onehumancorp/mono/src/server/auth"
)

type ConfigSyncTool struct {
	proxy *McpSyncProxy
}

func NewConfigSyncTool(proxy *McpSyncProxy) *ConfigSyncTool {
	return &ConfigSyncTool{proxy: proxy}
}

// ConfigPayload represents the expected structure for a configuration sync.
type ConfigPayload struct {
	ConfigData map[string]interface{} `json:"config_data"`
	Hash       string                 `json:"hash"`
}

// Execute triggers the configuration sync operation.
func (t *ConfigSyncTool) Execute(ctx context.Context, config map[string]interface{}, direction string) error {
	// Validate JSON Schema structure and depth to respect limits
	if err := t.validatePayload(config); err != nil {
		return fmt.Errorf("config validation failed: %w", err)
	}

	// Encrypt sensitive tokens recursively in config map
	configEncrypted := t.encryptSensitive(config)

	// Generate hash for the config state
	configBytes, err := json.Marshal(configEncrypted)
	if err != nil {
		return fmt.Errorf("failed to marshal config: %w", err)
	}

	hash := sha256.Sum256(configBytes)
	hashStr := hex.EncodeToString(hash[:])

	payload := map[string]interface{}{
		"config":         configEncrypted,
		"hash":           hashStr,
		"sync_direction": direction,
	}

	_, err = t.proxy.BufferIntegrationState(ctx, "mcp_config_sync", payload)
	if err != nil {
		return fmt.Errorf("failed to buffer config sync state: %w", err)
	}

	return t.proxy.SyncPendingStates(ctx)
}

func (t *ConfigSyncTool) validatePayload(config map[string]interface{}) error {
	var walk func(v interface{}) error
	walk = func(v interface{}) error {
		switch val := v.(type) {
		case map[string]interface{}:
			for k, vv := range val {
				if len(k) > 256 {
					return fmt.Errorf("key too long: %s", k)
				}
				if err := walk(vv); err != nil {
					return err
				}
			}
		case []interface{}:
			for _, item := range val {
				if err := walk(item); err != nil {
					return err
				}
			}
		case string:
			if len(val) > 10240 { // 10KB string limit
				return fmt.Errorf("value too long")
			}
		}
		return nil
	}
	return walk(config)
}

func (t *ConfigSyncTool) encryptSensitive(config map[string]interface{}) map[string]interface{} {
	var walk func(key string, v interface{}) interface{}
	walk = func(key string, v interface{}) interface{} {
		switch val := v.(type) {
		case map[string]interface{}:
			out := make(map[string]interface{})
			for k, vv := range val {
				out[k] = walk(k, vv)
			}
			return out
		case []interface{}:
			out := make([]interface{}, len(val))
			for i, item := range val {
				out[i] = walk(key, item) // Pass parent key down to array elements
			}
			return out
		case string:
			if key == "local_proxy_password" || key == "api_key" || key == "secret" {
				return auth.EncryptDeterministic(val)
			}
			return val
		default:
			return val
		}
	}

	result, _ := walk("", config).(map[string]interface{})
	return result
}

func (t *ConfigSyncTool) decryptSensitive(config map[string]interface{}) map[string]interface{} {
	var walk func(key string, v interface{}) interface{}
	walk = func(key string, v interface{}) interface{} {
		switch val := v.(type) {
		case map[string]interface{}:
			out := make(map[string]interface{})
			for k, vv := range val {
				out[k] = walk(k, vv)
			}
			return out
		case []interface{}:
			out := make([]interface{}, len(val))
			for i, item := range val {
				out[i] = walk(key, item)
			}
			return out
		case string:
			if key == "local_proxy_password" || key == "api_key" || key == "secret" {
				return auth.DecryptDeterministic(val)
			}
			return val
		default:
			return val
		}
	}

	result, _ := walk("", config).(map[string]interface{})
	return result
}

// GetHash calculates the hash of the current configuration.
func (t *ConfigSyncTool) GetHash(config map[string]interface{}) (string, error) {
	configBytes, err := json.Marshal(config)
	if err != nil {
		return "", fmt.Errorf("failed to marshal config: %w", err)
	}

	hash := sha256.Sum256(configBytes)
	return hex.EncodeToString(hash[:]), nil
}
