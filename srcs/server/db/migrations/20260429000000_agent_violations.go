package migrations

import (
	"database/sql"
	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigration(upAgentViolations, downAgentViolations)
}

func upAgentViolations(tx *sql.Tx) error {
	var dialect string
	if err := tx.QueryRow("SELECT sqlite_version()").Scan(&dialect); err == nil {
		// SQLite
		_, err = tx.Exec(`
			CREATE TABLE IF NOT EXISTS agent_violations (
				id TEXT PRIMARY KEY,
				tenant_id TEXT NOT NULL,
				agent_id TEXT NOT NULL,
				session_id TEXT NOT NULL,
				violation_type TEXT NOT NULL,
				details JSONB,
				created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
			);
		`)
		return err
	}

	// PostgreSQL
	_, err := tx.Exec(`
		CREATE TABLE IF NOT EXISTS agent_violations (
			id TEXT PRIMARY KEY,
			tenant_id TEXT NOT NULL,
			agent_id TEXT NOT NULL,
			session_id TEXT NOT NULL,
			violation_type TEXT NOT NULL,
			details JSONB,
			created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		);
		ALTER TABLE agent_violations ENABLE ROW LEVEL SECURITY;
	`)
	return err
}

func downAgentViolations(tx *sql.Tx) error {
	_, err := tx.Exec("DROP TABLE IF EXISTS agent_violations")
	return err
}
