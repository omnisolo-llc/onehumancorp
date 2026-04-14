package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"
)

type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

type HybridFSMCP struct {
	provider  FileSystemProvider
	escalator Escalator
}

func NewHybridFSMCP(provider FileSystemProvider, escalator Escalator) *HybridFSMCP {
	return &HybridFSMCP{
		provider:  provider,
		escalator: escalator,
	}
}

func (m *HybridFSMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "read_file",
			Description: "Reads the contents of a file.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "write_file",
			Description: "Writes data to a file.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}, "content": {"type": "string"}}, "required": ["path", "content"]}`,
		},
		{
			Name:        "list_directory",
			Description: "Lists the contents of a directory.",
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "rag_query",
			Description: "Queries the vector database.",
			InputSchema: `{"type": "object", "properties": {"query": {"type": "string"}}, "required": ["query"]}`,
		},
	}
}

func (m *HybridFSMCP) processLocalQuery(ctx context.Context, query string) (interface{}, error) {
	// In a real implementation, this would query the local SQLite Vector DB
	return map[string]interface{}{"status": "success", "source": "local", "result": "local_result"}, nil
}

func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
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
		return map[string]interface{}{"status": "success", "content": string(data)}, nil
	case "write_file":
		path, ok := arguments["path"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'path' argument")
		}
		content, ok := arguments["content"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'content' argument")
		}
		err := m.provider.WriteFile(ctx, path, []byte(content))
		if err != nil {
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
		return map[string]interface{}{"status": "success", "entries": entries}, nil
	case "rag_query":
		query, ok := arguments["query"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'query' argument")
		}

		if m.escalator != nil && m.escalator.ShouldEscalate(ctx, query) {
			res, err := m.escalator.Escalate(ctx, query)
			if err != nil {
				// Fallback to local processing if cloud is unreachable
				return m.processLocalQuery(ctx, query)
			}
			return map[string]interface{}{"status": "success", "source": "cloud", "result": res}, nil
		}
		return m.processLocalQuery(ctx, query)
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
