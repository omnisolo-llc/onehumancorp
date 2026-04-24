package db

import (
	"context"
	"encoding/json"
	"fmt"
	"math"
	"sort"
	"time"
)

// Finding represents an AutoDream architectural finding stored in the database.
type Finding struct {
	ID        string    `json:"id"`
	Timestamp time.Time `json:"timestamp"`
	Content   string    `json:"content"`
	Embedding []float32 `json:"embedding"`
}

// AutoDreamRepository handles storage and retrieval of AutoDream findings.
type AutoDreamRepository struct {
	db Provider
}

// NewAutoDreamRepository creates a new instance of AutoDreamRepository.
func NewAutoDreamRepository(db Provider) *AutoDreamRepository {
	return &AutoDreamRepository{db: db}
}

// Upsert inserts a finding into the database or updates it if it already exists.
func (r *AutoDreamRepository) Upsert(ctx context.Context, finding *Finding) error {
	embBytes, err := json.Marshal(finding.Embedding)
	if err != nil {
		return fmt.Errorf("failed to marshal embedding: %w", err)
	}

	var query string
	var args []any

	if r.db.IsSQLite() {
		query = `
			INSERT INTO autodream_findings (id, timestamp, content, embedding)
			VALUES (?, ?, ?, ?)
			ON CONFLICT(id) DO UPDATE SET
				timestamp = excluded.timestamp,
				content = excluded.content,
				embedding = excluded.embedding
		`
		args = []any{finding.ID, finding.Timestamp, finding.Content, string(embBytes)}
	} else {
		query = `
			INSERT INTO autodream_findings (id, timestamp, content, embedding)
			VALUES ($1, $2, $3, $4::vector)
			ON CONFLICT(id) DO UPDATE SET
				timestamp = EXCLUDED.timestamp,
				content = EXCLUDED.content,
				embedding = EXCLUDED.embedding
		`
		args = []any{finding.ID, finding.Timestamp, finding.Content, string(embBytes)}
	}

	_, err = r.db.Exec(ctx, query, args...)
	return err
}

// Search performs a semantic search for findings based on a query embedding.
func (r *AutoDreamRepository) Search(ctx context.Context, queryEmbedding []float32, limit int) ([]*Finding, error) {
	if r.db.IsSQLite() {
		return r.searchSQLite(ctx, queryEmbedding, limit)
	}
	return r.searchPostgres(ctx, queryEmbedding, limit)
}

func (r *AutoDreamRepository) searchPostgres(ctx context.Context, queryEmbedding []float32, limit int) ([]*Finding, error) {
	embBytes, err := json.Marshal(queryEmbedding)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal query embedding: %w", err)
	}

	// Using <=> for cosine distance in pgvector
	query := `
		SELECT id, timestamp, content, embedding::text
		FROM autodream_findings
		ORDER BY embedding <=> $1::vector
		LIMIT $2
	`
	rows, err := r.db.Query(ctx, query, string(embBytes), limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var findings []*Finding
	for rows.Next() {
		var f Finding
		var embStr string
		if err := rows.Scan(&f.ID, &f.Timestamp, &f.Content, &embStr); err != nil {
			return nil, err
		}
		if err := json.Unmarshal([]byte(embStr), &f.Embedding); err != nil {
			return nil, err
		}
		findings = append(findings, &f)
	}
	return findings, nil
}

func (r *AutoDreamRepository) searchSQLite(ctx context.Context, queryEmbedding []float32, limit int) ([]*Finding, error) {
	query := `SELECT id, timestamp, content, embedding FROM autodream_findings`
	rows, err := r.db.Query(ctx, query)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	type scoredFinding struct {
		finding *Finding
		score   float32
	}
	var scoredFindings []scoredFinding

	for rows.Next() {
		var f Finding
		var embStr string
		if err := rows.Scan(&f.ID, &f.Timestamp, &f.Content, &embStr); err != nil {
			return nil, err
		}
		if err := json.Unmarshal([]byte(embStr), &f.Embedding); err != nil {
			// Skip or handle error
			continue
		}
		score := cosineSimilarity(queryEmbedding, f.Embedding)
		scoredFindings = append(scoredFindings, scoredFinding{finding: &f, score: score})
	}

	sort.Slice(scoredFindings, func(i, j int) bool {
		return scoredFindings[i].score > scoredFindings[j].score
	})

	resultLimit := limit
	if len(scoredFindings) < limit {
		resultLimit = len(scoredFindings)
	}

	findings := make([]*Finding, resultLimit)
	for i := 0; i < resultLimit; i++ {
		findings[i] = scoredFindings[i].finding
	}
	return findings, nil
}

func cosineSimilarity(a, b []float32) float32 {
	if len(a) != len(b) || len(a) == 0 {
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
