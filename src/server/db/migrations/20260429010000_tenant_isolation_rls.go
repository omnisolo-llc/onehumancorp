package migrations

import (
	"context"
	"database/sql"
	"fmt"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upTenantIsolationRLS, downTenantIsolationRLS)
}

// tables to enforce RLS on (tenant-isolated tables)
var tenantTables = []string{
	"agent_inbox",
	"agent_memories",
	"agent_memory_embeddings",
	"agent_mesh_messages",
	"agent_missions",
	"agent_missions_new",
	"agent_missions_old",
	"agent_session_data",
	"agent_status",
	"agents",
	"autodream_knowledge",
	"autodream_memories",
	"autodream_memories_master",
	"capability_plugins",
	"consolidated_memory",
	"crdt_deltas",
	"distributed_locks",
	"embedding_cache",
	"hub_events",
	"hybrid_mcp_sync_queue",
	"knowledge_base",
	"llm_completion_cache",
	"llm_reason_cache",
	"local_cloud_sync_log",
	"local_mcp_rag_tasks",
	"mcp_audit_sync_log",
	"mcp_config_sync_log",
	"mcp_servers",
	"meeting_rooms",
	"meeting_transcripts",
	"memory_conflicts",
	"mesh_bridges",
	"ohc_memory",
	"ohc_memory_embeddings",
	"ohc_tasks",
	"revoked_tokens",
	"scheduled_tasks",
	"shared_task_list_dependencies",
	"shared_task_list_tasks",
	"shared_tasks",
	"shared_tasks_dag",
	"shared_tasks_decomposition",
	"shared_tasks_master",
	"shared_tasks_temp_for_some_reason",
	"shared_tasks_v2",
	"shared_tasks_v4",
	"state_machine_transitions",
	"sub_agent_jobs",
	"sub_agent_queue",
	"swarm_checkpoints",
	"swarm_dream_epochs",
	"swarm_long_term_memory",
	"swarm_memory",
	"swarm_memory_embeddings",
	"swarm_task_dependencies",
	"swarm_tasks",
	"swarm_truth_embeddings",
	"swarm_ultra_plans",
	"task_dependencies",
	"task_dependencies_dag",
	"task_dependencies_master",
	"tasks",
	"team_invites",
	"telemetry_buffer",
	"ultraplan_proposals",
	"ultraplan_votes",
}

func upTenantIsolationRLS(ctx context.Context, tx *sql.Tx) error {
	// Detect if we are on PostgreSQL safely without aborting its transaction
	// SQLite does not have information_schema, and will throw an error, but it won't abort the transaction.
	// PostgreSQL has information_schema, and will succeed.
	var isSQLite bool
	if _, err := tx.ExecContext(ctx, "SAVEPOINT info_schema_probe"); err == nil {
		var testVal int
		if err := tx.QueryRowContext(ctx, "SELECT 1 FROM information_schema.tables LIMIT 1").Scan(&testVal); err != nil {
			isSQLite = true
			tx.ExecContext(ctx, "ROLLBACK TO SAVEPOINT info_schema_probe")
		} else {
			tx.ExecContext(ctx, "RELEASE SAVEPOINT info_schema_probe")
		}
	}
	if isSQLite {
		return nil
	}

	for _, table := range tenantTables {
		// Check if table exists before altering
		var exists bool
		err = tx.QueryRowContext(ctx, "SELECT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_schema='public' AND table_name=$1)", table).Scan(&exists)
		if err != nil || !exists {
			continue
		}

		// 1. Add tenant_id if missing
		_, err = tx.ExecContext(ctx, fmt.Sprintf(`ALTER TABLE %s ADD COLUMN IF NOT EXISTS tenant_id VARCHAR(255) NOT NULL DEFAULT '';`, table))
		if err != nil {
			return fmt.Errorf("failed to add tenant_id to %s: %w", table, err)
		}

		// 2. Enable RLS
		_, err = tx.ExecContext(ctx, fmt.Sprintf(`ALTER TABLE %s ENABLE ROW LEVEL SECURITY;`, table))
		if err != nil {
			return fmt.Errorf("failed to enable RLS on %s: %w", table, err)
		}

		// 3. Recreate Policy
		_, err = tx.ExecContext(ctx, fmt.Sprintf(`DROP POLICY IF EXISTS tenant_isolation_policy ON %s;`, table))
		if err != nil {
			return fmt.Errorf("failed to drop policy on %s: %w", table, err)
		}

		_, err = tx.ExecContext(ctx, fmt.Sprintf(`CREATE POLICY tenant_isolation_policy ON %s USING (tenant_id = current_setting('app.current_tenant', true));`, table))
		if err != nil {
			return fmt.Errorf("failed to create policy on %s: %w", table, err)
		}
	}
	return nil
}

func downTenantIsolationRLS(ctx context.Context, tx *sql.Tx) error {
	var isSQLite bool
	if _, err := tx.ExecContext(ctx, "SAVEPOINT info_schema_probe"); err == nil {
		var testVal int
		if err := tx.QueryRowContext(ctx, "SELECT 1 FROM information_schema.tables LIMIT 1").Scan(&testVal); err != nil {
			isSQLite = true
			tx.ExecContext(ctx, "ROLLBACK TO SAVEPOINT info_schema_probe")
		} else {
			tx.ExecContext(ctx, "RELEASE SAVEPOINT info_schema_probe")
		}
	}
	if isSQLite {
		return nil
	}

	for _, table := range tenantTables {
		var exists bool
		err = tx.QueryRowContext(ctx, "SELECT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_schema='public' AND table_name=$1)", table).Scan(&exists)
		if err != nil || !exists {
			continue
		}

		_, _ = tx.ExecContext(ctx, fmt.Sprintf(`DROP POLICY IF EXISTS tenant_isolation_policy ON %s;`, table))
		_, _ = tx.ExecContext(ctx, fmt.Sprintf(`ALTER TABLE %s DISABLE ROW LEVEL SECURITY;`, table))
	}
	return nil
}
