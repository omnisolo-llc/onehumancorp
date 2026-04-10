package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"
)

type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

type FileSystemMCPServer struct {
	provider FileSystemProvider
}

func NewFileSystemMCPServer(provider FileSystemProvider) *FileSystemMCPServer {
	return &FileSystemMCPServer{provider: provider}
}

func (s *FileSystemMCPServer) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads a file from the filesystem",
			InputSchema: json.RawMessage(`{"type":"object","properties":{"path":{"type":"string"}},"required":["path"]}`),
		},
		{
			Name:        "write_file",
			Description: "Writes content to a file",
			InputSchema: json.RawMessage(`{"type":"object","properties":{"path":{"type":"string"},"content":{"type":"string"}},"required":["path","content"]}`),
		},
		{
			Name:        "list_dir",
			Description: "Lists files in a directory",
			InputSchema: json.RawMessage(`{"type":"object","properties":{"path":{"type":"string"}},"required":["path"]}`),
		},
	}
}

func (s *FileSystemMCPServer) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	switch toolName {
	case "read_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid 'path' argument")
		}
		data, err := s.provider.ReadFile(ctx, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success", "content": string(data)}, nil
	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid 'path' argument")
		}
		content, ok := arguments["content"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid 'content' argument")
		}
		err := s.provider.WriteFile(ctx, path, []byte(content))
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success"}, nil
	case "list_dir":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid 'path' argument")
		}
		entries, err := s.provider.ListDir(ctx, path)
		if err != nil {
			return nil, err
		}
		list := []string{}
		for _, e := range entries {
			list = append(list, e.Name())
		}
		return map[string]interface{}{"status": "success", "entries": list}, nil
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
