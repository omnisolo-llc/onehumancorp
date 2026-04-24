package memory

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
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
		INSERT INTO autodream_memories_master (id, organization_id, memory_type, content, embedding, created_at, source_task_id)
		VALUES ($1, $2, $3, $4, $5, $6, $7)
		ON CONFLICT (id) DO UPDATE SET
			content = EXCLUDED.content,
			embedding = EXCLUDED.embedding,
			created_at = EXCLUDED.created_at,
			source_task_id = EXCLUDED.source_task_id,
			memory_type = EXCLUDED.memory_type
	`

	_, err = r.db.Exec(ctx, query, record.ID, record.OrganizationID, record.MemoryType, record.Content, embBytes, record.CreatedAt, record.SourceTaskID)
	return err
}

func (r *VectorRepository) SemanticSearch(ctx context.Context, organizationID string, queryEmbedding []float32, limit int, threshold float64) ([]*EmbeddingRecord, error) {
	embBytes, err := json.Marshal(queryEmbedding)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal query embedding: %w", err)
	}

	var records []*EmbeddingRecord

	if r.db.IsSQLite() {
		// Because there is no sqlite-vec extension loaded by default in testing or easily mockable without errors,
		// we return empty gracefully but return the error so tests can intercept or we can properly mock it.
		query := `
			SELECT id, organization_id, memory_type, content, created_at, source_task_id
			FROM (
				SELECT id, organization_id, memory_type, content, created_at, source_task_id,
					(1.0 - vec_distance_cosine(embedding, $2)) AS score
				FROM autodream_memories_master
				WHERE organization_id = $1
				ORDER BY score DESC
				LIMIT $3
			) sub
			WHERE score >= $4
		`
		rows, err := r.db.Query(ctx, query, organizationID, string(embBytes), limit, threshold)
		if err != nil {
			return nil, fmt.Errorf("sqlite vector extension error: %w", err)
		}
		defer rows.Close()

		for rows.Next() {
			rec := &EmbeddingRecord{}
			if err := rows.Scan(&rec.ID, &rec.OrganizationID, &rec.MemoryType, &rec.Content, &rec.CreatedAt, &rec.SourceTaskID); err != nil {
				return nil, err
			}
			records = append(records, rec)
		}
	} else {
		// PgVector with score thresholding
		query := `
			SELECT id, organization_id, memory_type, content, created_at, source_task_id
			FROM (
				SELECT id, organization_id, memory_type, content, created_at, source_task_id,
					1 - (embedding <=> $2::vector) AS score
				FROM autodream_memories_master
				WHERE organization_id = $1
				ORDER BY embedding <=> $2::vector
				LIMIT $3
			) sub
			WHERE score >= $4
		`
		rows, err := r.db.Query(ctx, query, organizationID, string(embBytes), limit, threshold)
		if err != nil {
			return nil, fmt.Errorf("pgvector error: %w", err)
		}
		defer rows.Close()

		for rows.Next() {
			rec := &EmbeddingRecord{}
			if err := rows.Scan(&rec.ID, &rec.OrganizationID, &rec.MemoryType, &rec.Content, &rec.CreatedAt, &rec.SourceTaskID); err != nil {
				return nil, err
			}
			records = append(records, rec)
		}
	}

	return records, nil
}

func (r *VectorRepository) Delete(ctx context.Context, id string, organizationID string) error {
	query := `DELETE FROM autodream_memories_master WHERE id = $1 AND organization_id = $2`
	_, err := r.db.Exec(ctx, query, id, organizationID)
	return err
}

func (r *VectorRepository) PruneStale(ctx context.Context, organizationID string, memoryType string, olderThan time.Time) (int64, error) {
	// Be conservative: only prune if older than 180 days AND of a specific transient type,
	// or prune based on explicit tenant scopes. Here we keep it very safe.
	// We will never blindly delete all memories.
	query := `DELETE FROM autodream_memories_master WHERE organization_id = $1 AND memory_type = $2 AND created_at < $3`
	return r.db.Exec(ctx, query, organizationID, memoryType, olderThan)
}
