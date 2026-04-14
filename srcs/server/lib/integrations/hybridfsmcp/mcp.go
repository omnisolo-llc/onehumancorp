package hybridfsmcp

import (
	"context"
	"errors"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

type HybridFSMCP struct {
	provider FileSystemProvider
	escalator Escalator
}

func NewHybridFSMCP(provider FileSystemProvider, escalator Escalator) *HybridFSMCP {
	return &HybridFSMCP{provider: provider, escalator: escalator}
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
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}},
		"required": ["path"]}`,
		},
		{
			Name:        "rag_query",
			Description: "Performs a RAG query on local documents. May escalate to cloud for complex queries.",
			InputSchema: `{"type": "object", "properties": {"query": {"type": "string"}}, "required": ["query"]}`,
		},
	}
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
			telemetry.RecordRAGEscalation(ctx)

			// Simulate Cloud pgvector swarm execution
			return map[string]interface{}{"status": "success", "mode": "cloud_escalated", "result": "Cloud aggregated results for: " + query}, nil
		}

		// Simulate Local SQLite Vector DB execution
		return map[string]interface{}{"status": "success", "mode": "local", "result": "Local results for: " + query}, nil
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
