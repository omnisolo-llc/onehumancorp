package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"
	"os"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
	"github.com/onehumancorp/mono/srcs/server/auth"
)

type HybridFSMCP struct {
	provider FileSystemProvider
}

func NewHybridFSMCP() *HybridFSMCP {
	var provider FileSystemProvider
	if os.Getenv("OHC_MULTITENANT") == "true" {
		provider = NewCloudFSProvider()
	} else {
		provider = NewLocalFSProvider()
	}
	return &HybridFSMCP{provider: provider}
}

type Tool struct {
	Name        string      `json:"name"`
	Description string      `json:"description"`
	InputSchema interface{} `json:"inputSchema"`
}

func (m *HybridFSMCP) ListTools() []Tool {

	return []Tool{
		{
			Name:        "read_file",
			Description: "Read the contents of a file.",
			InputSchema: map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"path": map[string]interface{}{
						"type":        "string",
						"description": "The path to the file to read.",
					},
				},
				"required": []string{"path"},
			},
		},
		{
			Name:        "write_file",
			Description: "Write content to a file.",
			InputSchema: map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"path": map[string]interface{}{
						"type":        "string",
						"description": "The path to the file to write.",
					},
					"content": map[string]interface{}{
						"type":        "string",
						"description": "The plain text content to write to the file. This should not be encoded in base64 unless the file is explicitly a base64 encoded file.",
					},
				},
				"required": []string{"path", "content"},
			},
		},
		{
			Name:        "list_directory",
			Description: "List the contents of a directory.",
			InputSchema: map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"path": map[string]interface{}{
						"type":        "string",
						"description": "The path to the directory to list.",
					},
				},
				"required": []string{"path"},
			},
		},
		{
			Name:        "search_files",
			Description: "Search for files by pattern within a directory.",
			InputSchema: map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"path": map[string]interface{}{
						"type":        "string",
						"description": "The base path to search within.",
					},
					"pattern": map[string]interface{}{
						"type":        "string",
						"description": "The pattern to search for in filenames.",
					},
				},
				"required": []string{"path", "pattern"},
			},
		},
	}
}

func (m *HybridFSMCP) ExecuteTool(ctx context.Context, toolName string, args interface{}) (*mcp.ExecutionResult, error) {
	claims := auth.ClaimsFromContext(ctx)

	argsMap, ok := args.(map[string]interface{})
	if !ok {
		return mcp.FormatExecutionResult(toolName, "error", []byte("invalid arguments format"), false), fmt.Errorf("invalid arguments format")
	}

	switch toolName {
	case "read_file":
		path, _ := argsMap["path"].(string)
		if path == "" {
			return mcp.FormatExecutionResult(toolName, "error", []byte("missing path"), false), fmt.Errorf("missing path")
		}
		data, err := m.provider.ReadFile(ctx, claims, path)
		if err != nil {
			return mcp.FormatExecutionResult(toolName, "error", []byte(err.Error()), false), err
		}
		return mcp.FormatExecutionResult(toolName, "success", data, false), nil

	case "write_file":
		path, _ := argsMap["path"].(string)
		contentStr, _ := argsMap["content"].(string)
		if path == "" {
			return mcp.FormatExecutionResult(toolName, "error", []byte("missing path"), false), fmt.Errorf("missing path")
		}
		err := m.provider.WriteFile(ctx, claims, path, []byte(contentStr))
		if err != nil {
			return mcp.FormatExecutionResult(toolName, "error", []byte(err.Error()), false), err
		}
		resBytes, _ := json.Marshal(map[string]string{"status": "file written successfully"})
		return mcp.FormatExecutionResult(toolName, "success", resBytes, false), nil

	case "list_directory":
		path, _ := argsMap["path"].(string)
		if path == "" {
			return mcp.FormatExecutionResult(toolName, "error", []byte("missing path"), false), fmt.Errorf("missing path")
		}
		entries, err := m.provider.ListDir(ctx, claims, path)
		if err != nil {
			return mcp.FormatExecutionResult(toolName, "error", []byte(err.Error()), false), err
		}
		resBytes, _ := json.Marshal(entries)
		return mcp.FormatExecutionResult(toolName, "success", resBytes, false), nil

	case "search_files":
		path, _ := argsMap["path"].(string)
		pattern, _ := argsMap["pattern"].(string)
		if path == "" || pattern == "" {
			return mcp.FormatExecutionResult(toolName, "error", []byte("missing path or pattern"), false), fmt.Errorf("missing path or pattern")
		}
		matches, err := m.provider.SearchFiles(ctx, claims, path, pattern)
		if err != nil {
			return mcp.FormatExecutionResult(toolName, "error", []byte(err.Error()), false), err
		}
		resBytes, _ := json.Marshal(matches)
		return mcp.FormatExecutionResult(toolName, "success", resBytes, false), nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
