package migrations

import (
	"context"
	"database/sql"
	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upSubAgentTasks, downSubAgentTasks)
}

func upSubAgentTasks(ctx context.Context, tx *sql.Tx) error {
	var sqliteVersion string
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	isSQLite := err == nil

	if isSQLite {
		_, err = tx.ExecContext(ctx, `
			CREATE TABLE IF NOT EXISTS sub_agent_tasks (
				job_id TEXT PRIMARY KEY,
				queue_name TEXT NOT NULL,
				payload TEXT NOT NULL,
				status TEXT NOT NULL DEFAULT 'QUEUED',
				created_at DATETIME DEFAULT CURRENT_TIMESTAMP
			);
			CREATE INDEX IF NOT EXISTS idx_sub_agent_tasks_status ON sub_agent_tasks (status, queue_name);
		`)
	} else {
		_, err = tx.ExecContext(ctx, `
			CREATE TABLE IF NOT EXISTS sub_agent_tasks (
				job_id TEXT PRIMARY KEY,
				queue_name TEXT NOT NULL,
				payload JSONB NOT NULL,
				status TEXT NOT NULL DEFAULT 'QUEUED',
				created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
			);
			CREATE INDEX IF NOT EXISTS idx_sub_agent_tasks_status ON sub_agent_tasks (status, queue_name);
		`)
	}
	return err
}

func downSubAgentTasks(ctx context.Context, tx *sql.Tx) error {
	_, err := tx.ExecContext(ctx, "DROP TABLE IF EXISTS sub_agent_tasks;")
	return err
}
