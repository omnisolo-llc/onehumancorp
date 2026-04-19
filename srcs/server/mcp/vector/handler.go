package vector

import (
	"context"
	"fmt"

	"github.com/modelcontextprotocol/go-sdk/mcp"
)

// NewVectorMCPTools creates the MCP tools for vector storage
func NewVectorMCPTools(provider VectorStorageProvider) []mcp.Tool {
	return []mcp.Tool{
		{
			Name:        "vector_store",
			Description: "Stores an embedding vector in the universal vector storage.",
			InputSchema: struct {
				Type       string                 `json:"type"`
				Properties map[string]interface{} `json:"properties"`
				Required   []string               `json:"required,omitempty"`
			}{
				Type: "object",
				Properties: map[string]interface{}{
					"namespace": map[string]interface{}{
						"type":        "string",
						"description": "The namespace to store the vector in.",
					},
					"id": map[string]interface{}{
						"type":        "string",
						"description": "The unique ID for the vector.",
					},
					"vector": map[string]interface{}{
						"type": "array",
						"items": map[string]interface{}{
							"type": "number",
						},
						"description": "The float32 array representing the vector.",
					},
					"metadata": map[string]interface{}{
						"type":        "string",
						"description": "JSON string containing metadata for the vector.",
					},
				},
				Required: []string{"namespace", "id", "vector", "metadata"},
			},
		},
		{
			Name:        "vector_search",
			Description: "Retrieves similar vectors from the universal vector storage.",
			InputSchema: struct {
				Type       string                 `json:"type"`
				Properties map[string]interface{} `json:"properties"`
				Required   []string               `json:"required,omitempty"`
			}{
				Type: "object",
				Properties: map[string]interface{}{
					"namespace": map[string]interface{}{
						"type":        "string",
						"description": "The namespace to search in.",
					},
					"query_vector": map[string]interface{}{
						"type": "array",
						"items": map[string]interface{}{
							"type": "number",
						},
						"description": "The query vector.",
					},
					"top_k": map[string]interface{}{
						"type":        "integer",
						"description": "The number of top results to return.",
					},
				},
				Required: []string{"namespace", "query_vector", "top_k"},
			},
		},
	}
}

// HandleCall handles MCP tool execution calls for vector storage operations
func HandleCall(ctx context.Context, provider VectorStorageProvider, name string, args map[string]interface{}) (*mcp.CallToolResult, error) {
	switch name {
	case "vector_store":
		namespace, ok := args["namespace"].(string)
		if !ok {
			return nil, fmt.Errorf("invalid namespace argument")
		}
		id, ok := args["id"].(string)
		if !ok {
			return nil, fmt.Errorf("invalid id argument")
		}
		vectorInterface, ok := args["vector"].([]interface{})
		if !ok {
			return nil, fmt.Errorf("invalid vector argument")
		}
		vector := make([]float32, len(vectorInterface))
		for i, v := range vectorInterface {
			f, ok := v.(float64)
			if !ok {
				return nil, fmt.Errorf("invalid vector element type")
			}
			vector[i] = float32(f)
		}
		metadata, ok := args["metadata"].(string)
		if !ok {
			return nil, fmt.Errorf("invalid metadata argument")
		}

		err := provider.Store(namespace, id, vector, metadata)
		if err != nil {
			return nil, err
		}

		return &mcp.CallToolResult{
			Content: []mcp.Content{
				&mcp.TextContent{
					Text: fmt.Sprintf("Successfully stored vector with ID %s in namespace %s.", id, namespace),
				},
			},
			IsError: false,
		}, nil

	case "vector_search":
		namespace, ok := args["namespace"].(string)
		if !ok {
			return nil, fmt.Errorf("invalid namespace argument")
		}
		queryVectorInterface, ok := args["query_vector"].([]interface{})
		if !ok {
			return nil, fmt.Errorf("invalid query_vector argument")
		}
		queryVector := make([]float32, len(queryVectorInterface))
		for i, v := range queryVectorInterface {
			f, ok := v.(float64)
			if !ok {
				return nil, fmt.Errorf("invalid query_vector element type")
			}
			queryVector[i] = float32(f)
		}
		topKFloat, ok := args["top_k"].(float64)
		if !ok {
			return nil, fmt.Errorf("invalid top_k argument")
		}
		topK := int(topKFloat)

		results, err := provider.Search(namespace, queryVector, topK)
		if err != nil {
			return nil, err
		}

		resultsStr := ""
		for _, res := range results {
			resultsStr += fmt.Sprintf("ID: %s, Distance: %f, Metadata: %s\n", res.ID, res.Distance, res.Metadata)
		}

		return &mcp.CallToolResult{
			Content: []mcp.Content{
				&mcp.TextContent{
					Text: resultsStr,
				},
			},
			IsError: false,
		}, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", name)
	}
}
