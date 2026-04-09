package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
)

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

// ToolCallResponse represents the response from an MCP tool call.
type ToolCallResponse struct {
	Result string `json:"result"`
}

// HybridFSServer implements the MCP interface for FileSystem operations
type HybridFSServer struct {
	Provider FileSystemProvider
}

// NewHybridFSServer creates an MCP server matching the environment mode (OHC_MULTITENANT vs standalone)
func NewHybridFSServer(baseDir string) (*HybridFSServer, error) {
	var provider FileSystemProvider
	var err error

	if os.Getenv("OHC_MULTITENANT") == "true" {
		provider, err = NewCloudFSProvider(baseDir)
	} else {
		// Standalone mode is default
		provider, err = NewLocalFSProvider(baseDir)
	}

	if err != nil {
		return nil, err
	}

	return &HybridFSServer{Provider: provider}, nil
}

func (s *HybridFSServer) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads the content of a file in the Hybrid FS.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string", "description": "Path to the file to read"}}, "required": ["path"]}`,
		},
		{
			Name:        "write_file",
			Description: "Writes content to a file in the Hybrid FS.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string", "description": "Path to the file to write"}, "content": {"type": "string", "description": "Content to write"}}, "required": ["path", "content"]}`,
		},
		{
			Name:        "list_directory",
			Description: "Lists files and directories in a given path.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string", "description": "Path to the directory to list"}}, "required": ["path"]}`,
		},
	}
}

func (s *HybridFSServer) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (*ToolCallResponse, error) {
	switch toolName {
	case "read_file":
		pathIf, ok := arguments["path"]
		if !ok {
			return nil, fmt.Errorf("missing argument: path")
		}
		path, ok := pathIf.(string)
		if !ok {
			return nil, fmt.Errorf("invalid type for argument: path")
		}
		data, err := s.Provider.ReadFile(ctx, path)
		if err != nil {
			return nil, err
		}
		return &ToolCallResponse{
			Result: string(data),
		}, nil

	case "write_file":
		pathIf, ok := arguments["path"]
		if !ok {
			return nil, fmt.Errorf("missing argument: path")
		}
		path, ok := pathIf.(string)
		if !ok {
			return nil, fmt.Errorf("invalid type for argument: path")
		}
		contentIf, ok := arguments["content"]
		if !ok {
			return nil, fmt.Errorf("missing argument: content")
		}
		content, ok := contentIf.(string)
		if !ok {
			return nil, fmt.Errorf("invalid type for argument: content")
		}
		err := s.Provider.WriteFile(ctx, path, []byte(content))
		if err != nil {
			return nil, err
		}
		return &ToolCallResponse{
			Result: fmt.Sprintf("Successfully wrote %d bytes to %s", len(content), path),
		}, nil

	case "list_directory":
		pathIf, ok := arguments["path"]
		if !ok {
			return nil, fmt.Errorf("missing argument: path")
		}
		path, ok := pathIf.(string)
		if !ok {
			return nil, fmt.Errorf("invalid type for argument: path")
		}
		entries, err := s.Provider.ListDir(ctx, path)
		if err != nil {
			return nil, err
		}
		out, _ := json.Marshal(entries)
		return &ToolCallResponse{
			Result: string(out),
		}, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
