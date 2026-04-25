package migrations

import (
	"context"
	"database/sql"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upAutodreamMemoriesMaster, downAutodreamMemoriesMaster)
}

func upAutodreamMemoriesMaster(ctx context.Context, tx *sql.Tx) error {
	var sqliteVersion string
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	isSQLite := err == nil

	if !isSQLite {
		_, err := tx.ExecContext(ctx, "CREATE EXTENSION IF NOT EXISTS vector;")
		if err != nil {
			return err
		}

		_, err = tx.ExecContext(ctx, `
CREATE TABLE IF NOT EXISTS autodream_memories_master (
    id VARCHAR PRIMARY KEY,
    tenant_id VARCHAR NOT NULL,
    memory_type TEXT NOT NULL,
    content TEXT NOT NULL,
    embedding vector(1536),
    source_task_id VARCHAR,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
		`)
		if err != nil {
			return err
		}

		_, err = tx.ExecContext(ctx, "ALTER TABLE autodream_memories_master ENABLE ROW LEVEL SECURITY;")
		if err != nil {
			return err
		}

		_, err = tx.ExecContext(ctx, "CREATE POLICY tenant_isolation_policy ON autodream_memories_master AS RESTRICTIVE USING (tenant_id = current_setting('app.current_tenant'));")
		return err
	}

	_, err = tx.ExecContext(ctx, `
CREATE TABLE IF NOT EXISTS autodream_memories_master (
    id VARCHAR PRIMARY KEY,
    tenant_id VARCHAR NOT NULL,
    memory_type TEXT NOT NULL,
    content TEXT NOT NULL,
    embedding BLOB,
    source_task_id VARCHAR,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
	`)
	return err
}

func downAutodreamMemoriesMaster(ctx context.Context, tx *sql.Tx) error {
	var sqliteVersion string
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	isSQLite := err == nil

	if !isSQLite {
		_, err := tx.ExecContext(ctx, "DROP POLICY IF EXISTS tenant_isolation_policy ON autodream_memories_master;")
		if err != nil {
			return err
		}
		_, err = tx.ExecContext(ctx, "DROP TABLE IF EXISTS autodream_memories_master;")
		return err
	}

	_, err = tx.ExecContext(ctx, "DROP TABLE IF EXISTS autodream_memories_master;")
	return err
}
