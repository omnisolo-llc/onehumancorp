package db

import (
	"context"
	"database/sql"
	"strings"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upAutodreamMemoriesMaster, downAutodreamMemoriesMaster)
}

func upAutodreamMemoriesMaster(ctx context.Context, tx *sql.Tx) error {
	var isSQLite bool
	_, err := tx.ExecContext(ctx, "SAVEPOINT dialect_check")
	if err == nil {
		err = tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(new(string))
		if err != nil {
			_, _ = tx.ExecContext(ctx, "ROLLBACK TO SAVEPOINT dialect_check")
		} else {
			_, _ = tx.ExecContext(ctx, "RELEASE SAVEPOINT dialect_check")
		}
	} else {
		err = tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(new(string))
	}
	if err == nil {
		isSQLite = true
	}

	if !isSQLite {
		if _, err := tx.ExecContext(ctx, "CREATE EXTENSION IF NOT EXISTS vector;"); err != nil {
			return err
		}
	}

	createTableQuery := `
		CREATE TABLE IF NOT EXISTS autodream_memories_master (
			id VARCHAR PRIMARY KEY,
			tenant_id VARCHAR NOT NULL,
			memory_type TEXT NOT NULL,
			content TEXT NOT NULL,
`
	if isSQLite {
		createTableQuery += `			embedding BLOB,`
	} else {
		createTableQuery += `			embedding vector(1536),`
	}

	createTableQuery += `
			source_task_id VARCHAR,
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		);
	`

	if _, err := tx.ExecContext(ctx, createTableQuery); err != nil {
		return err
	}

	if !isSQLite {
		if _, err := tx.ExecContext(ctx, "ALTER TABLE autodream_memories_master ENABLE ROW LEVEL SECURITY; ALTER TABLE autodream_memories_master FORCE ROW LEVEL SECURITY;"); err != nil {
			// ignore error if not postgres
			if !strings.Contains(err.Error(), "ENABLE") {
				return err
			}
		}
	}

	return nil
}

func downAutodreamMemoriesMaster(ctx context.Context, tx *sql.Tx) error {
	_, err := tx.ExecContext(ctx, "DROP TABLE IF EXISTS autodream_memories_master;")
	return err
}
