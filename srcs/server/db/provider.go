package db

import (
	"context"
	"database/sql"
	"os"
)

// Provider defines the interface for database operations
type Provider interface {
	IsSQLite() bool
	SearchMemoriesQuery(ctx context.Context, db *sql.DB, orgID string, query string, embeddingBytes []byte, topK int) (*sql.Rows, error)
}

type SQLiteProvider struct{}

func (p *SQLiteProvider) IsSQLite() bool {
	return true
}

func (p *SQLiteProvider) SearchMemoriesQuery(ctx context.Context, db *sql.DB, orgID string, query string, _ []byte, topK int) (*sql.Rows, error) {
	return db.QueryContext(ctx, `
		SELECT id, organization_id, task_id, content
		FROM autodream_memories
		WHERE organization_id = ? AND content LIKE ?
		ORDER BY created_at DESC
		LIMIT ?
	`, orgID, "%"+query+"%", topK)
}

type PostgresProvider struct{}

func (p *PostgresProvider) IsSQLite() bool {
	return false
}

func (p *PostgresProvider) SearchMemoriesQuery(ctx context.Context, db *sql.DB, orgID string, _ string, embeddingBytes []byte, topK int) (*sql.Rows, error) {
	return db.QueryContext(ctx, `
		SELECT id, organization_id, task_id, content
		FROM autodream_memories
		WHERE organization_id = $1
		ORDER BY embedding <-> $2
		LIMIT $3
	`, orgID, string(embeddingBytes), topK)
}

type DynamicProvider struct{}

func (p *DynamicProvider) IsSQLite() bool {
	return os.Getenv("OHC_STANDALONE") == "true"
}

func (p *DynamicProvider) getProvider() Provider {
	if p.IsSQLite() {
		return &SQLiteProvider{}
	}
	return &PostgresProvider{}
}

func (p *DynamicProvider) SearchMemoriesQuery(ctx context.Context, db *sql.DB, orgID string, query string, embeddingBytes []byte, topK int) (*sql.Rows, error) {
	return p.getProvider().SearchMemoriesQuery(ctx, db, orgID, query, embeddingBytes, topK)
}

var GlobalProvider Provider = &DynamicProvider{}
