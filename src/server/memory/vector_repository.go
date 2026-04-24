package memory

import (
	"context"
	"encoding/json"
	"fmt"
	"time"
	"math"
	"sort"

	"github.com/onehumancorp/mono/src/server/db"
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
		INSERT INTO autodream_memories_master (id, organization_id, memory_type, content, embedding, created_at, source_task_id)
		VALUES ($1, $2, $3, $4, $5, $6, $7)
	`
	_, err = r.db.Exec(ctx, query, record.ID, record.OrganizationID, record.MemoryType, record.Content, embBytes, record.CreatedAt, record.SourceTaskID)
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
		WHERE id = $5 AND organization_id = $6
	`
	_, err = r.db.Exec(ctx, query, record.Content, embBytes, record.CreatedAt, record.SourceTaskID, record.ID, record.OrganizationID)
	return err
}

func (r *VectorRepository) DeleteOldMemories(ctx context.Context, memoryType string, retention time.Duration) error {
	threshold := time.Now().Add(-retention)
	query := `DELETE FROM autodream_memories_master WHERE memory_type = $1 AND created_at < $2`
	_, err := r.db.Exec(ctx, query, memoryType, threshold)
	return err
}

// cosineSimilarity computes cosine similarity between two vectors.
func cosineSimilarity(a, b []float32) float64 {
	var dotProduct, normA, normB float64
	for i := 0; i < len(a) && i < len(b); i++ {
		dotProduct += float64(a[i] * b[i])
		normA += float64(a[i] * a[i])
		normB += float64(b[i] * b[i])
	}
	if normA == 0 || normB == 0 {
		return 0
	}
	return dotProduct / (math.Sqrt(normA) * math.Sqrt(normB))
}

func (r *VectorRepository) SemanticSearch(ctx context.Context, organizationID string, queryEmbedding []float32, limit int, minScore float64) ([]SearchResult, error) {
	if r.db.IsSQLite() {
		// SQLite fallback: fetch all and compute in memory
		query := `
			SELECT id, organization_id, memory_type, content, embedding, created_at, source_task_id
			FROM autodream_memories_master
			WHERE organization_id = $1
		`
		rows, err := r.db.Query(ctx, query, organizationID)
		if err != nil {
			return nil, fmt.Errorf("failed to query memories: %w", err)
		}
		defer rows.Close()

		var results []SearchResult
		for rows.Next() {
			var rec EmbeddingRecord
			var embBytes []byte
			var createdAtStr string
			var sourceTaskID *string
			if err := rows.Scan(&rec.ID, &rec.OrganizationID, &rec.MemoryType, &rec.Content, &embBytes, &createdAtStr, &sourceTaskID); err != nil {
				continue
			}

			if sourceTaskID != nil {
				rec.SourceTaskID = *sourceTaskID
			}

			if embBytes != nil {
				var emb []float32
				if err := json.Unmarshal(embBytes, &emb); err == nil {
					rec.Embedding = emb
				}
			}

			if t, err := time.Parse("2006-01-02 15:04:05", createdAtStr); err == nil {
				rec.CreatedAt = t
			} else if t, err := time.Parse(time.RFC3339, createdAtStr); err == nil {
				rec.CreatedAt = t
			}

			score := cosineSimilarity(queryEmbedding, rec.Embedding)
			if score >= minScore {
				results = append(results, SearchResult{Record: &rec, Score: score})
			}
		}

		sort.Slice(results, func(i, j int) bool {
			return results[i].Score > results[j].Score // descending
		})

		if len(results) > limit {
			results = results[:limit]
		}
		return results, nil
	}

	// PostgreSQL pgvector
	embBytes, err := json.Marshal(queryEmbedding)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal query embedding: %w", err)
	}

	// For pgvector, we use `<->` (L2 distance), `<#>` (inner product), or `<=>` (cosine distance).
	// We want cosine similarity, which is 1 - cosine distance.
	// So `1 - (embedding <=> $2::vector) AS score`
	query := `
		SELECT * FROM (
			SELECT id, organization_id, memory_type, content, embedding, created_at, source_task_id,
				   1 - (embedding <=> $2::vector) AS score
			FROM autodream_memories_master
			WHERE organization_id = $1
			ORDER BY embedding <=> $2::vector
			LIMIT $3
		) sub WHERE score >= $4
	`

	rows, err := r.db.Query(ctx, query, organizationID, string(embBytes), limit, minScore)
	if err != nil {
		return nil, fmt.Errorf("failed to query memories: %w", err)
	}
	defer rows.Close()

	var results []SearchResult
	for rows.Next() {
		var rec EmbeddingRecord
		var score float64
		var sourceTaskID *string
		// pgvector returns vector as a string, but since we are using Scan, we can just receive it into a string or byte array
		var embStr string
		var createdAt time.Time
		if err := rows.Scan(&rec.ID, &rec.OrganizationID, &rec.MemoryType, &rec.Content, &embStr, &createdAt, &sourceTaskID, &score); err != nil {
			return nil, fmt.Errorf("failed to scan row: %w", err)
		}
		if sourceTaskID != nil {
			rec.SourceTaskID = *sourceTaskID
		}
		// Unmarshal the string back to []float32
		var emb []float32
		if err := json.Unmarshal([]byte(embStr), &emb); err == nil {
			rec.Embedding = emb
		}
		rec.CreatedAt = createdAt
		results = append(results, SearchResult{Record: &rec, Score: score})
	}
	return results, nil
}
