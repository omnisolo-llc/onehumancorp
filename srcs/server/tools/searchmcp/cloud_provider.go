package searchmcp

import (
	"context"
	"database/sql"
	"errors"
	"fmt"
)

type CloudSearchProvider struct {
	db *sql.DB
}

func NewCloudSearchProvider(db *sql.DB) *CloudSearchProvider {
	return &CloudSearchProvider{db: db}
}

func (p *CloudSearchProvider) Search(ctx context.Context, query string) ([]SearchResult, error) {
	claims := ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return nil, errors.New("unauthorized: missing claims or organization ID")
	}

	rows, err := p.db.QueryContext(ctx, "SELECT id, content FROM documents WHERE tenant_id = $1 AND content ILIKE $2 LIMIT 10", claims.OrganizationID, "%"+query+"%")
	if err != nil {
		return nil, fmt.Errorf("cloud search failed: %w", err)
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

func (p *CloudSearchProvider) Index(ctx context.Context, doc Document) error {
	claims := ClaimsFromContext(ctx)
	if claims == nil || claims.OrganizationID == "" {
		return errors.New("unauthorized: missing claims or organization ID")
	}

	_, err := p.db.ExecContext(ctx, "INSERT INTO documents(id, tenant_id, content) VALUES ($1, $2, $3)", doc.ID, claims.OrganizationID, doc.Content)
	if err != nil {
		return fmt.Errorf("cloud index failed: %w", err)
	}
	return nil
}
