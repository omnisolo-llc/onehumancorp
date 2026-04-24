package localstatefulproxy

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"

	"github.com/modelcontextprotocol/go-sdk/mcp"
)

// Tool represents an MCP tool definition for integration with standard handler.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

// ProxyTool implements the MCP Tool interface for the Local Stateful Execution Proxy.
type ProxyTool struct{}

// NewProxyTool creates a new instance of the Local Stateful Execution Proxy Tool.
func NewProxyTool() *ProxyTool {
	return &ProxyTool{}
}

// ListTools returns the list of available tools.
func (p *ProxyTool) ListTools() []Tool {
	return []Tool{
		{
			Name:        "local_stateful_proxy",
			Description: "Proxies execution commands and structured queries to the local standalone context. It allows cloud orchestration to securely interact with the local SQLite shard or namespace without exposing the entire local environment.",
			InputSchema: `{"type": "object", "properties": {"command": {"type": "string", "description": "The intended execution action (e.g., specific command or query)."}, "context_id": {"type": "string", "description": "Targeting specific local SQLite shard or namespace."}}, "required": ["command", "context_id"]}`,
		},
	}
}

// CallTool executes a tool by name (required by the backend tool handler).
func (p *ProxyTool) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	if toolName != "local_stateful_proxy" {
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}

	command, ok := arguments["command"].(string)
	if !ok || command == "" {
		return nil, fmt.Errorf("missing or invalid 'command' parameter")
	}

	contextID, ok := arguments["context_id"].(string)
	if !ok || contextID == "" {
		return nil, fmt.Errorf("missing or invalid 'context_id' parameter")
	}

	// In a real implementation, this would relay the command via OHC-SIP to the local standalone instance.
	// For now, we simulate the execution and log the action.
	slog.Info("[LocalStatefulProxy] Proxying command to context", "command", command, "contextID", contextID)

	// Simulate successful response
	responseData := map[string]string{
		"status":  "success",
		"message": fmt.Sprintf("Command '%s' successfully proxied to context '%s'", command, contextID),
	}

	return responseData, nil
}

// Info returns the official MCP SDK tool representation to conform to the MCP go-sdk interface.
func (p *ProxyTool) Info() mcp.Tool {
	return mcp.Tool{
		Name:        "local_stateful_proxy",
		Description: "Proxies execution commands and structured queries to the local standalone context.",
		InputSchema: map[string]interface{}{
			"type": "object",
			"properties": map[string]interface{}{
				"command": map[string]interface{}{
					"type": "string",
				},
				"context_id": map[string]interface{}{
					"type": "string",
				},
			},
			"required": []string{"command", "context_id"},
		},
	}
}

// Execute performs the proxied command against the given context_id (conforms to MCP SDK).
func (p *ProxyTool) Execute(ctx context.Context, args map[string]interface{}) (*mcp.CallToolResult, error) {
	command, ok := args["command"].(string)
	if !ok || command == "" {
		return nil, fmt.Errorf("missing or invalid 'command' parameter")
	}

	contextID, ok := args["context_id"].(string)
	if !ok || contextID == "" {
		return nil, fmt.Errorf("missing or invalid 'context_id' parameter")
	}

	// In a real implementation, this would relay the command via OHC-SIP to the local standalone instance.
	// For now, we simulate the execution and log the action.
	slog.Info("[LocalStatefulProxy] Proxying command to context", "command", command, "contextID", contextID)

	// Simulate successful response
	responseData := map[string]string{
		"status":  "success",
		"message": fmt.Sprintf("Command '%s' successfully proxied to context '%s'", command, contextID),
	}

	responseJSON, err := json.Marshal(responseData)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal response: %v", err)
	}

	// Create mcp.TextContent pointer to satisfy the interface since MarshalJSON has a pointer receiver.
	textContent := &mcp.TextContent{
		Text: string(responseJSON),
	}

	return &mcp.CallToolResult{
		Content: []mcp.Content{
			textContent,
		},
	}, nil
}
