package memory

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
)

type EmbeddingRecord struct {
	ID             string
	OrganizationID string
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
		INSERT INTO consolidated_memory (id, organization_id, source_type, content, embedding, created_at, agent_id)
		VALUES ($1, $2, $3, $4, $5::vector, $6, $7)
		ON CONFLICT (id) DO UPDATE SET
			content = EXCLUDED.content,
			embedding = EXCLUDED.embedding,
			source_type = EXCLUDED.source_type,
			updated_at = CURRENT_TIMESTAMP
	`
	if r.db.IsSQLite() {
		query = `
			INSERT INTO consolidated_memory (id, organization_id, source_type, content, embedding, created_at, agent_id)
			VALUES ($1, $2, $3, $4, $5, $6, $7)
			ON CONFLICT (id) DO UPDATE SET
				content = EXCLUDED.content,
				embedding = EXCLUDED.embedding,
				source_type = EXCLUDED.source_type
		`
	}
	_, err = r.db.Exec(ctx, query, record.ID, record.OrganizationID, record.MemoryType, record.Content, string(embBytes), record.CreatedAt, record.SourceTaskID)
	return err
}

func (r *VectorRepository) SemanticSearch(ctx context.Context, organizationID string, queryEmbedding []float32, limit int) ([]*EmbeddingRecord, error) {
	embBytes, err := json.Marshal(queryEmbedding)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal embedding: %w", err)
	}

	query := `
		SELECT id, organization_id, source_type, content, created_at
		FROM consolidated_memory
		WHERE organization_id = $1
		ORDER BY embedding <-> $2::vector
		LIMIT $3
	`
	if r.db.IsSQLite() {
		query = `
			SELECT id, organization_id, source_type, content, created_at
			FROM consolidated_memory
			WHERE organization_id = $1
			ORDER BY vec_distance_cosine(embedding, $2)
			LIMIT $3
		`
	}

	rows, err := r.db.Query(ctx, query, organizationID, string(embBytes), limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []*EmbeddingRecord
	for rows.Next() {
		var rec EmbeddingRecord
		if err := rows.Scan(&rec.ID, &rec.OrganizationID, &rec.MemoryType, &rec.Content, &rec.CreatedAt); err != nil {
			return nil, err
		}
		records = append(records, &rec)
	}
	return records, rows.Err()
}

type Conflict struct {
	ID1      string
	ID2      string
	Content1 string
	Content2 string
}

func (r *VectorRepository) FindConflicts(ctx context.Context, organizationID string, threshold float64) ([]Conflict, error) {
	query := `
		SELECT a.id, b.id, a.content, b.content
		FROM consolidated_memory a
		JOIN consolidated_memory b ON a.id < b.id AND a.organization_id = b.organization_id
		WHERE a.organization_id = $1 AND (a.embedding <-> b.embedding) < $2
	`
	if r.db.IsSQLite() {
		query = `
			SELECT a.id, b.id, a.content, b.content
			FROM consolidated_memory a
			JOIN consolidated_memory b ON a.id < b.id AND a.organization_id = b.organization_id
			WHERE a.organization_id = $1 AND vec_distance_cosine(a.embedding, b.embedding) < $2
		`
	}

	rows, err := r.db.Query(ctx, query, organizationID, threshold)
	if err != nil {
		return nil, fmt.Errorf("failed to find conflicts: %w", err)
	}
	defer rows.Close()

	var conflicts []Conflict
	for rows.Next() {
		var c Conflict
		if err := rows.Scan(&c.ID1, &c.ID2, &c.Content1, &c.Content2); err != nil {
			return nil, err
		}
		conflicts = append(conflicts, c)
	}
	return conflicts, nil
}

func (r *VectorRepository) DeleteMemories(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}
	// Simple deletion assuming few IDs
	for _, id := range ids {
		_, err := r.db.Exec(ctx, "DELETE FROM consolidated_memory WHERE id = $1", id)
		if err != nil {
			return err
		}
	}
	return nil
}

func (r *VectorRepository) PruneOlderThan(ctx context.Context, organizationID string, cutoff time.Time) error {
	_, err := r.db.Exec(ctx, "DELETE FROM consolidated_memory WHERE organization_id = $1 AND created_at < $2", organizationID, cutoff)
	return err
}
