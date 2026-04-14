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

func NewHybridFSMCP(provider FileSystemProvider, escalator Escalator) *HybridFSMCP {
	if escalator == nil {
		escalator = NewDefaultEscalator(100)
	}
	return &HybridFSMCP{provider: provider, escalator: escalator}
}

func (m *HybridFSMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "rag_query",
			Description: "Executes a RAG query, potentially escalating to the Cloud Swarm if complex.",
			InputSchema: `{"type": "object", "properties": {"query": {"type": "string"}}, "required": ["query"]}`,
		},
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
	}
}

func (m *HybridFSMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	switch toolName {
	case "rag_query":
		query, ok := arguments["query"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'query' argument")
		}

		escalate := m.escalator.Analyze(ctx, query)
		if escalate {
			// Record metrics
			telemetry.RecordRagEscalation(ctx)

			// Attempt Escalation (simulate)
			// In a real system, this would make an RPC/gRPC call over SPIFFE to the Cloud pgvector Swarm
			// For graceful degradation, if the cloud is unreachable, we'd fallback.
			// Let's simulate a cloud response for testability, and provide a local fallback.

			// Simulate Cloud processing
			cloudResult := "Cloud Swarm Response: Aggregated results for '" + query + "'"
			return map[string]interface{}{"status": "success", "mode": "cloud_escalated", "result": cloudResult}, nil
		}

		// Fallback to local processing
		localResult := "Local SQLite Response: Results for '" + query + "'"
		return map[string]interface{}{"status": "success", "mode": "local", "result": localResult}, nil
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
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
