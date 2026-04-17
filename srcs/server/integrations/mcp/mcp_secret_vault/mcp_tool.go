package mcp_secret_vault

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
)

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

// ListTools returns the list of available tools.
func (v *MCPSecretVault) ListTools() []Tool {
	return []Tool{
		{
			Name:        "get_secret",
			Description: "Retrieves a secret by key",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"key": {"type": "string"}, "tenant_id": {"type": "string"}}, "required": ["key"]}`),
		},
		{
			Name:        "set_secret",
			Description: "Stores or updates a secret securely",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"key": {"type": "string"}, "value": {"type": "string"}, "tenant_id": {"type": "string"}}, "required": ["key", "value"]}`),
		},
	}
}

// CallTool executes a tool by name.
func (v *MCPSecretVault) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	if toolName == "get_secret" {
		key, ok := arguments["key"].(string)
		if !ok || key == "" {
			return nil, errors.New("missing or invalid 'key' argument")
		}
		tenantID, _ := arguments["tenant_id"].(string)

		secret, err := v.GetSecret(ctx, key, tenantID)
		if err != nil {
			return nil, err
		}

		return map[string]interface{}{"secret": secret}, nil
	}

	if toolName == "set_secret" {
		key, ok := arguments["key"].(string)
		if !ok || key == "" {
			return nil, errors.New("missing or invalid 'key' argument")
		}
		value, ok := arguments["value"].(string)
		if !ok || value == "" {
			return nil, errors.New("missing or invalid 'value' argument")
		}
		tenantID, _ := arguments["tenant_id"].(string)

		err := v.SetSecret(ctx, key, value, tenantID)
		if err != nil {
			return nil, err
		}

		return map[string]interface{}{"status": "success"}, nil
	}

	return nil, fmt.Errorf("unknown tool: %s", toolName)
}
