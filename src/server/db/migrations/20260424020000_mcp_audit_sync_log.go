package migrations

import (
	"context"
	"database/sql"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upMCPAuditSyncLog, downMCPAuditSyncLog)
}

func upMCPAuditSyncLog(ctx context.Context, tx *sql.Tx) error {
	// Let's create a generic table or use a single migration to avoid goose crashing due to identical timestamp
	// Check for sqlite
	if _, err := tx.ExecContext(ctx, "SAVEPOINT check_sqlite"); err != nil {
		return err
	}
	var version string
	err := tx.QueryRowContext(ctx, "select sqlite_version()").Scan(&version)
	isSqlite := err == nil
	if !isSqlite {
		if _, err := tx.ExecContext(ctx, "ROLLBACK TO SAVEPOINT check_sqlite"); err != nil {
			return err
		}
	} else {
		if _, err := tx.ExecContext(ctx, "RELEASE SAVEPOINT check_sqlite"); err != nil {
			return err
		}
	}

	var query string
	if isSqlite {
		query = `CREATE TABLE IF NOT EXISTS mcp_audit_sync_log (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			tenant_id VARCHAR(255) NOT NULL,
			agent_id VARCHAR(255) NOT NULL,
			action VARCHAR(255) NOT NULL,
			resource VARCHAR(255) NOT NULL,
			status VARCHAR(50) NOT NULL,
			metadata TEXT,
			timestamp BIGINT NOT NULL,
			created_at BIGINT NOT NULL
		);`
	} else {
		query = `CREATE TABLE IF NOT EXISTS mcp_audit_sync_log (
			id SERIAL PRIMARY KEY,
			tenant_id VARCHAR(255) NOT NULL,
			agent_id VARCHAR(255) NOT NULL,
			action VARCHAR(255) NOT NULL,
			resource VARCHAR(255) NOT NULL,
			status VARCHAR(50) NOT NULL,
			metadata TEXT,
			timestamp BIGINT NOT NULL,
			created_at BIGINT NOT NULL
		);`
	}

	_, err = tx.ExecContext(ctx, query)
	return err
}

func downMCPAuditSyncLog(ctx context.Context, tx *sql.Tx) error {
	_, err := tx.ExecContext(ctx, "DROP TABLE IF EXISTS mcp_audit_sync_log;")
	return err
}
