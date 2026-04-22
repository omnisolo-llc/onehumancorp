package migrations

import (
	"context"
	"database/sql"
	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upCreateSharedTasksV4, downCreateSharedTasksV4)
}

func upCreateSharedTasksV4(ctx context.Context, tx *sql.Tx) error {
	var dialect string
	tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&dialect)

	if dialect != "" {
		// SQLite
		query := `
		CREATE TABLE IF NOT EXISTS shared_tasks_v5 (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING' CHECK (status IN ('PENDING', 'CLAIMED', 'DONE', 'FAILED', 'COMPLETED')),
			agent_id TEXT,
			priority TEXT NOT NULL DEFAULT 'P2',
			payload TEXT
		);`
		_, err := tx.ExecContext(ctx, query)
		return err
	}

	return nil // Postgres uses .sql
}

func downCreateSharedTasksV4(ctx context.Context, tx *sql.Tx) error {
	return nil
}
