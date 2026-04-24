package migrations

import (
	"context"
	"database/sql"
	"strings"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upWizardDraftsSchema, downWizardDraftsSchema)
}

func upWizardDraftsSchema(ctx context.Context, tx *sql.Tx) error {
	var sqliteVersion string
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	isSqlite := (err == nil && sqliteVersion != "")

	var query string
	if isSqlite {
		query = `
			CREATE TABLE IF NOT EXISTS wizard_drafts (
				user_id TEXT PRIMARY KEY,
				draft_json TEXT NOT NULL,
				updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
			);
		`
	} else {
		query = `
			CREATE TABLE IF NOT EXISTS wizard_drafts (
				user_id TEXT PRIMARY KEY,
				draft_json JSONB NOT NULL,
				updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
			);
		`
	}
	_, err = tx.ExecContext(ctx, query)
	return err
}

func downWizardDraftsSchema(ctx context.Context, tx *sql.Tx) error {
	_, err := tx.ExecContext(ctx, "DROP TABLE IF EXISTS wizard_drafts;")
	return err
}
