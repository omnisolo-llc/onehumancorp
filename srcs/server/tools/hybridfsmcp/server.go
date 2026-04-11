package hybridfsmcp

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"time"
)

// Server implements the MCP interface for file system access.
type Server struct {
	provider FileSystemProvider
}

// NewServer creates a new Server instance.
func NewServer(provider FileSystemProvider) *Server {
	return &Server{
		provider: provider,
	}
}

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	// In MCP Go server implementations, ensure the InputSchema field within the Tool struct is of type json.RawMessage rather than string so that JSON payloads are correctly encoded as nested objects rather than escaped strings.
	InputSchema json.RawMessage `json:"inputSchema"`
}

// ListTools returns the list of available tools.
func (s *Server) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads the contents of a file.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`),
		},
		{
			Name:        "write_file",
			Description: "Writes data to a file.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}, "data": {"type": "string"}}, "required": ["path", "data"]}`),
		},
		{
			Name:        "list_directory",
			Description: "Lists the contents of a directory.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`),
		},
	}
}

// CallTool executes a tool by name.
func (s *Server) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	switch toolName {
	case "read_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		return s.readFile(ctx, path)
	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		data, ok := arguments["data"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'data' argument")
		}
		return s.writeFile(ctx, path, []byte(data))
	case "list_directory":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		return s.listDirectory(ctx, path)
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

func (s *Server) readFile(ctx context.Context, path string) (interface{}, error) {
	data, err := s.provider.ReadFile(ctx, path)
	if err != nil {
		return nil, err
	}

	mode := "cloud"
	if s.provider.IsLocal() {
		mode = "standalone"
	}

	return map[string]interface{}{
		"status": "success",
		"mode":   mode,
		"data":   string(data),
	}, nil
}

func (s *Server) writeFile(ctx context.Context, path string, data []byte) (interface{}, error) {
	err := s.provider.WriteFile(ctx, path, data)
	if err != nil {
		return nil, err
	}

	mode := "cloud"
	if s.provider.IsLocal() {
		mode = "standalone"
	}

	return map[string]interface{}{
		"status": "success",
		"mode":   mode,
	}, nil
}

func (s *Server) listDirectory(ctx context.Context, path string) (interface{}, error) {
	infos, err := s.provider.ListDir(ctx, path)
	if err != nil {
		return nil, err
	}

	var results []map[string]interface{}
	for _, info := range infos {
		results = append(results, map[string]interface{}{
			"name":          info.Name,
			"size":          info.Size,
			"is_dir":        info.IsDir,
			"last_modified": info.LastModified.Format(time.RFC3339),
		})
	}

	mode := "cloud"
	if s.provider.IsLocal() {
		mode = "standalone"
	}

	return map[string]interface{}{
		"status":  "success",
		"mode":    mode,
		"results": results,
	}, nil
}
