package vector

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"strings"
)

type sqliteProvider struct {
	db *sql.DB
}

func newSQLiteProvider(db *sql.DB) *sqliteProvider {
	return &sqliteProvider{
		db: db,
	}
}

// float32SliceToBytes converts a float32 slice to a format suitable for sqlite-vss
// sqlite-vss expects vector as a JSON array of floats for ingestion typically
func float32SliceToJSON(vector []float32) string {
	strs := make([]string, len(vector))
	for i, v := range vector {
		strs[i] = fmt.Sprintf("%f", v)
	}
	return "[" + strings.Join(strs, ",") + "]"
}

func (p *sqliteProvider) Store(ctx context.Context, namespace, id string, vector []float32, metadata map[string]interface{}) error {
	metaJSON, err := json.Marshal(metadata)
	if err != nil {
		return fmt.Errorf("failed to marshal metadata: %w", err)
	}

	vectorJSON := float32SliceToJSON(vector)

	// sqlite-vss often requires inserting into a virtual table for vectors.
	// For a hybrid approach, we may store the base data in a normal table
	// and the vectors in a vss table, or just use the vss functions if they
	// support updating.
	// Assuming a schema where mcp_vector_store has vector column
	// or we use vss virtual table.
	// We will follow a standard approach assuming sqlite-vss is loaded
	// and handles the json format.

	// standard sqlite upsert:
	query := `
		INSERT INTO mcp_vector_store (namespace, id, embedding, metadata)
		VALUES (?, ?, ?, ?)
		ON CONFLICT (namespace, id)
		DO UPDATE SET embedding = excluded.embedding, metadata = excluded.metadata
	`

	_, err = p.db.ExecContext(ctx, query, namespace, id, vectorJSON, metaJSON)
	if err != nil {
		return fmt.Errorf("failed to store vector in sqlite: %w", err)
	}

	return nil
}

func (p *sqliteProvider) Search(ctx context.Context, namespace string, queryVector []float32, topK int) ([]SearchResult, error) {
	queryVectorJSON := float32SliceToJSON(queryVector)

	// sqlite-vss specific knn search using vss_distance
	// Note: sqlite-vss v0.1.x typically uses vss_search or vss_search_params
	// or we can use vss_distance(embedding, query)
	// Example standard query using vss_distance:
	query := `
		SELECT id, embedding, metadata, vss_distance(embedding, ?) AS score
		FROM mcp_vector_store
		WHERE namespace = ?
		ORDER BY score ASC
		LIMIT ?
	`

	rows, err := p.db.QueryContext(ctx, query, queryVectorJSON, namespace, topK)
	if err != nil {
		return nil, fmt.Errorf("failed to search vectors in sqlite: %w", err)
	}
	defer rows.Close()

	var results []SearchResult
	for rows.Next() {
		var id string
		var embeddingStr string // sqlite-vss returns vector as JSON or bytes, typically json for raw selection depending on schema
		var metaJSON []byte
		var score float32

		if err := rows.Scan(&id, &embeddingStr, &metaJSON, &score); err != nil {
			return nil, fmt.Errorf("failed to scan row: %w", err)
		}

		var metadata map[string]interface{}
		if len(metaJSON) > 0 {
			if err := json.Unmarshal(metaJSON, &metadata); err != nil {
				return nil, fmt.Errorf("failed to unmarshal metadata: %w", err)
			}
		}

		// parse vector json back to float32
		var vector []float32
		if err := json.Unmarshal([]byte(embeddingStr), &vector); err != nil {
			// maybe it was returned as BLOB, but we stored it as string/JSON.
			return nil, fmt.Errorf("failed to parse retrieved vector: %w", err)
		}

		results = append(results, SearchResult{
			ID:       id,
			Vector:   vector,
			Metadata: metadata,
			Score:    score,
		})
	}

	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("rows iteration error: %w", err)
	}

	return results, nil
}
