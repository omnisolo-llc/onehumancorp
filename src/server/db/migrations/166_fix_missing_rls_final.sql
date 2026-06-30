-- +goose Up
-- Add missing RLS policies to enforce tenant isolation
-- This is a cleaner sweep of missing policies

src/server/db/migrations/026_smart_pricing.sql-    USING (tenant_id::text = current_setting('app.current_tenant', true))
src/server/db/migrations/026_smart_pricing.sql-    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/026_smart_pricing.sql-    tenant_id UUID NOT NULL,
src/server/db/migrations/026_smart_pricing.sql-CREATE INDEX IF NOT EXISTS idx_active_discounts_tenant ON active_discounts(tenant_id);
src/server/db/migrations/026_smart_pricing.sql-    USING (tenant_id::text = current_setting('app.current_tenant', true))
src/server/db/migrations/026_smart_pricing.sql-    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for active_discounts
ALTER TABLE IF EXISTS active_discounts ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS active_discounts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_active_discounts ON active_discounts;
CREATE POLICY tenant_isolation_active_discounts ON active_discounts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/164_affiliate_marketing.sql-CREATE INDEX IF NOT EXISTS idx_affiliate_links_tenant ON affiliate_links(tenant_id);
src/server/migrations/164_affiliate_marketing.sql-CREATE POLICY tenant_isolation_affiliate_links ON affiliate_links USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/164_affiliate_marketing.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/164_affiliate_marketing.sql-    UNIQUE(tenant_id, customer_id)
src/server/migrations/164_affiliate_marketing.sql-CREATE INDEX IF NOT EXISTS idx_affiliate_ledgers_tenant ON affiliate_ledgers(tenant_id);
src/server/migrations/164_affiliate_marketing.sql-CREATE POLICY tenant_isolation_affiliate_ledgers ON affiliate_ledgers USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/164_affiliate_marketing.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/164_affiliate_marketing.sql-CREATE INDEX IF NOT EXISTS idx_affiliate_payouts_tenant ON affiliate_payouts(tenant_id);
src/server/migrations/164_affiliate_marketing.sql-CREATE POLICY tenant_isolation_affiliate_payouts ON affiliate_payouts USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/017_affiliate_marketing.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/017_affiliate_marketing.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/017_affiliate_marketing.sql-CREATE INDEX IF NOT EXISTS idx_affiliate_ledgers_tenant ON affiliate_ledgers(tenant_id);
src/server/db/migrations/017_affiliate_marketing.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/017_affiliate_marketing.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/017_affiliate_marketing.sql-CREATE INDEX IF NOT EXISTS idx_affiliate_payouts_tenant ON affiliate_payouts(tenant_id);
src/server/db/migrations/017_affiliate_marketing.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for affiliate_ledgers
ALTER TABLE IF EXISTS affiliate_ledgers ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS affiliate_ledgers ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_affiliate_ledgers ON affiliate_ledgers;
CREATE POLICY tenant_isolation_affiliate_ledgers ON affiliate_ledgers USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/164_affiliate_marketing.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/164_affiliate_marketing.sql-CREATE INDEX IF NOT EXISTS idx_affiliate_links_tenant ON affiliate_links(tenant_id);
src/server/migrations/164_affiliate_marketing.sql-CREATE POLICY tenant_isolation_affiliate_links ON affiliate_links USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/164_affiliate_marketing.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/164_affiliate_marketing.sql-    UNIQUE(tenant_id, customer_id)
src/server/migrations/164_affiliate_marketing.sql-CREATE INDEX IF NOT EXISTS idx_affiliate_ledgers_tenant ON affiliate_ledgers(tenant_id);
src/server/migrations/164_affiliate_marketing.sql-CREATE POLICY tenant_isolation_affiliate_ledgers ON affiliate_ledgers USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/164_affiliate_marketing.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/164_affiliate_marketing.sql-CREATE INDEX IF NOT EXISTS idx_affiliate_payouts_tenant ON affiliate_payouts(tenant_id);
src/server/migrations/164_affiliate_marketing.sql-CREATE POLICY tenant_isolation_affiliate_payouts ON affiliate_payouts USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/017_affiliate_marketing.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/017_affiliate_marketing.sql-CREATE INDEX IF NOT EXISTS idx_affiliate_links_tenant ON affiliate_links(tenant_id);
src/server/db/migrations/017_affiliate_marketing.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/017_affiliate_marketing.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/017_affiliate_marketing.sql-CREATE INDEX IF NOT EXISTS idx_affiliate_ledgers_tenant ON affiliate_ledgers(tenant_id);
src/server/db/migrations/017_affiliate_marketing.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/017_affiliate_marketing.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/017_affiliate_marketing.sql-CREATE INDEX IF NOT EXISTS idx_affiliate_payouts_tenant ON affiliate_payouts(tenant_id);
src/server/db/migrations/017_affiliate_marketing.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for affiliate_links
ALTER TABLE IF EXISTS affiliate_links ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS affiliate_links ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_affiliate_links ON affiliate_links;
CREATE POLICY tenant_isolation_affiliate_links ON affiliate_links USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/164_affiliate_marketing.sql-CREATE INDEX IF NOT EXISTS idx_affiliate_ledgers_tenant ON affiliate_ledgers(tenant_id);
src/server/migrations/164_affiliate_marketing.sql-CREATE POLICY tenant_isolation_affiliate_ledgers ON affiliate_ledgers USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/164_affiliate_marketing.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/164_affiliate_marketing.sql-CREATE INDEX IF NOT EXISTS idx_affiliate_payouts_tenant ON affiliate_payouts(tenant_id);
src/server/migrations/164_affiliate_marketing.sql-CREATE POLICY tenant_isolation_affiliate_payouts ON affiliate_payouts USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/017_affiliate_marketing.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/017_affiliate_marketing.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/017_affiliate_marketing.sql-CREATE INDEX IF NOT EXISTS idx_affiliate_payouts_tenant ON affiliate_payouts(tenant_id);
src/server/db/migrations/017_affiliate_marketing.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for affiliate_payouts
ALTER TABLE IF EXISTS affiliate_payouts ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS affiliate_payouts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_affiliate_payouts ON affiliate_payouts;
CREATE POLICY tenant_isolation_affiliate_payouts ON affiliate_payouts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/108_inventory_agent_actions.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/108_inventory_agent_actions.sql-            CREATE POLICY tenant_isolation_agent_action_requests ON agent_action_requests USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for agent_action_requests
ALTER TABLE IF EXISTS agent_action_requests ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS agent_action_requests ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_action_requests ON agent_action_requests;
CREATE POLICY tenant_isolation_agent_action_requests ON agent_action_requests USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/074_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/074_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/074_missing_tables.sql-CREATE INDEX IF NOT EXISTS idx_customer_timeline_tenant_customer ON customer_timeline(tenant_id, customer_id);
-- Adding RLS for agent_actions
ALTER TABLE IF EXISTS agent_actions ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS agent_actions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_actions ON agent_actions;
CREATE POLICY tenant_isolation_agent_actions ON agent_actions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    PRIMARY KEY (tenant_id, user_id)
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
-- Adding RLS for agent_approvals
ALTER TABLE IF EXISTS agent_approvals ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS agent_approvals ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_approvals ON agent_approvals;
CREATE POLICY tenant_isolation_agent_approvals ON agent_approvals USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/010_agent_departments.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/010_agent_departments.sql-    UNIQUE(tenant_id, department_type)
src/server/migrations/010_agent_departments.sql-CREATE POLICY tenant_isolation_agent_departments ON agent_departments USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for agent_departments
ALTER TABLE IF EXISTS agent_departments ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS agent_departments ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_departments ON agent_departments;
CREATE POLICY tenant_isolation_agent_departments ON agent_departments USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/143_agent_feed_items.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/143_agent_feed_items.sql-        CREATE POLICY tenant_isolation_agent_feed_items ON agent_feed_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/031_a_agent_feed.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/031_a_agent_feed.sql-        CREATE POLICY tenant_isolation_agent_feed_items ON agent_feed_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for agent_feed_items
ALTER TABLE IF EXISTS agent_feed_items ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS agent_feed_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_feed_items ON agent_feed_items;
CREATE POLICY tenant_isolation_agent_feed_items ON agent_feed_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_shared_tasks_v4 ON shared_tasks_v4 USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_shared_tasks ON shared_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_agent_approvals ON agent_approvals USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_onboarding_state ON onboarding_state USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_referrals ON referrals USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_competitor_metrics ON competitor_metrics USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_agent_violations ON agent_violations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for agent_inbox
ALTER TABLE IF EXISTS agent_inbox ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS agent_inbox ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_inbox ON agent_inbox;
CREATE POLICY tenant_isolation_agent_inbox ON agent_inbox USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/138_agent_jobs.sql-    tenant_id VARCHAR NOT NULL,
src/server/migrations/138_agent_jobs.sql-        USING (tenant_id = current_setting('app.current_tenant', true))
src/server/migrations/138_agent_jobs.sql-        WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for agent_jobs
ALTER TABLE IF EXISTS agent_jobs ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS agent_jobs ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_jobs ON agent_jobs;
CREATE POLICY tenant_isolation_agent_jobs ON agent_jobs USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/005_agent_kv_store.sql-    tenant_id VARCHAR(255) NOT NULL,
src/server/migrations/005_agent_kv_store.sql-    PRIMARY KEY (tenant_id, kv_key)
src/server/migrations/005_agent_kv_store.sql-CREATE POLICY tenant_isolation_agent_kv_store ON agent_kv_store USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for agent_kv_store
ALTER TABLE IF EXISTS agent_kv_store ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS agent_kv_store ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_kv_store ON agent_kv_store;
CREATE POLICY tenant_isolation_agent_kv_store ON agent_kv_store USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-CREATE POLICY tenant_isolation_users ON users USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/001_initial.sql-CREATE POLICY tenant_isolation_agents ON agents USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/001_initial.sql-CREATE POLICY tenant_isolation_tasks ON tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/001_initial.sql-CREATE POLICY tenant_isolation_products ON products USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/001_initial.sql-CREATE POLICY tenant_isolation_orders ON orders USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/001_initial.sql-CREATE POLICY tenant_isolation_customers ON customers USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/001_initial.sql-CREATE POLICY tenant_isolation_bookings ON bookings USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/001_initial.sql-CREATE POLICY tenant_isolation_agent_memories ON agent_memories USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/001_initial.sql-CREATE POLICY tenant_isolation_knowledge_embeddings ON knowledge_embeddings USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for agent_memories
ALTER TABLE IF EXISTS agent_memories ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS agent_memories ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_memories ON agent_memories;
CREATE POLICY tenant_isolation_agent_memories ON agent_memories USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
-- Adding RLS for agent_missions
ALTER TABLE IF EXISTS agent_missions ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS agent_missions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_missions ON agent_missions;
CREATE POLICY tenant_isolation_agent_missions ON agent_missions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
-- Adding RLS for agent_status
ALTER TABLE IF EXISTS agent_status ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS agent_status ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_status ON agent_status;
CREATE POLICY tenant_isolation_agent_status ON agent_status USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    organization_id TEXT NOT NULL,
-- Adding RLS for agent_violations
ALTER TABLE IF EXISTS agent_violations ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS agent_violations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_violations ON agent_violations;
CREATE POLICY tenant_isolation_agent_violations ON agent_violations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/001_initial.sql-    tenant_id TEXT DEFAULT 'system',
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
-- Adding RLS for agents
ALTER TABLE IF EXISTS agents ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS agents ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agents ON agents;
CREATE POLICY tenant_isolation_agents ON agents USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/008_data_model_architecture.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/008_data_model_architecture.sql-                        'CREATE POLICY %I ON %I USING (tenant_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (tenant_id::text = current_setting(''app.current_tenant'', true))',
-- Adding RLS for ai_memories
ALTER TABLE IF EXISTS ai_memories ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS ai_memories ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ai_memories ON ai_memories;
CREATE POLICY tenant_isolation_ai_memories ON ai_memories USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/147_offline_mutation_idempotency.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/147_offline_mutation_idempotency.sql-    USING (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/147_offline_mutation_idempotency.sql-CREATE INDEX IF NOT EXISTS idx_applied_client_mutations_tenant ON applied_client_mutations(tenant_id);
-- Adding RLS for applied_client_mutations
ALTER TABLE IF EXISTS applied_client_mutations ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS applied_client_mutations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_applied_client_mutations ON applied_client_mutations;
CREATE POLICY tenant_isolation_applied_client_mutations ON applied_client_mutations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/162_field_ops_appointments.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/162_field_ops_appointments.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/162_field_ops_appointments.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/162_field_ops_appointments.sql-    USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/162_field_ops_appointments.sql-    USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/162_field_ops_appointments.sql-    USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/036_a_autonomous_work_scheduling.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/036_a_autonomous_work_scheduling.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/036_a_autonomous_work_scheduling.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/036_a_autonomous_work_scheduling.sql-CREATE INDEX IF NOT EXISTS idx_appointments_tenant_id ON appointments(tenant_id);
src/server/db/migrations/036_a_autonomous_work_scheduling.sql-CREATE INDEX IF NOT EXISTS idx_appointments_staff ON appointments(tenant_id, staff_profile_id, scheduled_start_time);
src/server/db/migrations/036_a_autonomous_work_scheduling.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/036_a_autonomous_work_scheduling.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for appointments
ALTER TABLE IF EXISTS appointments ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS appointments ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_appointments ON appointments;
CREATE POLICY tenant_isolation_appointments ON appointments USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/028_assistant_workstation.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/028_assistant_workstation.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/028_assistant_workstation.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/028_assistant_workstation.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/028_assistant_workstation.sql-    UNIQUE (tenant_id, name)
-- Adding RLS for assistant_artifacts
ALTER TABLE IF EXISTS assistant_artifacts ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS assistant_artifacts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_artifacts ON assistant_artifacts;
CREATE POLICY tenant_isolation_assistant_artifacts ON assistant_artifacts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/028_assistant_workstation.sql-    UNIQUE (tenant_id, name)
src/server/db/migrations/028_assistant_workstation.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/028_assistant_workstation.sql-    UNIQUE (tenant_id, name)
src/server/db/migrations/028_assistant_workstation.sql-        CREATE POLICY tenant_isolation_assistant_workspaces ON assistant_workspaces USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/028_assistant_workstation.sql-        CREATE POLICY tenant_isolation_assistant_tasks ON assistant_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/028_assistant_workstation.sql-        CREATE POLICY tenant_isolation_assistant_messages ON assistant_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/028_assistant_workstation.sql-        CREATE POLICY tenant_isolation_assistant_artifacts ON assistant_artifacts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/028_assistant_workstation.sql-        CREATE POLICY tenant_isolation_assistant_file_changes ON assistant_file_changes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/028_assistant_workstation.sql-        CREATE POLICY tenant_isolation_assistant_memory_records ON assistant_memory_records USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/028_assistant_workstation.sql-        CREATE POLICY tenant_isolation_assistant_skills ON assistant_skills USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/028_assistant_workstation.sql-        CREATE POLICY tenant_isolation_assistant_connectors ON assistant_connectors USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for assistant_connectors
ALTER TABLE IF EXISTS assistant_connectors ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS assistant_connectors ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_connectors ON assistant_connectors;
CREATE POLICY tenant_isolation_assistant_connectors ON assistant_connectors USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/028_assistant_workstation.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/028_assistant_workstation.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/028_assistant_workstation.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/028_assistant_workstation.sql-    UNIQUE (tenant_id, name)
src/server/db/migrations/028_assistant_workstation.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/028_assistant_workstation.sql-    UNIQUE (tenant_id, name)
-- Adding RLS for assistant_file_changes
ALTER TABLE IF EXISTS assistant_file_changes ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS assistant_file_changes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_file_changes ON assistant_file_changes;
CREATE POLICY tenant_isolation_assistant_file_changes ON assistant_file_changes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/028_assistant_workstation.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/028_assistant_workstation.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/028_assistant_workstation.sql-    UNIQUE (tenant_id, name)
src/server/db/migrations/028_assistant_workstation.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/028_assistant_workstation.sql-    UNIQUE (tenant_id, name)
src/server/db/migrations/028_assistant_workstation.sql-        CREATE POLICY tenant_isolation_assistant_workspaces ON assistant_workspaces USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/028_assistant_workstation.sql-        CREATE POLICY tenant_isolation_assistant_tasks ON assistant_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for assistant_memory_records
ALTER TABLE IF EXISTS assistant_memory_records ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS assistant_memory_records ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_memory_records ON assistant_memory_records;
CREATE POLICY tenant_isolation_assistant_memory_records ON assistant_memory_records USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/028_assistant_workstation.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/028_assistant_workstation.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/028_assistant_workstation.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/028_assistant_workstation.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/028_assistant_workstation.sql-    tenant_id TEXT NOT NULL,
-- Adding RLS for assistant_messages
ALTER TABLE IF EXISTS assistant_messages ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS assistant_messages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_messages ON assistant_messages;
CREATE POLICY tenant_isolation_assistant_messages ON assistant_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/028_assistant_workstation.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/028_assistant_workstation.sql-    UNIQUE (tenant_id, name)
src/server/db/migrations/028_assistant_workstation.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/028_assistant_workstation.sql-    UNIQUE (tenant_id, name)
src/server/db/migrations/028_assistant_workstation.sql-        CREATE POLICY tenant_isolation_assistant_workspaces ON assistant_workspaces USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/028_assistant_workstation.sql-        CREATE POLICY tenant_isolation_assistant_tasks ON assistant_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/028_assistant_workstation.sql-        CREATE POLICY tenant_isolation_assistant_messages ON assistant_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/028_assistant_workstation.sql-        CREATE POLICY tenant_isolation_assistant_artifacts ON assistant_artifacts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/028_assistant_workstation.sql-        CREATE POLICY tenant_isolation_assistant_file_changes ON assistant_file_changes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for assistant_skills
ALTER TABLE IF EXISTS assistant_skills ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS assistant_skills ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_skills ON assistant_skills;
CREATE POLICY tenant_isolation_assistant_skills ON assistant_skills USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/028_assistant_workstation.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/028_assistant_workstation.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/028_assistant_workstation.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/028_assistant_workstation.sql-    tenant_id TEXT NOT NULL,
-- Adding RLS for assistant_tasks
ALTER TABLE IF EXISTS assistant_tasks ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS assistant_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_tasks ON assistant_tasks;
CREATE POLICY tenant_isolation_assistant_tasks ON assistant_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/028_assistant_workstation.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/028_assistant_workstation.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/028_assistant_workstation.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/028_assistant_workstation.sql-    tenant_id TEXT NOT NULL,
-- Adding RLS for assistant_workspaces
ALTER TABLE IF EXISTS assistant_workspaces ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS assistant_workspaces ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_workspaces ON assistant_workspaces;
CREATE POLICY tenant_isolation_assistant_workspaces ON assistant_workspaces USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/148_auto_reply_policies.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/148_auto_reply_policies.sql-    UNIQUE(tenant_id)
src/server/migrations/148_auto_reply_policies.sql-CREATE POLICY tenant_isolation_auto_reply_policies ON auto_reply_policies USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for auto_reply_policies
ALTER TABLE IF EXISTS auto_reply_policies ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS auto_reply_policies ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_auto_reply_policies ON auto_reply_policies;
CREATE POLICY tenant_isolation_auto_reply_policies ON auto_reply_policies USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT DEFAULT 'system',
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
-- Adding RLS for autodream_memories
ALTER TABLE IF EXISTS autodream_memories ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS autodream_memories ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_autodream_memories ON autodream_memories;
CREATE POLICY tenant_isolation_autodream_memories ON autodream_memories USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/029_unified_booking.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/029_unified_booking.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/029_unified_booking.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/029_unified_booking.sql-CREATE INDEX IF NOT EXISTS idx_availability_blocks_tenant_service ON availability_blocks(tenant_id, service_id, start_time);
src/server/db/migrations/029_unified_booking.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/029_unified_booking.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for availability_blocks
ALTER TABLE IF EXISTS availability_blocks ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS availability_blocks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_availability_blocks ON availability_blocks;
CREATE POLICY tenant_isolation_availability_blocks ON availability_blocks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/081_availability_ledger.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/081_availability_ledger.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/081_availability_ledger.sql-    USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/081_availability_ledger.sql-    USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/081_availability_ledger.sql-CREATE INDEX IF NOT EXISTS idx_availability_ledger_tenant_time ON availability_ledger(tenant_id, start_time, end_time);
src/server/migrations/081_availability_ledger.sql-CREATE INDEX IF NOT EXISTS idx_travel_buffers_tenant_booking ON travel_buffers(tenant_id, booking_id);
-- Adding RLS for availability_ledger
ALTER TABLE IF EXISTS availability_ledger ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS availability_ledger ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_availability_ledger ON availability_ledger;
CREATE POLICY tenant_isolation_availability_ledger ON availability_ledger USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/079_service_bookings.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/079_service_bookings.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/079_service_bookings.sql-    USING (tenant_id = current_setting('app.current_tenant_id', true));
src/server/migrations/079_service_bookings.sql-    USING (tenant_id = current_setting('app.current_tenant_id', true));
-- Adding RLS for availability_schedules
ALTER TABLE IF EXISTS availability_schedules ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS availability_schedules ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_availability_schedules ON availability_schedules;
CREATE POLICY tenant_isolation_availability_schedules ON availability_schedules USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/022_supply_chain.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/022_supply_chain.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/022_supply_chain.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/022_supply_chain.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
-- Adding RLS for bom_items
ALTER TABLE IF EXISTS bom_items ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS bom_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_bom_items ON bom_items;
CREATE POLICY tenant_isolation_bom_items ON bom_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/035_c_unified_booking_resources.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/035_c_unified_booking_resources.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/035_c_unified_booking_resources.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/035_c_unified_booking_resources.sql-CREATE INDEX IF NOT EXISTS idx_booking_resource_reservations_tenant_id ON booking_resource_reservations(tenant_id);
src/server/db/migrations/035_c_unified_booking_resources.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/035_c_unified_booking_resources.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for booking_resource_reservations
ALTER TABLE IF EXISTS booking_resource_reservations ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS booking_resource_reservations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_booking_resource_reservations ON booking_resource_reservations;
CREATE POLICY tenant_isolation_booking_resource_reservations ON booking_resource_reservations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/035_c_unified_booking_resources.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/035_c_unified_booking_resources.sql-CREATE INDEX IF NOT EXISTS idx_booking_resources_tenant_id ON booking_resources(tenant_id);
src/server/db/migrations/035_c_unified_booking_resources.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/035_c_unified_booking_resources.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/035_c_unified_booking_resources.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/035_c_unified_booking_resources.sql-CREATE INDEX IF NOT EXISTS idx_service_resource_requirements_tenant_id ON service_resource_requirements(tenant_id);
src/server/db/migrations/035_c_unified_booking_resources.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/035_c_unified_booking_resources.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/035_c_unified_booking_resources.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/035_c_unified_booking_resources.sql-CREATE INDEX IF NOT EXISTS idx_booking_resource_reservations_tenant_id ON booking_resource_reservations(tenant_id);
src/server/db/migrations/035_c_unified_booking_resources.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/035_c_unified_booking_resources.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for booking_resources
ALTER TABLE IF EXISTS booking_resources ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS booking_resources ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_booking_resources ON booking_resources;
CREATE POLICY tenant_isolation_booking_resources ON booking_resources USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/156_booking_slots.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/156_booking_slots.sql-CREATE INDEX IF NOT EXISTS idx_booking_slots_tenant_id ON booking_slots(tenant_id);
src/server/migrations/156_booking_slots.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/migrations/156_booking_slots.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/156_booking_slots.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/156_booking_slots.sql-CREATE INDEX IF NOT EXISTS idx_booking_slots_tenant_id ON booking_slots(tenant_id);
src/server/db/migrations/156_booking_slots.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/156_booking_slots.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for booking_slots
ALTER TABLE IF EXISTS booking_slots ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS booking_slots ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_booking_slots ON booking_slots;
CREATE POLICY tenant_isolation_booking_slots ON booking_slots USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/008_data_model_architecture.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/008_data_model_architecture.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-CREATE POLICY tenant_isolation_users ON users USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/001_initial.sql-CREATE POLICY tenant_isolation_agents ON agents USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/001_initial.sql-CREATE POLICY tenant_isolation_tasks ON tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/001_initial.sql-CREATE POLICY tenant_isolation_products ON products USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/132_b_create_bookings_table.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/132_b_create_bookings_table.sql-CREATE INDEX IF NOT EXISTS idx_bookings_tenant_id ON bookings(tenant_id);
src/server/db/migrations/132_b_create_bookings_table.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/132_b_create_bookings_table.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for bookings
ALTER TABLE IF EXISTS bookings ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS bookings ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_bookings ON bookings;
CREATE POLICY tenant_isolation_bookings ON bookings USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/009_builder.sql-    tenant_id UUID NOT NULL,
src/server/migrations/009_builder.sql-CREATE POLICY tenant_isolation_builder_sites ON builder_sites USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/009_builder.sql-CREATE POLICY tenant_isolation_builder_pages ON builder_pages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/009_builder.sql-CREATE POLICY tenant_isolation_builder_blocks ON builder_blocks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for builder_blocks
ALTER TABLE IF EXISTS builder_blocks ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS builder_blocks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_builder_blocks ON builder_blocks;
CREATE POLICY tenant_isolation_builder_blocks ON builder_blocks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/059_brand_toolboxes.sql-    tenant_id UUID NOT NULL,
src/server/migrations/059_brand_toolboxes.sql-CREATE INDEX IF NOT EXISTS idx_builder_brand_toolboxes_tenant_id
src/server/migrations/059_brand_toolboxes.sql-    ON builder_brand_toolboxes(tenant_id);
src/server/migrations/059_brand_toolboxes.sql-            USING (tenant_id::text = current_setting('app.current_tenant', true))
src/server/migrations/059_brand_toolboxes.sql-            WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for builder_brand_toolboxes
ALTER TABLE IF EXISTS builder_brand_toolboxes ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS builder_brand_toolboxes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_builder_brand_toolboxes ON builder_brand_toolboxes;
CREATE POLICY tenant_isolation_builder_brand_toolboxes ON builder_brand_toolboxes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/009_builder.sql-    tenant_id UUID NOT NULL,
src/server/migrations/009_builder.sql-    tenant_id UUID NOT NULL,
src/server/migrations/009_builder.sql-CREATE POLICY tenant_isolation_builder_sites ON builder_sites USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/009_builder.sql-CREATE POLICY tenant_isolation_builder_pages ON builder_pages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/009_builder.sql-CREATE POLICY tenant_isolation_builder_blocks ON builder_blocks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for builder_pages
ALTER TABLE IF EXISTS builder_pages ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS builder_pages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_builder_pages ON builder_pages;
CREATE POLICY tenant_isolation_builder_pages ON builder_pages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/009_builder.sql-    tenant_id UUID NOT NULL,
src/server/migrations/009_builder.sql-    tenant_id UUID NOT NULL,
src/server/migrations/009_builder.sql-    tenant_id UUID NOT NULL,
src/server/migrations/009_builder.sql-CREATE POLICY tenant_isolation_builder_sites ON builder_sites USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/009_builder.sql-CREATE POLICY tenant_isolation_builder_pages ON builder_pages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/009_builder.sql-CREATE POLICY tenant_isolation_builder_blocks ON builder_blocks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for builder_sites
ALTER TABLE IF EXISTS builder_sites ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS builder_sites ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_builder_sites ON builder_sites;
CREATE POLICY tenant_isolation_builder_sites ON builder_sites USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/017_business_milestones.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/017_business_milestones.sql-    UNIQUE(tenant_id, milestone_type)
src/server/migrations/017_business_milestones.sql-CREATE INDEX IF NOT EXISTS idx_business_milestones_tenant_id ON business_milestones(tenant_id);
src/server/migrations/017_business_milestones.sql-CREATE POLICY tenant_isolation_business_milestones ON business_milestones USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for business_milestones
ALTER TABLE IF EXISTS business_milestones ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS business_milestones ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_business_milestones ON business_milestones;
CREATE POLICY tenant_isolation_business_milestones ON business_milestones USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/013_business_data_model.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/013_business_data_model.sql-CREATE POLICY tenant_isolation_businesses ON businesses USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/013_business_data_model.sql-CREATE POLICY tenant_isolation_agent_memories ON agent_memories USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for businesses
ALTER TABLE IF EXISTS businesses ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS businesses ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_businesses ON businesses;
CREATE POLICY tenant_isolation_businesses ON businesses USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/079_service_bookings.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/079_service_bookings.sql-    USING (tenant_id = current_setting('app.current_tenant_id', true));
src/server/migrations/079_service_bookings.sql-    USING (tenant_id = current_setting('app.current_tenant_id', true));
-- Adding RLS for calendar_integrations
ALTER TABLE IF EXISTS calendar_integrations ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS calendar_integrations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_calendar_integrations ON calendar_integrations;
CREATE POLICY tenant_isolation_calendar_integrations ON calendar_integrations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/063_campaign_engine.sql-CREATE POLICY tenant_isolation_campaigns ON campaigns USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/063_campaign_engine.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/063_campaign_engine.sql-CREATE POLICY tenant_isolation_campaign_assets ON campaign_assets USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/063_campaign_engine.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/063_campaign_engine.sql-CREATE POLICY tenant_isolation_channel_executions ON channel_executions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/063_campaign_engine.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/063_campaign_engine.sql-CREATE POLICY tenant_isolation_promotion_codes ON promotion_codes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for campaign_assets
ALTER TABLE IF EXISTS campaign_assets ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS campaign_assets ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_campaign_assets ON campaign_assets;
CREATE POLICY tenant_isolation_campaign_assets ON campaign_assets USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/063_campaign_engine.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/063_campaign_engine.sql-CREATE POLICY tenant_isolation_campaigns ON campaigns USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/063_campaign_engine.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/063_campaign_engine.sql-CREATE POLICY tenant_isolation_campaign_assets ON campaign_assets USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/063_campaign_engine.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/063_campaign_engine.sql-CREATE POLICY tenant_isolation_channel_executions ON channel_executions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/063_campaign_engine.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/063_campaign_engine.sql-CREATE POLICY tenant_isolation_promotion_codes ON promotion_codes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for campaigns
ALTER TABLE IF EXISTS campaigns ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS campaigns ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_campaigns ON campaigns;
CREATE POLICY tenant_isolation_campaigns ON campaigns USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/120_omnichannel_cart.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/120_omnichannel_cart.sql-            CREATE POLICY tenant_isolation_carts ON carts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/120_omnichannel_cart.sql-            CREATE POLICY tenant_isolation_cart_items ON cart_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/121_omnichannel_cart.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/121_omnichannel_cart.sql-            CREATE POLICY tenant_isolation_carts ON carts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/121_omnichannel_cart.sql-            CREATE POLICY tenant_isolation_cart_items ON cart_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for cart_items
ALTER TABLE IF EXISTS cart_items ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS cart_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_cart_items ON cart_items;
CREATE POLICY tenant_isolation_cart_items ON cart_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/120_omnichannel_cart.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/120_omnichannel_cart.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/120_omnichannel_cart.sql-            CREATE POLICY tenant_isolation_carts ON carts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/120_omnichannel_cart.sql-            CREATE POLICY tenant_isolation_cart_items ON cart_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/121_omnichannel_cart.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/121_omnichannel_cart.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/121_omnichannel_cart.sql-            CREATE POLICY tenant_isolation_carts ON carts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/121_omnichannel_cart.sql-            CREATE POLICY tenant_isolation_cart_items ON cart_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for carts
ALTER TABLE IF EXISTS carts ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS carts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_carts ON carts;
CREATE POLICY tenant_isolation_carts ON carts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/132_a_cash_ledger_entry.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/132_a_cash_ledger_entry.sql-CREATE INDEX IF NOT EXISTS idx_cash_ledger_entries_tenant ON cash_ledger_entries(tenant_id);
src/server/db/migrations/132_a_cash_ledger_entry.sql-CREATE POLICY tenant_isolation_cash_ledger_entries ON cash_ledger_entries USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for cash_ledger_entries
ALTER TABLE IF EXISTS cash_ledger_entries ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS cash_ledger_entries ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_cash_ledger_entries ON cash_ledger_entries;
CREATE POLICY tenant_isolation_cash_ledger_entries ON cash_ledger_entries USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/063_campaign_engine.sql-CREATE POLICY tenant_isolation_campaign_assets ON campaign_assets USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/063_campaign_engine.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/063_campaign_engine.sql-CREATE POLICY tenant_isolation_channel_executions ON channel_executions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/063_campaign_engine.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/063_campaign_engine.sql-CREATE POLICY tenant_isolation_promotion_codes ON promotion_codes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for channel_executions
ALTER TABLE IF EXISTS channel_executions ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS channel_executions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_channel_executions ON channel_executions;
CREATE POLICY tenant_isolation_channel_executions ON channel_executions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    organization_id TEXT NOT NULL,
-- Adding RLS for competitor_metrics
ALTER TABLE IF EXISTS competitor_metrics ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS competitor_metrics ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_competitor_metrics ON competitor_metrics;
CREATE POLICY tenant_isolation_competitor_metrics ON competitor_metrics USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/156_sync_events_endpoint.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/156_sync_events_endpoint.sql-    USING (tenant_id = current_setting('app.current_tenant', TRUE));
src/server/db/migrations/156_sync_events_endpoint.sql-    USING (tenant_id = current_setting('app.current_tenant', TRUE));
src/server/db/migrations/156_sync_events_endpoint.sql-    USING (tenant_id = current_setting('app.current_tenant', TRUE));
-- Adding RLS for conflict_queue
ALTER TABLE IF EXISTS conflict_queue ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS conflict_queue ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_conflict_queue ON conflict_queue;
CREATE POLICY tenant_isolation_conflict_queue ON conflict_queue USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_shared_tasks_v4 ON shared_tasks_v4 USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_shared_tasks ON shared_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/039_a_consolidated_memory.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/039_a_consolidated_memory.sql-CREATE INDEX IF NOT EXISTS consolidated_memory_tenant_id_idx ON consolidated_memory(tenant_id);
-- Adding RLS for consolidated_memory
ALTER TABLE IF EXISTS consolidated_memory ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS consolidated_memory ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_consolidated_memory ON consolidated_memory;
CREATE POLICY tenant_isolation_consolidated_memory ON consolidated_memory USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/153_omnichannel_customer_memory_graph.sql-    USING (tenant_id = current_setting('app.current_tenant', true))
src/server/migrations/153_omnichannel_customer_memory_graph.sql-    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/153_omnichannel_customer_memory_graph.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/153_omnichannel_customer_memory_graph.sql-    USING (tenant_id = current_setting('app.current_tenant', true))
src/server/migrations/153_omnichannel_customer_memory_graph.sql-    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/153_omnichannel_customer_memory_graph.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/153_omnichannel_customer_memory_graph.sql-    USING (tenant_id = current_setting('app.current_tenant', true))
src/server/migrations/153_omnichannel_customer_memory_graph.sql-    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for context_snippets
ALTER TABLE IF EXISTS context_snippets ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS context_snippets ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_context_snippets ON context_snippets;
CREATE POLICY tenant_isolation_context_snippets ON context_snippets USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/069_conversational_checkout.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/069_conversational_checkout.sql-CREATE POLICY tenant_isolation_conversational_checkout_sessions ON conversational_checkout_sessions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for conversational_checkout_sessions
ALTER TABLE IF EXISTS conversational_checkout_sessions ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS conversational_checkout_sessions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_conversational_checkout_sessions ON conversational_checkout_sessions;
CREATE POLICY tenant_isolation_conversational_checkout_sessions ON conversational_checkout_sessions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/163_conversational_intake.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/163_conversational_intake.sql-CREATE INDEX IF NOT EXISTS idx_conversational_intakes_tenant_id ON conversational_intakes(tenant_id);
src/server/db/migrations/163_conversational_intake.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/163_conversational_intake.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for conversational_intakes
ALTER TABLE IF EXISTS conversational_intakes ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS conversational_intakes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_conversational_intakes ON conversational_intakes;
CREATE POLICY tenant_isolation_conversational_intakes ON conversational_intakes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/135_crdt_deltas.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/135_crdt_deltas.sql-    PRIMARY KEY (tenant_id, id)
src/server/migrations/135_crdt_deltas.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/migrations/135_crdt_deltas.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/134_b_crdt_deltas.sql-    tenant_id VARCHAR(255) NOT NULL,
src/server/db/migrations/134_b_crdt_deltas.sql-    PRIMARY KEY (tenant_id, id)
src/server/db/migrations/134_b_crdt_deltas.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/134_b_crdt_deltas.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for crdt_deltas
ALTER TABLE IF EXISTS crdt_deltas ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS crdt_deltas ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_crdt_deltas ON crdt_deltas;
CREATE POLICY tenant_isolation_crdt_deltas ON crdt_deltas USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/066_customer360_loyalty.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/066_customer360_loyalty.sql-CREATE INDEX IF NOT EXISTS idx_customer360_tenant_customer ON customer360(tenant_id, customer_id);
src/server/migrations/066_customer360_loyalty.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/066_customer360_loyalty.sql-    UNIQUE(tenant_id, customer_id)
src/server/migrations/066_customer360_loyalty.sql-CREATE INDEX IF NOT EXISTS idx_loyalty_ledger_tenant_customer ON loyalty_ledger(tenant_id, customer_id);
src/server/migrations/066_customer360_loyalty.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/066_customer360_loyalty.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for customer360
ALTER TABLE IF EXISTS customer360 ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS customer360 ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_customer360 ON customer360;
CREATE POLICY tenant_isolation_customer360 ON customer360 USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/035_a_customer_identities.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/035_a_customer_identities.sql-    UNIQUE(tenant_id, channel, channel_identity)
src/server/db/migrations/035_a_customer_identities.sql-        CREATE POLICY tenant_isolation_customer_identities ON customer_identities USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for customer_identities
ALTER TABLE IF EXISTS customer_identities ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS customer_identities ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_customer_identities ON customer_identities;
CREATE POLICY tenant_isolation_customer_identities ON customer_identities USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/145_multi_tenant_loyalty_core.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/145_multi_tenant_loyalty_core.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/145_multi_tenant_loyalty_core.sql-    UNIQUE(tenant_id, program_id, customer_id)
src/server/migrations/145_multi_tenant_loyalty_core.sql-CREATE INDEX IF NOT EXISTS idx_customer_loyalty_accounts_tenant_customer ON customer_loyalty_accounts(tenant_id, customer_id);
src/server/migrations/145_multi_tenant_loyalty_core.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/145_multi_tenant_loyalty_core.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/145_multi_tenant_loyalty_core.sql-CREATE INDEX IF NOT EXISTS idx_loyalty_transactions_tenant_account ON loyalty_transactions(tenant_id, account_id);
src/server/migrations/145_multi_tenant_loyalty_core.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/145_multi_tenant_loyalty_core.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/145_multi_tenant_loyalty_core.sql-CREATE INDEX IF NOT EXISTS idx_loyalty_rewards_tenant_program ON loyalty_rewards(tenant_id, program_id);
-- Adding RLS for customer_loyalty_accounts
ALTER TABLE IF EXISTS customer_loyalty_accounts ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS customer_loyalty_accounts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_customer_loyalty_accounts ON customer_loyalty_accounts;
CREATE POLICY tenant_isolation_customer_loyalty_accounts ON customer_loyalty_accounts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/074_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/074_missing_tables.sql-CREATE INDEX IF NOT EXISTS idx_customer_timeline_tenant_customer ON customer_timeline(tenant_id, customer_id);
src/server/migrations/074_missing_tables.sql-                    'CREATE POLICY %I ON %I USING (tenant_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (tenant_id::text = current_setting(''app.current_tenant'', true))',
-- Adding RLS for customer_timeline
ALTER TABLE IF EXISTS customer_timeline ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS customer_timeline ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_customer_timeline ON customer_timeline;
CREATE POLICY tenant_isolation_customer_timeline ON customer_timeline USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/008_data_model_architecture.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/008_data_model_architecture.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/008_data_model_architecture.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/008_data_model_architecture.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/008_data_model_architecture.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/120_customers_table.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/120_customers_table.sql-CREATE POLICY tenant_isolation_customers ON customers USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for customers
ALTER TABLE IF EXISTS customers ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS customers ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_customers ON customers;
CREATE POLICY tenant_isolation_customers ON customers USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/139_daily_work.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/139_daily_work.sql-    USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/139_daily_work.sql-    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/139_daily_work.sql-    USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/139_daily_work.sql-    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for daily_work_items
ALTER TABLE IF EXISTS daily_work_items ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS daily_work_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_daily_work_items ON daily_work_items;
CREATE POLICY tenant_isolation_daily_work_items ON daily_work_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/102_delivery_task_provider_tracking.sql-    organization_id TEXT NOT NULL,
src/server/migrations/102_delivery_task_provider_tracking.sql-CREATE INDEX IF NOT EXISTS idx_delivery_tasks_org ON delivery_tasks(organization_id);
src/server/migrations/102_delivery_task_provider_tracking.sql-CREATE INDEX IF NOT EXISTS idx_delivery_tasks_location ON delivery_tasks(organization_id, delivery_location_lat, delivery_location_lng);
src/server/migrations/102_delivery_task_provider_tracking.sql-USING (organization_id::text = current_setting('app.current_tenant', true))
src/server/migrations/102_delivery_task_provider_tracking.sql-WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
src/server/migrations/102_delivery_task_provider_tracking.sql-ON delivery_tasks(organization_id, provider, provider_delivery_id);
src/server/db/migrations/015_delivery_tables.sql-CREATE INDEX IF NOT EXISTS idx_route_plans_org_date ON route_plans(organization_id, delivery_date);
src/server/db/migrations/015_delivery_tables.sql-CREATE POLICY tenant_isolation_route_plans ON route_plans USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/015_delivery_tables.sql-    organization_id TEXT NOT NULL,
src/server/db/migrations/015_delivery_tables.sql-CREATE INDEX IF NOT EXISTS idx_delivery_tasks_org ON delivery_tasks(organization_id);
src/server/db/migrations/015_delivery_tables.sql-CREATE POLICY tenant_isolation_delivery_tasks ON delivery_tasks USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for delivery_tasks
ALTER TABLE IF EXISTS delivery_tasks ADD COLUMN IF NOT EXISTS organization_id VARCHAR;
ALTER TABLE IF EXISTS delivery_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_delivery_tasks ON delivery_tasks;
CREATE POLICY tenant_isolation_delivery_tasks ON delivery_tasks USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/015_delivery_tables.sql-    organization_id TEXT NOT NULL,
src/server/db/migrations/015_delivery_tables.sql-CREATE INDEX IF NOT EXISTS idx_delivery_zones_org ON delivery_zones(organization_id);
src/server/db/migrations/015_delivery_tables.sql-CREATE POLICY tenant_isolation_delivery_zones ON delivery_zones USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/015_delivery_tables.sql-    organization_id TEXT NOT NULL,
src/server/db/migrations/015_delivery_tables.sql-CREATE INDEX IF NOT EXISTS idx_route_plans_org_date ON route_plans(organization_id, delivery_date);
src/server/db/migrations/015_delivery_tables.sql-CREATE POLICY tenant_isolation_route_plans ON route_plans USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/015_delivery_tables.sql-    organization_id TEXT NOT NULL,
src/server/db/migrations/015_delivery_tables.sql-CREATE INDEX IF NOT EXISTS idx_delivery_tasks_org ON delivery_tasks(organization_id);
src/server/db/migrations/015_delivery_tables.sql-CREATE POLICY tenant_isolation_delivery_tasks ON delivery_tasks USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for delivery_zones
ALTER TABLE IF EXISTS delivery_zones ADD COLUMN IF NOT EXISTS organization_id VARCHAR;
ALTER TABLE IF EXISTS delivery_zones ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_delivery_zones ON delivery_zones;
CREATE POLICY tenant_isolation_delivery_zones ON delivery_zones USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));

