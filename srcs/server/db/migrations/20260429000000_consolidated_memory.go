package migrations

import (
	"context"
	"database/sql"
	"fmt"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upConsolidatedMemory20260429000000, downConsolidatedMemory20260429000000)
}

func upConsolidatedMemory20260429000000(ctx context.Context, tx *sql.Tx) error {
	var sqliteVersion string
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	isSQLite := err == nil

	if !isSQLite {
		_, err := tx.ExecContext(ctx, "CREATE EXTENSION IF NOT EXISTS vector")
		if err != nil {
			return fmt.Errorf("failed to create vector extension: %w", err)
		}

		query := `
		CREATE TABLE IF NOT EXISTS consolidated_memory (
			id TEXT PRIMARY KEY,
			organization_id TEXT,

			task_id TEXT,
			source_mission_id TEXT,
			processed_at TIMESTAMP WITH TIME ZONE,
			agent_id TEXT,
			content TEXT NOT NULL,
			embedding VECTOR(1536),
			source_type TEXT,

			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		);
		CREATE INDEX IF NOT EXISTS idx_consolidated_memory_embedding ON consolidated_memory USING hnsw (embedding vector_cosine_ops);
		`
		_, err = tx.ExecContext(ctx, query)
		return err
	}

	query := `
	CREATE TABLE IF NOT EXISTS consolidated_memory (
		id TEXT PRIMARY KEY,
		organization_id TEXT,

			task_id TEXT,
			source_mission_id TEXT,
			processed_at TIMESTAMP WITH TIME ZONE,
		agent_id TEXT,
		content TEXT NOT NULL,
		embedding TEXT,
		source_type TEXT,

		created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
	);
	CREATE INDEX IF NOT EXISTS idx_consolidated_memory_created_at ON consolidated_memory (created_at);
	`
	_, err = tx.ExecContext(ctx, query)
	return err
}

func downConsolidatedMemory20260429000000(ctx context.Context, tx *sql.Tx) error {
	// Not fully reverting table creation because other migrations might have created it
	return nil
}
