package searchmcp

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"strings"

	"github.com/onehumancorp/mono/src/server/auth"
	"github.com/onehumancorp/mono/src/server/db"
)

// Document represents a searchable document.
type Document struct {
	ID      string `json:"id"`
	Content string `json:"content"`
	Title   string `json:"title"`
}

// SearchResult represents a document retrieved by the search provider.
type SearchResult struct {
	ID      string  `json:"id"`
	Content string  `json:"content"`
	Title   string  `json:"title"`
	Score   float64 `json:"score"`
}

// SearchProvider is the interface for searching and indexing documents.
type SearchProvider interface {
	Search(ctx context.Context, query string) ([]SearchResult, error)
	Index(ctx context.Context, doc Document) error
}

// LocalSearchProvider implements SearchProvider using local SQLite FTS5.
type LocalSearchProvider struct {
	provider db.Provider
}

// NewLocalSearchProvider creates a new LocalSearchProvider.
func NewLocalSearchProvider(provider db.Provider) *LocalSearchProvider {
	return &LocalSearchProvider{provider: provider}
}

func (p *LocalSearchProvider) Search(ctx context.Context, query string) ([]SearchResult, error) {
	// Query local SQLite using FTS5 (assuming a table named local_search_index exists)
	// For testing and demonstration, returning mock or executing against provider
	rows, err := p.provider.Query(ctx, "SELECT id, title, content FROM local_search_index WHERE local_search_index MATCH ? ORDER BY rank LIMIT 10", query)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var results []SearchResult
	for rows.Next() {
		var id, title, content string
		if err := rows.Scan(&id, &title, &content); err == nil {
			results = append(results, SearchResult{ID: id, Title: title, Content: content, Score: 1.0})
		}
	}
	return results, nil
}

func (p *LocalSearchProvider) Index(ctx context.Context, doc Document) error {
	_, err := p.provider.Exec(ctx, "INSERT INTO local_search_index(id, title, content) VALUES(?, ?, ?)", doc.ID, doc.Title, doc.Content)
	return err
}

// CloudSearchProvider implements SearchProvider using Cloud PostgreSQL with pgvector.
type CloudSearchProvider struct {
	provider db.Provider
}

// NewCloudSearchProvider creates a new CloudSearchProvider.
func NewCloudSearchProvider(provider db.Provider) *CloudSearchProvider {
	return &CloudSearchProvider{provider: provider}
}

func (p *CloudSearchProvider) Search(ctx context.Context, query string) ([]SearchResult, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return nil, errors.New("unauthorized: missing claims or organization ID")
	}

	// Query Postgres using tenant isolation
	rows, err := p.provider.Query(ctx, "SELECT id, title, content FROM cloud_search_index WHERE tenant_id = $1 AND content ILIKE $2 LIMIT 10", claims.OrganizationID, "%"+query+"%")
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var results []SearchResult
	for rows.Next() {
		var id, title, content string
		if err := rows.Scan(&id, &title, &content); err == nil {
			results = append(results, SearchResult{ID: id, Title: title, Content: content, Score: 1.0})
		}
	}
	return results, nil
}

func (p *CloudSearchProvider) Index(ctx context.Context, doc Document) error {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return errors.New("unauthorized: missing claims or organization ID")
	}

	_, err := p.provider.Exec(ctx, "INSERT INTO cloud_search_index(tenant_id, id, title, content) VALUES($1, $2, $3, $4)", claims.OrganizationID, doc.ID, doc.Title, doc.Content)
	return err
}

// Tool represents an MCP tool definition.
type Tool struct {
	Name        string          `json:"name"`
	Description string          `json:"description"`
	InputSchema json.RawMessage `json:"inputSchema"`
}

// HybridSearchMCP implements the MCP interface for search operations.
type HybridSearchMCP struct {
	provider SearchProvider
}

// NewHybridSearchMCP creates a new HybridSearchMCP instance.
func NewHybridSearchMCP(provider SearchProvider) *HybridSearchMCP {
	return &HybridSearchMCP{provider: provider}
}

// ListTools returns the list of available tools.
func (m *HybridSearchMCP) ListTools() []Tool {
	return []Tool{
		{
			Name:        "unified_search",
			Description: "Searches documents and web context based on environment mode.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"query": {"type": "string"}}, "required": ["query"]}`),
		},
		{
			Name:        "index_document",
			Description: "Indexes a document into the search provider.",
			InputSchema: json.RawMessage(`{"type": "object", "properties": {"id": {"type": "string"}, "title": {"type": "string"}, "content": {"type": "string"}}, "required": ["id", "title", "content"]}`),
		},
	}
}

// CallTool executes a tool by name.
func (m *HybridSearchMCP) CallTool(ctx context.Context, toolName string, arguments map[string]interface{}) (interface{}, error) {
	switch toolName {
	case "unified_search":
		query, ok := arguments["query"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'query' argument")
		}
		results, err := m.provider.Search(ctx, query)
		if err != nil {
			return nil, err
		}
		return map[string]interface{}{"results": results}, nil
	case "index_document":
		id, ok := arguments["id"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'id' argument")
		}
		title, ok := arguments["title"].(string)
		if !ok {
			title = ""
		}
		content, ok := arguments["content"].(string)
		if !ok {
			return nil, errors.New("missing or invalid 'content' argument")
		}

		doc := Document{ID: id, Title: title, Content: content}
		if err := m.provider.Index(ctx, doc); err != nil {
			return nil, err
		}
		return map[string]interface{}{"status": "success"}, nil
	default:
		return nil, fmt.Errorf("unknown tool: %s", toolName)
	}
}

func envBoolDefault(key string, fallback bool) bool {
	val := os.Getenv(key)
	if val == "" {
		return fallback
	}
	return strings.ToLower(val) == "true" || val == "1"
}

// NewProviderFactory returns a SearchProvider based on environment configuration.
func NewProviderFactory(provider db.Provider) SearchProvider {
	if !envBoolDefault("OHC_STANDALONE", false) {
		return NewCloudSearchProvider(provider)
	}
	return NewLocalSearchProvider(provider)
}
