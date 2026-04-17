package mcp_secret_vault

import (
	"context"
	"database/sql"
	"errors"

	_ "github.com/lib/pq"
)

// CloudAdapter implements SecretStorage using PostgreSQL.
type CloudAdapter struct {
	db *sql.DB
}

// NewCloudAdapter creates a new CloudAdapter.
func NewCloudAdapter(db *sql.DB) *CloudAdapter {
	return &CloudAdapter{db: db}
}

// GetSecret retrieves a secret from PostgreSQL, respecting tenant isolation.
func (a *CloudAdapter) GetSecret(ctx context.Context, key string, tenantID string) (string, error) {
	if a.db == nil {
		return "", errors.New("database connection is nil")
	}

	query := `SELECT secret_value FROM tenant_secrets WHERE key = $1 AND tenant_id = $2`
	var value string
	err := a.db.QueryRowContext(ctx, query, key, tenantID).Scan(&value)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return "", errors.New("secret not found")
		}
		return "", err
	}
	return value, nil
}

// SetSecret stores a secret in PostgreSQL, respecting tenant isolation.
func (a *CloudAdapter) SetSecret(ctx context.Context, key string, value string, tenantID string) error {
	if a.db == nil {
		return errors.New("database connection is nil")
	}

	query := `
		INSERT INTO tenant_secrets (tenant_id, key, secret_value)
		VALUES ($1, $2, $3)
		ON CONFLICT (tenant_id, key)
		DO UPDATE SET secret_value = EXCLUDED.secret_value
	`
	_, err := a.db.ExecContext(ctx, query, tenantID, key, value)
	return err
}
