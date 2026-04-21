package migrations

import (
	"context"
	"database/sql"
	"fmt"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upKairosSharedTaskMesh, downKairosSharedTaskMesh)
}

func upKairosSharedTaskMesh(ctx context.Context, tx *sql.Tx) error {
	var sqliteVersion string
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	isSQLite := err == nil

	if !isSQLite {
		_, err := tx.ExecContext(ctx, "CREATE EXTENSION IF NOT EXISTS vector")
		if err != nil {
			return fmt.Errorf("failed to create vector extension: %w", err)
		}

		query := `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id UUID PRIMARY KEY,
			organization_id VARCHAR NOT NULL,
			title VARCHAR NOT NULL,
			description TEXT,
			status VARCHAR NOT NULL DEFAULT 'PENDING',
			priority VARCHAR NOT NULL DEFAULT 'P2',
			agent_id VARCHAR,
			dependencies JSONB NOT NULL DEFAULT '[]',
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		);
		`
		_, err = tx.ExecContext(ctx, query)
		if err != nil {
			return fmt.Errorf("failed to create shared_tasks pg: %w", err)
		}

		query = `
		CREATE TABLE IF NOT EXISTS consolidated_memory (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			agent_id TEXT,
			content TEXT NOT NULL,
			embedding VECTOR(1536),
			source_type TEXT NOT NULL,
			created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
		);
		`
		_, err = tx.ExecContext(ctx, query)
		if err != nil {
			return fmt.Errorf("failed to create consolidated_memory pg: %w", err)
		}

		query = `CREATE INDEX IF NOT EXISTS idx_consolidated_memory_embedding ON consolidated_memory USING hnsw (embedding vector_cosine_ops);`
		_, err = tx.ExecContext(ctx, query)
		if err != nil {
			return fmt.Errorf("failed to create index pg: %w", err)
		}

		return nil
	}

	query := `
	CREATE TABLE IF NOT EXISTS shared_tasks (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		title TEXT NOT NULL,
		description TEXT,
		status TEXT NOT NULL DEFAULT 'PENDING',
		priority TEXT NOT NULL DEFAULT 'P2',
		agent_id TEXT,
		dependencies TEXT NOT NULL DEFAULT '[]',
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
		updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
	);
	`
	_, err = tx.ExecContext(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to create shared_tasks sqlite: %w", err)
	}

	query = `
	CREATE TABLE IF NOT EXISTS consolidated_memory (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		agent_id TEXT,
		content TEXT NOT NULL,
		embedding TEXT,
		source_type TEXT NOT NULL,
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP
	);
	`
	_, err = tx.ExecContext(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to create consolidated_memory sqlite: %w", err)
	}

	return nil
}

func downKairosSharedTaskMesh(ctx context.Context, tx *sql.Tx) error {
	_, err := tx.ExecContext(ctx, "DROP TABLE IF EXISTS shared_tasks")
	if err != nil {
		return err
	}
	_, err = tx.ExecContext(ctx, "DROP TABLE IF EXISTS consolidated_memory")
	return err
}
