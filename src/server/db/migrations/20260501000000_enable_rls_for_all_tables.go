package migrations

import (
	"context"
	"database/sql"
	"fmt"
	"strings"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upEnableRLS, downEnableRLS)
}

func isSQLiteDialect() bool {
	return goose.GetDialect() == "sqlite3" || goose.GetDialect() == "sqlite"
}

func upEnableRLS(ctx context.Context, tx *sql.Tx) error {
	if isSQLiteDialect() {
		return nil
	}

	rows, err := tx.QueryContext(ctx, "SELECT schemaname || '.' || tablename FROM pg_catalog.pg_tables")
	if err != nil {
		return err
	}
	defer rows.Close()

	existingTables := make(map[string]bool)
	for rows.Next() {
		var table string
		if err := rows.Scan(&table); err != nil {
			return err
		}
		existingTables[table] = true
	}
	if err := rows.Err(); err != nil {
		return err
	}
	if err := rows.Err(); err != nil {
		return err
	}
	for k := range existingTables {
		if strings.HasPrefix(k, "public.") {
			existingTables[strings.TrimPrefix(k, "public.")] = true
		}
	}

	tableConfigs := map[string]string{
		"agent_inbox":                       "organization_id",
		"agent_memories":                    "tenant_id",
		"agent_memory_embeddings":           "tenant_id",
		"agent_mesh_messages":               "tenant_id",
		"agent_session_data":                "tenant_id",
		"agents":                            "tenant_id",
		"autodream_memories":                "organization_id",
		"autodream_memories_master":         "organization_id",
		"competitor_metrics":                "tenant_id",
		"consolidated_memory":               "tenant_id",
		"crdt_deltas":                       "tenant_id",
		"local_mcp_rag_tasks":               "tenant_id",
		"mcp_audit_sync_log":                "tenant_id",
		"mcp_config_sync_log":               "tenant_id",
		"mcp_servers":                       "tenant_id",
		"meeting_rooms":                     "organization_id",
		"meeting_transcripts":               "tenant_id",
		"memory_conflicts":                  "tenant_id",
		"mesh_bridges":                      "tenant_id",
		"ohc_memory_embeddings":             "organization_id",
		"ohc_tasks":                         "tenant_id",
		"revoked_tokens":                    "tenant_id",
		"roles":                             "tenant_id",
		"scheduled_tasks":                   "tenant_id",
		"shared_tasks":                      "tenant_id",
		"shared_tasks_dag":                  "tenant_id",
		"shared_tasks_decomposition":        "tenant_id",
		"shared_tasks_master":               "tenant_id",
		"shared_tasks_temp_for_some_reason": "tenant_id",
		"shared_tasks_v2":                   "tenant_id",
		"shared_tasks_v4":                   "tenant_id",
		"state_machine_transitions":         "tenant_id",
		"sub_agent_queue":                   "tenant_id",
		"swarm_truth_embeddings":            "tenant_id",
		"task_dependencies":                 "tenant_id",
		"task_dependencies_dag":             "tenant_id",
		"task_dependencies_master":          "tenant_id",
		"tasks":                             "organization_id",
		"usage_events":                      "tenant_id",
		"agent_missions":                    "tenant_id",
		"autodream_knowledge":               "tenant_id",
		"capability_plugins":                "tenant_id",
		"distributed_locks":                 "tenant_id",
		"embedding_cache":                   "tenant_id",
		"hub_events":                        "tenant_id",
		"hybrid_mcp_sync_queue":             "tenant_id",
		"knowledge_base":                    "tenant_id",
		"llm_completion_cache":              "tenant_id",
		"llm_reason_cache":                  "tenant_id",
		"local_cloud_sync_log":              "tenant_id",
		"shared_task_list_dependencies":     "tenant_id",
		"shared_task_list_tasks":            "tenant_id",
		"sub_agent_jobs":                    "tenant_id",
		"swarm_checkpoints":                 "tenant_id",
		"swarm_dream_epochs":                "tenant_id",
		"swarm_long_term_memory":            "tenant_id",
		"swarm_memory":                      "tenant_id",
		"swarm_memory_embeddings":           "tenant_id",
		"swarm_task_dependencies":           "tenant_id",
		"swarm_tasks":                       "tenant_id",
		"swarm_ultra_plans":                 "tenant_id",
		"team_invites":                      "tenant_id",
		"telemetry_buffer":                  "tenant_id",
		"ultraplan_proposals":               "tenant_id",
		"ultraplan_votes":                   "tenant_id",
		"users":                             "tenant_id",
	}

	for table, col := range tableConfigs {
		if !existingTables[table] {
			continue
		}

		_, err := tx.ExecContext(ctx, fmt.Sprintf("ALTER TABLE %s ENABLE ROW LEVEL SECURITY", table))
		if err != nil {
			return fmt.Errorf("failed to enable rls on %s: %w", table, err)
		}

		_, err = tx.ExecContext(ctx, fmt.Sprintf("DROP POLICY IF EXISTS tenant_isolation_policy ON %s", table))
		if err != nil {
			return fmt.Errorf("failed to drop old policy on %s: %w", table, err)
		}

		_, err = tx.ExecContext(ctx, fmt.Sprintf("CREATE POLICY tenant_isolation_policy ON %s USING (%s = current_setting('app.current_tenant', true))", table, col))
		if err != nil {
			return fmt.Errorf("failed to create policy on %s: %w", table, err)
		}
	}

	return nil
}

