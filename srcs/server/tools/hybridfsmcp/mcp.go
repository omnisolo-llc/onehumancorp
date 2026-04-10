package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"
	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
	"github.com/onehumancorp/mono/srcs/server/auth"
)

type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

type HybridFSMCP struct {
	provider FileSystemProvider
}

func NewHybridFSMCP(provider FileSystemProvider) *HybridFSMCP {
	return &HybridFSMCP{
		provider: provider,
	}
}

func (m *HybridFSMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Read the contents of a file.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}}`,
		},
		{
			Name:        "write_file",
			Description: "Write contents to a file.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "data": {"type": "string"}}}`,
		},
		{
			Name:        "list_directory",
			Description: "List files and directories in a given path.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}}`,
		},
		{
			Name:        "search_files",
			Description: "Search for files by name.",
			InputSchema: `{"type": "object", "properties": {"query": {"type": "string"}}}`,
		},
	}
}

func (m *HybridFSMCP) CallTool(ctx context.Context, name string, args map[string]interface{}) (*mcp.ExecutionResult, error) {
	claims := auth.ClaimsFromContext(ctx)

	switch name {
	case "read_file":
		path, ok := args["path"].(string)
		if !ok {
			return nil, fmt.Errorf("invalid path argument")
		}
		data, err := m.provider.ReadFile(ctx, claims, path)
		if err != nil {
			return nil, err
		}
		resultData, _ := json.Marshal(map[string]string{"content": string(data)})
		return mcp.FormatExecutionResult("read_file", "success", resultData, false), nil

	case "write_file":
		path, ok := args["path"].(string)
		if !ok {
			return nil, fmt.Errorf("invalid path argument")
		}
		dataStr, ok := args["data"].(string)
		if !ok {
			return nil, fmt.Errorf("invalid data argument")
		}
		err := m.provider.WriteFile(ctx, claims, path, []byte(dataStr))
		if err != nil {
			return nil, err
		}
		resultData, _ := json.Marshal(map[string]string{"status": "written"})
		return mcp.FormatExecutionResult("write_file", "success", resultData, false), nil

	case "list_directory":
		path, ok := args["path"].(string)
		if !ok {
			return nil, fmt.Errorf("invalid path argument")
		}
		infos, err := m.provider.ListDir(ctx, claims, path)
		if err != nil {
			return nil, err
		}
		var names []string
		for _, info := range infos {
			names = append(names, info.Name())
		}
		resultData, _ := json.Marshal(map[string][]string{"files": names})
		return mcp.FormatExecutionResult("list_directory", "success", resultData, false), nil

	case "search_files":
		query, ok := args["query"].(string)
		if !ok {
			return nil, fmt.Errorf("invalid query argument")
		}
		results, err := m.provider.SearchFiles(ctx, claims, query)
		if err != nil {
			return nil, err
		}
		resultData, _ := json.Marshal(map[string][]string{"files": results})
		return mcp.FormatExecutionResult("search_files", "success", resultData, false), nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", name)
	}
}
