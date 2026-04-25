package autodream

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"math"
	"sort"

	"github.com/onehumancorp/mono/src/server/db"
)

type KnowledgeRecord struct {
	ID        string
	Content   string
	Metadata  map[string]any
	Embedding []float32
}

type VectorStore interface {
	Store(ctx context.Context, id string, vector []float32, metadata map[string]any, content string) error
	Search(ctx context.Context, vector []float32, limit int) ([]*KnowledgeRecord, error)
}

type PGVectorStore struct {
	db db.Provider
}

func NewPGVectorStore(db db.Provider) *PGVectorStore {
	return &PGVectorStore{db: db}
}

func (s *PGVectorStore) Store(ctx context.Context, id string, vector []float32, metadata map[string]any, content string) error {
	metaBytes, err := json.Marshal(metadata)
	if err != nil {
		return fmt.Errorf("failed to marshal metadata: %w", err)
	}

	embBytes, err := json.Marshal(vector)
	if err != nil {
		return fmt.Errorf("failed to marshal embedding: %w", err)
	}

	query := `
		INSERT INTO knowledge_base (id, content, metadata, embedding)
		VALUES ($1, $2, $3, $4::vector)
		ON CONFLICT (id) DO UPDATE SET
			content = EXCLUDED.content,
			metadata = EXCLUDED.metadata,
			embedding = EXCLUDED.embedding
	`
	_, err = s.db.Exec(ctx, query, id, content, string(metaBytes), string(embBytes))
	return err
}

func (s *PGVectorStore) Search(ctx context.Context, vector []float32, limit int) ([]*KnowledgeRecord, error) {
	embBytes, err := json.Marshal(vector)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal embedding: %w", err)
	}

	query := `
		SELECT id, content, metadata, embedding::text
		FROM knowledge_base
		ORDER BY embedding <-> $1::vector
		LIMIT $2
	`
	rows, err := s.db.Query(ctx, query, string(embBytes), limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var records []*KnowledgeRecord
	for rows.Next() {
		var r KnowledgeRecord
		var metaStr sql.NullString
		var embStr sql.NullString
		if err := rows.Scan(&r.ID, &r.Content, &metaStr, &embStr); err != nil {
			return nil, err
		}
		if metaStr.Valid {
			if err := json.Unmarshal([]byte(metaStr.String), &r.Metadata); err != nil {
				return nil, err
			}
		}
		if embStr.Valid {
			if err := json.Unmarshal([]byte(embStr.String), &r.Embedding); err != nil {
				return nil, err
			}
		}
		records = append(records, &r)
	}
	return records, rows.Err()
}

type SQLiteVectorStore struct {
	db db.Provider
}

func NewSQLiteVectorStore(db db.Provider) *SQLiteVectorStore {
	return &SQLiteVectorStore{db: db}
}

func (s *SQLiteVectorStore) Store(ctx context.Context, id string, vector []float32, metadata map[string]any, content string) error {
	metaBytes, err := json.Marshal(metadata)
	if err != nil {
		return fmt.Errorf("failed to marshal metadata: %w", err)
	}

	embBytes, err := json.Marshal(vector)
	if err != nil {
		return fmt.Errorf("failed to marshal embedding: %w", err)
	}

	query := `
		INSERT INTO knowledge_base (id, content, metadata, embedding)
		VALUES ($1, $2, $3, $4)
		ON CONFLICT(id) DO UPDATE SET
			content=excluded.content,
			metadata=excluded.metadata,
			embedding=excluded.embedding
	`
	_, err = s.db.Exec(ctx, query, id, content, string(metaBytes), string(embBytes))
	return err
}

type scoredRecord struct {
	record *KnowledgeRecord
	score  float32
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

func (s *SQLiteVectorStore) Search(ctx context.Context, vector []float32, limit int) ([]*KnowledgeRecord, error) {
	// Fallback implementation: naive in-memory dot-product / cosine similarity
	query := `
		SELECT id, content, metadata, embedding
		FROM knowledge_base
	`
	rows, err := s.db.Query(ctx, query)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var allRecords []scoredRecord
	for rows.Next() {
		var r KnowledgeRecord
		var metaStr sql.NullString
		var embStr sql.NullString
		if err := rows.Scan(&r.ID, &r.Content, &metaStr, &embStr); err != nil {
			return nil, err
		}
		if metaStr.Valid {
			if err := json.Unmarshal([]byte(metaStr.String), &r.Metadata); err != nil {
				return nil, err
			}
		}
		if embStr.Valid {
			if err := json.Unmarshal([]byte(embStr.String), &r.Embedding); err != nil {
				return nil, err
			}
		}
		score := cosineSimilarity(vector, r.Embedding)
		allRecords = append(allRecords, scoredRecord{record: &r, score: score})
	}

	if err := rows.Err(); err != nil {
		return nil, err
	}

	sort.Slice(allRecords, func(i, j int) bool {
		return allRecords[i].score > allRecords[j].score
	})

	var records []*KnowledgeRecord
	for i := 0; i < len(allRecords) && i < limit; i++ {
		records = append(records, allRecords[i].record)
	}

	return records, nil
}