func downEnableRLS(ctx context.Context, tx *sql.Tx) error {
	if isSQLiteDialect() {
		return nil
	}

	rows, err := tx.QueryContext(ctx, "SELECT schemaname || '.' || tablename FROM pg_catalog.pg_tables")
	if err != nil {
		return err
	}
	defer rows.Close()

	existingTables := make(map[string]bool)
	for rows.Next() {
		var table string
		if err := rows.Scan(&table); err != nil {
			return err
		}
		existingTables[table] = true
	}
	for k := range existingTables {
		if strings.HasPrefix(k, "public.") {
			existingTables[strings.TrimPrefix(k, "public.")] = true
		}
	}

	tables := []string{
		"agent_inbox",
		"agent_memories",
		"agent_memory_embeddings",
		"agent_mesh_messages",
		"agent_session_data",
		"agents",
		"autodream_memories",
		"autodream_memories_master",
		"competitor_metrics",
		"consolidated_memory",
		"crdt_deltas",
		"local_mcp_rag_tasks",
		"mcp_audit_sync_log",
		"mcp_config_sync_log",
		"mcp_servers",
		"meeting_rooms",
		"meeting_transcripts",
		"memory_conflicts",
		"mesh_bridges",
		"ohc_memory_embeddings",
		"ohc_tasks",
		"revoked_tokens",
		"roles",
		"scheduled_tasks",
		"shared_tasks",
		"shared_tasks_dag",
		"shared_tasks_decomposition",
		"shared_tasks_master",
		"shared_tasks_temp_for_some_reason",
		"shared_tasks_v2",
		"shared_tasks_v4",
		"state_machine_transitions",
		"sub_agent_queue",
		"swarm_truth_embeddings",
		"task_dependencies",
		"task_dependencies_dag",
		"task_dependencies_master",
		"tasks",
		"usage_events",
		"agent_missions",
		"autodream_knowledge",
		"capability_plugins",
		"distributed_locks",
		"embedding_cache",
		"hub_events",
		"hybrid_mcp_sync_queue",
		"knowledge_base",
		"llm_completion_cache",
		"llm_reason_cache",
		"local_cloud_sync_log",
		"shared_task_list_dependencies",
		"shared_task_list_tasks",
		"sub_agent_jobs",
		"swarm_checkpoints",
		"swarm_dream_epochs",
		"swarm_long_term_memory",
		"swarm_memory",
		"swarm_memory_embeddings",
		"swarm_task_dependencies",
		"swarm_tasks",
		"swarm_ultra_plans",
		"team_invites",
		"telemetry_buffer",
		"ultraplan_proposals",
		"ultraplan_votes",
		"users",
	}

	for _, table := range tables {
		if !existingTables[table] {
			continue
		}

		_, err := tx.ExecContext(ctx, fmt.Sprintf("DROP POLICY IF EXISTS tenant_isolation_policy ON %s", table))
		if err != nil {
			return fmt.Errorf("failed to drop policy on %s: %w", table, err)
		}

		_, err = tx.ExecContext(ctx, fmt.Sprintf("ALTER TABLE %s DISABLE ROW LEVEL SECURITY", table))
		if err != nil {
			return fmt.Errorf("failed to disable rls on %s: %w", table, err)
		}
	}

	return nil
}
