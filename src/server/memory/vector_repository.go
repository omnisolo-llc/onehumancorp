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
	AgentID        string
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

func (r *VectorRepository) Upsert(ctx context.Context, record *EmbeddingRecord) error {
	embBytes, err := json.Marshal(record.Embedding)
	if err != nil {
		return fmt.Errorf("failed to marshal embedding: %w", err)
	}

	var query string
	if r.db.IsSQLite() {
		query = `
			INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type, created_at)
			VALUES ($1, $2, $3, $4, $5, $6, $7)
			ON CONFLICT(id) DO UPDATE SET
				content=excluded.content,
				embedding=excluded.embedding,
				created_at=excluded.created_at
		`
	} else {
		// Postgres uses pgvector which accepts a string formatted as '[1,2,3]'
		query = `
			INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type, created_at)
			VALUES ($1, $2, $3, $4, $5::vector, $6, $7)
			ON CONFLICT(id) DO UPDATE SET
				content=excluded.content,
				embedding=excluded.embedding,
				created_at=excluded.created_at
		`
	}

	_, err = r.db.Exec(ctx, query, record.ID, record.OrganizationID, record.AgentID, record.Content, string(embBytes), record.SourceType, record.CreatedAt)
	return err
}

func (r *VectorRepository) SemanticSearch(ctx context.Context, organizationID string, queryEmbedding []float32, limit int) ([]*EmbeddingRecord, error) {
	if r.db.IsSQLite() {
		// Fallback for SQLite: return latest memories since vector similarity is unavailable.
		query := `
			SELECT id, organization_id, COALESCE(agent_id, ''), content, embedding, source_type, created_at
			FROM consolidated_memory
			WHERE organization_id = $1
			ORDER BY created_at DESC
			LIMIT $2
		`
		rows, err := r.db.Query(ctx, query, organizationID, limit)
		if err != nil {
			return nil, fmt.Errorf("failed to query sqlite memory: %w", err)
		}
		defer rows.Close()

		var results []*EmbeddingRecord
		for rows.Next() {
			var rec EmbeddingRecord
			var embStr string
			if err := rows.Scan(&rec.ID, &rec.OrganizationID, &rec.AgentID, &rec.Content, &embStr, &rec.SourceType, &rec.CreatedAt); err != nil {
				continue
			}
			json.Unmarshal([]byte(embStr), &rec.Embedding)
			results = append(results, &rec)
		}
		return results, nil
	}

	embBytes, err := json.Marshal(queryEmbedding)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal query embedding: %w", err)
	}
	embStr := string(embBytes)

	query := `
		SELECT id, organization_id, COALESCE(agent_id, ''), content, embedding, source_type, created_at
		FROM consolidated_memory
		WHERE organization_id = $1
		ORDER BY embedding <-> $2::vector
		LIMIT $3
	`
	rows, err := r.db.Query(ctx, query, organizationID, embStr, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to query pgvector memory: %w", err)
	}
	defer rows.Close()

	var results []*EmbeddingRecord
	for rows.Next() {
		var rec EmbeddingRecord
		var embStrRes string
		if err := rows.Scan(&rec.ID, &rec.OrganizationID, &rec.AgentID, &rec.Content, &embStrRes, &rec.SourceType, &rec.CreatedAt); err != nil {
			continue
		}
		json.Unmarshal([]byte(embStrRes), &rec.Embedding)
		results = append(results, &rec)
	}
	return results, nil
}

func (r *VectorRepository) PruneStale(ctx context.Context, olderThan time.Time) error {
	// Only prune records explicitly marked as TASK_SUMMARY to be safe,
	// or prune records that haven't been accessed recently (if we had last_accessed_at)
	// Given the schema, we prune explicitly task summaries that are older than threshold.
	query := `DELETE FROM consolidated_memory WHERE created_at < $1 AND source_type = 'TASK_SUMMARY'`
	_, err := r.db.Exec(ctx, query, olderThan)
	return err
}

func (r *VectorRepository) Delete(ctx context.Context, id string) error {
	query := `DELETE FROM consolidated_memory WHERE id = $1`
	_, err := r.db.Exec(ctx, query, id)
	return err
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

