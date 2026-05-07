package searchmcp

import (
	"context"
	"errors"
	"fmt"
)

type HybridSearchMCP struct {
	provider SearchProvider
}

func NewHybridSearchMCP(provider SearchProvider) *HybridSearchMCP {
	return &HybridSearchMCP{provider: provider}
}

func (s *HybridSearchMCP) ListTools() []map[string]interface{} {
	return []map[string]interface{}{
		{
			"name":        "unified_search",
			"description": "Searches for documents and context.",
			"parameters": map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"query": map[string]interface{}{
						"type":        "string",
						"description": "The search query string.",
					},
				},
				"required": []string{"query"},
			},
		},
		{
			"name":        "index_document",
			"description": "Indexes a document for future searches.",
			"parameters": map[string]interface{}{
				"type": "object",
				"properties": map[string]interface{}{
					"id": map[string]interface{}{
						"type":        "string",
						"description": "The unique document ID.",
					},
					"content": map[string]interface{}{
						"type":        "string",
						"description": "The document content.",
					},
				},
				"required": []string{"id", "content"},
			},
		},
	}
}

func (s *HybridSearchMCP) CallTool(ctx context.Context, name string, args map[string]interface{}) (interface{}, error) {
	if s.provider == nil {
		return nil, errors.New("search provider not configured")
	}

	switch name {
	case "unified_search":
		query, ok := args["query"].(string)
		if !ok || query == "" {
			return nil, errors.New("invalid or missing 'query' argument")
		}

		results, err := s.provider.Search(ctx, query)
		if err != nil {
			return nil, fmt.Errorf("search failed: %w", err)
		}

		return map[string]interface{}{
			"results": results,
		}, nil

	case "index_document":
		id, ok := args["id"].(string)
		if !ok || id == "" {
			return nil, errors.New("invalid or missing 'id' argument")
		}
		content, ok := args["content"].(string)
		if !ok || content == "" {
			return nil, errors.New("invalid or missing 'content' argument")
		}

		doc := Document{
			ID:      id,
			Content: content,
		}

		err := s.provider.Index(ctx, doc)
		if err != nil {
			return nil, fmt.Errorf("index failed: %w", err)
		}

		return map[string]interface{}{
			"status": "success",
		}, nil

	default:
		return nil, fmt.Errorf("unknown tool: %s", name)
	}
}
