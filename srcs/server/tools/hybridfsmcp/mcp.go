package hybridfsmcp

import (
	"context"
	"encoding/base64"
	"errors"
	"fmt"
)

type FSInspectorMCP struct {
	provider FileSystemProvider
}

func NewFSInspectorMCP(provider FileSystemProvider) *FSInspectorMCP {
	return &FSInspectorMCP{provider: provider}
}

type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

func (m *FSInspectorMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads the contents of a file.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "write_file",
			Description: "Writes content to a file. Content should be base64 encoded.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "content_b64": {"type": "string"}}, "required": ["path", "content_b64"]}`,
		},
		{
			Name:        "list_directory",
			Description: "Lists contents of a directory.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
	}
}

func (m *FSInspectorMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	switch toolName {
	case "read_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		data, err := m.provider.ReadFile(ctx, path)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success", "content_b64": base64.StdEncoding.EncodeToString(data)}, nil
	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		contentB64, ok := arguments["content_b64"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'content_b64' argument")
		}
		data, err := base64.StdEncoding.DecodeString(contentB64)
		if err != nil {
			return nil, err
		}
		if err := m.provider.WriteFile(ctx, path, data); err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success"}, nil
	case "list_directory":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		entries, err := m.provider.ListDir(ctx, path)
		if err != nil {
			return nil, err
		}
		var results []map[string]interface{}
		for _, e := range entries {
			results = append(results, map[string]interface{}{
				"name":   e.Name,
				"is_dir": e.IsDir,
				"size":   e.Size,
			})
		}
		return map[string]interface{}{"status": "success", "entries": results}, nil
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
