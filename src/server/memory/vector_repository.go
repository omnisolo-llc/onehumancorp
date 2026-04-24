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
	OrganizationID string
	MemoryType   string
	Content      string
	Embedding    []float32
	CreatedAt    time.Time
	SourceTaskID string
}

type ConsolidatedMemoryRecord struct {
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

	query := `
		INSERT INTO autodream_memories_master (id, organization_id, memory_type, content, embedding, created_at, source_task_id)
		VALUES ($1, $2, $3, $4, $5, $6, $7)
	`
	_, err = r.db.Exec(ctx, query, record.ID, record.OrganizationID, record.MemoryType, record.Content, embBytes, record.CreatedAt, record.SourceTaskID)
	return err
}

func (r *VectorRepository) SemanticSearch(ctx context.Context, organizationID string, queryEmbedding []float32, limit int) ([]*EmbeddingRecord, error) {
	return nil, nil
}

func (r *VectorRepository) UpsertConsolidatedMemory(ctx context.Context, record *ConsolidatedMemoryRecord) error {
	embBytes, err := json.Marshal(record.Embedding)
	if err != nil {
		return fmt.Errorf("failed to marshal embedding: %w", err)
	}

	query := `
		INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type, created_at)
		VALUES ($1, $2, $3, $4, $5, $6, $7)
		ON CONFLICT (id) DO UPDATE SET
			content = EXCLUDED.content,
			embedding = EXCLUDED.embedding,
			source_type = EXCLUDED.source_type,
			created_at = EXCLUDED.created_at
	`
	_, err = r.db.Exec(ctx, query, record.ID, record.OrganizationID, record.AgentID, record.Content, embBytes, record.SourceType, record.CreatedAt)
	return err
}

func (r *VectorRepository) SearchConsolidatedMemories(ctx context.Context, organizationID string, agentID string, queryEmbedding []float32, limit int) ([]*ConsolidatedMemoryRecord, error) {
	embBytes, err := json.Marshal(queryEmbedding)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal query embedding: %w", err)
	}

	var query string
	var rows db.Rows

	if r.db.IsSQLite() {
		// Use UDF vec_distance_cosine
		query = `
			SELECT id, organization_id, agent_id, content, embedding, source_type, created_at
			FROM consolidated_memory
			WHERE organization_id = $1 AND (agent_id = $2 OR agent_id = '')
			ORDER BY vec_distance_cosine(embedding, $3) ASC
			LIMIT $4
		`
		rows, err = r.db.Query(ctx, query, organizationID, agentID, string(embBytes), limit)
	} else {
		// pgvector
		query = `
			SELECT id, organization_id, agent_id, content, embedding, source_type, created_at
			FROM consolidated_memory
			WHERE organization_id = $1 AND (agent_id = $2 OR agent_id = '')
			ORDER BY embedding <-> $3::vector
			LIMIT $4
		`
		rows, err = r.db.Query(ctx, query, organizationID, agentID, string(embBytes), limit)
	}

	if err != nil {
		return nil, fmt.Errorf("failed to query consolidated_memory: %w", err)
	}
	defer rows.Close()

	var results []*ConsolidatedMemoryRecord
	for rows.Next() {
		var rec ConsolidatedMemoryRecord
		var embData []byte
		var aID *string

		if err := rows.Scan(&rec.ID, &rec.OrganizationID, &aID, &rec.Content, &embData, &rec.SourceType, &rec.CreatedAt); err != nil {
			return nil, fmt.Errorf("failed to scan row: %w", err)
		}

		if aID != nil {
			rec.AgentID = *aID
		}

		if len(embData) > 0 {
			var emb []float32
			if err := json.Unmarshal(embData, &emb); err == nil {
				rec.Embedding = emb
			}
		}

		results = append(results, &rec)
	}
	return results, nil
}

func (r *VectorRepository) DeleteConsolidatedMemory(ctx context.Context, id string, organizationID string) error {
	query := `DELETE FROM consolidated_memory WHERE id = $1 AND organization_id = $2`
	_, err := r.db.Exec(ctx, query, id, organizationID)
	return err
}

func (r *VectorRepository) GetOldMemories(ctx context.Context, organizationID string, cutoff time.Time, limit int) ([]*ConsolidatedMemoryRecord, error) {
	query := `
		SELECT id, organization_id, agent_id, content, embedding, source_type, created_at
		FROM consolidated_memory
		WHERE organization_id = $1 AND created_at < $2
		LIMIT $3
	`
	rows, err := r.db.Query(ctx, query, organizationID, cutoff, limit)
	if err != nil {
		return nil, fmt.Errorf("failed to query old consolidated_memory: %w", err)
	}
	defer rows.Close()

	var results []*ConsolidatedMemoryRecord
	for rows.Next() {
		var rec ConsolidatedMemoryRecord
		var embData []byte
		var aID *string

		if err := rows.Scan(&rec.ID, &rec.OrganizationID, &aID, &rec.Content, &embData, &rec.SourceType, &rec.CreatedAt); err != nil {
			return nil, fmt.Errorf("failed to scan row: %w", err)
		}
		if aID != nil {
			rec.AgentID = *aID
		}
		if len(embData) > 0 {
			var emb []float32
			if err := json.Unmarshal(embData, &emb); err == nil {
				rec.Embedding = emb
			}
		}
		results = append(results, &rec)
	}
	return results, nil
}
