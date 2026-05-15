-- Migration: 003_rls_hardening.sql
-- Harden multi-tenant isolation by enabling RLS on all remaining tables
-- and adding strict tenant-based isolation policies.

-- Ensure ohc_bypassrls role exists (already in 002, but for idempotency)
DO $$
BEGIN
    IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = 'ohc_bypassrls') THEN
        CREATE ROLE ohc_bypassrls;
    END IF;
END
$$;

-- Function to enable RLS and add policy to a table
CREATE OR REPLACE FUNCTION enable_rls_isolation(table_name_text TEXT, column_name_text TEXT DEFAULT 'tenant_id')
RETURNS VOID AS $$
BEGIN
    EXECUTE format('ALTER TABLE %I ENABLE ROW LEVEL SECURITY', table_name_text);

    -- Drop existing policy if it exists to avoid errors on re-run
    EXECUTE format('DROP POLICY IF EXISTS %I ON %I', 'tenant_isolation_' || table_name_text, table_name_text);

    EXECUTE format('CREATE POLICY %I ON %I USING (%I::text = current_setting(%L, true))',
        'tenant_isolation_' || table_name_text,
        table_name_text,
        column_name_text,
        'app.current_tenant'
    );
END;
$$ LANGUAGE plpgsql;

-- Apply RLS to remaining tables
SELECT enable_rls_isolation('agent_approvals');
SELECT enable_rls_isolation('agent_inbox');
SELECT enable_rls_isolation('agent_missions');
SELECT enable_rls_isolation('agent_session_data', 'agent_id'); -- session data isolated by agent/context
SELECT enable_rls_isolation('agent_status');
SELECT enable_rls_isolation('agent_violations');
SELECT enable_rls_isolation('autodream_memories');
SELECT enable_rls_isolation('competitor_metrics');
SELECT enable_rls_isolation('consolidated_memory');
SELECT enable_rls_isolation('department_tasks');
SELECT enable_rls_isolation('hybrid_fs_sync_queue');
SELECT enable_rls_isolation('meeting_rooms');
SELECT enable_rls_isolation('meeting_transcripts');
SELECT enable_rls_isolation('memories');
SELECT enable_rls_isolation('onboarding_state');
SELECT enable_rls_isolation('order_items');
SELECT enable_rls_isolation('pages');
SELECT enable_rls_isolation('referrals');
SELECT enable_rls_isolation('roles', 'tenant_id');
SELECT enable_rls_isolation('shared_tasks');
SELECT enable_rls_isolation('shared_tasks_decomposition', 'organization_id');
SELECT enable_rls_isolation('shared_tasks_v4');
SELECT enable_rls_isolation('state_machine_transitions');
SELECT enable_rls_isolation('swarm_tasks', 'mission_id'); -- swarm tasks isolated by mission
SELECT enable_rls_isolation('swarm_truth_embeddings', 'memory_id'); -- truth isolated by memory id

-- Special case for tables without obvious tenant_id or those that are system-wide
-- but should still be protected.
ALTER TABLE revoked_tokens ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_revoked_tokens ON revoked_tokens;
CREATE POLICY tenant_isolation_revoked_tokens ON revoked_tokens USING (tenant_id::text = current_setting('app.current_tenant', true) OR current_user = 'ohc_bypassrls');

-- Cleanup
DROP FUNCTION enable_rls_isolation(TEXT, TEXT);
