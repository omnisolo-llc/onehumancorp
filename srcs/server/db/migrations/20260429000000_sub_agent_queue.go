package migrations

import (
	"context"
	"database/sql"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upSubAgentQueue, downSubAgentQueue)
}

func upSubAgentQueue(ctx context.Context, tx *sql.Tx) error {
	var version string
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&version)
	isSQLite := err == nil

	if isSQLite {
		query := `
CREATE TABLE IF NOT EXISTS sub_agent_queue (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    parent_task_id TEXT NOT NULL,
    payload TEXT,
    status TEXT NOT NULL DEFAULT 'QUEUED',
    worker_id TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_sub_agent_queue_status ON sub_agent_queue(status);
`
		_, err = tx.ExecContext(ctx, query)
		return err
	}

	query := `
CREATE TABLE IF NOT EXISTS sub_agent_queue (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    parent_task_id TEXT NOT NULL,
    payload JSONB,
    status TEXT NOT NULL DEFAULT 'QUEUED',
    worker_id TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_sub_agent_queue_status ON sub_agent_queue(status);
`
	_, err = tx.ExecContext(ctx, query)
	return err
}

func downSubAgentQueue(ctx context.Context, tx *sql.Tx) error {
	_, err := tx.ExecContext(ctx, "DROP TABLE IF EXISTS sub_agent_queue;")
	return err
}
