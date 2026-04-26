package db

import (
	"context"
	"database/sql"
	"fmt"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upAutodreamSyncDaemon20260430000000, downAutodreamSyncDaemon20260430000000)
}

func upAutodreamSyncDaemon20260430000000(ctx context.Context, tx *sql.Tx) error {
	var sqliteVersion string
	_, err := tx.ExecContext(ctx, "SAVEPOINT dialect_check")
	if err == nil {
		err = tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
		if err != nil {
			_, _ = tx.ExecContext(ctx, "ROLLBACK TO SAVEPOINT dialect_check")
		} else {
			_, _ = tx.ExecContext(ctx, "RELEASE SAVEPOINT dialect_check")
		}
	} else {
		err = tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	}
	isSQLite := err == nil

	if !isSQLite {
		_, err := tx.ExecContext(ctx, "CREATE EXTENSION IF NOT EXISTS vector")
		if err != nil {
			return fmt.Errorf("failed to create vector extension: %w", err)
		}

		query := `
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id UUID PRIMARY KEY,
			tenant_id TEXT NOT NULL,
			topic TEXT NOT NULL,
			content TEXT NOT NULL,
			embedding VECTOR(1536),
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		);
		CREATE INDEX IF NOT EXISTS idx_autodream_memories_embedding ON autodream_memories USING hnsw (embedding vector_cosine_ops);
		ALTER TABLE autodream_memories ENABLE ROW LEVEL SECURITY; ALTER TABLE autodream_memories FORCE ROW LEVEL SECURITY;
		CREATE POLICY tenant_isolation_policy ON autodream_memories
			USING (tenant_id = current_setting('app.current_tenant', true))
			WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
		`
		_, err = tx.ExecContext(ctx, query)
		return err
	}

	query := `
	CREATE TABLE IF NOT EXISTS autodream_memories (
		id TEXT PRIMARY KEY,
		tenant_id TEXT NOT NULL,
		topic TEXT NOT NULL,
		content TEXT NOT NULL,
		embedding TEXT,
		created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
	);
	CREATE INDEX IF NOT EXISTS idx_autodream_memories_created_at ON autodream_memories (created_at);
	`
	_, err = tx.ExecContext(ctx, query)
	return err
}

func downAutodreamSyncDaemon20260430000000(ctx context.Context, tx *sql.Tx) error {
	return nil
}
