package memory

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type EmbeddingRecord struct {
	ID             string
	OrganizationID string
	TenantID       string
	AgentID        string
	MemoryType     string
	Content        string
	Embedding      []float32
	SourceType     string
	CreatedAt      time.Time
	LastAccessedAt time.Time
	ConfidenceScore float64
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
		INSERT INTO consolidated_memory (id, organization_id, tenant_id, agent_id, content, embedding, source_type, created_at, last_accessed_at, confidence_score)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
	`
	// Use INSERT ... ON CONFLICT if UPSERT logic is required. Keeping it simple as per instructions.

	_, err = r.db.Exec(ctx, query, record.ID, record.OrganizationID, record.TenantID, record.AgentID, record.Content, embBytes, record.SourceType, record.CreatedAt, record.LastAccessedAt, record.ConfidenceScore)
	return err
}

func (r *VectorRepository) SemanticSearch(ctx context.Context, organizationID string, queryEmbedding []float32, limit int) ([]*EmbeddingRecord, error) {
	embBytes, err := json.Marshal(queryEmbedding)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal query embedding: %w", err)
	}

	var query string
	var rows db.Rows

	if r.db.IsSQLite() {
		// Basic retrieval without vector math for SQLite
		query = `
			SELECT id, organization_id, tenant_id, agent_id, content, source_type, created_at, last_accessed_at, confidence_score
			FROM consolidated_memory
			WHERE organization_id = $1 AND vec_distance_cosine(embedding, $2) < 0.15
			ORDER BY vec_distance_cosine(embedding, $2) ASC
			LIMIT $3
		`
		rows, err = r.db.Query(ctx, query, organizationID, string(embBytes), limit)
	} else {
		// pgvector
		query = `
			SELECT id, organization_id, tenant_id, agent_id, content, source_type, created_at, last_accessed_at, confidence_score
			FROM consolidated_memory
			WHERE organization_id = $1 AND embedding <-> $2::vector < 0.15
			ORDER BY embedding <-> $2::vector ASC
			LIMIT $3
		`
		// We format $2 as a string literal of the json array for pgvector, or pass string.
		rows, err = r.db.Query(ctx, query, organizationID, string(embBytes), limit)
	}

	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var results []*EmbeddingRecord
	for rows.Next() {
		var rec EmbeddingRecord
		// embedding is not selected back to save memory bandwidth in this example
		var tenantID sql.NullString
		var agentID sql.NullString
		var lastAccessedAt sql.NullTime
		var confidenceScore sql.NullFloat64

		err := rows.Scan(
			&rec.ID, &rec.OrganizationID, &tenantID, &agentID,
			&rec.Content, &rec.SourceType, &rec.CreatedAt,
			&lastAccessedAt, &confidenceScore,
		)
		if err != nil {
			return nil, err
		}
		if tenantID.Valid {
			rec.TenantID = tenantID.String
		}
		if agentID.Valid {
			rec.AgentID = agentID.String
		}
		if lastAccessedAt.Valid {
			rec.LastAccessedAt = lastAccessedAt.Time
		}
		if confidenceScore.Valid {
			rec.ConfidenceScore = confidenceScore.Float64
		} else {
			rec.ConfidenceScore = 1.0
		}
		results = append(results, &rec)
	}

	return results, nil
}

func (r *VectorRepository) DeleteStale(ctx context.Context, olderThan time.Time) error {
	query := `
		DELETE FROM consolidated_memory
		WHERE last_accessed_at < $1 AND confidence_score < 0.8
	`
	_, err := r.db.Exec(ctx, query, olderThan)
	return err
}

func (r *VectorRepository) Delete(ctx context.Context, id string) error {
	query := `DELETE FROM consolidated_memory WHERE id = $1`
	_, err := r.db.Exec(ctx, query, id)
	return err
}
