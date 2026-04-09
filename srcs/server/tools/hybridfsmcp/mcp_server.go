package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
)

type Server struct {
	provider FileSystemProvider
}

func NewServer() *Server {
	var provider FileSystemProvider

	if os.Getenv("OHC_MULTITENANT") == "true" {
		provider = &CloudFSProvider{BaseDir: "/data/workspace"}
	} else {
		// Standalone mode default workspace
		root := os.Getenv("WORKSPACE_ROOT")
		if root == "" {
			root = "."
		}
		provider = &LocalFSProvider{WorkspaceRoot: root}
	}

	return &Server{provider: provider}
}

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

func (s *Server) ListTools(ctx context.Context) ([]Tool, error) {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads the content of a file",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`),
		},
		{
			Name:        "write_file",
			Description: "Writes content to a file",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`),
		},
		{
			Name:        "list_directory",
			Description: "Lists contents of a directory",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`),
		},
	}, nil
}

// Result struct to mimic MCP CallTool signature
type CallToolResult struct {
	IsError bool          `json:"isError"`
	Content []ContentData `json:"content"`
}

type ContentData struct {
	Type string `json:"type"`
	Text string `json:"text"`
}

func (s *Server) CallTool(ctx context.Context, name string, arguments json.RawMessage) (*CallToolResult, error) {
	var args map[string]interface{}
	if err := json.Unmarshal(arguments, &args); err != nil {
		return nil, fmt.Errorf("invalid arguments: %w", err)
	}

	switch name {
	case "read_file":
		path, ok := args["path"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid path")
		}

		content, err := s.provider.ReadFile(ctx, path)
		if err != nil {
			return &CallToolResult{IsError: true, Content: []ContentData{{Type: "text", Text: err.Error()}}}, nil
		}
		return &CallToolResult{Content: []ContentData{{Type: "text", Text: string(content)}}}, nil

	case "write_file":
		path, ok := args["path"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid path")
		}
		contentStr, ok := args["content"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid content")
		}

		err := s.provider.WriteFile(ctx, path, []byte(contentStr))
		if err != nil {
			return &CallToolResult{IsError: true, Content: []ContentData{{Type: "text", Text: err.Error()}}}, nil
		}
		return &CallToolResult{Content: []ContentData{{Type: "text", Text: "File written successfully"}}}, nil

	case "list_directory":
		path, ok := args["path"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid path")
		}

		entries, err := s.provider.ListDir(ctx, path)
		if err != nil {
			return &CallToolResult{IsError: true, Content: []ContentData{{Type: "text", Text: err.Error()}}}, nil
		}

		out, _ := json.Marshal(entries)
		return &CallToolResult{Content: []ContentData{{Type: "text", Text: string(out)}}}, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", name)
	}
}
