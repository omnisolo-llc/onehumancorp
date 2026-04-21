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
	return err
}

func downLocalMcpRagTasks(ctx context.Context, tx *sql.Tx) error {
	_, err := tx.ExecContext(ctx, "DROP TABLE IF EXISTS local_mcp_rag_tasks;")
	return err
}
