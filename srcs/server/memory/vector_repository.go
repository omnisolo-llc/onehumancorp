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
	TaskID         string
	Content        string
	Embedding      []float32
	SourceType     string
	CreatedAt      time.Time
}

type VectorRepository struct {
	db db.Provider
}

func NewVectorRepository(db db.Provider) *VectorRepository {
	return &VectorRepository{db: db}
}

func (r *VectorRepository) formatFloat32SliceForVector(vec []float32) string {
	b, _ := json.Marshal(vec)
	return string(b)
}

func (r *VectorRepository) Upsert(ctx context.Context, record *EmbeddingRecord) error {
	embStr := r.formatFloat32SliceForVector(record.Embedding)

	var query string
	if r.db.IsSQLite() {
		query = `
			INSERT INTO autodream_memories (id, organization_id, task_id, content, embedding, source_type, created_at)
			VALUES ($1, $2, $3, $4, $5, $6, $7)
		`
	} else {
		query = `
			INSERT INTO autodream_memories (id, organization_id, task_id, content, embedding, source_type, created_at)
			VALUES ($1, $2, $3, $4, $5::vector, $6, $7)
		`
	}

	_, err := r.db.Exec(ctx, query, record.ID, record.OrganizationID, record.TaskID, record.Content, embStr, record.SourceType, record.CreatedAt)
	return err
}

func (r *VectorRepository) SemanticSearch(ctx context.Context, organizationID string, queryEmbedding []float32, limit int) ([]*EmbeddingRecord, error) {
	return r.SemanticSearchWithThreshold(ctx, organizationID, queryEmbedding, limit, 1.0)
}

func (r *VectorRepository) SemanticSearchWithThreshold(ctx context.Context, organizationID string, queryEmbedding []float32, limit int, threshold float64) ([]*EmbeddingRecord, error) {
	var query string
	var args []interface{}
	embStr := r.formatFloat32SliceForVector(queryEmbedding)

	if r.db.IsSQLite() {
		// Use sqlite-vec standard distance function vec_distance_cosine
		query = `
			SELECT id, organization_id, task_id, content, source_type, created_at
			FROM autodream_memories
			WHERE organization_id = $1 AND vec_distance_cosine(embedding, $2) <= $3
			ORDER BY vec_distance_cosine(embedding, $2) ASC
			LIMIT $4
		`
		args = []interface{}{organizationID, embStr, threshold, limit}
	} else {
		query = `
			SELECT id, organization_id, task_id, content, source_type, created_at
			FROM autodream_memories
			WHERE organization_id = $1 AND embedding <=> $2::vector <= $3
			ORDER BY embedding <=> $2::vector ASC
			LIMIT $4
		`
		args = []interface{}{organizationID, embStr, threshold, limit}
	}

	rows, err := r.db.Query(ctx, query, args...)
	if err != nil {
		return nil, fmt.Errorf("failed to execute semantic search: %w", err)
	}
	defer rows.Close()

	var results []*EmbeddingRecord
	for rows.Next() {
		var rec EmbeddingRecord
		var taskID sql.NullString
		if err := rows.Scan(&rec.ID, &rec.OrganizationID, &taskID, &rec.Content, &rec.SourceType, &rec.CreatedAt); err != nil {
			return nil, fmt.Errorf("failed to scan row: %w", err)
		}
		if taskID.Valid {
			rec.TaskID = taskID.String
		}
		results = append(results, &rec)
	}
	return results, nil
}

func (r *VectorRepository) Prune(ctx context.Context, threshold time.Time) error {
	query := `DELETE FROM autodream_memories WHERE created_at < $1`
	_, err := r.db.Exec(ctx, query, threshold)
	return err
}

func (r *VectorRepository) Delete(ctx context.Context, id string) error {
	query := `DELETE FROM autodream_memories WHERE id = $1`
	_, err := r.db.Exec(ctx, query, id)
	return err
}

func (r *VectorRepository) GetByID(ctx context.Context, id string, fetchEmbedding bool) (*EmbeddingRecord, error) {
	var query string
	if fetchEmbedding {
		query = `
			SELECT id, organization_id, task_id, content, source_type, created_at, embedding
			FROM autodream_memories
			WHERE id = $1
		`
	} else {
		query = `
			SELECT id, organization_id, task_id, content, source_type, created_at
			FROM autodream_memories
			WHERE id = $1
		`
	}

	var rec EmbeddingRecord
	var taskID sql.NullString

	if fetchEmbedding {
		var embStr string
		err := r.db.QueryRow(ctx, query, id).Scan(&rec.ID, &rec.OrganizationID, &taskID, &rec.Content, &rec.SourceType, &rec.CreatedAt, &embStr)
		if err != nil {
			return nil, err
		}
		if taskID.Valid {
			rec.TaskID = taskID.String
		}
		var emb []float32
		if err := json.Unmarshal([]byte(embStr), &emb); err == nil {
			rec.Embedding = emb
		}
	} else {
		err := r.db.QueryRow(ctx, query, id).Scan(&rec.ID, &rec.OrganizationID, &taskID, &rec.Content, &rec.SourceType, &rec.CreatedAt)
		if err != nil {
			return nil, err
		}
		if taskID.Valid {
			rec.TaskID = taskID.String
		}
	}

	return &rec, nil
}
