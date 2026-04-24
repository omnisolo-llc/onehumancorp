package migrations

import (
	"context"
	"database/sql"
	"strings"

	"github.com/pressly/goose/v3"
)

func init() {
	goose.AddMigrationContext(upEnableRLSAndCleanup20260424192904, downEnableRLSAndCleanup20260424192904)
}

func upEnableRLSAndCleanup20260424192904(ctx context.Context, tx *sql.Tx) error {
	var sqliteVersion string
	err := tx.QueryRowContext(ctx, "SELECT sqlite_version()").Scan(&sqliteVersion)
	isSQLite := err == nil

	if !isSQLite {
		// PostgreSQL migrations
		_, err := tx.ExecContext(ctx, `
			DROP TABLE IF EXISTS shared_tasks_temp_for_some_reason CASCADE;
			ALTER TABLE scheduled_tasks ENABLE ROW LEVEL SECURITY;
			DROP POLICY IF EXISTS "tenant_isolation_policy" ON scheduled_tasks;
			CREATE POLICY "tenant_isolation_policy" ON scheduled_tasks USING (organization_id = current_setting('app.current_tenant')::text);
			ALTER TABLE users ENABLE ROW LEVEL SECURITY;
			DROP POLICY IF EXISTS "tenant_isolation_policy" ON users;
			CREATE POLICY "tenant_isolation_policy" ON users USING (organization_id = current_setting('app.current_tenant')::text);
			ALTER TABLE shared_tasks_decomposition ENABLE ROW LEVEL SECURITY;
			DROP POLICY IF EXISTS "tenant_isolation_policy" ON shared_tasks_decomposition;
			CREATE POLICY "tenant_isolation_policy" ON shared_tasks_decomposition USING (organization_id = current_setting('app.current_tenant')::varchar);
			ALTER TABLE agent_memory_embeddings ENABLE ROW LEVEL SECURITY;
			DROP POLICY IF EXISTS "tenant_isolation_policy" ON agent_memory_embeddings;
			CREATE POLICY "tenant_isolation_policy" ON agent_memory_embeddings USING (organization_id = current_setting('app.current_tenant')::text);
			ALTER TABLE consolidated_memory ENABLE ROW LEVEL SECURITY;
			DROP POLICY IF EXISTS "tenant_isolation_policy" ON consolidated_memory;
			CREATE POLICY "tenant_isolation_policy" ON consolidated_memory USING (organization_id = current_setting('app.current_tenant')::text);
			ALTER TABLE shared_tasks_master ENABLE ROW LEVEL SECURITY;
			DROP POLICY IF EXISTS "tenant_isolation_policy" ON shared_tasks_master;
			CREATE POLICY "tenant_isolation_policy" ON shared_tasks_master USING (organization_id = current_setting('app.current_tenant')::varchar);
			ALTER TABLE shared_tasks_dag ENABLE ROW LEVEL SECURITY;
			DROP POLICY IF EXISTS "tenant_isolation_policy" ON shared_tasks_dag;
			CREATE POLICY "tenant_isolation_policy" ON shared_tasks_dag USING (organization_id = current_setting('app.current_tenant')::varchar);
			ALTER TABLE mesh_bridges ENABLE ROW LEVEL SECURITY;
			DROP POLICY IF EXISTS "tenant_isolation_policy" ON mesh_bridges;
			CREATE POLICY "tenant_isolation_policy" ON mesh_bridges USING (organization_id = current_setting('app.current_tenant')::varchar);
			ALTER TABLE shared_tasks ENABLE ROW LEVEL SECURITY;
			DROP POLICY IF EXISTS "tenant_isolation_policy" ON shared_tasks;
			CREATE POLICY "tenant_isolation_policy" ON shared_tasks USING (organization_id = current_setting('app.current_tenant')::text);
			ALTER TABLE shared_tasks_v4 ENABLE ROW LEVEL SECURITY;
			DROP POLICY IF EXISTS "tenant_isolation_policy" ON shared_tasks_v4;
			CREATE POLICY "tenant_isolation_policy" ON shared_tasks_v4 USING (organization_id = current_setting('app.current_tenant')::varchar);
			ALTER TABLE agent_memories ENABLE ROW LEVEL SECURITY;
			DROP POLICY IF EXISTS "tenant_isolation_policy" ON agent_memories;
			CREATE POLICY "tenant_isolation_policy" ON agent_memories USING (organization_id = current_setting('app.current_tenant')::varchar);
			ALTER TABLE agents ENABLE ROW LEVEL SECURITY;
			DROP POLICY IF EXISTS "tenant_isolation_policy" ON agents;
			CREATE POLICY "tenant_isolation_policy" ON agents USING (organization_id = current_setting('app.current_tenant')::text);
			ALTER TABLE shared_tasks_v2 ENABLE ROW LEVEL SECURITY;
			DROP POLICY IF EXISTS "tenant_isolation_policy" ON shared_tasks_v2;
			CREATE POLICY "tenant_isolation_policy" ON shared_tasks_v2 USING (organization_id = current_setting('app.current_tenant')::varchar);
			ALTER TABLE usage_events ENABLE ROW LEVEL SECURITY;
			DROP POLICY IF EXISTS "tenant_isolation_policy" ON usage_events;
			CREATE POLICY "tenant_isolation_policy" ON usage_events USING (organization_id = current_setting('app.current_tenant')::text);
			ALTER TABLE tasks ENABLE ROW LEVEL SECURITY;
			DROP POLICY IF EXISTS "tenant_isolation_policy" ON tasks;
			CREATE POLICY "tenant_isolation_policy" ON tasks USING (organization_id = current_setting('app.current_tenant')::uuid);
			ALTER TABLE autodream_memories ENABLE ROW LEVEL SECURITY;
			DROP POLICY IF EXISTS "tenant_isolation_policy" ON autodream_memories;
			CREATE POLICY "tenant_isolation_policy" ON autodream_memories USING (organization_id = current_setting('app.current_tenant')::text);
			ALTER TABLE competitor_metrics ENABLE ROW LEVEL SECURITY;
			DROP POLICY IF EXISTS "tenant_isolation_policy" ON competitor_metrics;
			CREATE POLICY "tenant_isolation_policy" ON competitor_metrics USING (organization_id = current_setting('app.current_tenant')::text);
			ALTER TABLE ohc_tasks ENABLE ROW LEVEL SECURITY;
			DROP POLICY IF EXISTS "tenant_isolation_policy" ON ohc_tasks;
			CREATE POLICY "tenant_isolation_policy" ON ohc_tasks USING (organization_id = current_setting('app.current_tenant')::text);
			ALTER TABLE sub_agent_queue ENABLE ROW LEVEL SECURITY;
			DROP POLICY IF EXISTS "tenant_isolation_policy" ON sub_agent_queue;
			CREATE POLICY "tenant_isolation_policy" ON sub_agent_queue USING (organization_id = current_setting('app.current_tenant')::text);
		`)
		if err != nil && !strings.Contains(err.Error(), "does not exist") {
			return err
		}
		return nil
	}

	return nil
}

func downEnableRLSAndCleanup20260424192904(ctx context.Context, tx *sql.Tx) error {
	// Not easily reversible
	return nil
}
