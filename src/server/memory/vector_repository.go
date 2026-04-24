package memory

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
)

type EmbeddingRecord struct {
	ID             string
	OrganizationID string
	AgentID        string
	MemoryType     string
	Content        string
	Embedding      []float32
	CreatedAt      time.Time
	SourceTaskID   string
}

type VectorRepository struct {
	db db.Provider
}

func NewVectorRepository(db db.Provider) *VectorRepository {
	return &VectorRepository{db: db}
}

func (r *VectorRepository) Upsert(ctx context.Context, record *EmbeddingRecord) error {
	embBytes, err := json.Marshal(record.Embedding)
	if err != nil {
		return fmt.Errorf("failed to marshal embedding: %w", err)
	}

	query := `
		INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type, created_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7)
		ON CONFLICT (id) DO UPDATE SET
			content = EXCLUDED.content,
			embedding = EXCLUDED.embedding,
			source_type = EXCLUDED.source_type
	`

	_, err = r.db.Exec(ctx, query, record.ID, record.OrganizationID, record.AgentID, record.Content, embBytes, record.MemoryType, record.CreatedAt)
	return err
}

func (r *VectorRepository) SemanticSearch(ctx context.Context, organizationID string, queryEmbedding []float32, limit int) ([]*EmbeddingRecord, error) {
	embBytes, err := json.Marshal(queryEmbedding)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal query embedding: %w", err)
	}

	var query string
	var rows db.Rows
	var qErr error

	if r.db.IsSQLite() {
		query = `
			SELECT id, organization_id, agent_id, source_type, content, embedding, created_at
			FROM consolidated_memory
			WHERE organization_id = $1
			ORDER BY vec_distance_cosine(embedding, $2) ASC
			LIMIT $3
		`
		rows, qErr = r.db.Query(ctx, query, organizationID, string(embBytes), limit)
	} else {
		query = `
			SELECT id, organization_id, agent_id, source_type, content, embedding, created_at
			FROM consolidated_memory
			WHERE organization_id = $1
			ORDER BY embedding <-> $2
			LIMIT $3
		`
		rows, qErr = r.db.Query(ctx, query, organizationID, string(embBytes), limit)
	}

	if qErr != nil {
		return nil, fmt.Errorf("semantic search query failed: %w", qErr)
	}
	defer rows.Close()

	var results []*EmbeddingRecord
	for rows.Next() {
		var rec EmbeddingRecord
		var embStr string
		var agentID sql.NullString
		if err := rows.Scan(&rec.ID, &rec.OrganizationID, &agentID, &rec.MemoryType, &rec.Content, &embStr, &rec.CreatedAt); err != nil {
			return nil, fmt.Errorf("failed to scan row: %w", err)
		}
		if agentID.Valid {
			rec.AgentID = agentID.String
		}
		if err := json.Unmarshal([]byte(embStr), &rec.Embedding); err != nil {
			// Skip bad embeddings or handle appropriately
			continue
		}
		results = append(results, &rec)
	}

	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("rows iteration error: %w", err)
	}

	return results, nil
}
