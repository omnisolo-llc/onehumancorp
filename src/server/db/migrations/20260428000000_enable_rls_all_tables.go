package migrations

import (
	"context"
	"database/sql"
	"strings"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upEnableRLSAllTables20260428000000, downEnableRLSAllTables20260428000000)
}

func upEnableRLSAllTables20260428000000(ctx context.Context, tx *sql.Tx) error {
	var sqliteVersion string
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	isSQLite := err == nil

	if !isSQLite {
		tables := []string{
			"tasks", "shared_tasks", "mcp_servers", "local_mcp_rag_tasks",
			"autodream_memories_master", "mcp_audit_sync_log", "crdt_deltas",
			"agent_missions", "users", "agent_memories", "shared_tasks_dag",
			"scheduled_tasks", "shared_tasks_decomposition", "agent_memory_embeddings",
			"autodream_memories", "ohc_memory_embeddings", "competitor_metrics",
			"usage_events", "mesh_bridges", "shared_tasks_v2", "agents",
			"shared_tasks_master", "consolidated_memory", "sub_agent_queue",
			"mcp_config_sync_log", "shared_tasks_temp_for_some_reason",
			"shared_tasks_v4", "ohc_tasks",
		}

		for _, table := range tables {
			query := "ALTER TABLE " + table + " ENABLE ROW LEVEL SECURITY;"
			_, execErr := tx.ExecContext(ctx, query)
			if execErr != nil && !strings.Contains(execErr.Error(), "does not exist") {
				// Ignore errors for tables that might not exist in all environments or test schemas
			}
		}
	}
	return nil
}

func downEnableRLSAllTables20260428000000(ctx context.Context, tx *sql.Tx) error {
	var sqliteVersion string
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	isSQLite := err == nil

	if !isSQLite {
		tables := []string{
			"tasks", "shared_tasks", "mcp_servers", "local_mcp_rag_tasks",
			"autodream_memories_master", "mcp_audit_sync_log", "crdt_deltas",
			"agent_missions", "users", "agent_memories", "shared_tasks_dag",
			"scheduled_tasks", "shared_tasks_decomposition", "agent_memory_embeddings",
			"autodream_memories", "ohc_memory_embeddings", "competitor_metrics",
			"usage_events", "mesh_bridges", "shared_tasks_v2", "agents",
			"shared_tasks_master", "consolidated_memory", "sub_agent_queue",
			"mcp_config_sync_log", "shared_tasks_temp_for_some_reason",
			"shared_tasks_v4", "ohc_tasks",
		}

		for _, table := range tables {
			query := "ALTER TABLE " + table + " DISABLE ROW LEVEL SECURITY;"
			_, _ = tx.ExecContext(ctx, query)
		}
	}
	return nil
}