src/server/migrations/006_dead_letters.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/006_dead_letters.sql-CREATE POLICY tenant_isolation_department_dead_letters ON department_dead_letters USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for department_dead_letters
ALTER TABLE IF EXISTS department_dead_letters ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS department_dead_letters ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_department_dead_letters ON department_dead_letters;
CREATE POLICY tenant_isolation_department_dead_letters ON department_dead_letters USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT DEFAULT 'system',
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
-- Adding RLS for department_tasks
ALTER TABLE IF EXISTS department_tasks ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS department_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_department_tasks ON department_tasks;
CREATE POLICY tenant_isolation_department_tasks ON department_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/022_supply_chain.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/022_supply_chain.sql-                    'CREATE POLICY %I ON %I USING (tenant_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (tenant_id::text = current_setting(''app.current_tenant'', true))',
-- Adding RLS for depletion_logs
ALTER TABLE IF EXISTS depletion_logs ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS depletion_logs ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_depletion_logs ON depletion_logs;
CREATE POLICY tenant_isolation_depletion_logs ON depletion_logs USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/137_field_service_quoting.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/137_field_service_quoting.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/137_field_service_quoting.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/137_field_service_quoting.sql-CREATE INDEX IF NOT EXISTS idx_deposit_requirements_tenant_id ON deposit_requirements(tenant_id);
src/server/db/migrations/137_field_service_quoting.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/137_field_service_quoting.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for deposit_requirements
ALTER TABLE IF EXISTS deposit_requirements ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS deposit_requirements ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_deposit_requirements ON deposit_requirements;
CREATE POLICY tenant_isolation_deposit_requirements ON deposit_requirements USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/156_sync_events_endpoint.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/156_sync_events_endpoint.sql-    PRIMARY KEY (tenant_id, entity_type, entity_id)
src/server/db/migrations/156_sync_events_endpoint.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/156_sync_events_endpoint.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/156_sync_events_endpoint.sql-    USING (tenant_id = current_setting('app.current_tenant', TRUE));
src/server/db/migrations/156_sync_events_endpoint.sql-    USING (tenant_id = current_setting('app.current_tenant', TRUE));
src/server/db/migrations/156_sync_events_endpoint.sql-    USING (tenant_id = current_setting('app.current_tenant', TRUE));
-- Adding RLS for entity_versions
ALTER TABLE IF EXISTS entity_versions ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS entity_versions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_entity_versions ON entity_versions;
CREATE POLICY tenant_isolation_entity_versions ON entity_versions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/136_location_escalation.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/migrations/136_location_escalation.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/136_location_escalation.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/136_location_escalation.sql-CREATE INDEX IF NOT EXISTS idx_escalations_tenant_id ON escalations(tenant_id);
src/server/migrations/136_location_escalation.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/migrations/136_location_escalation.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/135_c_location_escalation.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/135_c_location_escalation.sql-        CREATE POLICY tenant_isolation_locations ON locations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/135_c_location_escalation.sql-        CREATE POLICY tenant_isolation_role_assignments ON role_assignments USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/135_c_location_escalation.sql-        CREATE POLICY tenant_isolation_escalations ON escalations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for escalations
ALTER TABLE IF EXISTS escalations ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS escalations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_escalations ON escalations;
CREATE POLICY tenant_isolation_escalations ON escalations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/128_quote_requests.sql-CREATE POLICY tenant_isolation_quote_requests ON quote_requests USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/128_quote_requests.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/128_quote_requests.sql-CREATE POLICY tenant_isolation_estimates ON estimates USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/128_quote_requests.sql-    estimate_id IN (SELECT id FROM estimates WHERE tenant_id::text = current_setting('app.current_tenant', true))
src/server/migrations/128_quote_requests.sql-    estimate_id IN (SELECT id FROM estimates WHERE tenant_id::text = current_setting('app.current_tenant', true))
src/server/db/migrations/137_field_service_quoting.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/137_field_service_quoting.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/137_field_service_quoting.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/137_field_service_quoting.sql-CREATE INDEX IF NOT EXISTS idx_estimates_tenant_id ON estimates(tenant_id);
src/server/db/migrations/137_field_service_quoting.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/137_field_service_quoting.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/137_field_service_quoting.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/137_field_service_quoting.sql-CREATE INDEX IF NOT EXISTS idx_deposit_requirements_tenant_id ON deposit_requirements(tenant_id);
src/server/db/migrations/137_field_service_quoting.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/137_field_service_quoting.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for estimates
ALTER TABLE IF EXISTS estimates ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS estimates ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_estimates ON estimates;
CREATE POLICY tenant_isolation_estimates ON estimates USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/019_subscriptions.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/019_subscriptions.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/019_subscriptions.sql-CREATE INDEX IF NOT EXISTS idx_fulfillment_batches_tenant ON fulfillment_batches(tenant_id);
src/server/db/migrations/019_subscriptions.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for fulfillment_batches
ALTER TABLE IF EXISTS fulfillment_batches ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS fulfillment_batches ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_fulfillment_batches ON fulfillment_batches;
CREATE POLICY tenant_isolation_fulfillment_batches ON fulfillment_batches USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/130_documentation_schema.sql-    tenant_id VARCHAR(255) NOT NULL,
src/server/db/migrations/130_documentation_schema.sql-CREATE INDEX IF NOT EXISTS idx_help_articles_tenant_id ON help_articles(tenant_id);
src/server/db/migrations/130_documentation_schema.sql-    tenant_id VARCHAR(255) NOT NULL,
src/server/db/migrations/130_documentation_schema.sql-CREATE INDEX IF NOT EXISTS idx_video_tutorials_tenant_id ON video_tutorials(tenant_id);
src/server/db/migrations/130_documentation_schema.sql-    tenant_id VARCHAR(255) NOT NULL,
src/server/db/migrations/130_documentation_schema.sql-CREATE INDEX IF NOT EXISTS idx_tooltips_tenant_id ON tooltips(tenant_id);
src/server/db/migrations/130_documentation_schema.sql-    tenant_id VARCHAR(255) NOT NULL,
src/server/db/migrations/130_documentation_schema.sql-CREATE INDEX IF NOT EXISTS idx_walkthrough_steps_tenant_id ON walkthrough_steps(tenant_id);
-- Adding RLS for help_articles
ALTER TABLE IF EXISTS help_articles ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS help_articles ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_help_articles ON help_articles;
CREATE POLICY tenant_isolation_help_articles ON help_articles USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    organization_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
-- Adding RLS for hybrid_fs_sync_queue
ALTER TABLE IF EXISTS hybrid_fs_sync_queue ADD COLUMN IF NOT EXISTS organization_id VARCHAR;
ALTER TABLE IF EXISTS hybrid_fs_sync_queue ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_hybrid_fs_sync_queue ON hybrid_fs_sync_queue;
CREATE POLICY tenant_isolation_hybrid_fs_sync_queue ON hybrid_fs_sync_queue USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/139_daily_work.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/139_daily_work.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/139_daily_work.sql-    USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/139_daily_work.sql-    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/139_daily_work.sql-    USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/139_daily_work.sql-    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for inbound_signals
ALTER TABLE IF EXISTS inbound_signals ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS inbound_signals ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_inbound_signals ON inbound_signals;
CREATE POLICY tenant_isolation_inbound_signals ON inbound_signals USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/070_inbox_messages.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/070_inbox_messages.sql-CREATE INDEX IF NOT EXISTS idx_inbox_messages_tenant_created_at ON inbox_messages(tenant_id, created_at DESC);
src/server/migrations/070_inbox_messages.sql-CREATE POLICY tenant_isolation_inbox_messages ON inbox_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for inbox_messages
ALTER TABLE IF EXISTS inbox_messages ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS inbox_messages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_inbox_messages ON inbox_messages;
CREATE POLICY tenant_isolation_inbox_messages ON inbox_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/118_incident_resolution.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/118_incident_resolution.sql-CREATE INDEX IF NOT EXISTS idx_incidents_tenant_id ON incidents(tenant_id);
src/server/migrations/118_incident_resolution.sql-            EXECUTE 'CREATE POLICY tenant_isolation_incidents ON incidents USING (tenant_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (tenant_id::text = current_setting(''app.current_tenant'', true))';
-- Adding RLS for incidents
ALTER TABLE IF EXISTS incidents ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS incidents ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_incidents ON incidents;
CREATE POLICY tenant_isolation_incidents ON incidents USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/160_integration_credentials.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/160_integration_credentials.sql-            USING (tenant_id::text = current_setting('app.current_tenant', true))
src/server/db/migrations/160_integration_credentials.sql-            WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for integration_credentials
ALTER TABLE IF EXISTS integration_credentials ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS integration_credentials ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_integration_credentials ON integration_credentials;
CREATE POLICY tenant_isolation_integration_credentials ON integration_credentials USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/153_omnichannel_customer_memory_graph.sql-    USING (tenant_id = current_setting('app.current_tenant', true))
src/server/migrations/153_omnichannel_customer_memory_graph.sql-    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/153_omnichannel_customer_memory_graph.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/153_omnichannel_customer_memory_graph.sql-    USING (tenant_id = current_setting('app.current_tenant', true))
src/server/migrations/153_omnichannel_customer_memory_graph.sql-    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for interaction_event_jobs
ALTER TABLE IF EXISTS interaction_event_jobs ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS interaction_event_jobs ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_interaction_event_jobs ON interaction_event_jobs;
CREATE POLICY tenant_isolation_interaction_event_jobs ON interaction_event_jobs USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/153_omnichannel_customer_memory_graph.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/153_omnichannel_customer_memory_graph.sql-    USING (tenant_id = current_setting('app.current_tenant', true))
src/server/migrations/153_omnichannel_customer_memory_graph.sql-    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/153_omnichannel_customer_memory_graph.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/153_omnichannel_customer_memory_graph.sql-    USING (tenant_id = current_setting('app.current_tenant', true))
src/server/migrations/153_omnichannel_customer_memory_graph.sql-    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/153_omnichannel_customer_memory_graph.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/153_omnichannel_customer_memory_graph.sql-    USING (tenant_id = current_setting('app.current_tenant', true))
src/server/migrations/153_omnichannel_customer_memory_graph.sql-    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for interaction_events
ALTER TABLE IF EXISTS interaction_events ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS interaction_events ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_interaction_events ON interaction_events;
CREATE POLICY tenant_isolation_interaction_events ON interaction_events USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/074_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/074_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/074_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/074_missing_tables.sql-CREATE INDEX IF NOT EXISTS idx_customer_timeline_tenant_customer ON customer_timeline(tenant_id, customer_id);
-- Adding RLS for interactions
ALTER TABLE IF EXISTS interactions ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS interactions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_interactions ON interactions;
CREATE POLICY tenant_isolation_interactions ON interactions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/146_interactive_proposals.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/146_interactive_proposals.sql-    USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/146_interactive_proposals.sql-    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/146_interactive_proposals.sql-    USING (proposal_id IN (SELECT id FROM interactive_proposals WHERE tenant_id = current_setting('app.current_tenant', true)))
src/server/db/migrations/146_interactive_proposals.sql-    WITH CHECK (proposal_id IN (SELECT id FROM interactive_proposals WHERE tenant_id = current_setting('app.current_tenant', true)));
-- Adding RLS for interactive_proposals
ALTER TABLE IF EXISTS interactive_proposals ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS interactive_proposals ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_interactive_proposals ON interactive_proposals;
CREATE POLICY tenant_isolation_interactive_proposals ON interactive_proposals USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/108_inventory_agent_actions.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/108_inventory_agent_actions.sql-            CREATE POLICY tenant_isolation_inventory_levels ON inventory_levels USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/108_inventory_agent_actions.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/108_inventory_agent_actions.sql-            CREATE POLICY tenant_isolation_agent_action_requests ON agent_action_requests USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for inventory_levels
ALTER TABLE IF EXISTS inventory_levels ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS inventory_levels ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_inventory_levels ON inventory_levels;
CREATE POLICY tenant_isolation_inventory_levels ON inventory_levels USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/132_predictive_supply_chain.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/132_predictive_supply_chain.sql-CREATE INDEX IF NOT EXISTS idx_inventory_predictions_tenant ON inventory_predictions(tenant_id);
src/server/migrations/132_predictive_supply_chain.sql-            CREATE POLICY tenant_isolation_inventory_predictions ON inventory_predictions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/018_predictive_supply_chain.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/018_predictive_supply_chain.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/018_predictive_supply_chain.sql-CREATE INDEX IF NOT EXISTS idx_inventory_predictions_tenant ON inventory_predictions(tenant_id);
src/server/db/migrations/018_predictive_supply_chain.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for inventory_predictions
ALTER TABLE IF EXISTS inventory_predictions ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS inventory_predictions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_inventory_predictions ON inventory_predictions;
CREATE POLICY tenant_isolation_inventory_predictions ON inventory_predictions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/125_autonomous_payment_recovery.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/125_autonomous_payment_recovery.sql-CREATE INDEX IF NOT EXISTS idx_invoice_communication_events_tenant_id ON invoice_communication_events(tenant_id);
src/server/db/migrations/125_autonomous_payment_recovery.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/125_autonomous_payment_recovery.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for invoice_communication_events
ALTER TABLE IF EXISTS invoice_communication_events ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS invoice_communication_events ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_invoice_communication_events ON invoice_communication_events;
CREATE POLICY tenant_isolation_invoice_communication_events ON invoice_communication_events USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/114_invoicing_agent.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/114_invoicing_agent.sql-CREATE INDEX IF NOT EXISTS idx_invoice_line_items_tenant ON invoice_line_items(tenant_id);
src/server/migrations/114_invoicing_agent.sql-CREATE POLICY tenant_isolation_invoice_line_items ON invoice_line_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/114_invoicing_agent.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/114_invoicing_agent.sql-CREATE INDEX IF NOT EXISTS idx_payment_events_tenant ON payment_events(tenant_id);
src/server/migrations/114_invoicing_agent.sql-CREATE POLICY tenant_isolation_payment_events ON payment_events USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/034_agentic_invoicing.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/034_agentic_invoicing.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/034_agentic_invoicing.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/034_agentic_invoicing.sql-CREATE INDEX IF NOT EXISTS idx_invoice_line_items_tenant_id ON invoice_line_items(tenant_id);
src/server/db/migrations/034_agentic_invoicing.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/034_agentic_invoicing.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for invoice_line_items
ALTER TABLE IF EXISTS invoice_line_items ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS invoice_line_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_invoice_line_items ON invoice_line_items;
CREATE POLICY tenant_isolation_invoice_line_items ON invoice_line_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/078_quote_engine.sql-CREATE POLICY tenant_isolation_quotes ON quotes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/078_quote_engine.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/078_quote_engine.sql-CREATE POLICY tenant_isolation_invoices ON invoices USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/034_agentic_invoicing.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/034_agentic_invoicing.sql-CREATE INDEX IF NOT EXISTS idx_invoices_tenant_id ON invoices(tenant_id);
src/server/db/migrations/034_agentic_invoicing.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/034_agentic_invoicing.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/034_agentic_invoicing.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/034_agentic_invoicing.sql-CREATE INDEX IF NOT EXISTS idx_invoice_line_items_tenant_id ON invoice_line_items(tenant_id);
src/server/db/migrations/034_agentic_invoicing.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/034_agentic_invoicing.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for invoices
ALTER TABLE IF EXISTS invoices ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS invoices ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_invoices ON invoices;
CREATE POLICY tenant_isolation_invoices ON invoices USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/162_service_routes_and_job_locations.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/162_service_routes_and_job_locations.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/162_service_routes_and_job_locations.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/162_service_routes_and_job_locations.sql-CREATE INDEX IF NOT EXISTS idx_job_locations_tenant_id ON job_locations(tenant_id);
src/server/db/migrations/162_service_routes_and_job_locations.sql-CREATE INDEX IF NOT EXISTS idx_job_locations_route ON job_locations(tenant_id, service_route_id, sequence_order);
src/server/db/migrations/162_service_routes_and_job_locations.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/162_service_routes_and_job_locations.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/162_field_service_routing.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/162_field_service_routing.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/162_field_service_routing.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/162_field_service_routing.sql-CREATE INDEX IF NOT EXISTS idx_job_locations_tenant_id ON job_locations(tenant_id);
src/server/db/migrations/162_field_service_routing.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/162_field_service_routing.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for job_locations
ALTER TABLE IF EXISTS job_locations ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS job_locations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_job_locations ON job_locations;
CREATE POLICY tenant_isolation_job_locations ON job_locations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/162_field_ops_appointments.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/162_field_ops_appointments.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/162_field_ops_appointments.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/162_field_ops_appointments.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/162_field_ops_appointments.sql-    USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/036_a_autonomous_work_scheduling.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/036_a_autonomous_work_scheduling.sql-CREATE INDEX IF NOT EXISTS idx_job_templates_tenant_id ON job_templates(tenant_id);
src/server/db/migrations/036_a_autonomous_work_scheduling.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/036_a_autonomous_work_scheduling.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/036_a_autonomous_work_scheduling.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/036_a_autonomous_work_scheduling.sql-CREATE INDEX IF NOT EXISTS idx_staff_profiles_tenant_id ON staff_profiles(tenant_id);
src/server/db/migrations/036_a_autonomous_work_scheduling.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/036_a_autonomous_work_scheduling.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/036_a_autonomous_work_scheduling.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
-- Adding RLS for job_templates
ALTER TABLE IF EXISTS job_templates ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS job_templates ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_job_templates ON job_templates;
CREATE POLICY tenant_isolation_job_templates ON job_templates USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-CREATE POLICY tenant_isolation_users ON users USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/001_initial.sql-CREATE POLICY tenant_isolation_agents ON agents USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/001_initial.sql-CREATE POLICY tenant_isolation_tasks ON tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/001_initial.sql-CREATE POLICY tenant_isolation_products ON products USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/001_initial.sql-CREATE POLICY tenant_isolation_orders ON orders USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/001_initial.sql-CREATE POLICY tenant_isolation_customers ON customers USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/001_initial.sql-CREATE POLICY tenant_isolation_bookings ON bookings USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/001_initial.sql-CREATE POLICY tenant_isolation_agent_memories ON agent_memories USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/001_initial.sql-CREATE POLICY tenant_isolation_knowledge_embeddings ON knowledge_embeddings USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/001_initial.sql-CREATE POLICY tenant_isolation_roles ON roles USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/001_initial.sql-CREATE POLICY tenant_isolation_revoked_tokens ON revoked_tokens USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for knowledge_embeddings
ALTER TABLE IF EXISTS knowledge_embeddings ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS knowledge_embeddings ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_knowledge_embeddings ON knowledge_embeddings;
CREATE POLICY tenant_isolation_knowledge_embeddings ON knowledge_embeddings USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/025_lead_gen_campaign.sql-    tenant_id UUID NOT NULL,
src/server/db/migrations/025_lead_gen_campaign.sql-    CONSTRAINT fk_lead_gen_campaign_tenant FOREIGN KEY (tenant_id) REFERENCES tenants(id) ON DELETE CASCADE
src/server/db/migrations/025_lead_gen_campaign.sql-    USING (tenant_id::text = current_setting('app.current_tenant', true))
src/server/db/migrations/025_lead_gen_campaign.sql-    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for lead_gen_campaigns
ALTER TABLE IF EXISTS lead_gen_campaigns ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS lead_gen_campaigns ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_lead_gen_campaigns ON lead_gen_campaigns;
CREATE POLICY tenant_isolation_lead_gen_campaigns ON lead_gen_campaigns USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/111_leads_and_opportunities.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/111_leads_and_opportunities.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/111_leads_and_opportunities.sql-CREATE INDEX IF NOT EXISTS idx_leads_tenant ON leads(tenant_id);
src/server/migrations/111_leads_and_opportunities.sql-CREATE INDEX IF NOT EXISTS idx_opportunities_tenant ON opportunities(tenant_id);
src/server/migrations/111_leads_and_opportunities.sql-CREATE POLICY tenant_isolation_leads ON leads USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/111_leads_and_opportunities.sql-CREATE POLICY tenant_isolation_opportunities ON opportunities USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/031_b_leads_and_opportunities.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/031_b_leads_and_opportunities.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/031_b_leads_and_opportunities.sql-CREATE INDEX IF NOT EXISTS idx_leads_tenant ON leads(tenant_id);
src/server/db/migrations/031_b_leads_and_opportunities.sql-CREATE INDEX IF NOT EXISTS idx_opportunities_tenant ON opportunities(tenant_id);
src/server/db/migrations/031_b_leads_and_opportunities.sql-CREATE POLICY tenant_isolation_leads ON leads USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/031_b_leads_and_opportunities.sql-CREATE POLICY tenant_isolation_opportunities ON opportunities USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for leads
ALTER TABLE IF EXISTS leads ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS leads ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_leads ON leads;
CREATE POLICY tenant_isolation_leads ON leads USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/080_ledger.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/080_ledger.sql-    PRIMARY KEY (tenant_id, account_id)
src/server/migrations/080_ledger.sql-CREATE INDEX IF NOT EXISTS idx_ledger_accounts_tenant ON ledger_accounts(tenant_id);
src/server/migrations/080_ledger.sql-CREATE POLICY tenant_isolation_ledger_accounts ON ledger_accounts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/080_ledger.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/080_ledger.sql-    PRIMARY KEY (tenant_id, tx_id)
src/server/migrations/080_ledger.sql-CREATE INDEX IF NOT EXISTS idx_ledger_transactions_tenant ON ledger_transactions(tenant_id);
src/server/migrations/080_ledger.sql-CREATE POLICY tenant_isolation_ledger_transactions ON ledger_transactions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/080_ledger.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/080_ledger.sql-    PRIMARY KEY (tenant_id, entry_id),
src/server/migrations/080_ledger.sql-    FOREIGN KEY (tenant_id, tx_id) REFERENCES ledger_transactions(tenant_id, tx_id),
src/server/migrations/080_ledger.sql-    FOREIGN KEY (tenant_id, account_id) REFERENCES ledger_accounts(tenant_id, account_id)
src/server/migrations/080_ledger.sql-CREATE INDEX IF NOT EXISTS idx_ledger_entries_tenant_tx ON ledger_entries(tenant_id, tx_id);
src/server/migrations/080_ledger.sql-CREATE INDEX IF NOT EXISTS idx_ledger_entries_tenant_account ON ledger_entries(tenant_id, account_id);
src/server/migrations/103_ledger.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/103_ledger.sql-    PRIMARY KEY (tenant_id, account_id)
src/server/migrations/103_ledger.sql-CREATE INDEX IF NOT EXISTS idx_ledger_accounts_tenant ON ledger_accounts(tenant_id);
src/server/migrations/103_ledger.sql-CREATE POLICY tenant_isolation_ledger_accounts ON ledger_accounts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/103_ledger.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/103_ledger.sql-    PRIMARY KEY (tenant_id, tx_id)
src/server/migrations/103_ledger.sql-CREATE INDEX IF NOT EXISTS idx_ledger_transactions_tenant ON ledger_transactions(tenant_id);
src/server/migrations/103_ledger.sql-CREATE POLICY tenant_isolation_ledger_transactions ON ledger_transactions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/103_ledger.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/103_ledger.sql-    PRIMARY KEY (tenant_id, entry_id),
src/server/migrations/103_ledger.sql-    FOREIGN KEY (tenant_id, tx_id) REFERENCES ledger_transactions(tenant_id, tx_id),
src/server/migrations/103_ledger.sql-    FOREIGN KEY (tenant_id, account_id) REFERENCES ledger_accounts(tenant_id, account_id)
src/server/migrations/103_ledger.sql-CREATE INDEX IF NOT EXISTS idx_ledger_entries_tenant_tx ON ledger_entries(tenant_id, tx_id);
src/server/migrations/103_ledger.sql-CREATE INDEX IF NOT EXISTS idx_ledger_entries_tenant_account ON ledger_entries(tenant_id, account_id);
-- Adding RLS for ledger_accounts
ALTER TABLE IF EXISTS ledger_accounts ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS ledger_accounts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ledger_accounts ON ledger_accounts;
CREATE POLICY tenant_isolation_ledger_accounts ON ledger_accounts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/080_ledger.sql-CREATE POLICY tenant_isolation_ledger_transactions ON ledger_transactions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/080_ledger.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/080_ledger.sql-    PRIMARY KEY (tenant_id, entry_id),
src/server/migrations/080_ledger.sql-    FOREIGN KEY (tenant_id, tx_id) REFERENCES ledger_transactions(tenant_id, tx_id),
src/server/migrations/080_ledger.sql-    FOREIGN KEY (tenant_id, account_id) REFERENCES ledger_accounts(tenant_id, account_id)
src/server/migrations/080_ledger.sql-CREATE INDEX IF NOT EXISTS idx_ledger_entries_tenant_tx ON ledger_entries(tenant_id, tx_id);
src/server/migrations/080_ledger.sql-CREATE INDEX IF NOT EXISTS idx_ledger_entries_tenant_account ON ledger_entries(tenant_id, account_id);
src/server/migrations/080_ledger.sql-CREATE POLICY tenant_isolation_ledger_entries ON ledger_entries USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/103_ledger.sql-CREATE POLICY tenant_isolation_ledger_transactions ON ledger_transactions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/103_ledger.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/103_ledger.sql-    PRIMARY KEY (tenant_id, entry_id),
src/server/migrations/103_ledger.sql-    FOREIGN KEY (tenant_id, tx_id) REFERENCES ledger_transactions(tenant_id, tx_id),
src/server/migrations/103_ledger.sql-    FOREIGN KEY (tenant_id, account_id) REFERENCES ledger_accounts(tenant_id, account_id)
src/server/migrations/103_ledger.sql-CREATE INDEX IF NOT EXISTS idx_ledger_entries_tenant_tx ON ledger_entries(tenant_id, tx_id);
src/server/migrations/103_ledger.sql-CREATE INDEX IF NOT EXISTS idx_ledger_entries_tenant_account ON ledger_entries(tenant_id, account_id);
src/server/migrations/103_ledger.sql-CREATE POLICY tenant_isolation_ledger_entries ON ledger_entries USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for ledger_entries
ALTER TABLE IF EXISTS ledger_entries ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS ledger_entries ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ledger_entries ON ledger_entries;
CREATE POLICY tenant_isolation_ledger_entries ON ledger_entries USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/080_ledger.sql-CREATE POLICY tenant_isolation_ledger_accounts ON ledger_accounts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/080_ledger.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/080_ledger.sql-    PRIMARY KEY (tenant_id, tx_id)
src/server/migrations/080_ledger.sql-CREATE INDEX IF NOT EXISTS idx_ledger_transactions_tenant ON ledger_transactions(tenant_id);
src/server/migrations/080_ledger.sql-CREATE POLICY tenant_isolation_ledger_transactions ON ledger_transactions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/080_ledger.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/080_ledger.sql-    PRIMARY KEY (tenant_id, entry_id),
src/server/migrations/080_ledger.sql-    FOREIGN KEY (tenant_id, tx_id) REFERENCES ledger_transactions(tenant_id, tx_id),
src/server/migrations/080_ledger.sql-    FOREIGN KEY (tenant_id, account_id) REFERENCES ledger_accounts(tenant_id, account_id)
src/server/migrations/080_ledger.sql-CREATE INDEX IF NOT EXISTS idx_ledger_entries_tenant_tx ON ledger_entries(tenant_id, tx_id);
src/server/migrations/080_ledger.sql-CREATE INDEX IF NOT EXISTS idx_ledger_entries_tenant_account ON ledger_entries(tenant_id, account_id);
src/server/migrations/080_ledger.sql-CREATE POLICY tenant_isolation_ledger_entries ON ledger_entries USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/103_ledger.sql-CREATE POLICY tenant_isolation_ledger_accounts ON ledger_accounts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/103_ledger.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/103_ledger.sql-    PRIMARY KEY (tenant_id, tx_id)
src/server/migrations/103_ledger.sql-CREATE INDEX IF NOT EXISTS idx_ledger_transactions_tenant ON ledger_transactions(tenant_id);
src/server/migrations/103_ledger.sql-CREATE POLICY tenant_isolation_ledger_transactions ON ledger_transactions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/103_ledger.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/103_ledger.sql-    PRIMARY KEY (tenant_id, entry_id),
src/server/migrations/103_ledger.sql-    FOREIGN KEY (tenant_id, tx_id) REFERENCES ledger_transactions(tenant_id, tx_id),
src/server/migrations/103_ledger.sql-    FOREIGN KEY (tenant_id, account_id) REFERENCES ledger_accounts(tenant_id, account_id)
src/server/migrations/103_ledger.sql-CREATE INDEX IF NOT EXISTS idx_ledger_entries_tenant_tx ON ledger_entries(tenant_id, tx_id);
src/server/migrations/103_ledger.sql-CREATE INDEX IF NOT EXISTS idx_ledger_entries_tenant_account ON ledger_entries(tenant_id, account_id);
src/server/migrations/103_ledger.sql-CREATE POLICY tenant_isolation_ledger_entries ON ledger_entries USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for ledger_transactions
ALTER TABLE IF EXISTS ledger_transactions ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS ledger_transactions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ledger_transactions ON ledger_transactions;
CREATE POLICY tenant_isolation_ledger_transactions ON ledger_transactions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/136_location_escalation.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/136_location_escalation.sql-CREATE INDEX IF NOT EXISTS idx_locations_tenant_id ON locations(tenant_id);
src/server/migrations/136_location_escalation.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/migrations/136_location_escalation.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/136_location_escalation.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/136_location_escalation.sql-CREATE INDEX IF NOT EXISTS idx_role_assignments_tenant_id ON role_assignments(tenant_id);
src/server/migrations/136_location_escalation.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/migrations/136_location_escalation.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/136_location_escalation.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/135_c_location_escalation.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/135_c_location_escalation.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/135_c_location_escalation.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/135_c_location_escalation.sql-        CREATE POLICY tenant_isolation_locations ON locations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/135_c_location_escalation.sql-        CREATE POLICY tenant_isolation_role_assignments ON role_assignments USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/135_c_location_escalation.sql-        CREATE POLICY tenant_isolation_escalations ON escalations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for locations
ALTER TABLE IF EXISTS locations ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS locations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_locations ON locations;
CREATE POLICY tenant_isolation_locations ON locations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/066_customer360_loyalty.sql-CREATE INDEX IF NOT EXISTS idx_customer360_tenant_customer ON customer360(tenant_id, customer_id);
src/server/migrations/066_customer360_loyalty.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/066_customer360_loyalty.sql-    UNIQUE(tenant_id, customer_id)
src/server/migrations/066_customer360_loyalty.sql-CREATE INDEX IF NOT EXISTS idx_loyalty_ledger_tenant_customer ON loyalty_ledger(tenant_id, customer_id);
src/server/migrations/066_customer360_loyalty.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/066_customer360_loyalty.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for loyalty_ledger
ALTER TABLE IF EXISTS loyalty_ledger ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS loyalty_ledger ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_loyalty_ledger ON loyalty_ledger;
CREATE POLICY tenant_isolation_loyalty_ledger ON loyalty_ledger USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/145_multi_tenant_loyalty_core.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/145_multi_tenant_loyalty_core.sql-CREATE INDEX IF NOT EXISTS idx_loyalty_programs_tenant ON loyalty_programs(tenant_id);
src/server/migrations/145_multi_tenant_loyalty_core.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/145_multi_tenant_loyalty_core.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/145_multi_tenant_loyalty_core.sql-    UNIQUE(tenant_id, program_id, customer_id)
src/server/migrations/145_multi_tenant_loyalty_core.sql-CREATE INDEX IF NOT EXISTS idx_customer_loyalty_accounts_tenant_customer ON customer_loyalty_accounts(tenant_id, customer_id);
src/server/migrations/145_multi_tenant_loyalty_core.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/145_multi_tenant_loyalty_core.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/145_multi_tenant_loyalty_core.sql-CREATE INDEX IF NOT EXISTS idx_loyalty_transactions_tenant_account ON loyalty_transactions(tenant_id, account_id);
-- Adding RLS for loyalty_programs
ALTER TABLE IF EXISTS loyalty_programs ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS loyalty_programs ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_loyalty_programs ON loyalty_programs;
CREATE POLICY tenant_isolation_loyalty_programs ON loyalty_programs USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/145_multi_tenant_loyalty_core.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/145_multi_tenant_loyalty_core.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/145_multi_tenant_loyalty_core.sql-CREATE INDEX IF NOT EXISTS idx_loyalty_rewards_tenant_program ON loyalty_rewards(tenant_id, program_id);
src/server/migrations/145_multi_tenant_loyalty_core.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for loyalty_rewards
ALTER TABLE IF EXISTS loyalty_rewards ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS loyalty_rewards ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_loyalty_rewards ON loyalty_rewards;
CREATE POLICY tenant_isolation_loyalty_rewards ON loyalty_rewards USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/145_multi_tenant_loyalty_core.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/145_multi_tenant_loyalty_core.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/145_multi_tenant_loyalty_core.sql-CREATE INDEX IF NOT EXISTS idx_loyalty_transactions_tenant_account ON loyalty_transactions(tenant_id, account_id);
src/server/migrations/145_multi_tenant_loyalty_core.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/145_multi_tenant_loyalty_core.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/145_multi_tenant_loyalty_core.sql-CREATE INDEX IF NOT EXISTS idx_loyalty_rewards_tenant_program ON loyalty_rewards(tenant_id, program_id);
src/server/migrations/145_multi_tenant_loyalty_core.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for loyalty_transactions
ALTER TABLE IF EXISTS loyalty_transactions ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS loyalty_transactions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_loyalty_transactions ON loyalty_transactions;
CREATE POLICY tenant_isolation_loyalty_transactions ON loyalty_transactions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/002_mcp_config_sync.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/002_mcp_config_sync.sql-    UNIQUE (tenant_id, config_key)
src/server/db/migrations/002_mcp_config_sync.sql-CREATE INDEX IF NOT EXISTS idx_mcp_config_sync_log_tenant_id ON mcp_config_sync_log(tenant_id);
src/server/db/migrations/002_mcp_config_sync.sql-    USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for mcp_config_sync_log
ALTER TABLE IF EXISTS mcp_config_sync_log ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS mcp_config_sync_log ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_mcp_config_sync_log ON mcp_config_sync_log;
CREATE POLICY tenant_isolation_mcp_config_sync_log ON mcp_config_sync_log USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/015_mcp_servers.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/015_mcp_servers.sql-CREATE INDEX IF NOT EXISTS idx_mcp_servers_tenant_id ON mcp_servers(tenant_id);
src/server/migrations/015_mcp_servers.sql-CREATE POLICY tenant_isolation_mcp_servers ON mcp_servers USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for mcp_servers
ALTER TABLE IF EXISTS mcp_servers ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS mcp_servers ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_mcp_servers ON mcp_servers;
CREATE POLICY tenant_isolation_mcp_servers ON mcp_servers USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_shared_tasks_v4 ON shared_tasks_v4 USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_shared_tasks ON shared_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_agent_approvals ON agent_approvals USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_onboarding_state ON onboarding_state USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_referrals ON referrals USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_competitor_metrics ON competitor_metrics USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_agent_violations ON agent_violations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_hybrid_fs_sync_queue ON hybrid_fs_sync_queue USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_department_tasks ON department_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_autodream_memories ON autodream_memories USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for meeting_rooms
ALTER TABLE IF EXISTS meeting_rooms ADD COLUMN IF NOT EXISTS organization_id VARCHAR;
ALTER TABLE IF EXISTS meeting_rooms ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_meeting_rooms ON meeting_rooms;
CREATE POLICY tenant_isolation_meeting_rooms ON meeting_rooms USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));

