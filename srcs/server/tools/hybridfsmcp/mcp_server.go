package hybridfsmcp

import (
	"context"
	"encoding/json"
	"fmt"
)

type MCPServer struct {
	provider FileSystemProvider
}

func NewMCPServer(provider FileSystemProvider) *MCPServer {
	return &MCPServer{provider: provider}
}

type ReadFileArgs struct {
	Path string `json:"path"`
}

type WriteFileArgs struct {
	Path    string `json:"path"`
	Content string `json:"content"`
}

type ListDirArgs struct {
	Path string `json:"path"`
}

func (s *MCPServer) ExecuteTool(ctx context.Context, toolName string, argsJSON json.RawMessage) (interface{}, error) {
	switch toolName {
	case "read_file":
		var args ReadFileArgs
		if err := json.Unmarshal(argsJSON, &args); err != nil {
			return nil, fmt.Errorf("invalid arguments: %w", err)
		}
		content, err := s.provider.ReadFile(ctx, args.Path)
		if err != nil {
			return nil, err
		}
		return string(content), nil
	case "write_file":
		var args WriteFileArgs
		if err := json.Unmarshal(argsJSON, &args); err != nil {
			return nil, fmt.Errorf("invalid arguments: %w", err)
		}
		err := s.provider.WriteFile(ctx, args.Path, []byte(args.Content))
		if err != nil {
			return nil, err
		}
		return "File written successfully", nil
	case "list_directory":
		var args ListDirArgs
		if err := json.Unmarshal(argsJSON, &args); err != nil {
			return nil, fmt.Errorf("invalid arguments: %w", err)
		}
		entries, err := s.provider.ListDir(ctx, args.Path)
		if err != nil {
			return nil, err
		}
        var names []string
        for _, entry := range entries {
            if entry.IsDir() {
                names = append(names, entry.Name()+"/")
            } else {
                names = append(names, entry.Name())
            }
        }
		return names, nil
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
