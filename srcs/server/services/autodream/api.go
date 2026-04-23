package autodream

import (
	"context"
	"fmt"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// TruthSearchResult represents a single search result.
type TruthSearchResult struct {
	MemoryID string  `json:"memory_id"`
	Context  string  `json:"context"`
	Distance float64 `json:"distance"`
}

// SearchKnowledge performs a cosine similarity search on the knowledge_embeddings table.
func SearchKnowledge(ctx context.Context, pool db.Provider, client LLMClient, tenantID string, queryText string, limit int) ([]TruthSearchResult, error) {
	if limit <= 0 {
		limit = 5
	}

	var embedding []float32
	if client != nil {
		emb, err := client.GenerateEmbedding(ctx, queryText)
		if err == nil && len(emb) == 1536 {
			embedding = emb
		}
	}

	if len(embedding) == 0 {
		embedding = make([]float32, 1536)
	}

	if pool.IsSQLite() {
		// SQLite fallback
		query := `
			SELECT id, content, 0 as distance
			FROM knowledge_embeddings
			WHERE tenant_id = $1
			ORDER BY created_at DESC
			LIMIT $2
		`
		rows, err := pool.Query(ctx, query, tenantID, limit)
		if err != nil {
			return nil, fmt.Errorf("failed to search knowledge embeddings with SQLite fallback: %w", err)
		}
		defer rows.Close()

		var results []TruthSearchResult
		for rows.Next() {
			var res TruthSearchResult
			if err := rows.Scan(&res.MemoryID, &res.Context, &res.Distance); err != nil {
				continue
			}
			results = append(results, res)
		}
		return results, nil
	}

	// Postgres pgvector
	strs := make([]string, len(embedding))
	for i, v := range embedding {
		strs[i] = fmt.Sprintf("%f", v)
	}
	embStr := "[" + strings.Join(strs, ",") + "]"

	query := `
		SELECT id, content, embedding <=> $1::vector as distance
		FROM knowledge_embeddings
		WHERE tenant_id = $2
		ORDER BY distance ASC
		LIMIT $3
	`
	rows, err := pool.Query(ctx, query, embStr, tenantID, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to search knowledge embeddings with pgvector: %w", err)
	}
	defer rows.Close()

	var results []TruthSearchResult
	for rows.Next() {
		var res TruthSearchResult
		if err := rows.Scan(&res.MemoryID, &res.Context, &res.Distance); err != nil {
			continue
		}
		results = append(results, res)
	}
	return results, nil
}
