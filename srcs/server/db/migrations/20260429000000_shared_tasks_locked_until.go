package db

import (
	"context"
	"database/sql"
	"strings"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upSharedTasksLockedUntil20260429000000, downSharedTasksLockedUntil20260429000000)
}

func upSharedTasksLockedUntil20260429000000(ctx context.Context, tx *sql.Tx) error {
	var sqliteVersion string
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	isSQLite := err == nil

	if !isSQLite {
		// PostgreSQL migrations
		_, err := tx.ExecContext(ctx, `
			ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS locked_until TIMESTAMPTZ;
		`)
		if err != nil && !strings.Contains(err.Error(), "already exists") {
			return err
		}

		_, err = tx.ExecContext(ctx, `
			CREATE INDEX IF NOT EXISTS idx_shared_tasks_locked_until ON shared_tasks(locked_until);
		`)
		if err != nil {
			return err
		}

		return nil
	}

	// SQLite migrations
	_, err = tx.ExecContext(ctx, `
		ALTER TABLE shared_tasks ADD COLUMN locked_until DATETIME;
	`)
	if err != nil && !strings.Contains(err.Error(), "duplicate column name") {
		return err
	}

	_, err = tx.ExecContext(ctx, `
		CREATE INDEX IF NOT EXISTS idx_shared_tasks_locked_until ON shared_tasks(locked_until);
	`)
	return err
}

func downSharedTasksLockedUntil20260429000000(ctx context.Context, tx *sql.Tx) error {
	var sqliteVersion string
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	isSQLite := err == nil

	if !isSQLite {
		_, err := tx.ExecContext(ctx, "DROP INDEX IF EXISTS idx_shared_tasks_locked_until")
		if err != nil {
			return err
		}
		_, err = tx.ExecContext(ctx, "ALTER TABLE shared_tasks DROP COLUMN IF EXISTS locked_until")
		return err
	}

	_, err = tx.ExecContext(ctx, "DROP INDEX IF EXISTS idx_shared_tasks_locked_until")
	if err != nil {
		return err
	}
	// SQLite does not support DROP COLUMN until newer versions, but we can try
	_, err = tx.ExecContext(ctx, "ALTER TABLE shared_tasks DROP COLUMN locked_until")
	if err != nil && strings.Contains(err.Error(), "syntax error") {
		return nil // Ignore for old SQLite
	}
	return err
}
