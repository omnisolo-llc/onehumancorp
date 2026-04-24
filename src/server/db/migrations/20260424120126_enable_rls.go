package migrations

import (
	"context"
	"database/sql"
	"strings"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upEnableRLS20260424120126, downEnableRLS20260424120126)
}

func upEnableRLS20260424120126(ctx context.Context, tx *sql.Tx) error {
	if _, err := tx.ExecContext(ctx, "SAVEPOINT check_sqlite"); err != nil {
		return err
	}
	var sqliteVersion string
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	isSQLite := err == nil
	if !isSQLite {
		if _, err := tx.ExecContext(ctx, "ROLLBACK TO SAVEPOINT check_sqlite"); err != nil {
			return err
		}
	} else {
		if _, err := tx.ExecContext(ctx, "RELEASE SAVEPOINT check_sqlite"); err != nil {
			return err
		}
	}

	if !isSQLite {
		// PostgreSQL migrations
		_, err := tx.ExecContext(ctx, `
			ALTER TABLE ohc_memory_embeddings ENABLE ROW LEVEL SECURITY;
			ALTER TABLE autodream_memories_master ENABLE ROW LEVEL SECURITY;
			ALTER TABLE mcp_servers ENABLE ROW LEVEL SECURITY;
			ALTER TABLE mcp_config_sync_log ENABLE ROW LEVEL SECURITY;
			ALTER TABLE mcp_audit_sync_log ENABLE ROW LEVEL SECURITY;
			ALTER TABLE crdt_deltas ENABLE ROW LEVEL SECURITY;
			ALTER TABLE local_mcp_rag_tasks ENABLE ROW LEVEL SECURITY;
		`)
		if err != nil && !strings.Contains(err.Error(), "does not exist") {
			return err
		}
		return nil
	}

	return nil
}

func downEnableRLS20260424120126(ctx context.Context, tx *sql.Tx) error {
	// Not easily reversible or necessary to reverse
	return nil
}
