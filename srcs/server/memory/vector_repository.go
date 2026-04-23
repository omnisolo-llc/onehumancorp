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
	`
	// Use INSERT ... ON CONFLICT if UPSERT logic is required. Keeping it simple as per instructions.

	if r.db.IsSQLite() {
		query = `
			INSERT INTO autodream_memories_master (id, organization_id, memory_type, content, embedding, created_at, source_task_id)
			VALUES (?, ?, ?, ?, ?, ?, ?)
		`
		_, err = r.db.Exec(ctx, query, record.ID, record.OrganizationID, record.MemoryType, record.Content, embBytes, record.CreatedAt, record.SourceTaskID)
	} else {
		_, err = r.db.Exec(ctx, query, record.ID, record.OrganizationID, record.MemoryType, record.Content, embBytes, record.CreatedAt, record.SourceTaskID)
	}

	return err
}

func (r *VectorRepository) SemanticSearch(ctx context.Context, organizationID string, queryEmbedding []float32, limit int) ([]*EmbeddingRecord, error) {
	embBytes, err := json.Marshal(queryEmbedding)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal query embedding: %w", err)
	}

	var rows db.Rows
	var qErr error

	if r.db.IsSQLite() {
		// Fallback for SQLite: no true vector search, just return recent memories for the organization
		query := `
			SELECT id, organization_id, memory_type, content, embedding, created_at, source_task_id
			FROM autodream_memories_master
			WHERE organization_id = ?
			ORDER BY created_at DESC
			LIMIT ?
		`
		rows, qErr = r.db.Query(ctx, query, organizationID, limit)
	} else {
		// Postgres with pgvector: we have embedding as BYTEA and convert it to vector for distance calculation
		// Assuming vector type is accessible via convert_from($2, 'UTF8')::vector in our hybrid setup.
		// But in our migrations it says embedding is BYTEA. We need to cast json bytes.
		query := `
			SELECT id, organization_id, memory_type, content, embedding, created_at, source_task_id
			FROM autodream_memories_master
			WHERE organization_id = $1
			ORDER BY convert_from(embedding, 'UTF8')::vector <-> convert_from($2, 'UTF8')::vector ASC
			LIMIT $3
		`
		rows, qErr = r.db.Query(ctx, query, organizationID, embBytes, limit)
	}

	if qErr != nil {
		return nil, fmt.Errorf("failed to query memories: %w", qErr)
	}
	if rows != nil {
		defer rows.Close()
	} else {
		return nil, nil
	}

	var results []*EmbeddingRecord
	for rows.Next() {
		var rec EmbeddingRecord
		var eBytes []byte
		var sourceTaskID *string
		if err := rows.Scan(&rec.ID, &rec.OrganizationID, &rec.MemoryType, &rec.Content, &eBytes, &rec.CreatedAt, &sourceTaskID); err != nil {
			return nil, fmt.Errorf("failed to scan memory record: %w", err)
		}
		if eBytes != nil {
			var emb []float32
			if err := json.Unmarshal(eBytes, &emb); err == nil {
				rec.Embedding = emb
			}
		}
		if sourceTaskID != nil {
			rec.SourceTaskID = *sourceTaskID
		}
		results = append(results, &rec)
	}
	return results, nil
}

func (r *VectorRepository) Delete(ctx context.Context, id string, organizationID string) error {
	var query string
	if r.db.IsSQLite() {
		query = `DELETE FROM autodream_memories_master WHERE id = ? AND organization_id = ?`
	} else {
		query = `DELETE FROM autodream_memories_master WHERE id = $1 AND organization_id = $2`
	}
	_, err := r.db.Exec(ctx, query, id, organizationID)
	return err
}

func (r *VectorRepository) PruneStaleContext(ctx context.Context, organizationID string, olderThan time.Time) (int64, error) {
	var query string
	// Never prune permanent facts
	if r.db.IsSQLite() {
		query = `
			DELETE FROM autodream_memories_master
			WHERE organization_id = ? AND created_at < ? AND memory_type NOT IN ('PERMANENT_FACT', 'OWNER_OVERRIDE', 'RESOLVED_FACT')
		`
	} else {
		query = `
			DELETE FROM autodream_memories_master
			WHERE organization_id = $1 AND created_at < $2 AND memory_type NOT IN ('PERMANENT_FACT', 'OWNER_OVERRIDE', 'RESOLVED_FACT')
		`
	}

	rowsAffected, err := r.db.Exec(ctx, query, organizationID, olderThan)
	if err != nil {
		return 0, fmt.Errorf("failed to prune stale context: %w", err)
	}
	return rowsAffected, nil
}
