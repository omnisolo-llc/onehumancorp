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

func (r *VectorRepository) SemanticSearch(ctx context.Context, organizationID string, queryEmbedding []float32, limit int) ([]*EmbeddingRecord, error) {
	embBytes, err := json.Marshal(queryEmbedding)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal query embedding: %w", err)
	}

	var query string
	if r.db.IsSQLite() {
		query = `
			SELECT id, organization_id, memory_type, content, embedding, created_at, source_task_id
			FROM autodream_memories_master
			WHERE organization_id = $1
			ORDER BY vec_distance(embedding, $2) ASC
			LIMIT $3
		`
	} else {
		query = `
			SELECT id, organization_id, memory_type, content, embedding, created_at, source_task_id
			FROM autodream_memories_master
			WHERE organization_id = $1
			ORDER BY embedding <=> $2::vector
			LIMIT $3
		`
	}

	var rows db.Rows
	if r.db.IsSQLite() {
		rows, err = r.db.Query(ctx, query, organizationID, embBytes, limit)
	} else {
		rows, err = r.db.Query(ctx, query, organizationID, string(embBytes), limit)
	}
	if err != nil {
		return nil, fmt.Errorf("failed to query vector repository: %w", err)
	}
	defer rows.Close()

	var results []*EmbeddingRecord
	for rows.Next() {
		var rec EmbeddingRecord
		var dbEmbBytes interface{} // Can be string or []byte
		var srcTaskID *string

		if err := rows.Scan(
			&rec.ID,
			&rec.OrganizationID,
			&rec.MemoryType,
			&rec.Content,
			&dbEmbBytes,
			&rec.CreatedAt,
			&srcTaskID,
		); err != nil {
			return nil, fmt.Errorf("failed to scan memory record: %w", err)
		}

		if srcTaskID != nil {
			rec.SourceTaskID = *srcTaskID
		}

		switch v := dbEmbBytes.(type) {
		case string:
			if err := json.Unmarshal([]byte(v), &rec.Embedding); err != nil {
				return nil, fmt.Errorf("failed to unmarshal db embedding from string: %w", err)
			}
		case []byte:
			if err := json.Unmarshal(v, &rec.Embedding); err != nil {
				return nil, fmt.Errorf("failed to unmarshal db embedding from bytes: %w", err)
			}
		case nil:
			// No embedding
		default:
			return nil, fmt.Errorf("unknown type for db embedding: %T", dbEmbBytes)
		}

		results = append(results, &rec)
	}

	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("error during row iteration: %w", err)
	}

	return results, nil
}
