package memory

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
)

type EmbeddingRecord struct {
	ID           string
	OrganizationID     string
	MemoryType   string
	Content      string
	Embedding    []float32
	CreatedAt    time.Time
	SourceTaskID string
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
	`
	// Use INSERT ... ON CONFLICT if UPSERT logic is required. Keeping it simple as per instructions.

	_, err = r.db.Exec(ctx, query, record.ID, record.OrganizationID, record.MemoryType, record.Content, embBytes, record.CreatedAt, record.SourceTaskID)
	return err
}

func formatFloat32SliceForVector(embedding []float32) string {
	if len(embedding) == 0 {
		return "[]"
	}
	res := "["
	for i, v := range embedding {
		if i > 0 {
			res += ","
		}
		res += fmt.Sprintf("%f", v)
	}
	res += "]"
	return res
}

func (r *VectorRepository) Prune(ctx context.Context, organizationID string, olderThan time.Time) error {
	query := `
		DELETE FROM autodream_memories_master
		WHERE organization_id = $1 AND created_at < $2
	`
	_, err := r.db.Exec(ctx, query, organizationID, olderThan)
	return err
}

func (r *VectorRepository) Delete(ctx context.Context, ids []string) error {
	if len(ids) == 0 {
		return nil
	}

	// Create query: DELETE FROM autodream_memories_master WHERE id IN ($1, $2, ...)
	query := "DELETE FROM autodream_memories_master WHERE id IN ("
	args := make([]interface{}, len(ids))
	for i, id := range ids {
		if i > 0 {
			query += ","
		}
		query += fmt.Sprintf("$%d", i+1)
		args[i] = id
	}
	query += ")"

	_, err := r.db.Exec(ctx, query, args...)
	return err
}

func (r *VectorRepository) SemanticSearch(ctx context.Context, organizationID string, queryEmbedding []float32, limit int) ([]*EmbeddingRecord, error) {
	if r.db.IsSQLite() {
		// Fallback for SQLite: latest retrieval since vector operations are not natively supported
		query := `
			SELECT id, organization_id, memory_type, content, embedding, created_at, source_task_id
			FROM autodream_memories_master
			WHERE organization_id = $1
			ORDER BY created_at DESC
			LIMIT $2
		`
		rows, err := r.db.Query(ctx, query, organizationID, limit)
		if err != nil {
			return nil, fmt.Errorf("failed to perform sqlite fallback search: %w", err)
		}
		defer rows.Close()

		var results []*EmbeddingRecord
		for rows.Next() {
			var rec EmbeddingRecord
			var embBytes []byte
			if err := rows.Scan(&rec.ID, &rec.OrganizationID, &rec.MemoryType, &rec.Content, &embBytes, &rec.CreatedAt, &rec.SourceTaskID); err != nil {
				continue
			}
			if len(embBytes) > 0 {
				_ = json.Unmarshal(embBytes, &rec.Embedding)
			}
			results = append(results, &rec)
		}
		return results, nil
	}

	embStr := formatFloat32SliceForVector(queryEmbedding)
	query := `
		SELECT id, organization_id, memory_type, content, embedding, created_at, source_task_id
		FROM autodream_memories_master
		WHERE organization_id = $1
		ORDER BY embedding <-> $2::vector ASC
		LIMIT $3
	`
	rows, err := r.db.Query(ctx, query, organizationID, embStr, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to perform pgvector semantic search: %w", err)
	}
	defer rows.Close()

	var results []*EmbeddingRecord
	for rows.Next() {
		var rec EmbeddingRecord
		var embBytes []byte
		if err := rows.Scan(&rec.ID, &rec.OrganizationID, &rec.MemoryType, &rec.Content, &embBytes, &rec.CreatedAt, &rec.SourceTaskID); err != nil {
			continue
		}
		// pgvector returns a string format, e.g. "[0.1, 0.2]"
		// For our EmbeddingRecord, if it is returned as string from pgx, we might need to parse it.
		// Since we scan into []byte, we can try to unmarshal it as JSON.
		if len(embBytes) > 0 {
			var floatArray []float32
			if err := json.Unmarshal(embBytes, &floatArray); err == nil {
				rec.Embedding = floatArray
			} else {
				// Fallback to parse Postgres vector string "[0.1,0.2]"
				var stringArray []float32
				_ = json.Unmarshal([]byte(embBytes), &stringArray)
				rec.Embedding = stringArray
			}
		}
		results = append(results, &rec)
	}
	return results, nil
}
