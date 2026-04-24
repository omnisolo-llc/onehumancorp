package migrations

import (
	"context"
	"database/sql"
	"fmt"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upAutodreamMemoriesDecomposition20260427000000, downAutodreamMemoriesDecomposition20260427000000)
}

func upAutodreamMemoriesDecomposition20260427000000(ctx context.Context, tx *sql.Tx) error {
	var sqliteVersion string
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	isSQLite := err == nil

	// Add auto_dreamed column to shared_tasks_decomposition
	alterTasksQuery := "ALTER TABLE shared_tasks_decomposition ADD COLUMN auto_dreamed BOOLEAN DEFAULT false;"
	_, err = tx.ExecContext(ctx, alterTasksQuery)
	if err != nil {
		fmt.Printf("Info: adding auto_dreamed column to shared_tasks_decomposition: %v\n", err)
	}

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
		);
		CREATE INDEX IF NOT EXISTS idx_autodream_memories_embedding ON autodream_memories USING hnsw (embedding vector_cosine_ops);
		`
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
	);
	CREATE INDEX IF NOT EXISTS idx_autodream_memories_created_at ON autodream_memories (created_at);
	`
	_, err = tx.ExecContext(ctx, query)
	return err
}

func downAutodreamMemoriesDecomposition20260427000000(ctx context.Context, tx *sql.Tx) error {
	// Not fully reverting table creation because other migrations might have created it
	// Just a simple down if we really want to reverse.
	return nil
}
