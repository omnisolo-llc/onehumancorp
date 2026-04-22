package migrations

import (
	"context"
	"database/sql"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upAgentKVStore20260429, downAgentKVStore20260429)
}

func isSQLiteDB(ctx context.Context, tx *sql.Tx) bool {
	// Use a savepoint to prevent PostgreSQL transaction aborts
	_, err := tx.ExecContext(ctx, "SAVEPOINT sqlite_check")
	if err != nil {
		return false
	}

	var sqliteVersion string
	err = tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)

	if err != nil {
		tx.ExecContext(ctx, "ROLLBACK TO SAVEPOINT sqlite_check")
		return false
	}
	tx.ExecContext(ctx, "RELEASE SAVEPOINT sqlite_check")
	return true
}

func upAgentKVStore20260429(ctx context.Context, tx *sql.Tx) error {
	isSQLite := isSQLiteDB(ctx, tx)

	var stmts []string

	if isSQLite {
		stmts = []string{
			`CREATE TABLE IF NOT EXISTS agent_kv_store (
				tenant_id TEXT NOT NULL,
				kv_key TEXT NOT NULL,
				kv_value TEXT NOT NULL,
				updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
				PRIMARY KEY (tenant_id, kv_key)
			);`,
		}
	} else {
		stmts = []string{
			`CREATE TABLE IF NOT EXISTS agent_kv_store (
				tenant_id TEXT NOT NULL,
				kv_key TEXT NOT NULL,
				kv_value TEXT NOT NULL,
				updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
				PRIMARY KEY (tenant_id, kv_key)
			);`,
			`ALTER TABLE agent_kv_store ENABLE ROW LEVEL SECURITY;`,
			`CREATE POLICY isolate_tenant_agent_kv_store ON agent_kv_store
				USING (tenant_id = current_setting('app.current_tenant', true));`,
		}
	}

	for _, stmt := range stmts {
		if _, err := tx.ExecContext(ctx, stmt); err != nil {
			return err
		}
	}

	return nil
}

func downAgentKVStore20260429(ctx context.Context, tx *sql.Tx) error {
	isSQLite := isSQLiteDB(ctx, tx)

	if !isSQLite {
		if _, err := tx.ExecContext(ctx, "DROP POLICY IF EXISTS isolate_tenant_agent_kv_store ON agent_kv_store;"); err != nil {
			return err
		}
	}

	_, err := tx.ExecContext(ctx, "DROP TABLE IF EXISTS agent_kv_store;")
	return err
}
