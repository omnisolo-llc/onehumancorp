package migrations

import (
	"context"
	"database/sql"
	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upConsolidMemory, downConsolidMemory)
}

func upConsolidMemory(ctx context.Context, tx *sql.Tx) error {
	var dialect string
	tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&dialect)

	if dialect != "" {
		// SQLite
		query := `
		CREATE TABLE IF NOT EXISTS consolidated_memory (
			id TEXT PRIMARY KEY,
			content TEXT NOT NULL,
			embedding TEXT,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		);`
		_, err := tx.ExecContext(ctx, query)
		return err
	}

	return nil // Postgres uses .sql
}

func downConsolidMemory(ctx context.Context, tx *sql.Tx) error {
	return nil
}
