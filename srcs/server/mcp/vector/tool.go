package vector

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
)

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string `json:"name"`
	Description string `json:"description"`
	InputSchema string `json:"inputSchema"`
}

// VectorMCP implements the MCP interface for vector storage operations.
type VectorMCP struct {
	provider VectorStorageProvider
}

// NewVectorMCP creates a new VectorMCP instance.
func NewVectorMCP(provider VectorStorageProvider) *VectorMCP {
	return &VectorMCP{
		provider: provider,
	}
}

// ListTools returns the list of available tools.
func (m *VectorMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "vector_store",
			Description: "Stores an embedding vector with associated metadata.",
			InputSchema: `{"type": "object", "properties": {"namespace": {"type": "string"}, "id": {"type": "string"}, "vector": {"type": "array", "items": {"type": "number"}}, "metadata": {"type": "object"}}, "required": ["namespace", "id", "vector"]}`,
		},
		{
			Name:        "vector_search",
			Description: "Retrieves similar vectors based on a query vector.",
			InputSchema: `{"type": "object", "properties": {"namespace": {"type": "string"}, "query_vector": {"type": "array", "items": {"type": "number"}}, "top_k": {"type": "integer"}}, "required": ["namespace", "query_vector", "top_k"]}`,
		},
	}
}

// CallTool executes a tool by name.
func (m *VectorMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	switch toolName {
	case "vector_store":
		return m.handleVectorStore(ctx, arguments)
	case "vector_search":
		return m.handleVectorSearch(ctx, arguments)
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

func (m *VectorMCP) handleVectorStore(ctx context.Context, arguments map[string]interface{}) (interface{}, error) {
	namespace, ok := arguments["namespace"].(string)
	if !ok || namespace == "" {
		return nil, errors.New("missing or invalid 'namespace' argument")
	}

	id, ok := arguments["id"].(string)
	if !ok || id == "" {
		return nil, errors.New("missing or invalid 'id' argument")
	}

	vectorRaw, ok := arguments["vector"].([]interface{})
	if !ok {
		return nil, errors.New("missing or invalid 'vector' argument")
	}

	vector := make([]float32, len(vectorRaw))
	for i, v := range vectorRaw {
		val, err := parseFloat32(v)
		if err != nil {
			return nil, fmt.Errorf("invalid vector element at index %d: %w", i, err)
		}
		vector[i] = val
	}

	metadata := make(map[string]interface{})
	if metaRaw, ok := arguments["metadata"].(map[string]interface{}); ok {
		metadata = metaRaw
	} else if metaStr, ok := arguments["metadata"].(string); ok && metaStr != "" {
		if err := json.Unmarshal([]byte(metaStr), &metadata); err != nil {
			return nil, fmt.Errorf("failed to parse metadata json: %w", err)
		}
	}

	err := m.provider.Store(ctx, namespace, id, vector, metadata)
	if err != nil {
		return nil, fmt.Errorf("failed to store vector: %w", err)
	}

	return map[string]interface{}{"status": "success"}, nil
}

func (m *VectorMCP) handleVectorSearch(ctx context.Context, arguments map[string]interface{}) (interface{}, error) {
	namespace, ok := arguments["namespace"].(string)
	if !ok || namespace == "" {
		return nil, errors.New("missing or invalid 'namespace' argument")
	}

	vectorRaw, ok := arguments["query_vector"].([]interface{})
	if !ok {
		return nil, errors.New("missing or invalid 'query_vector' argument")
	}

	vector := make([]float32, len(vectorRaw))
	for i, v := range vectorRaw {
		val, err := parseFloat32(v)
		if err != nil {
			return nil, fmt.Errorf("invalid vector element at index %d: %w", i, err)
		}
		vector[i] = val
	}

	var topK int
	switch v := arguments["top_k"].(type) {
	case float64:
		topK = int(v)
	case int:
		topK = v
	default:
		return nil, errors.New("missing or invalid 'top_k' argument")
	}

	if topK <= 0 {
		return nil, errors.New("'top_k' must be greater than 0")
	}

	results, err := m.provider.Search(ctx, namespace, vector, topK)
	if err != nil {
		return nil, fmt.Errorf("failed to search vectors: %w", err)
	}

	return map[string]interface{}{
		"status":  "success",
		"results": results,
	}, nil
}

func parseFloat32(v interface{}) (float32, error) {
	switch val := v.(type) {
	case float64:
		return float32(val), nil
	case float32:
		return val, nil
	case int:
		return float32(val), nil
	case json.Number:
		f, err := val.Float64()
		return float32(f), err
	default:
		return 0, fmt.Errorf("unsupported numeric type: %T", v)
	}
}
