package migrations

import (
	"context"
	"database/sql"
	"fmt"
	"log/slog"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upAutodreamAgentMemories, downAutodreamAgentMemories)
}

func upAutodreamAgentMemories(ctx context.Context, tx *sql.Tx) error {
	var sqliteVersion string
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	isSQLite := err == nil

	// Add auto_dreamed column to tasks
	alterTasksQuery := "ALTER TABLE tasks ADD COLUMN auto_dreamed BOOLEAN DEFAULT false;"
	_, err = tx.ExecContext(ctx, alterTasksQuery)
	if err != nil {
		// Log but don't fail if column already exists (sqlite compatibility)
		slog.Info("Info: adding auto_dreamed column", "error", err)
	}

	if !isSQLite {
		_, err := tx.ExecContext(ctx, "CREATE EXTENSION IF NOT EXISTS vector")
		if err != nil {
			return fmt.Errorf("failed to create vector extension: %w", err)
		}

		query := `
		CREATE TABLE IF NOT EXISTS agent_memories (
			id UUID PRIMARY KEY,
			organization_id UUID NOT NULL,
			task_id UUID,
			raw_content TEXT,
			summary_embedding VECTOR(1536),
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		)`
		_, err = tx.ExecContext(ctx, query)
		return err
	}

	query := `
	CREATE TABLE IF NOT EXISTS agent_memories (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		task_id TEXT,
		raw_content TEXT,
		summary_embedding TEXT,
		created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
	)`
	_, err = tx.ExecContext(ctx, query)
	return err
}

func downAutodreamAgentMemories(ctx context.Context, tx *sql.Tx) error {
	_, err := tx.ExecContext(ctx, "DROP TABLE IF EXISTS agent_memories")
	return err
}
