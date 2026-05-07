package searchmcp

import (
	"context"
	"database/sql"
	"fmt"
)

type LocalSearchProvider struct {
	db *sql.DB
}

func NewLocalSearchProvider(db *sql.DB) *LocalSearchProvider {
	return &LocalSearchProvider{db: db}
}

func (p *LocalSearchProvider) Search(ctx context.Context, query string) ([]SearchResult, error) {
	rows, err := p.db.QueryContext(ctx, "SELECT id, content FROM documents WHERE content MATCH ? ORDER BY rank LIMIT 10", query)
	if err != nil {
		return nil, fmt.Errorf("local search failed: %w", err)
	}
	defer rows.Close()

	var results []SearchResult
	for rows.Next() {
		var res SearchResult
		if err := rows.Scan(&res.ID, &res.Content); err != nil {
			return nil, err
		}
		res.Score = 1.0
		results = append(results, res)
	}

	if err := rows.Err(); err != nil {
		return nil, err
	}
	return results, nil
}

func (p *LocalSearchProvider) Index(ctx context.Context, doc Document) error {
	_, err := p.db.ExecContext(ctx, "INSERT INTO documents(id, content) VALUES (?, ?)", doc.ID, doc.Content)
	if err != nil {
		return fmt.Errorf("local index failed: %w", err)
	}
	return nil
}
