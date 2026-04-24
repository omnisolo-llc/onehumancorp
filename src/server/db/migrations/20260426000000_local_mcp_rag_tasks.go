package migrations

import (
	"context"
	"database/sql"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upLocalMcpRagTasks, downLocalMcpRagTasks)
}

func upLocalMcpRagTasks(ctx context.Context, tx *sql.Tx) error {
	query := `
CREATE TABLE local_mcp_rag_tasks (
	id TEXT PRIMARY KEY,
	tenant_id TEXT NOT NULL,
	payload TEXT NOT NULL,
	escalation_status TEXT NOT NULL DEFAULT 'local',
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
	updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
`
	_, err := tx.ExecContext(ctx, query)
	if err != nil {
		return err
	}

	isSqlite := false
	var version string
	err = tx.QueryRowContext(ctx, "select sqlite_version()").Scan(&version)
	if err == nil {
		isSqlite = true
	}
	if !isSqlite {
		_, err = tx.ExecContext(ctx, "ALTER TABLE local_mcp_rag_tasks ENABLE ROW LEVEL SECURITY;")
		if err != nil {
			return err
		}
	}

	return nil
}

func downLocalMcpRagTasks(ctx context.Context, tx *sql.Tx) error {
	_, err := tx.ExecContext(ctx, "DROP TABLE IF EXISTS local_mcp_rag_tasks;")
	return err
}
