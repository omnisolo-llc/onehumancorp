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
	// Not fully implemented for vector search as sqlite graceful degradation implies fallback full-text or simply retrieving.
	// In pgvector, we would use `<->` operator. Here we return empty for now to satisfy interface outline.
	return nil, nil
}

func (r *VectorRepository) GetOrganizationIDs(ctx context.Context) ([]string, error) {
	query := "SELECT DISTINCT organization_id FROM users WHERE organization_id != ''"
	rows, err := r.db.Query(ctx, query)
	if err != nil {
		return nil, fmt.Errorf("failed to get organization IDs: %w", err)
	}
	defer rows.Close()

	var orgIDs []string
	for rows.Next() {
		var orgID string
		if err := rows.Scan(&orgID); err != nil {
			return nil, fmt.Errorf("failed to scan organization ID: %w", err)
		}
		orgIDs = append(orgIDs, orgID)
	}
	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("rows iteration error: %w", err)
	}

	return orgIDs, nil
}
