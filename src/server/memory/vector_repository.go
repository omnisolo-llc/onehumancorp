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
	TenantID       string
	MemoryType     string
	Content        string
	Embedding      []float32
	CreatedAt      time.Time
	SourceTaskID   string
}

type SearchResult struct {
	Record *EmbeddingRecord
	Score  float64
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
		INSERT INTO autodream_memories_master (id, tenant_id, memory_type, content, embedding, created_at, source_task_id)
		VALUES ($1, $2, $3, $4, $5, $6, $7)
	`
	_, err = r.db.Exec(ctx, query, record.ID, record.TenantID, record.MemoryType, record.Content, embBytes, record.CreatedAt, record.SourceTaskID)
	return err
}

func (r *VectorRepository) UpdateRecord(ctx context.Context, record *EmbeddingRecord) error {
	embBytes, err := json.Marshal(record.Embedding)
	if err != nil {
		return fmt.Errorf("failed to marshal embedding: %w", err)
	}
	query := `
		UPDATE autodream_memories_master
		SET content = $1, embedding = $2, created_at = $3, source_task_id = $4
		WHERE id = $5 AND tenant_id = $6
	`
	_, err = r.db.Exec(ctx, query, record.Content, embBytes, record.CreatedAt, record.SourceTaskID, record.ID, record.TenantID)
	return err
}

func (r *VectorRepository) DeleteOldMemories(ctx context.Context, memoryType string, retention time.Duration) error {
	threshold := time.Now().Add(-retention)
	query := `DELETE FROM autodream_memories_master WHERE memory_type = $1 AND created_at < $2`
	_, err := r.db.Exec(ctx, query, memoryType, threshold)
	return err
}

func (r *VectorRepository) SemanticSearch(ctx context.Context, tenantID string, queryEmbedding []float32, limit int) ([]SearchResult, error) {
	embBytes, err := json.Marshal(queryEmbedding)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal query embedding: %w", err)
	}

	var query string
	if r.db.IsSQLite() {
		// SQLite vector search using sqlite-vec
		// Using vec_distance_cosine
		query = `
			SELECT id, tenant_id, memory_type, content, embedding, created_at, source_task_id,
			       1 - vec_distance_cosine(embedding, $2) AS score
			FROM autodream_memories_master
			WHERE tenant_id = $1 AND (1 - vec_distance_cosine(embedding, $2)) > 0.90
			ORDER BY vec_distance_cosine(embedding, $2)
			LIMIT $3
		`
	} else {
		// PostgreSQL pgvector
		query = `
			SELECT id, tenant_id, memory_type, content, embedding, created_at, source_task_id,
			       1 - (embedding <=> $2::vector) AS score
			FROM autodream_memories_master
			WHERE tenant_id = $1 AND (1 - (embedding <=> $2::vector)) > 0.90
			ORDER BY embedding <=> $2::vector
			LIMIT $3
		`
	}

	rows, err := r.db.Query(ctx, query, tenantID, string(embBytes), limit)
	if err != nil {
		return nil, fmt.Errorf("failed to query memories: %w", err)
	}
	defer rows.Close()

	var results []SearchResult
	for rows.Next() {
		var rec EmbeddingRecord
		var score float64
		var sourceTaskID *string
		var embStr string
		var createdAtStr interface{}

		if r.db.IsSQLite() {
			if err := rows.Scan(&rec.ID, &rec.TenantID, &rec.MemoryType, &rec.Content, &embStr, &createdAtStr, &sourceTaskID, &score); err != nil {
				return nil, fmt.Errorf("failed to scan row: %w", err)
			}

			// Handle SQLite time format
			switch v := createdAtStr.(type) {
			case string:
				if t, err := time.Parse("2006-01-02 15:04:05", v); err == nil {
					rec.CreatedAt = t
				} else if t, err := time.Parse(time.RFC3339, v); err == nil {
					rec.CreatedAt = t
				}
			case time.Time:
				rec.CreatedAt = v
			}
		} else {
			var createdAt time.Time
			if err := rows.Scan(&rec.ID, &rec.TenantID, &rec.MemoryType, &rec.Content, &embStr, &createdAt, &sourceTaskID, &score); err != nil {
				return nil, fmt.Errorf("failed to scan row: %w", err)
			}
			rec.CreatedAt = createdAt
		}

		if sourceTaskID != nil {
			rec.SourceTaskID = *sourceTaskID
		}

		var emb []float32
		if err := json.Unmarshal([]byte(embStr), &emb); err == nil {
			rec.Embedding = emb
		}

		results = append(results, SearchResult{Record: &rec, Score: score})
	}
	return results, nil
}
