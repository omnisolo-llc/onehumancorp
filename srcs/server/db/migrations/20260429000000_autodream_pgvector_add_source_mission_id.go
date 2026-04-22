package migrations

import (
	"context"
	"database/sql"
	"fmt"
	"strings"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upAutodreamMemoriesSourceMissionId, downAutodreamMemoriesSourceMissionId)
}

func upAutodreamMemoriesSourceMissionId(ctx context.Context, tx *sql.Tx) error {
	var sqliteVersion string
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	isSQLite := err == nil

	if !isSQLite {
		_, err := tx.ExecContext(ctx, "CREATE EXTENSION IF NOT EXISTS vector")
		if err != nil {
			return fmt.Errorf("failed to create vector extension: %w", err)
		}
	}

	var alterTasksQuery string
	if isSQLite {
		alterTasksQuery = "ALTER TABLE autodream_memories ADD COLUMN source_mission_id TEXT;"
	} else {
		alterTasksQuery = "ALTER TABLE autodream_memories ADD COLUMN IF NOT EXISTS source_mission_id TEXT;"
	}

	_, err = tx.ExecContext(ctx, alterTasksQuery)
	if err != nil {
		if isSQLite && strings.Contains(err.Error(), "duplicate column name") {
			fmt.Printf("Info: source_mission_id column already exists: %v\n", err)
			return nil
		}
		if isSQLite && strings.Contains(err.Error(), "no such table: autodream_memories") {
			// This means the table doesn't exist yet so we should create it
			query := `
			CREATE TABLE IF NOT EXISTS autodream_memories (
				id TEXT PRIMARY KEY,
				organization_id TEXT NOT NULL,
				task_id TEXT,
				content TEXT NOT NULL,
				embedding TEXT,
				source_mission_id TEXT,
				source_type TEXT NOT NULL DEFAULT 'auto_dream',
				created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
			);
			CREATE INDEX IF NOT EXISTS idx_autodream_memories_created_at ON autodream_memories (created_at);
			`
			_, err = tx.ExecContext(ctx, query)
			return err
		}
		if !isSQLite && strings.Contains(err.Error(), "relation \"autodream_memories\" does not exist") {
			query := `
			CREATE TABLE IF NOT EXISTS autodream_memories (
				id TEXT PRIMARY KEY,
				organization_id TEXT NOT NULL,
				task_id TEXT,
				content TEXT NOT NULL,
				embedding VECTOR(1536),
				source_mission_id TEXT,
				source_type TEXT NOT NULL DEFAULT 'auto_dream',
				created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
			);
			CREATE INDEX IF NOT EXISTS idx_autodream_memories_embedding ON autodream_memories USING hnsw (embedding vector_cosine_ops);
			`
			_, err = tx.ExecContext(ctx, query)
			return err
		}
		return fmt.Errorf("adding source_mission_id column to autodream_memories: %v", err)
	}

	return nil
}

func downAutodreamMemoriesSourceMissionId(ctx context.Context, tx *sql.Tx) error {
	var sqliteVersion string
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	isSQLite := err == nil

	var alterTasksQuery string
	if isSQLite {
		alterTasksQuery = "ALTER TABLE autodream_memories DROP COLUMN source_mission_id;"
	} else {
		alterTasksQuery = "ALTER TABLE autodream_memories DROP COLUMN IF EXISTS source_mission_id;"
	}

	_, err = tx.ExecContext(ctx, alterTasksQuery)
	if err != nil {
		if isSQLite && strings.Contains(err.Error(), "no such column") {
			return nil
		}
		return err
	}
	return nil
}
