package migrations

import (
	"context"
	"database/sql"
	"strings"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upKairosAdditionalColumns20260427010000, downKairosAdditionalColumns20260427010000)
}

func upKairosAdditionalColumns20260427010000(ctx context.Context, tx *sql.Tx) error {
	var sqliteVersion string
	_, err := tx.ExecContext(ctx, "SAVEPOINT check_sqlite")
	if err != nil {
		return err
	}
	err = tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	if err == nil {
		tx.ExecContext(ctx, "RELEASE SAVEPOINT check_sqlite")
	} else {
		tx.ExecContext(ctx, "ROLLBACK TO SAVEPOINT check_sqlite")
	}
	isSQLite := err == nil

	if !isSQLite {
		// PostgreSQL migrations
		_, err := tx.ExecContext(ctx, `
			ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS parent_id UUID REFERENCES shared_tasks(id);
		`)
		if err != nil && !strings.Contains(err.Error(), "already exists") {
			return err
		}

		_, err = tx.ExecContext(ctx, `
			ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS epic_id VARCHAR(255);
		`)
		if err != nil && !strings.Contains(err.Error(), "already exists") {
			return err
		}

		_, err = tx.ExecContext(ctx, `
			ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS assigned_agent VARCHAR(255);
		`)
		if err != nil && !strings.Contains(err.Error(), "already exists") {
			return err
		}

		_, err = tx.ExecContext(ctx, `
			CREATE TABLE IF NOT EXISTS agent_mesh_messages (
				id UUID PRIMARY KEY,
				tenant_id VARCHAR NOT NULL,
				sender VARCHAR(255) NOT NULL,
				recipient VARCHAR(255),
				channel VARCHAR(100) NOT NULL,
				content JSONB NOT NULL,
				created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
			)
		`)
		if err != nil {
			return err
		}

		_, err = tx.ExecContext(ctx, `
			ALTER TABLE agent_mesh_messages ENABLE ROW LEVEL SECURITY;
		`)
		return err
	}

	// SQLite migrations
	_, err = tx.ExecContext(ctx, `
		ALTER TABLE shared_tasks ADD COLUMN parent_id TEXT REFERENCES shared_tasks(id);
	`)
	if err != nil && !strings.Contains(err.Error(), "duplicate column name") {
		return err
	}

	_, err = tx.ExecContext(ctx, `
		ALTER TABLE shared_tasks ADD COLUMN epic_id TEXT;
	`)
	if err != nil && !strings.Contains(err.Error(), "duplicate column name") {
		return err
	}

	_, err = tx.ExecContext(ctx, `
		ALTER TABLE shared_tasks ADD COLUMN assigned_agent TEXT;
	`)
	if err != nil && !strings.Contains(err.Error(), "duplicate column name") {
		return err
	}

	_, err = tx.ExecContext(ctx, `
		CREATE TABLE IF NOT EXISTS agent_mesh_messages (
			id TEXT PRIMARY KEY,
			tenant_id TEXT NOT NULL,
			sender TEXT NOT NULL,
			recipient TEXT,
			channel TEXT NOT NULL,
			content TEXT NOT NULL,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		)
	`)
	return err
}

func downKairosAdditionalColumns20260427010000(ctx context.Context, tx *sql.Tx) error {
	var sqliteVersion string
	_, err := tx.ExecContext(ctx, "SAVEPOINT check_sqlite")
	if err != nil {
		return err
	}
	err = tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	if err == nil {
		tx.ExecContext(ctx, "RELEASE SAVEPOINT check_sqlite")
	} else {
		tx.ExecContext(ctx, "ROLLBACK TO SAVEPOINT check_sqlite")
	}
	isSQLite := err == nil

	if !isSQLite {
		_, err := tx.ExecContext(ctx, "DROP TABLE IF EXISTS agent_mesh_messages")
		if err != nil {
			return err
		}
		_, err = tx.ExecContext(ctx, "ALTER TABLE shared_tasks DROP COLUMN IF EXISTS parent_id, DROP COLUMN IF EXISTS epic_id, DROP COLUMN IF EXISTS assigned_agent")
		return err
	}

	_, err = tx.ExecContext(ctx, "DROP TABLE IF EXISTS agent_mesh_messages")
	return err
}
