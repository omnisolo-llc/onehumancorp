package hybridfsmcp

import (
	"context"
	"fmt"
)

// MCP tool names
const (
	ToolReadFile  = "read_file"
	ToolWriteFile = "write_file"
	ToolListDir   = "list_directory"
)

// Server exposes filesystem operations as MCP tools
type Server struct {
	provider FileSystemProvider
}

func NewServer(provider FileSystemProvider) *Server {
	return &Server{provider: provider}
}

// CallTool executes a tool with the given name and arguments.
func (s *Server) CallTool(ctx context.Context, name string, args map[string]interface{}) (interface{}, error) {
	switch name {
	case ToolReadFile:
		path, ok := args["path"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid 'path' argument")
		}
		data, err := s.provider.ReadFile(ctx, path)
		if err != nil {
			return nil, err
		}
		return string(data), nil

	case ToolWriteFile:
		path, ok := args["path"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid 'path' argument")
		}
		content, ok := args["content"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid 'content' argument")
		}
		err := s.provider.WriteFile(ctx, path, []byte(content))
		if err != nil {
			return nil, err
		}
		return "Successfully wrote file", nil

	case ToolListDir:
		path, ok := args["path"].(string)
		if !ok {
			return nil, fmt.Errorf("missing or invalid 'path' argument")
		}
		entries, err := s.provider.ListDir(ctx, path)
		if err != nil {
			return nil, err
		}
		var names []string
		for _, e := range entries {
			if e.IsDir() {
				names = append(names, e.Name()+"/")
			} else {
				names = append(names, e.Name())
			}
		}
		return names, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", name)
	}
}
