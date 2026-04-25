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
	isSqlite := false
	if _, err := tx.ExecContext(ctx, "SAVEPOINT sqlite_probe"); err == nil {
		if err := tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(new(string)); err == nil {
			isSqlite = true
			tx.ExecContext(ctx, "RELEASE SAVEPOINT sqlite_probe")
		} else {
			tx.ExecContext(ctx, "ROLLBACK TO SAVEPOINT sqlite_probe")
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
