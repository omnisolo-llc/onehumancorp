package migrations

import (
	"context"
	"database/sql"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upSharedTasksV420260426020000, downSharedTasksV420260426020000)
}

func upSharedTasksV420260426020000(ctx context.Context, tx *sql.Tx) error {
	query := `
		CREATE TABLE IF NOT EXISTS shared_tasks_v4 (
			id VARCHAR PRIMARY KEY,
			organization_id VARCHAR NOT NULL,
			title VARCHAR NOT NULL,
			description TEXT,
			status VARCHAR NOT NULL DEFAULT 'PENDING',
			agent_id VARCHAR,
			priority VARCHAR NOT NULL DEFAULT 'P2',
			payload TEXT,
			parent_plan_id TEXT,
			dependencies TEXT NOT NULL DEFAULT '[]',
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		)
	`
	_, err := tx.ExecContext(ctx, query)
	return err
}

func downSharedTasksV420260426020000(ctx context.Context, tx *sql.Tx) error {
	query := `DROP TABLE IF EXISTS shared_tasks_v4`
	_, err := tx.ExecContext(ctx, query)
	return err
}
