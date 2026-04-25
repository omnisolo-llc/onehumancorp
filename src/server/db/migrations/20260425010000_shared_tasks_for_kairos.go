package migrations

import (
	"context"
	"database/sql"
	"fmt"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upSharedTasksForKairos20260425010000, downSharedTasksForKairos20260425010000)
}

func upSharedTasksForKairos20260425010000(ctx context.Context, tx *sql.Tx) error {
	var sqliteVersion string
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	isSQLite := err == nil

	if !isSQLite {
		query1 := `
		CREATE TABLE IF NOT EXISTS epics (
			id UUID PRIMARY KEY DEFAULT gen_random_uuid()
		)`
		_, err = tx.ExecContext(ctx, query1)
		if err != nil {
			return err
		}

		query2 := `
		CREATE TABLE IF NOT EXISTS tasks (
			id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
			epic_id UUID REFERENCES epics(id),
			title VARCHAR(255) NOT NULL,
			status VARCHAR(50) NOT NULL CHECK (status IN ('PENDING', 'CLAIMED', 'DONE', 'FAILED')),
			assigned_agent VARCHAR(100),
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		)`
		_, err = tx.ExecContext(ctx, query2)
		if err != nil {
			return err
		}

		query3 := `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
			tenant_id VARCHAR NOT NULL,
			organization_id VARCHAR NOT NULL,
			title VARCHAR NOT NULL,
			description TEXT,
			status VARCHAR NOT NULL DEFAULT 'PENDING',
			assigned_agent_id VARCHAR,
			dependencies JSONB DEFAULT '[]',
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		)`
		_, err = tx.ExecContext(ctx, query3)
		if err != nil {
			return err
		}

		query4 := `ALTER TABLE shared_tasks ENABLE ROW LEVEL SECURITY;`
		_, err = tx.ExecContext(ctx, query4)

		return err
	}

	query1 := `
	CREATE TABLE IF NOT EXISTS epics (
		id TEXT PRIMARY KEY
	)`
	_, err = tx.ExecContext(ctx, query1)
	if err != nil {
		return err
	}

	query2 := `
	CREATE TABLE IF NOT EXISTS tasks (
		id TEXT PRIMARY KEY,
		epic_id TEXT REFERENCES epics(id),
		title TEXT NOT NULL,
		status TEXT NOT NULL CHECK (status IN ('PENDING', 'CLAIMED', 'DONE', 'FAILED')),
		assigned_agent TEXT,
		created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
		updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
	)`
	_, err = tx.ExecContext(ctx, query2)
	if err != nil {
		return err
	}

	query3 := `
	CREATE TABLE IF NOT EXISTS shared_tasks (
		id TEXT PRIMARY KEY,
		tenant_id TEXT NOT NULL,
		organization_id TEXT NOT NULL,
		title TEXT NOT NULL,
		description TEXT,
		status TEXT NOT NULL DEFAULT 'PENDING',
		assigned_agent_id TEXT,
		dependencies JSON DEFAULT '[]',
		created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
		updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
	)`
	_, err = tx.ExecContext(ctx, query3)
	return err
}

func downSharedTasksForKairos20260425010000(ctx context.Context, tx *sql.Tx) error {
	_, err := tx.ExecContext(ctx, "DROP TABLE IF EXISTS shared_tasks")
	if err != nil {
		return err
	}
	_, err = tx.ExecContext(ctx, "DROP TABLE IF EXISTS tasks")
	if err != nil {
		return err
	}
	_, err = tx.ExecContext(ctx, "DROP TABLE IF EXISTS epics")
	return err
}
