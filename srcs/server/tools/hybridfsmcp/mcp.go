package hybridfsmcp

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
)

// HybridFSMCP implements the MCP interface for hybrid file system access.
type HybridFSMCP struct {
	provider FileSystemProvider
}

// NewHybridFSMCP creates a new HybridFSMCP instance.
func NewHybridFSMCP(provider FileSystemProvider) *HybridFSMCP {
	return &HybridFSMCP{
		provider: provider,
	}
}

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

// ListTools returns the list of available tools.
func (m *HybridFSMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads the contents of a file.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "write_file",
			Description: "Writes content to a file.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`,
		},
		{
			Name:        "list_directory",
			Description: "Lists files and directories in a path.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "search_files",
			Description: "Searches for files matching a glob pattern.",
			InputSchema: `{"type": "object", "properties": {"pattern": {"type": "string"}}, "required": ["pattern"]}`,
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	switch toolName {
	case "read_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		return m.readFile(ctx, path)
	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		content, ok := arguments["content"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'content' argument")
		}
		return m.writeFile(ctx, path, []byte(content))
	case "list_directory":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		return m.listDirectory(ctx, path)
	case "search_files":
		pattern, ok := arguments["pattern"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'pattern' argument")
		}
		return m.searchFiles(ctx, pattern)
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

func (m *HybridFSMCP) getMode() string {
	if m.provider.IsLocal() {
		return "standalone"
	}
	return "cloud"
}

func (m *HybridFSMCP) readFile(ctx context.Context, path string) (interface{}, error) {
	data, err := m.provider.ReadFile(ctx, path)
	if err != nil {
		return nil, err
	}

	// Check if data is valid UTF-8, else we might want to base64 encode it
	// For this proxy, we'll assume string cast is fine for now
	content := string(data)

	return map[string]interface{}{
		"status":  "success",
		"mode":    m.getMode(),
		"content": content,
	}, nil
}

func (m *HybridFSMCP) writeFile(ctx context.Context, path string, data []byte) (interface{}, error) {
	err := m.provider.WriteFile(ctx, path, data)
	if err != nil {
		return nil, err
	}

	return map[string]interface{}{
		"status": "success",
		"mode":   m.getMode(),
		"path":   path,
	}, nil
}

func (m *HybridFSMCP) listDirectory(ctx context.Context, path string) (interface{}, error) {
	entries, err := m.provider.ListDir(ctx, path)
	if err != nil {
		return nil, err
	}

	return map[string]interface{}{
		"status":  "success",
		"mode":    m.getMode(),
		"entries": entries,
	}, nil
}

func (m *HybridFSMCP) searchFiles(ctx context.Context, pattern string) (interface{}, error) {
	matches, err := m.provider.SearchFiles(ctx, pattern)
	if err != nil {
		return nil, err
	}

	return map[string]interface{}{
		"status":  "success",
		"mode":    m.getMode(),
		"matches": matches,
	}, nil
}

// ExecutionResult is referenced in memories, keeping format consistent if needed
func FormatExecutionResult(toolID string, status string, resultData []byte, escalation bool) map[string]interface{} {
	return map[string]interface{}{
		"tool_id":           toolID,
		"status":            status,
		"result_data":       json.RawMessage(resultData),
		"hybrid_escalation": escalation,
		"escalation":        escalation,
	}
}
