1. **Fix Tenant Isolation Leakage**:
   - I will add PostgreSQL Row-Level Security policies to tables that already have `ENABLE ROW LEVEL SECURITY`.
   - In `srcs/server/db/migrations/20260429000000_kairos_master_blueprint_pg.sql`, append the `CREATE POLICY tenant_isolation_policy ON <table> USING (tenant_id = current_setting('app.current_tenant_id', true));` statement for `kairos_shared_tasks`, `kairos_state_transitions`, `kairos_sub_agent_jobs`, and `autodream_vector_memories`.
   - In `srcs/server/db/migrations/20260427010000_kairos_additional_columns.go`, append the `CREATE POLICY` to the `agent_mesh_messages` table's up migration.

2. **Pre-commit**:
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

3. **Submit the change**:
   - After confirming `bazelisk test //...` passes, call the submit tool.

