package migrations

import (
	"context"
	"database/sql"
	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upTenantIsolationPolicies20260430000000, downTenantIsolationPolicies20260430000000)
}

func upTenantIsolationPolicies20260430000000(ctx context.Context, tx *sql.Tx) error {
	var sqliteVersion string
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	isSQLite := err == nil

	if isSQLite {
		return nil
	}

	query := `
		DO $$
		DECLARE
			r RECORD;
		BEGIN
			FOR r IN
				SELECT table_name
				FROM information_schema.columns
				WHERE column_name = 'tenant_id' AND table_schema = 'public'
			LOOP
				EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY;', r.table_name);
				EXECUTE format('DROP POLICY IF EXISTS tenant_isolation_policy ON %I;', r.table_name);
				EXECUTE format('CREATE POLICY tenant_isolation_policy ON %I USING (tenant_id = current_setting(''app.current_tenant_id'', true));', r.table_name);
			END LOOP;
		END $$;
	`
	_, err = tx.ExecContext(ctx, query)
	return err
}

func downTenantIsolationPolicies20260430000000(ctx context.Context, tx *sql.Tx) error {
	var sqliteVersion string
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	isSQLite := err == nil

	if isSQLite {
		return nil
	}

	query := `
		DO $$
		DECLARE
			r RECORD;
		BEGIN
			FOR r IN
				SELECT table_name
				FROM information_schema.columns
				WHERE column_name = 'tenant_id' AND table_schema = 'public'
			LOOP
				EXECUTE format('DROP POLICY IF EXISTS tenant_isolation_policy ON %I;', r.table_name);
			END LOOP;
		END $$;
	`
	_, err = tx.ExecContext(ctx, query)
	return err
}
