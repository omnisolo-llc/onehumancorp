package migrations

import (
	"context"
	"database/sql"
	"fmt"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upEnableMissingRLS, downEnableMissingRLS)
}

func upEnableMissingRLS(ctx context.Context, tx *sql.Tx) error {
	// Attempt SAVEPOINT for sqlite detection
	_, err := tx.ExecContext(ctx, "SAVEPOINT sqlite_check")
	if err != nil {
		return fmt.Errorf("failed to create savepoint: %w", err)
	}

	var sqliteVersion string
	err = tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	isSqlite := err == nil

	// Always rollback the savepoint to clear any ABORTED transaction state in Postgres
	_, err = tx.ExecContext(ctx, "ROLLBACK TO SAVEPOINT sqlite_check")
	if err != nil {
		return fmt.Errorf("failed to rollback savepoint: %w", err)
	}

	if isSqlite {
		// SQLite doesn't support ENABLE ROW LEVEL SECURITY
		return nil
	}

	// We are on Postgres. Enable RLS on all identified tables.
	tables := []string{
		"agent_memories",
		"agent_memory_embeddings",
		"agents",
		"autodream_memories",
		"autodream_memories_master",
		"autodream_vector_memories",
		"competitor_metrics",
		"consolidated_memory",
		"crdt_deltas",
		"kairos_shared_tasks",
		"kairos_state_transitions",
		"kairos_sub_agent_jobs",
		"mcp_config_sync_log",
		"mcp_servers",
		"mesh_bridges",
		"ohc_memory_embeddings",
		"ohc_tasks",
		"scheduled_tasks",
		"shared_tasks",
		"shared_tasks_dag",
		"shared_tasks_decomposition",
		"shared_tasks_master",
		"shared_tasks_v2",
		"shared_tasks_v4",
		"sub_agent_queue",
		"tasks",
		"usage_events",
		"users",
		"local_mcp_rag_tasks",
		"mcp_audit_sync_log",
	}

	for _, table := range tables {
		query := fmt.Sprintf("ALTER TABLE IF EXISTS %s ENABLE ROW LEVEL SECURITY;", table)
		_, err := tx.ExecContext(ctx, query)
		if err != nil {
			return fmt.Errorf("failed to enable RLS on %s: %w", table, err)
		}
	}

	return nil
}

func downEnableMissingRLS(ctx context.Context, tx *sql.Tx) error {
	return nil
}
