package vector

import (
	"database/sql"
	"encoding/json"
	"fmt"
)

// SearchResult represents a single retrieved vector document.
type SearchResult struct {
	ID       string                 `json:"id"`
	Score    float64                `json:"score"`
	Metadata map[string]interface{} `json:"metadata"`
}

// VectorStorageProvider defines the generic interface for vector storage backends.
type VectorStorageProvider interface {
	Store(namespace string, id string, vector []float32, metadata map[string]interface{}) error
	Search(namespace string, queryVector []float32, topK int) ([]SearchResult, error)
}

// pgvectorProvider implements VectorStorageProvider for PostgreSQL with pgvector.
type pgvectorProvider struct {
	db *sql.DB
}

// Store inserts a vector into the pgvector table.
func (p *pgvectorProvider) Store(namespace string, id string, vector []float32, metadata map[string]interface{}) error {
	metaJSON, err := json.Marshal(metadata)
	if err != nil {
		return fmt.Errorf("failed to marshal metadata: %w", err)
	}

	// pgvector uses string representation for vectors: '[1.1, 2.2, ...]'
	vectorStr := float32SliceToString(vector)

	query := `
		INSERT INTO vector_store (namespace, id, embedding, metadata)
		VALUES ($1, $2, $3::vector, $4)
		ON CONFLICT (namespace, id) DO UPDATE
		SET embedding = EXCLUDED.embedding, metadata = EXCLUDED.metadata
	`
	_, err = p.db.Exec(query, namespace, id, vectorStr, metaJSON)
	return err
}

// Search retrieves top-k closest vectors using pgvector's <=> cosine distance operator.
func (p *pgvectorProvider) Search(namespace string, queryVector []float32, topK int) ([]SearchResult, error) {
	vectorStr := float32SliceToString(queryVector)

	// <=> is cosine distance in pgvector
	query := `
		SELECT id, (embedding <=> $1::vector) AS distance, metadata
		FROM vector_store
		WHERE namespace = $2
		ORDER BY distance ASC
		LIMIT $3
	`
	rows, err := p.db.Query(query, vectorStr, namespace, topK)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var results []SearchResult
	for rows.Next() {
		var res SearchResult
		var metaStr string
		var distance float64
		if err := rows.Scan(&res.ID, &distance, &metaStr); err != nil {
			return nil, err
		}
		// Convert distance to a similarity score if needed, or just return distance.
		// We'll return 1 - distance to approximate similarity score for cosine.
		res.Score = 1.0 - distance

		var meta map[string]interface{}
		if err := json.Unmarshal([]byte(metaStr), &meta); err != nil {
			return nil, err
		}
		res.Metadata = meta
		results = append(results, res)
	}
	return results, nil
}

// sqliteVssProvider implements VectorStorageProvider for SQLite with sqlite-vss.
type sqliteVssProvider struct {
	db *sql.DB
}

// Store inserts a vector into the sqlite-vss tables.
func (p *sqliteVssProvider) Store(namespace string, id string, vector []float32, metadata map[string]interface{}) error {
	metaJSON, err := json.Marshal(metadata)
	if err != nil {
		return fmt.Errorf("failed to marshal metadata: %w", err)
	}

	// sqlite-vss expects JSON array string for embeddings.
	vectorStr := float32SliceToString(vector)

	query := `
		INSERT OR REPLACE INTO vector_store (namespace, id, embedding, metadata)
		VALUES (?, ?, ?, ?)
	`
	_, err = p.db.Exec(query, namespace, id, vectorStr, string(metaJSON))
	return err
}

