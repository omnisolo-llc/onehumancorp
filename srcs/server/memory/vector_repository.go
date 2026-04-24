package memory

import (
	"context"
	"encoding/json"
	"fmt"
	"math"
	"sort"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type EmbeddingRecord struct {
	ID             string
	OrganizationID string
	AgentID        *string
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

// Upsert inserts a new memory into consolidated_memory. Since conflict resolution is handled separately,
// this acts as an append-only operation, though it can use ON CONFLICT if needed.
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
				content = excluded.content,
				embedding = excluded.embedding,
				created_at = excluded.created_at
		`
	} else {
		query = `
			INSERT INTO consolidated_memory (id, organization_id, agent_id, content, embedding, source_type, created_at)
			VALUES ($1, $2, $3, $4, $5, $6, $7)
			ON CONFLICT(id) DO UPDATE SET
				content = excluded.content,
				embedding = excluded.embedding,
				created_at = excluded.created_at
		`
	}

	_, err = r.db.Exec(ctx, query, record.ID, record.OrganizationID, record.AgentID, record.Content, string(embBytes), record.MemoryType, record.CreatedAt)
	return err
}

func (r *VectorRepository) SemanticSearch(ctx context.Context, organizationID string, queryEmbedding []float32, limit int) ([]*EmbeddingRecord, error) {
	if r.db.IsSQLite() {
		query := `SELECT id, organization_id, agent_id, source_type, content, embedding, created_at FROM consolidated_memory WHERE organization_id = $1`
		rows, err := r.db.Query(ctx, query, organizationID)
		if err != nil {
			return nil, err
		}
		defer rows.Close()

		type scoredRecord struct {
			record *EmbeddingRecord
			score  float32
		}
		var results []scoredRecord

		for rows.Next() {
			var id, orgId, sourceType, content string
			var agentId *string
			var embeddingStr string
			var createdAt time.Time

			if err := rows.Scan(&id, &orgId, &agentId, &sourceType, &content, &embeddingStr, &createdAt); err != nil {
				continue
			}

			var emb []float32
			if err := json.Unmarshal([]byte(embeddingStr), &emb); err != nil {
				continue
			}

			rec := &EmbeddingRecord{
				ID:             id,
				OrganizationID: orgId,
				AgentID:        agentId,
				MemoryType:     sourceType,
				Content:        content,
				Embedding:      emb,
				CreatedAt:      createdAt,
			}

			score := cosineSimilarity(queryEmbedding, emb)
			results = append(results, scoredRecord{record: rec, score: score})
		}

		sort.Slice(results, func(i, j int) bool {
			return results[i].score > results[j].score
		})

		if len(results) > limit {
			results = results[:limit]
		}

		var final []*EmbeddingRecord
		for _, r := range results {
			final = append(final, r.record)
		}
		return final, nil
	} else {
		embBytes, err := json.Marshal(queryEmbedding)
		if err != nil {
			return nil, err
		}

		query := `
			SELECT id, organization_id, agent_id, source_type, content, embedding, created_at
			FROM consolidated_memory
			WHERE organization_id = $1
			ORDER BY embedding <-> $2::vector
			LIMIT $3
		`
		rows, err := r.db.Query(ctx, query, organizationID, string(embBytes), limit)
		if err != nil {
			return nil, err
		}
		defer rows.Close()

		var final []*EmbeddingRecord
		for rows.Next() {
			var id, orgId, sourceType, content string
			var agentId *string
			var embeddingStr string
			var createdAt time.Time

			if err := rows.Scan(&id, &orgId, &agentId, &sourceType, &content, &embeddingStr, &createdAt); err != nil {
				continue
			}

			var emb []float32
			if err := json.Unmarshal([]byte(embeddingStr), &emb); err != nil {
			}

			final = append(final, &EmbeddingRecord{
				ID:             id,
				OrganizationID: orgId,
				AgentID:        agentId,
				MemoryType:     sourceType,
				Content:        content,
				Embedding:      emb,
				CreatedAt:      createdAt,
			})
		}
		return final, nil
	}
}

func (r *VectorRepository) Delete(ctx context.Context, id string) error {
	_, err := r.db.Exec(ctx, "DELETE FROM consolidated_memory WHERE id = $1", id)
	return err
}

func (r *VectorRepository) PruneStaleMemories(ctx context.Context, before time.Time) (int64, error) {
	// A simple time-based prune is too aggressive.
	// We should only prune if the memory is explicitly marked or if we have another signal.
	// For now we will not do hard deletes based purely on time. This should be a soft delete or archival logic.
	// We'll update the worker logic to do this carefully.
	return 0, nil
}

func (r *VectorRepository) FindRecentMemories(ctx context.Context, orgID string, since time.Time) ([]*EmbeddingRecord, error) {
	// Utility for the LLM conflict resolution pass
	query := `SELECT id, organization_id, agent_id, source_type, content, embedding, created_at FROM consolidated_memory WHERE organization_id = $1 AND created_at >= $2`
	rows, err := r.db.Query(ctx, query, orgID, since)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var final []*EmbeddingRecord
	for rows.Next() {
		var id, orgId, sourceType, content string
		var agentId *string
		var embeddingStr string
		var createdAt time.Time
		if err := rows.Scan(&id, &orgId, &agentId, &sourceType, &content, &embeddingStr, &createdAt); err != nil {
			continue
		}
		var emb []float32
		if err := json.Unmarshal([]byte(embeddingStr), &emb); err != nil {
		}
		final = append(final, &EmbeddingRecord{
			ID:             id,
			OrganizationID: orgId,
			AgentID:        agentId,
			MemoryType:     sourceType,
			Content:        content,
			Embedding:      emb,
			CreatedAt:      createdAt,
		})
	}
	return final, nil
}

func cosineSimilarity(a, b []float32) float32 {
	if len(a) != len(b) {
		return 0
	}
	var dotProduct, normA, normB float32
	for i := range a {
		dotProduct += a[i] * b[i]
		normA += a[i] * a[i]
		normB += b[i] * b[i]
	}
	if normA == 0 || normB == 0 {
		return 0
	}
	return dotProduct / (float32(math.Sqrt(float64(normA))) * float32(math.Sqrt(float64(normB))))
}
