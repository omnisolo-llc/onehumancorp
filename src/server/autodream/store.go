package autodream

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"

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

func (s *SQLiteVectorStore) Search(ctx context.Context, vector []float32, limit int) ([]*KnowledgeRecord, error) {
	embBytes, err := json.Marshal(vector)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal query embedding: %w", err)
	}

	query := `
		SELECT id, content, metadata, embedding
		FROM knowledge_base
		ORDER BY vec_distance_cosine(embedding, $1) ASC
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

	if err := rows.Err(); err != nil {
		return nil, err
	}

	return records, nil
}
