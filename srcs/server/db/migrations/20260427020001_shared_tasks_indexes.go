package migrations

import (
	"context"
	"database/sql"
	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upSharedTasksIndexes, downSharedTasksIndexes)
}

func upSharedTasksIndexes(ctx context.Context, tx *sql.Tx) error {
	var sqliteVersion string
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	if err == nil {
		// SQLite
		_, err = tx.ExecContext(ctx, "ALTER TABLE shared_tasks ADD COLUMN locked_until TIMESTAMPTZ")
		if err != nil && err.Error() != "duplicate column name: locked_until" {
		    // Ignore duplicate column name
		}
		_, err = tx.ExecContext(ctx, "CREATE INDEX IF NOT EXISTS idx_shared_tasks_status ON shared_tasks(status)")
		if err != nil {
			return err
		}
		_, err = tx.ExecContext(ctx, "CREATE INDEX IF NOT EXISTS idx_shared_tasks_locked_until ON shared_tasks(locked_until)")
		if err != nil {
			return err
		}
		return nil
	}

	// PostgreSQL
	_, err = tx.ExecContext(ctx, "ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS locked_until TIMESTAMPTZ")
	if err != nil {
		return err
	}
	_, err = tx.ExecContext(ctx, "CREATE INDEX IF NOT EXISTS idx_shared_tasks_status ON shared_tasks(status)")
	if err != nil {
		return err
	}
	_, err = tx.ExecContext(ctx, "CREATE INDEX IF NOT EXISTS idx_shared_tasks_locked_until ON shared_tasks(locked_until)")
	if err != nil {
		return err
	}
	return nil
}

func downSharedTasksIndexes(ctx context.Context, tx *sql.Tx) error {
	var sqliteVersion string
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	if err == nil {
		// SQLite
		_, err = tx.ExecContext(ctx, "DROP INDEX IF EXISTS idx_shared_tasks_locked_until")
		if err != nil {
			return err
		}
		_, err = tx.ExecContext(ctx, "DROP INDEX IF EXISTS idx_shared_tasks_status")
		if err != nil {
			return err
		}
		// SQLite does not support drop column, so we ignore
		return nil
	}

	// PostgreSQL
	_, err = tx.ExecContext(ctx, "DROP INDEX IF EXISTS idx_shared_tasks_locked_until")
	if err != nil {
		return err
	}
	_, err = tx.ExecContext(ctx, "DROP INDEX IF EXISTS idx_shared_tasks_status")
	if err != nil {
		return err
	}
	_, err = tx.ExecContext(ctx, "ALTER TABLE shared_tasks DROP COLUMN IF EXISTS locked_until")
	if err != nil {
		return err
	}
	return nil
}
