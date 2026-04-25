package vectorragmcp

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"

	"github.com/onehumancorp/mono/src/server/db"
)

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

// VectorRAGMCP implements the MCP interface for vector RAG operations.
type VectorRAGMCP struct {
	provider db.Provider
}

// NewVectorRAGMCP creates a new VectorRAGMCP instance.
func NewVectorRAGMCP(provider db.Provider) *VectorRAGMCP {
	return &VectorRAGMCP{
		provider: provider,
	}
}

// ListTools returns the list of available tools.
func (m *VectorRAGMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "semantic_search",
			Description: "Performs semantic search across memories based on the database backend.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"query": {"type": "string"}, "limit": {"type": "integer"}, "organization_id": {"type": "string"}}, "required": ["query", "organization_id"]}`),
		},
	}
}

// CallTool executes a tool by name.
func (m *VectorRAGMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	switch toolName {
	case "semantic_search":
		query, ok := arguments["query"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'query' argument")
		}
		orgID, ok := arguments["organization_id"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'organization_id' argument")
		}

		limitFloat, ok := arguments["limit"].(float64)
		limit := 5
		if ok {
			limit = int(limitFloat)
		}

		results, err := m.provider.SearchMemories(ctx, orgID, query, limit)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"results": results}, nil
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}