src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_shared_tasks_v4 ON shared_tasks_v4 USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_shared_tasks ON shared_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_agent_approvals ON agent_approvals USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_onboarding_state ON onboarding_state USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_referrals ON referrals USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_competitor_metrics ON competitor_metrics USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_agent_violations ON agent_violations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_hybrid_fs_sync_queue ON hybrid_fs_sync_queue USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_department_tasks ON department_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_autodream_memories ON autodream_memories USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_state_machine_transitions ON state_machine_transitions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/002_missing_tables.sql-CREATE POLICY tenant_isolation_pages ON pages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for meeting_transcripts
ALTER TABLE IF EXISTS meeting_transcripts ADD COLUMN IF NOT EXISTS organization_id VARCHAR;
ALTER TABLE IF EXISTS meeting_transcripts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_meeting_transcripts ON meeting_transcripts;
CREATE POLICY tenant_isolation_meeting_transcripts ON meeting_transcripts USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));

src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
-- Adding RLS for memories
ALTER TABLE IF EXISTS memories ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS memories ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_memories ON memories;
CREATE POLICY tenant_isolation_memories ON memories USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/065_migration_jobs.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/065_migration_jobs.sql-CREATE INDEX IF NOT EXISTS idx_migration_jobs_tenant_id ON migration_jobs(tenant_id);
src/server/migrations/065_migration_jobs.sql-        EXECUTE 'CREATE POLICY tenant_isolation_migration_jobs ON migration_jobs USING (tenant_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (tenant_id::text = current_setting(''app.current_tenant'', true))';
-- Adding RLS for migration_jobs
ALTER TABLE IF EXISTS migration_jobs ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS migration_jobs ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_migration_jobs ON migration_jobs;
CREATE POLICY tenant_isolation_migration_jobs ON migration_jobs USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/117_split_payments.sql-CREATE INDEX IF NOT EXISTS idx_multi_party_splits_resource ON multi_party_splits(tenant_id, resource_type, resource_id);
src/server/migrations/117_split_payments.sql-CREATE POLICY tenant_isolation_multi_party_splits ON multi_party_splits USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/117_split_payments.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/117_split_payments.sql-CREATE INDEX IF NOT EXISTS idx_multi_party_split_ledgers_tenant ON multi_party_split_ledgers(tenant_id);
src/server/migrations/117_split_payments.sql-CREATE POLICY tenant_isolation_multi_party_split_ledgers ON multi_party_split_ledgers USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/117_split_payments.sql-CREATE INDEX IF NOT EXISTS idx_multi_party_splits_resource ON multi_party_splits(tenant_id, resource_type, resource_id);
src/server/db/migrations/117_split_payments.sql-CREATE POLICY tenant_isolation_multi_party_splits ON multi_party_splits USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/117_split_payments.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/117_split_payments.sql-CREATE INDEX IF NOT EXISTS idx_multi_party_split_ledgers_tenant ON multi_party_split_ledgers(tenant_id);
src/server/db/migrations/117_split_payments.sql-CREATE POLICY tenant_isolation_multi_party_split_ledgers ON multi_party_split_ledgers USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for multi_party_split_ledgers
ALTER TABLE IF EXISTS multi_party_split_ledgers ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS multi_party_split_ledgers ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_multi_party_split_ledgers ON multi_party_split_ledgers;
CREATE POLICY tenant_isolation_multi_party_split_ledgers ON multi_party_split_ledgers USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/117_split_payments.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/117_split_payments.sql-CREATE INDEX IF NOT EXISTS idx_multi_party_splits_tenant ON multi_party_splits(tenant_id);
src/server/migrations/117_split_payments.sql-CREATE INDEX IF NOT EXISTS idx_multi_party_splits_resource ON multi_party_splits(tenant_id, resource_type, resource_id);
src/server/migrations/117_split_payments.sql-CREATE POLICY tenant_isolation_multi_party_splits ON multi_party_splits USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/117_split_payments.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/117_split_payments.sql-CREATE INDEX IF NOT EXISTS idx_multi_party_split_ledgers_tenant ON multi_party_split_ledgers(tenant_id);
src/server/migrations/117_split_payments.sql-CREATE POLICY tenant_isolation_multi_party_split_ledgers ON multi_party_split_ledgers USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/117_split_payments.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/117_split_payments.sql-CREATE INDEX IF NOT EXISTS idx_multi_party_splits_tenant ON multi_party_splits(tenant_id);
src/server/db/migrations/117_split_payments.sql-CREATE INDEX IF NOT EXISTS idx_multi_party_splits_resource ON multi_party_splits(tenant_id, resource_type, resource_id);
src/server/db/migrations/117_split_payments.sql-CREATE POLICY tenant_isolation_multi_party_splits ON multi_party_splits USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/117_split_payments.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/117_split_payments.sql-CREATE INDEX IF NOT EXISTS idx_multi_party_split_ledgers_tenant ON multi_party_split_ledgers(tenant_id);
src/server/db/migrations/117_split_payments.sql-CREATE POLICY tenant_isolation_multi_party_split_ledgers ON multi_party_split_ledgers USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for multi_party_splits
ALTER TABLE IF EXISTS multi_party_splits ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS multi_party_splits ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_multi_party_splits ON multi_party_splits;
CREATE POLICY tenant_isolation_multi_party_splits ON multi_party_splits USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/130_mutation_queue_and_sync_events.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/130_mutation_queue_and_sync_events.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/130_mutation_queue_and_sync_events.sql-    USING (tenant_id = current_setting('app.current_tenant', TRUE));
src/server/migrations/130_mutation_queue_and_sync_events.sql-    USING (tenant_id = current_setting('app.current_tenant', TRUE));
-- Adding RLS for mutation_queue
ALTER TABLE IF EXISTS mutation_queue ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS mutation_queue ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_mutation_queue ON mutation_queue;
CREATE POLICY tenant_isolation_mutation_queue ON mutation_queue USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/139_newsletter_drafts.sql-    tenant_id UUID NOT NULL,
src/server/migrations/139_newsletter_drafts.sql-    USING (tenant_id = current_setting('app.current_tenant')::UUID);
src/server/migrations/139_newsletter_drafts.sql-CREATE INDEX IF NOT EXISTS idx_newsletter_drafts_tenant_id ON newsletter_drafts(tenant_id);
-- Adding RLS for newsletter_drafts
ALTER TABLE IF EXISTS newsletter_drafts ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS newsletter_drafts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_newsletter_drafts ON newsletter_drafts;
CREATE POLICY tenant_isolation_newsletter_drafts ON newsletter_drafts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/024_b_neighborhood_mesh.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/024_b_neighborhood_mesh.sql-CREATE INDEX IF NOT EXISTS idx_ohc_collective_tenant ON ohc_collective(tenant_id);
src/server/db/migrations/024_b_neighborhood_mesh.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/024_b_neighborhood_mesh.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/024_b_neighborhood_mesh.sql-    PRIMARY KEY (collective_id, tenant_id)
src/server/db/migrations/024_b_neighborhood_mesh.sql-CREATE INDEX IF NOT EXISTS idx_collective_member_tenant ON ohc_collective_member(tenant_id);
src/server/db/migrations/024_b_neighborhood_mesh.sql-USING (tenant_id = current_setting(app.current_tenant, true)) WITH CHECK (tenant_id = current_setting(app.current_tenant, true));
src/server/db/migrations/024_b_neighborhood_mesh.sql-    originating_tenant_id TEXT NOT NULL,
src/server/db/migrations/024_b_neighborhood_mesh.sql-    target_tenant_id TEXT NOT NULL,
src/server/db/migrations/024_b_neighborhood_mesh.sql-USING (originating_tenant_id = current_setting(app.current_tenant, true) OR target_tenant_id = current_setting(app.current_tenant, true));
-- Adding RLS for ohc_collective
ALTER TABLE IF EXISTS ohc_collective ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS ohc_collective ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_collective ON ohc_collective;
CREATE POLICY tenant_isolation_ohc_collective ON ohc_collective USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/024_b_neighborhood_mesh.sql-USING (originating_tenant_id = current_setting(app.current_tenant, true) OR target_tenant_id = current_setting(app.current_tenant, true));
src/server/db/migrations/024_b_neighborhood_mesh.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/024_b_neighborhood_mesh.sql-    PRIMARY KEY (collective_id, buyer_id, tenant_id)
src/server/db/migrations/024_b_neighborhood_mesh.sql-CREATE INDEX IF NOT EXISTS idx_collective_loyalty_balance_tenant ON ohc_collective_loyalty_balance(tenant_id);
src/server/db/migrations/024_b_neighborhood_mesh.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for ohc_collective_loyalty_balance
ALTER TABLE IF EXISTS ohc_collective_loyalty_balance ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS ohc_collective_loyalty_balance ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_collective_loyalty_balance ON ohc_collective_loyalty_balance;
CREATE POLICY tenant_isolation_ohc_collective_loyalty_balance ON ohc_collective_loyalty_balance USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/024_b_neighborhood_mesh.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/024_b_neighborhood_mesh.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/024_b_neighborhood_mesh.sql-    PRIMARY KEY (collective_id, tenant_id)
src/server/db/migrations/024_b_neighborhood_mesh.sql-CREATE INDEX IF NOT EXISTS idx_collective_member_tenant ON ohc_collective_member(tenant_id);
src/server/db/migrations/024_b_neighborhood_mesh.sql-USING (tenant_id = current_setting(app.current_tenant, true)) WITH CHECK (tenant_id = current_setting(app.current_tenant, true));
src/server/db/migrations/024_b_neighborhood_mesh.sql-    originating_tenant_id TEXT NOT NULL,
src/server/db/migrations/024_b_neighborhood_mesh.sql-    target_tenant_id TEXT NOT NULL,
src/server/db/migrations/024_b_neighborhood_mesh.sql-USING (originating_tenant_id = current_setting(app.current_tenant, true) OR target_tenant_id = current_setting(app.current_tenant, true));
src/server/db/migrations/024_b_neighborhood_mesh.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/024_b_neighborhood_mesh.sql-    PRIMARY KEY (collective_id, buyer_id, tenant_id)
src/server/db/migrations/024_b_neighborhood_mesh.sql-CREATE INDEX IF NOT EXISTS idx_collective_loyalty_balance_tenant ON ohc_collective_loyalty_balance(tenant_id);
src/server/db/migrations/024_b_neighborhood_mesh.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for ohc_collective_member
ALTER TABLE IF EXISTS ohc_collective_member ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS ohc_collective_member ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_collective_member ON ohc_collective_member;
CREATE POLICY tenant_isolation_ohc_collective_member ON ohc_collective_member USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/067_multi_currency_localization.sql-    tenant_id TEXT NOT NULL, -- Supporting per-tenant custom strings
src/server/migrations/067_multi_currency_localization.sql-    UNIQUE(tenant_id, locale, key)
src/server/migrations/067_multi_currency_localization.sql-CREATE INDEX IF NOT EXISTS idx_ohc_i18n_lookup ON ohc_i18n_strings(tenant_id, locale);
src/server/migrations/067_multi_currency_localization.sql-USING (tenant_id = 'SYSTEM' OR tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = 'SYSTEM' OR tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/067_multi_currency_localization.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/067_multi_currency_localization.sql-CREATE INDEX IF NOT EXISTS idx_ohc_multi_currency_ledger_tenant ON ohc_multi_currency_ledger(tenant_id, created_at DESC);
src/server/migrations/067_multi_currency_localization.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for ohc_i18n_strings
ALTER TABLE IF EXISTS ohc_i18n_strings ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS ohc_i18n_strings ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_i18n_strings ON ohc_i18n_strings;
CREATE POLICY tenant_isolation_ohc_i18n_strings ON ohc_i18n_strings USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/060_job_queue_and_ledger.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/060_job_queue_and_ledger.sql-CREATE INDEX IF NOT EXISTS idx_ohc_job_queue_tenant_status ON ohc_job_queue(tenant_id, status);
src/server/migrations/060_job_queue_and_ledger.sql-CREATE POLICY tenant_isolation_ohc_job_queue ON ohc_job_queue USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/060_job_queue_and_ledger.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/060_job_queue_and_ledger.sql-CREATE INDEX IF NOT EXISTS idx_ohc_universal_ledger_tenant ON ohc_universal_ledger(tenant_id, created_at);
src/server/migrations/060_job_queue_and_ledger.sql-CREATE INDEX IF NOT EXISTS idx_ohc_universal_ledger_dept ON ohc_universal_ledger(tenant_id, department);
src/server/migrations/060_job_queue_and_ledger.sql-CREATE POLICY tenant_isolation_ohc_universal_ledger ON ohc_universal_ledger USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/014_job_queue_and_ledger.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/014_job_queue_and_ledger.sql-ON ohc_job_queue(tenant_id);
src/server/db/migrations/014_job_queue_and_ledger.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/014_job_queue_and_ledger.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/014_job_queue_and_ledger.sql-ON ohc_universal_ledger(tenant_id, created_at DESC);
src/server/db/migrations/014_job_queue_and_ledger.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for ohc_job_queue
ALTER TABLE IF EXISTS ohc_job_queue ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS ohc_job_queue ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_job_queue ON ohc_job_queue;
CREATE POLICY tenant_isolation_ohc_job_queue ON ohc_job_queue USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/067_multi_currency_localization.sql-USING (tenant_id = 'SYSTEM' OR tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = 'SYSTEM' OR tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/067_multi_currency_localization.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/067_multi_currency_localization.sql-CREATE INDEX IF NOT EXISTS idx_ohc_multi_currency_ledger_tenant ON ohc_multi_currency_ledger(tenant_id, created_at DESC);
src/server/migrations/067_multi_currency_localization.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for ohc_multi_currency_ledger
ALTER TABLE IF EXISTS ohc_multi_currency_ledger ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS ohc_multi_currency_ledger ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_multi_currency_ledger ON ohc_multi_currency_ledger;
CREATE POLICY tenant_isolation_ohc_multi_currency_ledger ON ohc_multi_currency_ledger USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/024_b_neighborhood_mesh.sql-USING (tenant_id = current_setting(app.current_tenant, true)) WITH CHECK (tenant_id = current_setting(app.current_tenant, true));
src/server/db/migrations/024_b_neighborhood_mesh.sql-    originating_tenant_id TEXT NOT NULL,
src/server/db/migrations/024_b_neighborhood_mesh.sql-    target_tenant_id TEXT NOT NULL,
src/server/db/migrations/024_b_neighborhood_mesh.sql-USING (originating_tenant_id = current_setting(app.current_tenant, true) OR target_tenant_id = current_setting(app.current_tenant, true));
src/server/db/migrations/024_b_neighborhood_mesh.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/024_b_neighborhood_mesh.sql-    PRIMARY KEY (collective_id, buyer_id, tenant_id)
src/server/db/migrations/024_b_neighborhood_mesh.sql-CREATE INDEX IF NOT EXISTS idx_collective_loyalty_balance_tenant ON ohc_collective_loyalty_balance(tenant_id);
src/server/db/migrations/024_b_neighborhood_mesh.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for ohc_shared_offer
ALTER TABLE IF EXISTS ohc_shared_offer ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS ohc_shared_offer ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_shared_offer ON ohc_shared_offer;
CREATE POLICY tenant_isolation_ohc_shared_offer ON ohc_shared_offer USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/016_staff_mesh.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/016_staff_mesh.sql-ON ohc_staff_member(tenant_id);
src/server/db/migrations/016_staff_mesh.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/016_staff_mesh.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/016_staff_mesh.sql-ON ohc_timecard_event(tenant_id, staff_id);
src/server/db/migrations/016_staff_mesh.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for ohc_staff_member
ALTER TABLE IF EXISTS ohc_staff_member ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS ohc_staff_member ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_staff_member ON ohc_staff_member;
CREATE POLICY tenant_isolation_ohc_staff_member ON ohc_staff_member USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/016_staff_mesh.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/016_staff_mesh.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/016_staff_mesh.sql-ON ohc_timecard_event(tenant_id, staff_id);
src/server/db/migrations/016_staff_mesh.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for ohc_timecard_event
ALTER TABLE IF EXISTS ohc_timecard_event ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS ohc_timecard_event ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_timecard_event ON ohc_timecard_event;
CREATE POLICY tenant_isolation_ohc_timecard_event ON ohc_timecard_event USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/105_translation_preferences.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/105_translation_preferences.sql-    UNIQUE(tenant_id)
src/server/migrations/105_translation_preferences.sql-CREATE INDEX IF NOT EXISTS idx_ohc_translation_prefs_tenant ON ohc_translation_preferences(tenant_id);
src/server/migrations/105_translation_preferences.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/migrations/105_translation_preferences.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for ohc_translation_preferences
ALTER TABLE IF EXISTS ohc_translation_preferences ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS ohc_translation_preferences ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_translation_preferences ON ohc_translation_preferences;
CREATE POLICY tenant_isolation_ohc_translation_preferences ON ohc_translation_preferences USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/060_job_queue_and_ledger.sql-CREATE POLICY tenant_isolation_ohc_job_queue ON ohc_job_queue USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/060_job_queue_and_ledger.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/060_job_queue_and_ledger.sql-CREATE INDEX IF NOT EXISTS idx_ohc_universal_ledger_tenant ON ohc_universal_ledger(tenant_id, created_at);
src/server/migrations/060_job_queue_and_ledger.sql-CREATE INDEX IF NOT EXISTS idx_ohc_universal_ledger_dept ON ohc_universal_ledger(tenant_id, department);
src/server/migrations/060_job_queue_and_ledger.sql-CREATE POLICY tenant_isolation_ohc_universal_ledger ON ohc_universal_ledger USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/014_job_queue_and_ledger.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/014_job_queue_and_ledger.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/014_job_queue_and_ledger.sql-ON ohc_universal_ledger(tenant_id, created_at DESC);
src/server/db/migrations/014_job_queue_and_ledger.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for ohc_universal_ledger
ALTER TABLE IF EXISTS ohc_universal_ledger ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS ohc_universal_ledger ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_universal_ledger ON ohc_universal_ledger;
CREATE POLICY tenant_isolation_ohc_universal_ledger ON ohc_universal_ledger USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/031_c_omni_inbox_messages.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/031_c_omni_inbox_messages.sql-        CREATE POLICY tenant_isolation_omni_inbox_messages ON omni_inbox_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for omni_inbox_messages
ALTER TABLE IF EXISTS omni_inbox_messages ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS omni_inbox_messages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_omni_inbox_messages ON omni_inbox_messages;
CREATE POLICY tenant_isolation_omni_inbox_messages ON omni_inbox_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    PRIMARY KEY (tenant_id, user_id)
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/126_a_onboarding_state_table.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/126_a_onboarding_state_table.sql-    PRIMARY KEY (tenant_id, user_id)
src/server/db/migrations/126_a_onboarding_state_table.sql-            USING (tenant_id::text = current_setting('app.current_tenant', true))
src/server/db/migrations/126_a_onboarding_state_table.sql-            WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for onboarding_state
ALTER TABLE IF EXISTS onboarding_state ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS onboarding_state ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_onboarding_state ON onboarding_state;
CREATE POLICY tenant_isolation_onboarding_state ON onboarding_state USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/111_leads_and_opportunities.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/111_leads_and_opportunities.sql-CREATE INDEX IF NOT EXISTS idx_leads_tenant ON leads(tenant_id);
src/server/migrations/111_leads_and_opportunities.sql-CREATE INDEX IF NOT EXISTS idx_opportunities_tenant ON opportunities(tenant_id);
src/server/migrations/111_leads_and_opportunities.sql-CREATE POLICY tenant_isolation_leads ON leads USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/111_leads_and_opportunities.sql-CREATE POLICY tenant_isolation_opportunities ON opportunities USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/031_b_leads_and_opportunities.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/031_b_leads_and_opportunities.sql-CREATE INDEX IF NOT EXISTS idx_leads_tenant ON leads(tenant_id);
src/server/db/migrations/031_b_leads_and_opportunities.sql-CREATE INDEX IF NOT EXISTS idx_opportunities_tenant ON opportunities(tenant_id);
src/server/db/migrations/031_b_leads_and_opportunities.sql-CREATE POLICY tenant_isolation_leads ON leads USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/031_b_leads_and_opportunities.sql-CREATE POLICY tenant_isolation_opportunities ON opportunities USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for opportunities
ALTER TABLE IF EXISTS opportunities ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS opportunities ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_opportunities ON opportunities;
CREATE POLICY tenant_isolation_opportunities ON opportunities USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-CREATE POLICY tenant_isolation_users ON users USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for order_items
ALTER TABLE IF EXISTS order_items ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS order_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_order_items ON order_items;
CREATE POLICY tenant_isolation_order_items ON order_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/008_data_model_architecture.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/008_data_model_architecture.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/008_data_model_architecture.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
-- Adding RLS for order_line_items
ALTER TABLE IF EXISTS order_line_items ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS order_line_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_order_line_items ON order_line_items;
CREATE POLICY tenant_isolation_order_line_items ON order_line_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/008_data_model_architecture.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/008_data_model_architecture.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/008_data_model_architecture.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/008_data_model_architecture.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
-- Adding RLS for orders
ALTER TABLE IF EXISTS orders ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS orders ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_orders ON orders;
CREATE POLICY tenant_isolation_orders ON orders USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
-- Adding RLS for pages
ALTER TABLE IF EXISTS pages ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS pages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_pages ON pages;
CREATE POLICY tenant_isolation_pages ON pages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/114_invoicing_agent.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/114_invoicing_agent.sql-CREATE INDEX IF NOT EXISTS idx_payment_events_tenant ON payment_events(tenant_id);
src/server/migrations/114_invoicing_agent.sql-CREATE POLICY tenant_isolation_payment_events ON payment_events USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for payment_events
ALTER TABLE IF EXISTS payment_events ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS payment_events ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_payment_events ON payment_events;
CREATE POLICY tenant_isolation_payment_events ON payment_events USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/142_omni_payment_ledger.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/142_omni_payment_ledger.sql-    PRIMARY KEY (tenant_id, payment_id)
src/server/migrations/142_omni_payment_ledger.sql-CREATE INDEX IF NOT EXISTS idx_payment_intents_tenant ON payment_intents(tenant_id);
src/server/migrations/142_omni_payment_ledger.sql-    USING (tenant_id::text = current_setting('app.current_tenant', true))
src/server/migrations/142_omni_payment_ledger.sql-    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for payment_intents
ALTER TABLE IF EXISTS payment_intents ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS payment_intents ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_payment_intents ON payment_intents;
CREATE POLICY tenant_isolation_payment_intents ON payment_intents USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/022_supply_chain.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/022_supply_chain.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/022_supply_chain.sql-                    'CREATE POLICY %I ON %I USING (tenant_id::text = current_setting(''app.current_tenant'', true)) WITH CHECK (tenant_id::text = current_setting(''app.current_tenant'', true))',
-- Adding RLS for po_line_items
ALTER TABLE IF EXISTS po_line_items ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS po_line_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_po_line_items ON po_line_items;
CREATE POLICY tenant_isolation_po_line_items ON po_line_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/076_pos_offline_transactions.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/076_pos_offline_transactions.sql-            CREATE POLICY tenant_isolation_pos_offline_transactions ON pos_offline_transactions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/021_pos_offline_sync.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/021_pos_offline_sync.sql-ON pos_offline_transactions(tenant_id, status);
src/server/db/migrations/021_pos_offline_sync.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for pos_offline_transactions
ALTER TABLE IF EXISTS pos_offline_transactions ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS pos_offline_transactions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_pos_offline_transactions ON pos_offline_transactions;
CREATE POLICY tenant_isolation_pos_offline_transactions ON pos_offline_transactions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/106_terminal_sessions.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/106_terminal_sessions.sql-    UNIQUE(tenant_id, device_id)
src/server/migrations/106_terminal_sessions.sql-            CREATE POLICY tenant_isolation_pos_terminal_sessions ON pos_terminal_sessions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/027_pos_terminal_sessions.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/027_pos_terminal_sessions.sql-    UNIQUE(tenant_id, device_id)
src/server/db/migrations/027_pos_terminal_sessions.sql-            CREATE POLICY tenant_isolation_pos_terminal_sessions ON pos_terminal_sessions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for pos_terminal_sessions
ALTER TABLE IF EXISTS pos_terminal_sessions ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS pos_terminal_sessions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_pos_terminal_sessions ON pos_terminal_sessions;
CREATE POLICY tenant_isolation_pos_terminal_sessions ON pos_terminal_sessions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/151_pre_order_waitlist_campaigns.sql-    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
src/server/db/migrations/151_pre_order_waitlist_campaigns.sql-    tenant_id UUID NOT NULL,
src/server/db/migrations/151_pre_order_waitlist_campaigns.sql-    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
src/server/db/migrations/151_pre_order_waitlist_campaigns.sql-    WITH CHECK (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
src/server/db/migrations/151_pre_order_waitlist_campaigns.sql-    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
src/server/db/migrations/151_pre_order_waitlist_campaigns.sql-    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
-- Adding RLS for pre_order_entries
ALTER TABLE IF EXISTS pre_order_entries ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS pre_order_entries ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_pre_order_entries ON pre_order_entries;
CREATE POLICY tenant_isolation_pre_order_entries ON pre_order_entries USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/124_dynamic_pricing_v2.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/124_dynamic_pricing_v2.sql-CREATE INDEX IF NOT EXISTS idx_price_history_tenant_target ON price_history(tenant_id, target_id);
src/server/migrations/124_dynamic_pricing_v2.sql-    USING (tenant_id::text = current_setting('app.current_tenant', true))
src/server/migrations/124_dynamic_pricing_v2.sql-    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for price_history
ALTER TABLE IF EXISTS price_history ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS price_history ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_price_history ON price_history;
CREATE POLICY tenant_isolation_price_history ON price_history USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/024_a_interactive_quoting.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/024_a_interactive_quoting.sql-CREATE POLICY tenant_isolation_quotes ON quotes USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/024_a_interactive_quoting.sql-    quote_id IN (SELECT id FROM quotes WHERE tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/024_a_interactive_quoting.sql-    quote_id IN (SELECT id FROM quotes WHERE tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/024_a_interactive_quoting.sql-CREATE POLICY tenant_isolation_pricing_heuristics ON pricing_heuristics USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for pricing_heuristics
ALTER TABLE IF EXISTS pricing_heuristics ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS pricing_heuristics ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_pricing_heuristics ON pricing_heuristics;
CREATE POLICY tenant_isolation_pricing_heuristics ON pricing_heuristics USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/122_dynamic_pricing_rules.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/122_dynamic_pricing_rules.sql-CREATE INDEX IF NOT EXISTS idx_pricing_rules_tenant ON pricing_rules(tenant_id);
src/server/migrations/122_dynamic_pricing_rules.sql-    USING (tenant_id::text = current_setting('app.current_tenant', true))
src/server/migrations/122_dynamic_pricing_rules.sql-    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/039_b_dynamic_pricing_rules.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/039_b_dynamic_pricing_rules.sql-CREATE INDEX IF NOT EXISTS idx_pricing_rules_tenant ON pricing_rules(tenant_id);
src/server/db/migrations/039_b_dynamic_pricing_rules.sql-    USING (tenant_id::text = current_setting('app.current_tenant', true))
src/server/db/migrations/039_b_dynamic_pricing_rules.sql-    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for pricing_rules
ALTER TABLE IF EXISTS pricing_rules ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS pricing_rules ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_pricing_rules ON pricing_rules;
CREATE POLICY tenant_isolation_pricing_rules ON pricing_rules USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/071_product_variants.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/071_product_variants.sql-CREATE POLICY tenant_isolation_product_variants ON product_variants USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for product_variants
ALTER TABLE IF EXISTS product_variants ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS product_variants ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_product_variants ON product_variants;
CREATE POLICY tenant_isolation_product_variants ON product_variants USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/008_data_model_architecture.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/008_data_model_architecture.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/008_data_model_architecture.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/008_data_model_architecture.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/008_data_model_architecture.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
-- Adding RLS for products
ALTER TABLE IF EXISTS products ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS products ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_products ON products;
CREATE POLICY tenant_isolation_products ON products USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/123_projects.sql-CREATE INDEX IF NOT EXISTS idx_projects_tenant ON projects(tenant_id);
src/server/migrations/123_projects.sql-CREATE POLICY tenant_isolation_projects ON projects USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/123_projects.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/123_projects.sql-CREATE INDEX IF NOT EXISTS idx_project_tasks_tenant ON project_tasks(tenant_id);
src/server/migrations/123_projects.sql-CREATE POLICY tenant_isolation_project_tasks ON project_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/123_b_projects.sql-CREATE INDEX IF NOT EXISTS idx_projects_tenant ON projects(tenant_id);
src/server/db/migrations/123_b_projects.sql-CREATE POLICY tenant_isolation_projects ON projects USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/123_b_projects.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/123_b_projects.sql-CREATE INDEX IF NOT EXISTS idx_project_tasks_tenant ON project_tasks(tenant_id);
src/server/db/migrations/123_b_projects.sql-CREATE POLICY tenant_isolation_project_tasks ON project_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for project_tasks
ALTER TABLE IF EXISTS project_tasks ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS project_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_project_tasks ON project_tasks;
CREATE POLICY tenant_isolation_project_tasks ON project_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/123_projects.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/123_projects.sql-CREATE INDEX IF NOT EXISTS idx_projects_tenant ON projects(tenant_id);
src/server/migrations/123_projects.sql-CREATE POLICY tenant_isolation_projects ON projects USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/123_projects.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/123_projects.sql-CREATE INDEX IF NOT EXISTS idx_project_tasks_tenant ON project_tasks(tenant_id);
src/server/migrations/123_projects.sql-CREATE POLICY tenant_isolation_project_tasks ON project_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/123_b_projects.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/123_b_projects.sql-CREATE INDEX IF NOT EXISTS idx_projects_tenant ON projects(tenant_id);
src/server/db/migrations/123_b_projects.sql-CREATE POLICY tenant_isolation_projects ON projects USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/123_b_projects.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/123_b_projects.sql-CREATE INDEX IF NOT EXISTS idx_project_tasks_tenant ON project_tasks(tenant_id);
src/server/db/migrations/123_b_projects.sql-CREATE POLICY tenant_isolation_project_tasks ON project_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for projects
ALTER TABLE IF EXISTS projects ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS projects ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_projects ON projects;
CREATE POLICY tenant_isolation_projects ON projects USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/063_campaign_engine.sql-CREATE POLICY tenant_isolation_channel_executions ON channel_executions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/063_campaign_engine.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/063_campaign_engine.sql-CREATE POLICY tenant_isolation_promotion_codes ON promotion_codes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for promotion_codes
ALTER TABLE IF EXISTS promotion_codes ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS promotion_codes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_promotion_codes ON promotion_codes;
CREATE POLICY tenant_isolation_promotion_codes ON promotion_codes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/138_a_autonomous_proposals.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/138_a_autonomous_proposals.sql-    USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/138_a_autonomous_proposals.sql-    WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/138_a_autonomous_proposals.sql-    USING (proposal_id IN (SELECT id FROM proposals WHERE tenant_id = current_setting('app.current_tenant', true)))
src/server/db/migrations/138_a_autonomous_proposals.sql-    WITH CHECK (proposal_id IN (SELECT id FROM proposals WHERE tenant_id = current_setting('app.current_tenant', true)));
src/server/db/migrations/138_c_proposals.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/138_c_proposals.sql-CREATE POLICY tenant_isolation_proposals ON proposals USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/138_c_proposals.sql-    proposal_id IN (SELECT id FROM proposals WHERE tenant_id::text = current_setting('app.current_tenant', true))
src/server/db/migrations/138_c_proposals.sql-    proposal_id IN (SELECT id FROM proposals WHERE tenant_id::text = current_setting('app.current_tenant', true))
-- Adding RLS for proposals
ALTER TABLE IF EXISTS proposals ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS proposals ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_proposals ON proposals;
CREATE POLICY tenant_isolation_proposals ON proposals USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/132_predictive_supply_chain.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/132_predictive_supply_chain.sql-CREATE INDEX IF NOT EXISTS idx_purchase_orders_tenant ON purchase_orders(tenant_id);
src/server/migrations/132_predictive_supply_chain.sql-            CREATE POLICY tenant_isolation_purchase_orders ON purchase_orders USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/132_predictive_supply_chain.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/132_predictive_supply_chain.sql-CREATE INDEX IF NOT EXISTS idx_inventory_predictions_tenant ON inventory_predictions(tenant_id);
src/server/migrations/022_supply_chain.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/022_supply_chain.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/022_supply_chain.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/018_predictive_supply_chain.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/018_predictive_supply_chain.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/018_predictive_supply_chain.sql-CREATE INDEX IF NOT EXISTS idx_purchase_orders_tenant ON purchase_orders(tenant_id);
src/server/db/migrations/018_predictive_supply_chain.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/018_predictive_supply_chain.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/018_predictive_supply_chain.sql-CREATE INDEX IF NOT EXISTS idx_inventory_predictions_tenant ON inventory_predictions(tenant_id);
src/server/db/migrations/018_predictive_supply_chain.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for purchase_orders
ALTER TABLE IF EXISTS purchase_orders ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS purchase_orders ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_purchase_orders ON purchase_orders;
CREATE POLICY tenant_isolation_purchase_orders ON purchase_orders USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/128_quote_requests.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/128_quote_requests.sql-CREATE POLICY tenant_isolation_quote_requests ON quote_requests USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/128_quote_requests.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/128_quote_requests.sql-CREATE POLICY tenant_isolation_estimates ON estimates USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/128_quote_requests.sql-    estimate_id IN (SELECT id FROM estimates WHERE tenant_id::text = current_setting('app.current_tenant', true))
src/server/migrations/128_quote_requests.sql-    estimate_id IN (SELECT id FROM estimates WHERE tenant_id::text = current_setting('app.current_tenant', true))
src/server/db/migrations/160_quote_request.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/160_quote_request.sql-CREATE POLICY tenant_isolation_quote_requests ON quote_requests USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for quote_requests
ALTER TABLE IF EXISTS quote_requests ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS quote_requests ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_quote_requests ON quote_requests;
CREATE POLICY tenant_isolation_quote_requests ON quote_requests USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/078_quote_engine.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/078_quote_engine.sql-CREATE POLICY tenant_isolation_quotes ON quotes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/078_quote_engine.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/078_quote_engine.sql-CREATE POLICY tenant_isolation_invoices ON invoices USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/024_a_interactive_quoting.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/024_a_interactive_quoting.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/024_a_interactive_quoting.sql-CREATE POLICY tenant_isolation_quotes ON quotes USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/024_a_interactive_quoting.sql-    quote_id IN (SELECT id FROM quotes WHERE tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/024_a_interactive_quoting.sql-    quote_id IN (SELECT id FROM quotes WHERE tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/024_a_interactive_quoting.sql-CREATE POLICY tenant_isolation_pricing_heuristics ON pricing_heuristics USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for quotes
ALTER TABLE IF EXISTS quotes ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS quotes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_quotes ON quotes;
CREATE POLICY tenant_isolation_quotes ON quotes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/022_supply_chain.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/022_supply_chain.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/022_supply_chain.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/022_supply_chain.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/022_supply_chain.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
-- Adding RLS for raw_materials
ALTER TABLE IF EXISTS raw_materials ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS raw_materials ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_raw_materials ON raw_materials;
CREATE POLICY tenant_isolation_raw_materials ON raw_materials USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/030_a_recovery_agent.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/030_a_recovery_agent.sql-        CREATE POLICY tenant_isolation_recovery_campaigns ON recovery_campaigns USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/030_a_recovery_agent.sql-        CREATE POLICY tenant_isolation_recovery_attempts ON recovery_attempts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for recovery_attempts
ALTER TABLE IF EXISTS recovery_attempts ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS recovery_attempts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_recovery_attempts ON recovery_attempts;
CREATE POLICY tenant_isolation_recovery_attempts ON recovery_attempts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/030_a_recovery_agent.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/030_a_recovery_agent.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/030_a_recovery_agent.sql-        CREATE POLICY tenant_isolation_recovery_campaigns ON recovery_campaigns USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/030_a_recovery_agent.sql-        CREATE POLICY tenant_isolation_recovery_attempts ON recovery_attempts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for recovery_campaigns
ALTER TABLE IF EXISTS recovery_campaigns ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS recovery_campaigns ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_recovery_campaigns ON recovery_campaigns;
CREATE POLICY tenant_isolation_recovery_campaigns ON recovery_campaigns USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/073_reputation_and_referral.sql-CREATE POLICY tenant_isolation_reviews ON reviews USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/073_reputation_and_referral.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/073_reputation_and_referral.sql-CREATE INDEX IF NOT EXISTS idx_referral_codes_tenant ON referral_codes(tenant_id);
src/server/migrations/073_reputation_and_referral.sql-CREATE INDEX IF NOT EXISTS idx_referral_codes_customer ON referral_codes(tenant_id, customer_id);
src/server/migrations/073_reputation_and_referral.sql-CREATE POLICY tenant_isolation_referral_codes ON referral_codes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for referral_codes
ALTER TABLE IF EXISTS referral_codes ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS referral_codes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_referral_codes ON referral_codes;
CREATE POLICY tenant_isolation_referral_codes ON referral_codes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/002_missing_tables.sql-    PRIMARY KEY (tenant_id, user_id)
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/166_add_referrals_table.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/166_add_referrals_table.sql-CREATE INDEX IF NOT EXISTS idx_referrals_tenant_id ON referrals(tenant_id);
src/server/db/migrations/166_add_referrals_table.sql-        CREATE POLICY tenant_isolation_referrals ON referrals USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for referrals
ALTER TABLE IF EXISTS referrals ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS referrals ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_referrals ON referrals;
CREATE POLICY tenant_isolation_referrals ON referrals USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/073_reputation_and_referral.sql-    tenant_id TEXT UNIQUE NOT NULL,
src/server/migrations/073_reputation_and_referral.sql-CREATE INDEX IF NOT EXISTS idx_reputation_profiles_tenant ON reputation_profiles(tenant_id);
src/server/migrations/073_reputation_and_referral.sql-CREATE POLICY tenant_isolation_reputation_profiles ON reputation_profiles USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/073_reputation_and_referral.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/073_reputation_and_referral.sql-CREATE INDEX IF NOT EXISTS idx_reviews_tenant ON reviews(tenant_id);
src/server/migrations/073_reputation_and_referral.sql-CREATE POLICY tenant_isolation_reviews ON reviews USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/073_reputation_and_referral.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/073_reputation_and_referral.sql-CREATE INDEX IF NOT EXISTS idx_referral_codes_tenant ON referral_codes(tenant_id);
src/server/migrations/073_reputation_and_referral.sql-CREATE INDEX IF NOT EXISTS idx_referral_codes_customer ON referral_codes(tenant_id, customer_id);
src/server/migrations/073_reputation_and_referral.sql-CREATE POLICY tenant_isolation_referral_codes ON referral_codes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for reputation_profiles
ALTER TABLE IF EXISTS reputation_profiles ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS reputation_profiles ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_reputation_profiles ON reputation_profiles;
CREATE POLICY tenant_isolation_reputation_profiles ON reputation_profiles USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/073_reputation_and_referral.sql-CREATE POLICY tenant_isolation_reputation_profiles ON reputation_profiles USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/073_reputation_and_referral.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/073_reputation_and_referral.sql-CREATE INDEX IF NOT EXISTS idx_reviews_tenant ON reviews(tenant_id);
src/server/migrations/073_reputation_and_referral.sql-CREATE POLICY tenant_isolation_reviews ON reviews USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/073_reputation_and_referral.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/073_reputation_and_referral.sql-CREATE INDEX IF NOT EXISTS idx_referral_codes_tenant ON referral_codes(tenant_id);
src/server/migrations/073_reputation_and_referral.sql-CREATE INDEX IF NOT EXISTS idx_referral_codes_customer ON referral_codes(tenant_id, customer_id);
src/server/migrations/073_reputation_and_referral.sql-CREATE POLICY tenant_isolation_referral_codes ON referral_codes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for reviews
ALTER TABLE IF EXISTS reviews ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS reviews ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_reviews ON reviews;
CREATE POLICY tenant_isolation_reviews ON reviews USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/001_initial.sql-    tenant_id TEXT DEFAULT 'system',
src/server/migrations/001_initial.sql-    tenant_id TEXT DEFAULT 'system',
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
-- Adding RLS for revoked_tokens
ALTER TABLE IF EXISTS revoked_tokens ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS revoked_tokens ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_revoked_tokens ON revoked_tokens;
CREATE POLICY tenant_isolation_revoked_tokens ON revoked_tokens USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/136_location_escalation.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/migrations/136_location_escalation.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/136_location_escalation.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/136_location_escalation.sql-CREATE INDEX IF NOT EXISTS idx_role_assignments_tenant_id ON role_assignments(tenant_id);
src/server/migrations/136_location_escalation.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/migrations/136_location_escalation.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/136_location_escalation.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/136_location_escalation.sql-CREATE INDEX IF NOT EXISTS idx_escalations_tenant_id ON escalations(tenant_id);
src/server/migrations/136_location_escalation.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/migrations/136_location_escalation.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/135_c_location_escalation.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/135_c_location_escalation.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/135_c_location_escalation.sql-        CREATE POLICY tenant_isolation_locations ON locations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/135_c_location_escalation.sql-        CREATE POLICY tenant_isolation_role_assignments ON role_assignments USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/135_c_location_escalation.sql-        CREATE POLICY tenant_isolation_escalations ON escalations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for role_assignments
ALTER TABLE IF EXISTS role_assignments ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS role_assignments ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_role_assignments ON role_assignments;
CREATE POLICY tenant_isolation_role_assignments ON role_assignments USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/001_initial.sql-    tenant_id TEXT DEFAULT 'system',
src/server/migrations/001_initial.sql-    tenant_id TEXT DEFAULT 'system',
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
-- Adding RLS for roles
ALTER TABLE IF EXISTS roles ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS roles ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_roles ON roles;
CREATE POLICY tenant_isolation_roles ON roles USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/015_delivery_tables.sql-CREATE INDEX IF NOT EXISTS idx_delivery_zones_org ON delivery_zones(organization_id);
src/server/db/migrations/015_delivery_tables.sql-CREATE POLICY tenant_isolation_delivery_zones ON delivery_zones USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/015_delivery_tables.sql-    organization_id TEXT NOT NULL,
src/server/db/migrations/015_delivery_tables.sql-CREATE INDEX IF NOT EXISTS idx_route_plans_org_date ON route_plans(organization_id, delivery_date);
src/server/db/migrations/015_delivery_tables.sql-CREATE POLICY tenant_isolation_route_plans ON route_plans USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/015_delivery_tables.sql-    organization_id TEXT NOT NULL,
src/server/db/migrations/015_delivery_tables.sql-CREATE INDEX IF NOT EXISTS idx_delivery_tasks_org ON delivery_tasks(organization_id);
src/server/db/migrations/015_delivery_tables.sql-CREATE POLICY tenant_isolation_delivery_tasks ON delivery_tasks USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for route_plans
ALTER TABLE IF EXISTS route_plans ADD COLUMN IF NOT EXISTS organization_id VARCHAR;
ALTER TABLE IF EXISTS route_plans ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_route_plans ON route_plans;
CREATE POLICY tenant_isolation_route_plans ON route_plans USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));

src/server/migrations/162_field_ops_appointments.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/162_field_ops_appointments.sql-    USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/162_field_ops_appointments.sql-    USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/162_field_ops_appointments.sql-    USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/162_field_ops_appointments.sql-    USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for route_stops
ALTER TABLE IF EXISTS route_stops ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS route_stops ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_route_stops ON route_stops;
CREATE POLICY tenant_isolation_route_stops ON route_stops USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/132_c_seo_discovery_reports.sql-    tenant_id UUID NOT NULL,
src/server/db/migrations/132_c_seo_discovery_reports.sql-CREATE INDEX IF NOT EXISTS idx_seo_discovery_reports_tenant_id ON seo_discovery_reports(tenant_id);
src/server/db/migrations/132_c_seo_discovery_reports.sql-USING (tenant_id::text = current_setting('app.current_tenant', true))
src/server/db/migrations/132_c_seo_discovery_reports.sql-WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for seo_discovery_reports
ALTER TABLE IF EXISTS seo_discovery_reports ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS seo_discovery_reports ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_seo_discovery_reports ON seo_discovery_reports;
CREATE POLICY tenant_isolation_seo_discovery_reports ON seo_discovery_reports USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/137_field_service_quoting.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/137_field_service_quoting.sql-CREATE INDEX IF NOT EXISTS idx_service_leads_tenant_id ON service_leads(tenant_id);
src/server/db/migrations/137_field_service_quoting.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/137_field_service_quoting.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/137_field_service_quoting.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/137_field_service_quoting.sql-CREATE INDEX IF NOT EXISTS idx_estimates_tenant_id ON estimates(tenant_id);
src/server/db/migrations/137_field_service_quoting.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/137_field_service_quoting.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/137_field_service_quoting.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
-- Adding RLS for service_leads
ALTER TABLE IF EXISTS service_leads ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS service_leads ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_service_leads ON service_leads;
CREATE POLICY tenant_isolation_service_leads ON service_leads USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/134_service_requests.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/134_service_requests.sql-CREATE POLICY tenant_isolation_service_requests ON service_requests USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for service_requests
ALTER TABLE IF EXISTS service_requests ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS service_requests ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_service_requests ON service_requests;
CREATE POLICY tenant_isolation_service_requests ON service_requests USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/035_c_unified_booking_resources.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/035_c_unified_booking_resources.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/035_c_unified_booking_resources.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/035_c_unified_booking_resources.sql-CREATE INDEX IF NOT EXISTS idx_service_resource_requirements_tenant_id ON service_resource_requirements(tenant_id);
src/server/db/migrations/035_c_unified_booking_resources.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/035_c_unified_booking_resources.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/035_c_unified_booking_resources.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/035_c_unified_booking_resources.sql-CREATE INDEX IF NOT EXISTS idx_booking_resource_reservations_tenant_id ON booking_resource_reservations(tenant_id);
src/server/db/migrations/035_c_unified_booking_resources.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/035_c_unified_booking_resources.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for service_resource_requirements
ALTER TABLE IF EXISTS service_resource_requirements ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS service_resource_requirements ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_service_resource_requirements ON service_resource_requirements;
CREATE POLICY tenant_isolation_service_resource_requirements ON service_resource_requirements USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/162_field_ops_appointments.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/162_field_ops_appointments.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/162_field_ops_appointments.sql-    USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/162_field_ops_appointments.sql-    USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/162_field_ops_appointments.sql-    USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/162_field_ops_appointments.sql-    USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/162_service_routes_and_job_locations.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/162_service_routes_and_job_locations.sql-CREATE INDEX IF NOT EXISTS idx_service_routes_tenant_id ON service_routes(tenant_id);
src/server/db/migrations/162_service_routes_and_job_locations.sql-CREATE INDEX IF NOT EXISTS idx_service_routes_staff_date ON service_routes(tenant_id, staff_profile_id, route_date);
src/server/db/migrations/162_service_routes_and_job_locations.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/162_service_routes_and_job_locations.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/162_service_routes_and_job_locations.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/162_service_routes_and_job_locations.sql-CREATE INDEX IF NOT EXISTS idx_job_locations_tenant_id ON job_locations(tenant_id);
src/server/db/migrations/162_service_routes_and_job_locations.sql-CREATE INDEX IF NOT EXISTS idx_job_locations_route ON job_locations(tenant_id, service_route_id, sequence_order);
src/server/db/migrations/162_service_routes_and_job_locations.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/162_service_routes_and_job_locations.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/162_field_service_routing.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/162_field_service_routing.sql-CREATE INDEX IF NOT EXISTS idx_service_routes_tenant_id ON service_routes(tenant_id);
src/server/db/migrations/162_field_service_routing.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/162_field_service_routing.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/162_field_service_routing.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/162_field_service_routing.sql-CREATE INDEX IF NOT EXISTS idx_job_locations_tenant_id ON job_locations(tenant_id);
src/server/db/migrations/162_field_service_routing.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/162_field_service_routing.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for service_routes
ALTER TABLE IF EXISTS service_routes ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS service_routes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_service_routes ON service_routes;
CREATE POLICY tenant_isolation_service_routes ON service_routes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/008_data_model_architecture.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/008_data_model_architecture.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/008_data_model_architecture.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/008_data_model_architecture.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/008_data_model_architecture.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/029_unified_booking.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/029_unified_booking.sql-CREATE INDEX IF NOT EXISTS idx_services_tenant_id ON services(tenant_id);
src/server/db/migrations/029_unified_booking.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/029_unified_booking.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/029_unified_booking.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/029_unified_booking.sql-CREATE INDEX IF NOT EXISTS idx_availability_blocks_tenant_service ON availability_blocks(tenant_id, service_id, start_time);
src/server/db/migrations/029_unified_booking.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/029_unified_booking.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for services
ALTER TABLE IF EXISTS services ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS services ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_services ON services;
CREATE POLICY tenant_isolation_services ON services USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/014_shared_tasks.sql-    organization_id TEXT NOT NULL,
src/server/migrations/014_shared_tasks.sql-CREATE INDEX IF NOT EXISTS idx_shared_tasks_organization_id ON shared_tasks(organization_id);
src/server/migrations/014_shared_tasks.sql-CREATE POLICY tenant_isolation_shared_tasks ON shared_tasks USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/001_shared_tasks.sql-    organization_id TEXT NOT NULL,
src/server/db/migrations/001_shared_tasks.sql-CREATE INDEX IF NOT EXISTS idx_shared_tasks_organization_id ON shared_tasks(organization_id);
src/server/db/migrations/001_shared_tasks.sql-CREATE POLICY tenant_isolation_shared_tasks ON shared_tasks USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for shared_tasks
ALTER TABLE IF EXISTS shared_tasks ADD COLUMN IF NOT EXISTS organization_id VARCHAR;
ALTER TABLE IF EXISTS shared_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shared_tasks ON shared_tasks;
CREATE POLICY tenant_isolation_shared_tasks ON shared_tasks USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));

src/server/migrations/058_shared_tasks_decomposition_table.sql-    organization_id VARCHAR NOT NULL,
src/server/migrations/002_missing_tables.sql-    organization_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
-- Adding RLS for shared_tasks_decomposition
ALTER TABLE IF EXISTS shared_tasks_decomposition ADD COLUMN IF NOT EXISTS organization_id VARCHAR;
ALTER TABLE IF EXISTS shared_tasks_decomposition ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition;
CREATE POLICY tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));

src/server/migrations/002_missing_tables.sql-    organization_id VARCHAR NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
-- Adding RLS for shared_tasks_v4
ALTER TABLE IF EXISTS shared_tasks_v4 ADD COLUMN IF NOT EXISTS organization_id VARCHAR;
ALTER TABLE IF EXISTS shared_tasks_v4 ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shared_tasks_v4 ON shared_tasks_v4;
CREATE POLICY tenant_isolation_shared_tasks_v4 ON shared_tasks_v4 USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));

