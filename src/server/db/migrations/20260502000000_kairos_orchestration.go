package migrations

import (
	"context"
	"database/sql"
	"fmt"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upKairosOrchestration20260502000000, downKairosOrchestration20260502000000)
}

func upKairosOrchestration20260502000000(ctx context.Context, tx *sql.Tx) error {
	var sqliteVersion string
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	isSQLite := err == nil

	// We create mcp_tool_state here
	query := `
		CREATE TABLE IF NOT EXISTS mcp_tool_state (
			tool_id TEXT NOT NULL,
			key TEXT NOT NULL,
			value TEXT,
			updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
			PRIMARY KEY (tool_id, key)
		)
	`
	if isSQLite {
		query = `
		CREATE TABLE IF NOT EXISTS mcp_tool_state (
			tool_id TEXT NOT NULL,
			key TEXT NOT NULL,
			value TEXT,
			updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			PRIMARY KEY (tool_id, key)
		)
		`
	}

	_, err = tx.ExecContext(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to create mcp_tool_state: %w", err)
	}

	return nil
}

func downKairosOrchestration20260502000000(ctx context.Context, tx *sql.Tx) error {
	_, err := tx.ExecContext(ctx, "DROP TABLE IF EXISTS mcp_tool_state")
	return err
}
