package migrations

import (
	"context"
	"database/sql"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upKairosPhase120260413165953, downKairosPhase120260413165953)
}

func upKairosPhase120260413165953(ctx context.Context, tx *sql.Tx) error {
	var sqliteVersion string
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	isSQLite := err == nil

	if !isSQLite {
		query1 := `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			tenant_id TEXT NOT NULL,
			parent_plan_id TEXT,
			title TEXT NOT NULL,
			status TEXT NOT NULL DEFAULT 'PENDING',
			assigned_agent_id TEXT,
			created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
		)`
		if _, err = tx.ExecContext(ctx, query1); err != nil {
			return err
		}

		query2 := `ALTER TABLE shared_tasks ENABLE ROW LEVEL SECURITY;`
		if _, err = tx.ExecContext(ctx, query2); err != nil {
            return err
        }
		return nil
	}

	query1 := `
	CREATE TABLE IF NOT EXISTS shared_tasks (
		id TEXT PRIMARY KEY,
		tenant_id TEXT NOT NULL,
		parent_plan_id TEXT,
		title TEXT NOT NULL,
		status TEXT NOT NULL DEFAULT 'PENDING',
		assigned_agent_id TEXT,
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
		updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
	)`
	_, err = tx.ExecContext(ctx, query1)
	return err
}

func downKairosPhase120260413165953(ctx context.Context, tx *sql.Tx) error {
	_, err := tx.ExecContext(ctx, "DROP TABLE IF EXISTS shared_tasks")
	return err
}