src/server/migrations/163_shift_coordination.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/163_shift_coordination.sql-CREATE INDEX IF NOT EXISTS idx_shifts_tenant_id ON shifts(tenant_id);
src/server/migrations/163_shift_coordination.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/migrations/163_shift_coordination.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/163_shift_coordination.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/163_shift_coordination.sql-CREATE INDEX IF NOT EXISTS idx_staff_availability_tenant_id ON staff_availability(tenant_id);
src/server/migrations/163_shift_coordination.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/migrations/163_shift_coordination.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/163_shifts.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/163_shifts.sql-CREATE INDEX IF NOT EXISTS idx_shifts_tenant_id ON shifts(tenant_id);
src/server/db/migrations/163_shifts.sql-USING (tenant_id::text = current_setting('app.current_tenant', true))
src/server/db/migrations/163_shifts.sql-WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/163_shifts.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/163_shifts.sql-CREATE INDEX IF NOT EXISTS idx_staff_availability_tenant_id ON staff_availability(tenant_id);
src/server/db/migrations/163_shifts.sql-USING (tenant_id::text = current_setting('app.current_tenant', true))
src/server/db/migrations/163_shifts.sql-WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/163_shift_coordination.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/163_shift_coordination.sql-CREATE INDEX IF NOT EXISTS idx_shifts_tenant_id ON shifts(tenant_id);
src/server/db/migrations/163_shift_coordination.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/163_shift_coordination.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/163_shift_coordination.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/163_shift_coordination.sql-CREATE INDEX IF NOT EXISTS idx_staff_availability_tenant_id ON staff_availability(tenant_id);
src/server/db/migrations/163_shift_coordination.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/163_shift_coordination.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/163_shifts_and_availability.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/163_shifts_and_availability.sql-CREATE INDEX IF NOT EXISTS idx_shifts_tenant_id ON shifts(tenant_id);
src/server/db/migrations/163_shifts_and_availability.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/163_shifts_and_availability.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/163_shifts_and_availability.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/163_shifts_and_availability.sql-CREATE INDEX IF NOT EXISTS idx_staff_availability_tenant_id ON staff_availability(tenant_id);
src/server/db/migrations/163_shifts_and_availability.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/163_shifts_and_availability.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for shifts
ALTER TABLE IF EXISTS shifts ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS shifts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shifts ON shifts;
CREATE POLICY tenant_isolation_shifts ON shifts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/026_smart_pricing.sql-    tenant_id UUID NOT NULL,
src/server/db/migrations/026_smart_pricing.sql-CREATE INDEX IF NOT EXISTS idx_smart_pricing_policies_tenant ON smart_pricing_policies(tenant_id);
src/server/db/migrations/026_smart_pricing.sql-    USING (tenant_id::text = current_setting('app.current_tenant', true))
src/server/db/migrations/026_smart_pricing.sql-    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/026_smart_pricing.sql-    tenant_id UUID NOT NULL,
src/server/db/migrations/026_smart_pricing.sql-CREATE INDEX IF NOT EXISTS idx_active_discounts_tenant ON active_discounts(tenant_id);
src/server/db/migrations/026_smart_pricing.sql-    USING (tenant_id::text = current_setting('app.current_tenant', true))
src/server/db/migrations/026_smart_pricing.sql-    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for smart_pricing_policies
ALTER TABLE IF EXISTS smart_pricing_policies ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS smart_pricing_policies ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_smart_pricing_policies ON smart_pricing_policies;
CREATE POLICY tenant_isolation_smart_pricing_policies ON smart_pricing_policies USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/163_shift_coordination.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/migrations/163_shift_coordination.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/163_shift_coordination.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/163_shift_coordination.sql-CREATE INDEX IF NOT EXISTS idx_staff_availability_tenant_id ON staff_availability(tenant_id);
src/server/migrations/163_shift_coordination.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/migrations/163_shift_coordination.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/163_shifts.sql-USING (tenant_id::text = current_setting('app.current_tenant', true))
src/server/db/migrations/163_shifts.sql-WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/163_shifts.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/163_shifts.sql-CREATE INDEX IF NOT EXISTS idx_staff_availability_tenant_id ON staff_availability(tenant_id);
src/server/db/migrations/163_shifts.sql-USING (tenant_id::text = current_setting('app.current_tenant', true))
src/server/db/migrations/163_shifts.sql-WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/163_shift_coordination.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/163_shift_coordination.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/163_shift_coordination.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/163_shift_coordination.sql-CREATE INDEX IF NOT EXISTS idx_staff_availability_tenant_id ON staff_availability(tenant_id);
src/server/db/migrations/163_shift_coordination.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/163_shift_coordination.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/163_shifts_and_availability.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/163_shifts_and_availability.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/163_shifts_and_availability.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/163_shifts_and_availability.sql-CREATE INDEX IF NOT EXISTS idx_staff_availability_tenant_id ON staff_availability(tenant_id);
src/server/db/migrations/163_shifts_and_availability.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/163_shifts_and_availability.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for staff_availability
ALTER TABLE IF EXISTS staff_availability ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS staff_availability ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_staff_availability ON staff_availability;
CREATE POLICY tenant_isolation_staff_availability ON staff_availability USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/036_a_autonomous_work_scheduling.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/036_a_autonomous_work_scheduling.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/036_a_autonomous_work_scheduling.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/036_a_autonomous_work_scheduling.sql-CREATE INDEX IF NOT EXISTS idx_staff_profiles_tenant_id ON staff_profiles(tenant_id);
src/server/db/migrations/036_a_autonomous_work_scheduling.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/036_a_autonomous_work_scheduling.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/036_a_autonomous_work_scheduling.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/036_a_autonomous_work_scheduling.sql-CREATE INDEX IF NOT EXISTS idx_appointments_tenant_id ON appointments(tenant_id);
src/server/db/migrations/036_a_autonomous_work_scheduling.sql-CREATE INDEX IF NOT EXISTS idx_appointments_staff ON appointments(tenant_id, staff_profile_id, scheduled_start_time);
src/server/db/migrations/036_a_autonomous_work_scheduling.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/036_a_autonomous_work_scheduling.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for staff_profiles
ALTER TABLE IF EXISTS staff_profiles ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS staff_profiles ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_staff_profiles ON staff_profiles;
CREATE POLICY tenant_isolation_staff_profiles ON staff_profiles USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/002_missing_tables.sql-    tenant_id TEXT DEFAULT 'system',
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
-- Adding RLS for state_machine_transitions
ALTER TABLE IF EXISTS state_machine_transitions ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS state_machine_transitions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_state_machine_transitions ON state_machine_transitions;
CREATE POLICY tenant_isolation_state_machine_transitions ON state_machine_transitions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/104_sub_agent_queue.sql-    tenant_id VARCHAR NOT NULL,
src/server/migrations/104_sub_agent_queue.sql-    ON sub_agent_queue(tenant_id, status, scheduled_at);
src/server/migrations/104_sub_agent_queue.sql-    USING (tenant_id::text = current_setting('app.current_tenant', true))
src/server/migrations/104_sub_agent_queue.sql-    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for sub_agent_queue
ALTER TABLE IF EXISTS sub_agent_queue ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS sub_agent_queue ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_sub_agent_queue ON sub_agent_queue;
CREATE POLICY tenant_isolation_sub_agent_queue ON sub_agent_queue USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/019_subscriptions.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/019_subscriptions.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/019_subscriptions.sql-CREATE INDEX IF NOT EXISTS idx_subscribers_tenant ON subscribers(tenant_id);
src/server/db/migrations/019_subscriptions.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/019_subscriptions.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/019_subscriptions.sql-CREATE INDEX IF NOT EXISTS idx_fulfillment_batches_tenant ON fulfillment_batches(tenant_id);
src/server/db/migrations/019_subscriptions.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for subscribers
ALTER TABLE IF EXISTS subscribers ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS subscribers ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_subscribers ON subscribers;
CREATE POLICY tenant_isolation_subscribers ON subscribers USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/019_subscriptions.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/019_subscriptions.sql-CREATE INDEX IF NOT EXISTS idx_subscription_plans_tenant ON subscription_plans(tenant_id);
src/server/db/migrations/019_subscriptions.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/019_subscriptions.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/019_subscriptions.sql-CREATE INDEX IF NOT EXISTS idx_subscribers_tenant ON subscribers(tenant_id);
src/server/db/migrations/019_subscriptions.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/019_subscriptions.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/019_subscriptions.sql-CREATE INDEX IF NOT EXISTS idx_fulfillment_batches_tenant ON fulfillment_batches(tenant_id);
src/server/db/migrations/020_zero_touch_subscriptions.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/020_zero_touch_subscriptions.sql-CREATE INDEX IF NOT EXISTS idx_subscription_plans_tenant_id ON subscription_plans(tenant_id);
src/server/db/migrations/020_zero_touch_subscriptions.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/020_zero_touch_subscriptions.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/020_zero_touch_subscriptions.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/020_zero_touch_subscriptions.sql-CREATE INDEX IF NOT EXISTS idx_subscriptions_tenant_id ON subscriptions(tenant_id);
src/server/db/migrations/020_zero_touch_subscriptions.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/020_zero_touch_subscriptions.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for subscription_plans
ALTER TABLE IF EXISTS subscription_plans ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS subscription_plans ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_subscription_plans ON subscription_plans;
CREATE POLICY tenant_isolation_subscription_plans ON subscription_plans USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/020_zero_touch_subscriptions.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/020_zero_touch_subscriptions.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/020_zero_touch_subscriptions.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/020_zero_touch_subscriptions.sql-CREATE INDEX IF NOT EXISTS idx_subscriptions_tenant_id ON subscriptions(tenant_id);
src/server/db/migrations/020_zero_touch_subscriptions.sql-USING (tenant_id = current_setting('app.current_tenant', true))
src/server/db/migrations/020_zero_touch_subscriptions.sql-WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for subscriptions
ALTER TABLE IF EXISTS subscriptions ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS subscriptions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_subscriptions ON subscriptions;
CREATE POLICY tenant_isolation_subscriptions ON subscriptions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    PRIMARY KEY (tenant_id, user_id)
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/002_missing_tables.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/003_swarm_tasks.sql-    tenant_id TEXT NOT NULL DEFAULT 'default_tenant',
src/server/db/migrations/003_swarm_tasks.sql-    USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for swarm_tasks
ALTER TABLE IF EXISTS swarm_tasks ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS swarm_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_swarm_tasks ON swarm_tasks;
CREATE POLICY tenant_isolation_swarm_tasks ON swarm_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/157_sync_events_conflict_queue.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/157_sync_events_conflict_queue.sql-    USING (tenant_id = current_setting('app.current_tenant', TRUE));
src/server/migrations/157_sync_events_conflict_queue.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/157_sync_events_conflict_queue.sql-    USING (tenant_id = current_setting('app.current_tenant', TRUE));
-- Adding RLS for sync_conflict_queue
ALTER TABLE IF EXISTS sync_conflict_queue ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS sync_conflict_queue ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_sync_conflict_queue ON sync_conflict_queue;
CREATE POLICY tenant_isolation_sync_conflict_queue ON sync_conflict_queue USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/130_mutation_queue_and_sync_events.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/130_mutation_queue_and_sync_events.sql-    USING (tenant_id = current_setting('app.current_tenant', TRUE));
src/server/migrations/130_mutation_queue_and_sync_events.sql-    USING (tenant_id = current_setting('app.current_tenant', TRUE));
src/server/db/migrations/156_sync_events_endpoint.sql-    PRIMARY KEY (tenant_id, entity_type, entity_id)
src/server/db/migrations/156_sync_events_endpoint.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/156_sync_events_endpoint.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/156_sync_events_endpoint.sql-    USING (tenant_id = current_setting('app.current_tenant', TRUE));
src/server/db/migrations/156_sync_events_endpoint.sql-    USING (tenant_id = current_setting('app.current_tenant', TRUE));
src/server/db/migrations/156_sync_events_endpoint.sql-    USING (tenant_id = current_setting('app.current_tenant', TRUE));
-- Adding RLS for sync_events
ALTER TABLE IF EXISTS sync_events ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS sync_events ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_sync_events ON sync_events;
CREATE POLICY tenant_isolation_sync_events ON sync_events USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/004_task_dependencies.sql-    tenant_id TEXT NOT NULL DEFAULT 'default_tenant',
src/server/db/migrations/004_task_dependencies.sql-    USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for task_dependencies
ALTER TABLE IF EXISTS task_dependencies ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS task_dependencies ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_task_dependencies ON task_dependencies;
CREATE POLICY tenant_isolation_task_dependencies ON task_dependencies USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
-- Adding RLS for tasks
ALTER TABLE IF EXISTS tasks ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_tasks ON tasks;
CREATE POLICY tenant_isolation_tasks ON tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/068_team_invites.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/068_team_invites.sql-CREATE POLICY tenant_isolation_team_invites ON team_invites USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/126_b_team_invites.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/126_b_team_invites.sql-        CREATE POLICY tenant_isolation_team_invites ON team_invites USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for team_invites
ALTER TABLE IF EXISTS team_invites ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS team_invites ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_team_invites ON team_invites;
CREATE POLICY tenant_isolation_team_invites ON team_invites USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/032_b_telemetry_mesh.sql-    tenant_id TEXT NOT NULL DEFAULT 'default_tenant',
src/server/db/migrations/032_b_telemetry_mesh.sql-    USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for telemetry_buffer
ALTER TABLE IF EXISTS telemetry_buffer ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS telemetry_buffer ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_telemetry_buffer ON telemetry_buffer;
CREATE POLICY tenant_isolation_telemetry_buffer ON telemetry_buffer USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/056_ai_budgets.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/056_ai_budgets.sql-    PRIMARY KEY (tenant_id, year_month)
src/server/migrations/056_ai_budgets.sql-CREATE POLICY tenant_isolation_tenant_ai_budgets ON tenant_ai_budgets USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for tenant_ai_budgets
ALTER TABLE IF EXISTS tenant_ai_budgets ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS tenant_ai_budgets ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_tenant_ai_budgets ON tenant_ai_budgets;
CREATE POLICY tenant_isolation_tenant_ai_budgets ON tenant_ai_budgets USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/156_tenant_feed_items.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/156_tenant_feed_items.sql-        CREATE POLICY tenant_isolation_tenant_feed_items ON tenant_feed_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for tenant_feed_items
ALTER TABLE IF EXISTS tenant_feed_items ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS tenant_feed_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_tenant_feed_items ON tenant_feed_items;
CREATE POLICY tenant_isolation_tenant_feed_items ON tenant_feed_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/157_sync_events_conflict_queue.sql-    USING (tenant_id = current_setting('app.current_tenant', TRUE));
src/server/migrations/157_sync_events_conflict_queue.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/157_sync_events_conflict_queue.sql-    USING (tenant_id = current_setting('app.current_tenant', TRUE));
-- Adding RLS for test_sync_entities
ALTER TABLE IF EXISTS test_sync_entities ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS test_sync_entities ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_test_sync_entities ON test_sync_entities;
CREATE POLICY tenant_isolation_test_sync_entities ON test_sync_entities USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/129_a_tool_integrations.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/129_a_tool_integrations.sql-            USING (tenant_id::text = current_setting('app.current_tenant', true))
src/server/db/migrations/129_a_tool_integrations.sql-            WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for tool_integrations
ALTER TABLE IF EXISTS tool_integrations ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS tool_integrations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_tool_integrations ON tool_integrations;
CREATE POLICY tenant_isolation_tool_integrations ON tool_integrations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/130_documentation_schema.sql-CREATE INDEX IF NOT EXISTS idx_video_tutorials_tenant_id ON video_tutorials(tenant_id);
src/server/db/migrations/130_documentation_schema.sql-    tenant_id VARCHAR(255) NOT NULL,
src/server/db/migrations/130_documentation_schema.sql-CREATE INDEX IF NOT EXISTS idx_tooltips_tenant_id ON tooltips(tenant_id);
src/server/db/migrations/130_documentation_schema.sql-    tenant_id VARCHAR(255) NOT NULL,
src/server/db/migrations/130_documentation_schema.sql-CREATE INDEX IF NOT EXISTS idx_walkthrough_steps_tenant_id ON walkthrough_steps(tenant_id);
-- Adding RLS for tooltips
ALTER TABLE IF EXISTS tooltips ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS tooltips ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_tooltips ON tooltips;
CREATE POLICY tenant_isolation_tooltips ON tooltips USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/081_availability_ledger.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/081_availability_ledger.sql-    USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/081_availability_ledger.sql-    USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/migrations/081_availability_ledger.sql-CREATE INDEX IF NOT EXISTS idx_availability_ledger_tenant_time ON availability_ledger(tenant_id, start_time, end_time);
src/server/migrations/081_availability_ledger.sql-CREATE INDEX IF NOT EXISTS idx_travel_buffers_tenant_booking ON travel_buffers(tenant_id, booking_id);
-- Adding RLS for travel_buffers
ALTER TABLE IF EXISTS travel_buffers ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS travel_buffers ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_travel_buffers ON travel_buffers;
CREATE POLICY tenant_isolation_travel_buffers ON travel_buffers USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/109_triage_items.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/109_triage_items.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/109_triage_items.sql-CREATE INDEX IF NOT EXISTS idx_triage_items_tenant_status ON triage_items(tenant_id, status);
src/server/migrations/109_triage_items.sql-CREATE POLICY tenant_isolation_triage_items ON triage_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/109_triage_items.sql-CREATE POLICY tenant_isolation_triage_actions ON triage_proposed_actions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for triage_items
ALTER TABLE IF EXISTS triage_items ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS triage_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_triage_items ON triage_items;
CREATE POLICY tenant_isolation_triage_items ON triage_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/109_triage_items.sql-    tenant_id TEXT NOT NULL,
src/server/migrations/109_triage_items.sql-CREATE INDEX IF NOT EXISTS idx_triage_items_tenant_status ON triage_items(tenant_id, status);
src/server/migrations/109_triage_items.sql-CREATE POLICY tenant_isolation_triage_items ON triage_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/109_triage_items.sql-CREATE POLICY tenant_isolation_triage_actions ON triage_proposed_actions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for triage_proposed_actions
ALTER TABLE IF EXISTS triage_proposed_actions ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS triage_proposed_actions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_triage_proposed_actions ON triage_proposed_actions;
CREATE POLICY tenant_isolation_triage_proposed_actions ON triage_proposed_actions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/150_unified_inbox_triage.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/150_unified_inbox_triage.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/150_unified_inbox_triage.sql-        CREATE POLICY tenant_isolation_unified_threads ON unified_threads USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/150_unified_inbox_triage.sql-        CREATE POLICY tenant_isolation_unified_messages ON unified_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/150_unified_inbox_triage.sql-        CREATE POLICY tenant_isolation_unified_triage_actions ON unified_triage_actions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for unified_messages
ALTER TABLE IF EXISTS unified_messages ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS unified_messages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_unified_messages ON unified_messages;
CREATE POLICY tenant_isolation_unified_messages ON unified_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/150_unified_inbox_triage.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/150_unified_inbox_triage.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/150_unified_inbox_triage.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/150_unified_inbox_triage.sql-        CREATE POLICY tenant_isolation_unified_threads ON unified_threads USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/150_unified_inbox_triage.sql-        CREATE POLICY tenant_isolation_unified_messages ON unified_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/150_unified_inbox_triage.sql-        CREATE POLICY tenant_isolation_unified_triage_actions ON unified_triage_actions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for unified_threads
ALTER TABLE IF EXISTS unified_threads ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS unified_threads ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_unified_threads ON unified_threads;
CREATE POLICY tenant_isolation_unified_threads ON unified_threads USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/150_unified_inbox_triage.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/150_unified_inbox_triage.sql-        CREATE POLICY tenant_isolation_unified_threads ON unified_threads USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/150_unified_inbox_triage.sql-        CREATE POLICY tenant_isolation_unified_messages ON unified_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/db/migrations/150_unified_inbox_triage.sql-        CREATE POLICY tenant_isolation_unified_triage_actions ON unified_triage_actions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
-- Adding RLS for unified_triage_actions
ALTER TABLE IF EXISTS unified_triage_actions ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS unified_triage_actions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_unified_triage_actions ON unified_triage_actions;
CREATE POLICY tenant_isolation_unified_triage_actions ON unified_triage_actions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT DEFAULT 'system',
src/server/migrations/001_initial.sql-    tenant_id TEXT DEFAULT 'system',
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/001_initial.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
-- Adding RLS for users
ALTER TABLE IF EXISTS users ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS users ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_users ON users;
CREATE POLICY tenant_isolation_users ON users USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/migrations/132_predictive_supply_chain.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/132_predictive_supply_chain.sql-CREATE INDEX IF NOT EXISTS idx_vendors_tenant ON vendors(tenant_id);
src/server/migrations/132_predictive_supply_chain.sql-            CREATE POLICY tenant_isolation_vendors ON vendors USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/132_predictive_supply_chain.sql-    tenant_id TEXT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/132_predictive_supply_chain.sql-CREATE INDEX IF NOT EXISTS idx_purchase_orders_tenant ON purchase_orders(tenant_id);
src/server/migrations/132_predictive_supply_chain.sql-            CREATE POLICY tenant_isolation_purchase_orders ON purchase_orders USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
src/server/migrations/022_supply_chain.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/022_supply_chain.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/022_supply_chain.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/022_supply_chain.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/022_supply_chain.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/migrations/022_supply_chain.sql-    tenant_id TEXT REFERENCES tenants(id) ON DELETE CASCADE,
src/server/db/migrations/018_predictive_supply_chain.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/018_predictive_supply_chain.sql-CREATE INDEX IF NOT EXISTS idx_vendors_tenant ON vendors(tenant_id);
src/server/db/migrations/018_predictive_supply_chain.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/018_predictive_supply_chain.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/018_predictive_supply_chain.sql-CREATE INDEX IF NOT EXISTS idx_purchase_orders_tenant ON purchase_orders(tenant_id);
src/server/db/migrations/018_predictive_supply_chain.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
src/server/db/migrations/018_predictive_supply_chain.sql-    tenant_id TEXT NOT NULL,
src/server/db/migrations/018_predictive_supply_chain.sql-CREATE INDEX IF NOT EXISTS idx_inventory_predictions_tenant ON inventory_predictions(tenant_id);
src/server/db/migrations/018_predictive_supply_chain.sql-USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));
-- Adding RLS for vendors
ALTER TABLE IF EXISTS vendors ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS vendors ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_vendors ON vendors;
CREATE POLICY tenant_isolation_vendors ON vendors USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/130_documentation_schema.sql-CREATE INDEX IF NOT EXISTS idx_help_articles_tenant_id ON help_articles(tenant_id);
src/server/db/migrations/130_documentation_schema.sql-    tenant_id VARCHAR(255) NOT NULL,
src/server/db/migrations/130_documentation_schema.sql-CREATE INDEX IF NOT EXISTS idx_video_tutorials_tenant_id ON video_tutorials(tenant_id);
src/server/db/migrations/130_documentation_schema.sql-    tenant_id VARCHAR(255) NOT NULL,
src/server/db/migrations/130_documentation_schema.sql-CREATE INDEX IF NOT EXISTS idx_tooltips_tenant_id ON tooltips(tenant_id);
src/server/db/migrations/130_documentation_schema.sql-    tenant_id VARCHAR(255) NOT NULL,
src/server/db/migrations/130_documentation_schema.sql-CREATE INDEX IF NOT EXISTS idx_walkthrough_steps_tenant_id ON walkthrough_steps(tenant_id);
-- Adding RLS for video_tutorials
ALTER TABLE IF EXISTS video_tutorials ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS video_tutorials ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_video_tutorials ON video_tutorials;
CREATE POLICY tenant_isolation_video_tutorials ON video_tutorials USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/151_pre_order_waitlist_campaigns.sql-    tenant_id UUID NOT NULL,
src/server/db/migrations/151_pre_order_waitlist_campaigns.sql-    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
src/server/db/migrations/151_pre_order_waitlist_campaigns.sql-    WITH CHECK (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
src/server/db/migrations/151_pre_order_waitlist_campaigns.sql-    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
src/server/db/migrations/151_pre_order_waitlist_campaigns.sql-    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
src/server/db/migrations/151_pre_order_waitlist_campaigns.sql-    tenant_id UUID NOT NULL,
src/server/db/migrations/151_pre_order_waitlist_campaigns.sql-    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
-- Adding RLS for waitlist_campaigns
ALTER TABLE IF EXISTS waitlist_campaigns ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS waitlist_campaigns ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_waitlist_campaigns ON waitlist_campaigns;
CREATE POLICY tenant_isolation_waitlist_campaigns ON waitlist_campaigns USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

src/server/db/migrations/130_documentation_schema.sql-CREATE INDEX IF NOT EXISTS idx_tooltips_tenant_id ON tooltips(tenant_id);
src/server/db/migrations/130_documentation_schema.sql-    tenant_id VARCHAR(255) NOT NULL,
src/server/db/migrations/130_documentation_schema.sql-CREATE INDEX IF NOT EXISTS idx_walkthrough_steps_tenant_id ON walkthrough_steps(tenant_id);
-- Adding RLS for walkthrough_steps
ALTER TABLE IF EXISTS walkthrough_steps ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS walkthrough_steps ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_walkthrough_steps ON walkthrough_steps;
CREATE POLICY tenant_isolation_walkthrough_steps ON walkthrough_steps USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
