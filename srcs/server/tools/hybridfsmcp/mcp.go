package hybridfsmcp

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

// HybridFSMCP implements the MCP interface for file system operations.
type HybridFSMCP struct {
	provider FileSystemProvider
}

// NewHybridFSMCP creates a new HybridFSMCP instance, choosing the provider based on the environment.
func NewHybridFSMCP() (*HybridFSMCP, error) {
	var provider FileSystemProvider
	var err error

	if os.Getenv("OHC_MULTITENANT") == "true" {
		provider, err = NewCloudFSProvider()
	} else {
		provider, err = NewLocalFSProvider()
	}

	if err != nil {
		return nil, fmt.Errorf("failed to initialize FS provider: %w", err)
	}

	return &HybridFSMCP{provider: provider}, nil
}

// ListTools returns the list of available tools.
func (m *HybridFSMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads the content of a file.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`),
		},
		{
			Name:        "write_file",
			Description: "Writes content to a file.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`),
		},
		{
			Name:        "list_directory",
			Description: "Lists the contents of a directory.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`),
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	claims := auth.ClaimsFromContext(ctx)
	// Cloud provider requires claims, local provider does not strictly require them but accepts them
	if os.Getenv("OHC_MULTITENANT") == "true" && claims == nil {
		return nil, errors.New("unauthorized: missing claims for multitenant mode")
	}

	switch toolName {
	case "read_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		data, err := m.provider.ReadFile(ctx, claims, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success", "content": string(data)}, nil

	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		contentStr, ok := arguments["content"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'content' argument")
		}

		var contentBytes []byte
		// Handle potential base64 or raw string depending on how it's sent. Let's assume raw string for now based on schema.
		contentBytes = []byte(contentStr)

		err := m.provider.WriteFile(ctx, claims, path, contentBytes)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success"}, nil

	case "list_directory":
		path, ok := arguments["path"].(string)
		if !ok {
			// default to root
			path = "."
		}
		entries, err := m.provider.ListDir(ctx, claims, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success", "entries": entries}, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

func (m *HybridFSMCP) HandleRequest(ctx context.Context, req []byte) ([]byte, error) {
	var request map[string]interface{}
	if err := json.Unmarshal(req, &request); err != nil {
		return nil, fmt.Errorf("invalid request json: %w", err)
	}

	method, _ := request["method"].(string)
	id := request["id"] // JSON-RPC id

	if method == "tools/list" {
		tools := m.ListTools()
		return json.Marshal(map[string]interface{}{
			"jsonrpc": "2.0",
			"id":      id,
			"result": map[string]interface{}{
				"tools": tools,
			},
		})
	} else if method == "tools/call" {
		params, _ := request["params"].(map[string]interface{})
		name, _ := params["name"].(string)
		args, _ := params["arguments"].(map[string]interface{})

		res, err := m.CallTool(ctx, name, args)
		if err != nil {
			return json.Marshal(map[string]interface{}{
				"jsonrpc": "2.0",
				"id":      id,
				"error": map[string]interface{}{
					"code":    -32000,
					"message": err.Error(),
				},
			})
		}

		resJSON, _ := json.Marshal(res)
		return json.Marshal(map[string]interface{}{
			"jsonrpc": "2.0",
			"id":      id,
			"result": map[string]interface{}{
				"content": []map[string]interface{}{
					{"type": "text", "text": string(resJSON)},
				},
			},
		})
	}

	return nil, fmt.Errorf("unsupported method %s", method)
}