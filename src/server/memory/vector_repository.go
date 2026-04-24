package memory

import (
	"context"
	"encoding/json"
	"fmt"
	"math"
	"sort"
	"time"

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
		ON CONFLICT(id) DO UPDATE SET
			content = excluded.content,
			embedding = excluded.embedding,
			created_at = excluded.created_at,
			source_task_id = excluded.source_task_id
	`

	_, err = r.db.Exec(ctx, query, record.ID, record.OrganizationID, record.MemoryType, record.Content, embBytes, record.CreatedAt, record.SourceTaskID)
	return err
}

func cosineSimilarity(a, b []float32) float64 {
	if len(a) != len(b) {
		return 0
	}
	var dot, magA, magB float64
	for i := range a {
		dot += float64(a[i]) * float64(b[i])
		magA += float64(a[i]) * float64(a[i])
		magB += float64(b[i]) * float64(b[i])
	}
	if magA == 0 || magB == 0 {
		return 0
	}
	return dot / (math.Sqrt(magA) * math.Sqrt(magB))
}

func (r *VectorRepository) SemanticSearch(ctx context.Context, organizationID string, queryEmbedding []float32, limit int) ([]*EmbeddingRecord, error) {
	embBytes, err := json.Marshal(queryEmbedding)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal query embedding: %w", err)
	}

	var query string
	var rows db.Rows

	if r.db.IsSQLite() {
		query = `
			SELECT id, organization_id, memory_type, content, embedding, created_at, source_task_id
			FROM autodream_memories_master
			WHERE organization_id = $1
		`
		rows, err = r.db.Query(ctx, query, organizationID)
	} else {
		query = `
			SELECT id, organization_id, memory_type, content, embedding, created_at, source_task_id
			FROM autodream_memories_master
			WHERE organization_id = $1
			ORDER BY embedding <-> $2::vector
			LIMIT $3
		`
		rows, err = r.db.Query(ctx, query, organizationID, string(embBytes), limit)
	}

	if err != nil {
		return nil, fmt.Errorf("failed to query memories: %w", err)
	}
	defer rows.Close()

	var records []*EmbeddingRecord
	for rows.Next() {
		var rec EmbeddingRecord
		var embData []byte
		err := rows.Scan(
			&rec.ID,
			&rec.OrganizationID,
			&rec.MemoryType,
			&rec.Content,
			&embData,
			&rec.CreatedAt,
			&rec.SourceTaskID,
		)
		if err != nil {
			return nil, fmt.Errorf("failed to scan memory record: %w", err)
		}

		if len(embData) > 0 {
			if err := json.Unmarshal(embData, &rec.Embedding); err != nil {
				// Some data might be plain string depending on the vector driver
			}
		}
		records = append(records, &rec)
	}

	if r.db.IsSQLite() {
		type scoredRecord struct {
			record *EmbeddingRecord
			score  float64
		}
		var scored []scoredRecord
		for _, rec := range records {
			score := cosineSimilarity(queryEmbedding, rec.Embedding)
			scored = append(scored, scoredRecord{record: rec, score: score})
		}
		sort.Slice(scored, func(i, j int) bool {
			return scored[i].score > scored[j].score // higher similarity first
		})
		var top []*EmbeddingRecord
		for i := 0; i < len(scored) && i < limit; i++ {
			top = append(top, scored[i].record)
		}
		return top, nil
	}

	return records, nil
}

func (r *VectorRepository) PruneStaleContext(ctx context.Context, organizationID string, olderThan time.Time) (int64, error) {
	query := `DELETE FROM autodream_memories_master WHERE organization_id = $1 AND created_at < $2`
	return r.db.Exec(ctx, query, organizationID, olderThan)
}
