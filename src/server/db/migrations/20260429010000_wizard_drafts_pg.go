package migrations

import (
	"context"
	"database/sql"
	"fmt"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(up20260429010000, down20260429010000)
}

func up20260429010000(ctx context.Context, tx *sql.Tx) error {
	var isSQLite bool
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version();").Scan(new(string))
	if err == nil {
		isSQLite = true
	}

	if isSQLite {
		return nil
	}

	queries := []string{
		"ALTER TABLE wizard_drafts ENABLE ROW LEVEL SECURITY;",
		"CREATE POLICY tenant_isolation_wizard_drafts ON wizard_drafts USING (user_id = current_setting('app.current_tenant', true));",
	}

	for _, q := range queries {
		if _, err := tx.ExecContext(ctx, q); err != nil {
			return fmt.Errorf("failed to run query %q: %w", q, err)
		}
	}
	return nil
}

func down20260429010000(ctx context.Context, tx *sql.Tx) error {
	var isSQLite bool
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version();").Scan(new(string))
	if err == nil {
		isSQLite = true
	}

	if isSQLite {
		return nil
	}

	queries := []string{
		"DROP POLICY IF EXISTS tenant_isolation_wizard_drafts ON wizard_drafts;",
	}

	for _, q := range queries {
		if _, err := tx.ExecContext(ctx, q); err != nil {
			return fmt.Errorf("failed to run query %q: %w", q, err)
		}
	}
	return nil
}
