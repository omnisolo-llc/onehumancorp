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
	provider  FileSystemProvider
	escalator Escalator
}

func NewHybridFSMCP(provider FileSystemProvider) *HybridFSMCP {
	return &HybridFSMCP{provider: provider}
}

func NewHybridFSMCPWithEscalator(provider FileSystemProvider, escalator Escalator) *HybridFSMCP {
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
			InputSchema: `{"type": "object", "properties": {"path": {"type": "string"}}, "required": ["path"]}`,
		},
		{
			Name:        "rag_query",
			Description: "Performs a RAG query, potentially escalating to cloud.",
			InputSchema: `{"type": "object", "properties": {"query": {"type": "string"}}, "required": ["query"]}`,
		},
	}
}

func (m *HybridFSMCP) executeRAGLocal(ctx context.Context, query string) (string, error) {
	// Simulated local processing
	return fmt.Sprintf("Local RAG Result for: %s", query), nil
}

func (m *HybridFSMCP) executeRAGCloud(ctx context.Context, query string) (string, error) {
	// In a real scenario, this would use a cloud sync protocol. We mock it or handle fallback here.
	// We'll simulate a failure for testing fallback if context contains a specific value
	if ctx.Value("fail_cloud") != nil {
		return "", errors.New("cloud connection failed")
	}
	return fmt.Sprintf("Cloud RAG Result for: %s", query), nil
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

		if m.escalator != nil && m.escalator.AnalyzeComplexity(ctx, query) {
			telemetry.RecordRagEscalation(ctx)
			result, err := m.executeRAGCloud(ctx, query)
			if err != nil {
				// Fallback
				result, err = m.executeRAGLocal(ctx, query)
				if err != nil {
					return nil, err
				}
				return map[string]interface{}{"status": "success", "source": "local_fallback", "result": result}, nil
			}
			return map[string]interface{}{"status": "success", "source": "cloud", "result": result}, nil
		}

		result, err := m.executeRAGLocal(ctx, query)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success", "source": "local", "result": result}, nil
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