// Search retrieves top-k closest vectors using sqlite-vss vss_distance function.
func (p *sqliteVssProvider) Search(namespace string, queryVector []float32, topK int) ([]SearchResult, error) {
	vectorStr := float32SliceToString(queryVector)

	query := `
		SELECT id, vss_distance(embedding, ?) AS distance, metadata
		FROM vector_store
		WHERE namespace = ?
		ORDER BY distance ASC
		LIMIT ?
	`
	rows, err := p.db.Query(query, vectorStr, namespace, topK)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var results []SearchResult
	for rows.Next() {
		var res SearchResult
		var metaStr string
		var distance float64
		if err := rows.Scan(&res.ID, &distance, &metaStr); err != nil {
			return nil, err
		}

		res.Score = 1.0 - distance

		var meta map[string]interface{}
		if err := json.Unmarshal([]byte(metaStr), &meta); err != nil {
			return nil, err
		}
		res.Metadata = meta
		results = append(results, res)
	}
	return results, nil
}

// NewVectorStorageTransport is a factory function that wires the correct provider.
func NewVectorStorageTransport(mode string, db *sql.DB) (VectorStorageProvider, error) {
	if mode == "cloud" {
		return &pgvectorProvider{db: db}, nil
	} else if mode == "standalone" {
		return &sqliteVssProvider{db: db}, nil
	}
	return nil, fmt.Errorf("unsupported mode: %s", mode)
}

// MCPHandler is an MCP tool handler that wraps a VectorStorageProvider.
type MCPHandler struct {
	provider VectorStorageProvider
}

// NewMCPHandler creates a new MCPHandler.
func NewMCPHandler(provider VectorStorageProvider) *MCPHandler {
	return &MCPHandler{
		provider: provider,
	}
}

// ExecuteStore handles the vector_store MCP tool call.
func (h *MCPHandler) ExecuteStore(arguments map[string]interface{}) (map[string]interface{}, error) {
	namespace, ok := arguments["namespace"].(string)
	if !ok {
		return nil, fmt.Errorf("missing or invalid 'namespace'")
	}
	id, ok := arguments["id"].(string)
	if !ok {
		return nil, fmt.Errorf("missing or invalid 'id'")
	}
	vectorInterface, ok := arguments["vector"].([]interface{})
	if !ok {
		return nil, fmt.Errorf("missing or invalid 'vector'")
	}
	var vector []float32
	for _, v := range vectorInterface {
		if f64, ok := v.(float64); ok {
			vector = append(vector, float32(f64))
		} else {
			return nil, fmt.Errorf("invalid vector element type")
		}
	}
	metadata, ok := arguments["metadata"].(map[string]interface{})
	if !ok {
		metadata = make(map[string]interface{})
	}

	err := h.provider.Store(namespace, id, vector, metadata)
	if err != nil {
		return nil, err
	}

	return map[string]interface{}{"status": "success"}, nil
}

// ExecuteSearch handles the vector_search MCP tool call.
func (h *MCPHandler) ExecuteSearch(arguments map[string]interface{}) (map[string]interface{}, error) {
	namespace, ok := arguments["namespace"].(string)
	if !ok {
		return nil, fmt.Errorf("missing or invalid 'namespace'")
	}
	queryVectorInterface, ok := arguments["query_vector"].([]interface{})
	if !ok {
		return nil, fmt.Errorf("missing or invalid 'query_vector'")
	}
	var queryVector []float32
	for _, v := range queryVectorInterface {
		if f64, ok := v.(float64); ok {
			queryVector = append(queryVector, float32(f64))
		} else {
			return nil, fmt.Errorf("invalid query_vector element type")
		}
	}
	topKFloat, ok := arguments["top_k"].(float64)
	if !ok {
		return nil, fmt.Errorf("missing or invalid 'top_k'")
	}
	topK := int(topKFloat)

	results, err := h.provider.Search(namespace, queryVector, topK)
	if err != nil {
		return nil, err
	}

	var formattedResults []map[string]interface{}
	for _, res := range results {
		formattedResults = append(formattedResults, map[string]interface{}{
			"id":       res.ID,
			"score":    res.Score,
			"metadata": res.Metadata,
		})
	}

	return map[string]interface{}{
		"status":  "success",
		"results": formattedResults,
	}, nil
}

func float32SliceToString(vector []float32) string {
	bytes, _ := json.Marshal(vector)
	return string(bytes)
}
