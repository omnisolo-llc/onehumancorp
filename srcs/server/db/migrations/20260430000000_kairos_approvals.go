package migrations

import (
	"context"
	"database/sql"
	"strings"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upKairosApprovals20260430000000, downKairosApprovals20260430000000)
}

func upKairosApprovals20260430000000(ctx context.Context, tx *sql.Tx) error {
	var sqliteVersion string
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	isSQLite := err == nil

	if !isSQLite {
		// PostgreSQL migrations
		queries := []string{
			`ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS action_risk VARCHAR(50);`,
			`ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS approval_status VARCHAR(50);`,
			`ALTER TABLE shared_tasks ADD COLUMN IF NOT EXISTS proposed_content TEXT;`,

			// Master Blueprint Tables
			`ALTER TABLE kairos_shared_tasks ADD COLUMN IF NOT EXISTS action_risk VARCHAR(50);`,
			`ALTER TABLE kairos_shared_tasks ADD COLUMN IF NOT EXISTS approval_status VARCHAR(50);`,
			`ALTER TABLE kairos_shared_tasks ADD COLUMN IF NOT EXISTS proposed_content TEXT;`,
		}

		for _, q := range queries {
			_, err := tx.ExecContext(ctx, q)
			if err != nil && !strings.Contains(err.Error(), "already exists") {
				return err
			}
		}
		return nil
	}

	// SQLite migrations
	queries := []string{
		`ALTER TABLE shared_tasks ADD COLUMN action_risk TEXT;`,
		`ALTER TABLE shared_tasks ADD COLUMN approval_status TEXT;`,
		`ALTER TABLE shared_tasks ADD COLUMN proposed_content TEXT;`,

		`ALTER TABLE kairos_shared_tasks ADD COLUMN action_risk TEXT;`,
		`ALTER TABLE kairos_shared_tasks ADD COLUMN approval_status TEXT;`,
		`ALTER TABLE kairos_shared_tasks ADD COLUMN proposed_content TEXT;`,
	}

	for _, q := range queries {
		_, err := tx.ExecContext(ctx, q)
		if err != nil && !strings.Contains(err.Error(), "duplicate column name") && !strings.Contains(err.Error(), "no such table") {
			return err
		}
	}

	return nil
}

func downKairosApprovals20260430000000(ctx context.Context, tx *sql.Tx) error {
	var sqliteVersion string
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	isSQLite := err == nil

	if !isSQLite {
		_, err = tx.ExecContext(ctx, "ALTER TABLE shared_tasks DROP COLUMN IF EXISTS action_risk, DROP COLUMN IF EXISTS approval_status, DROP COLUMN IF EXISTS proposed_content")
		if err != nil && !strings.Contains(err.Error(), "does not exist") {
			return err
		}
		_, err = tx.ExecContext(ctx, "ALTER TABLE kairos_shared_tasks DROP COLUMN IF EXISTS action_risk, DROP COLUMN IF EXISTS approval_status, DROP COLUMN IF EXISTS proposed_content")
		if err != nil && !strings.Contains(err.Error(), "does not exist") {
			return err
		}
		return nil
	}

	// SQLite doesn't easily support DROP COLUMN in older versions,
	// but goose for recent sqlite supports it if version is high enough
	_, _ = tx.ExecContext(ctx, "ALTER TABLE shared_tasks DROP COLUMN action_risk;")
	_, _ = tx.ExecContext(ctx, "ALTER TABLE shared_tasks DROP COLUMN approval_status;")
	_, _ = tx.ExecContext(ctx, "ALTER TABLE shared_tasks DROP COLUMN proposed_content;")

	_, _ = tx.ExecContext(ctx, "ALTER TABLE kairos_shared_tasks DROP COLUMN action_risk;")
	_, _ = tx.ExecContext(ctx, "ALTER TABLE kairos_shared_tasks DROP COLUMN approval_status;")
	_, _ = tx.ExecContext(ctx, "ALTER TABLE kairos_shared_tasks DROP COLUMN proposed_content;")
	return nil
}
