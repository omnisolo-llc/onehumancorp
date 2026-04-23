package vector

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"

	"github.com/pgvector/pgvector-go"
)

type pgvectorProvider struct {
	db *sql.DB
}

func newPGVectorProvider(db *sql.DB) *pgvectorProvider {
	return &pgvectorProvider{
		db: db,
	}
}

func (p *pgvectorProvider) Store(ctx context.Context, namespace, id string, vector []float32, metadata map[string]interface{}) error {
	metaJSON, err := json.Marshal(metadata)
	if err != nil {
		return fmt.Errorf("failed to marshal metadata: %w", err)
	}

	query := `
		INSERT INTO mcp_vector_store (namespace, id, embedding, metadata)
		VALUES ($1, $2, $3, $4)
		ON CONFLICT (namespace, id)
		DO UPDATE SET embedding = EXCLUDED.embedding, metadata = EXCLUDED.metadata
	`

	_, err = p.db.ExecContext(ctx, query, namespace, id, pgvector.NewVector(vector), metaJSON)
	if err != nil {
		return fmt.Errorf("failed to store vector in pgvector: %w", err)
	}

	return nil
}

func (p *pgvectorProvider) Search(ctx context.Context, namespace string, queryVector []float32, topK int) ([]SearchResult, error) {
	query := `
		SELECT id, embedding, metadata, embedding <=> $1 AS score
		FROM mcp_vector_store
		WHERE namespace = $2
		ORDER BY embedding <=> $1
		LIMIT $3
	`

	rows, err := p.db.QueryContext(ctx, query, pgvector.NewVector(queryVector), namespace, topK)
	if err != nil {
		return nil, fmt.Errorf("failed to search vectors in pgvector: %w", err)
	}
	defer rows.Close()

	var results []SearchResult
	for rows.Next() {
		var id string
		var embedding pgvector.Vector
		var metaJSON []byte
		var score float32

		if err := rows.Scan(&id, &embedding, &metaJSON, &score); err != nil {
			return nil, fmt.Errorf("failed to scan row: %w", err)
		}

		var metadata map[string]interface{}
		if len(metaJSON) > 0 {
			if err := json.Unmarshal(metaJSON, &metadata); err != nil {
				return nil, fmt.Errorf("failed to unmarshal metadata: %w", err)
			}
		}

		results = append(results, SearchResult{
			ID:       id,
			Vector:   embedding.Slice(),
			Metadata: metadata,
			Score:    score,
		})
	}

	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("rows iteration error: %w", err)
	}

	return results, nil
}
