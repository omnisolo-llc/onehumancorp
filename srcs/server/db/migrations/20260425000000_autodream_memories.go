package migrations

import (
	"context"
	"database/sql"
	"fmt"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upAutodreamMemories20260425000000, downAutodreamMemories20260425000000)
}

func upAutodreamMemories20260425000000(ctx context.Context, tx *sql.Tx) error {
	var sqliteVersion string
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	isSQLite := err == nil

	if !isSQLite {
		_, err := tx.ExecContext(ctx, "CREATE EXTENSION IF NOT EXISTS vector")
		if err != nil {
			return fmt.Errorf("failed to create vector extension: %w", err)
		}

		query := `
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			task_id TEXT,
			content TEXT NOT NULL,
			embedding VECTOR(1536),
			source_type TEXT NOT NULL DEFAULT 'auto_dream',
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		)`
		_, err = tx.ExecContext(ctx, query)
		return err
	}

	query := `
	CREATE TABLE IF NOT EXISTS autodream_memories (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		task_id TEXT,
		content TEXT NOT NULL,
		embedding TEXT,
		source_type TEXT NOT NULL DEFAULT 'auto_dream',
		created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
	)`
	_, err = tx.ExecContext(ctx, query)
	return err
}

func downAutodreamMemories20260425000000(ctx context.Context, tx *sql.Tx) error {
	_, err := tx.ExecContext(ctx, "DROP TABLE IF EXISTS autodream_memories")
	return err
}
