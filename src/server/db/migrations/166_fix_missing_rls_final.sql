-- +goose Up
-- Add missing RLS policies to enforce tenant isolation
-- This is a cleaner sweep of missing policies

-- Adding RLS for active_discounts
ALTER TABLE IF EXISTS active_discounts ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS active_discounts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_active_discounts ON active_discounts;
CREATE POLICY tenant_isolation_active_discounts ON active_discounts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for affiliate_ledgers
ALTER TABLE IF EXISTS affiliate_ledgers ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS affiliate_ledgers ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_affiliate_ledgers ON affiliate_ledgers;
CREATE POLICY tenant_isolation_affiliate_ledgers ON affiliate_ledgers USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for affiliate_links
ALTER TABLE IF EXISTS affiliate_links ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS affiliate_links ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_affiliate_links ON affiliate_links;
CREATE POLICY tenant_isolation_affiliate_links ON affiliate_links USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for affiliate_payouts
ALTER TABLE IF EXISTS affiliate_payouts ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS affiliate_payouts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_affiliate_payouts ON affiliate_payouts;
CREATE POLICY tenant_isolation_affiliate_payouts ON affiliate_payouts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for agent_action_requests
ALTER TABLE IF EXISTS agent_action_requests ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS agent_action_requests ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_action_requests ON agent_action_requests;
CREATE POLICY tenant_isolation_agent_action_requests ON agent_action_requests USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for agent_actions
ALTER TABLE IF EXISTS agent_actions ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS agent_actions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_actions ON agent_actions;
CREATE POLICY tenant_isolation_agent_actions ON agent_actions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for agent_approvals
ALTER TABLE IF EXISTS agent_approvals ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS agent_approvals ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_approvals ON agent_approvals;
CREATE POLICY tenant_isolation_agent_approvals ON agent_approvals USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for agent_departments
ALTER TABLE IF EXISTS agent_departments ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS agent_departments ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_departments ON agent_departments;
CREATE POLICY tenant_isolation_agent_departments ON agent_departments USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for agent_feed_items
ALTER TABLE IF EXISTS agent_feed_items ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS agent_feed_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_feed_items ON agent_feed_items;
CREATE POLICY tenant_isolation_agent_feed_items ON agent_feed_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for agent_inbox
ALTER TABLE IF EXISTS agent_inbox ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS agent_inbox ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_inbox ON agent_inbox;
CREATE POLICY tenant_isolation_agent_inbox ON agent_inbox USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for agent_jobs
ALTER TABLE IF EXISTS agent_jobs ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS agent_jobs ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_jobs ON agent_jobs;
CREATE POLICY tenant_isolation_agent_jobs ON agent_jobs USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for agent_kv_store
ALTER TABLE IF EXISTS agent_kv_store ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS agent_kv_store ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_kv_store ON agent_kv_store;
CREATE POLICY tenant_isolation_agent_kv_store ON agent_kv_store USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for agent_memories
ALTER TABLE IF EXISTS agent_memories ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS agent_memories ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_memories ON agent_memories;
CREATE POLICY tenant_isolation_agent_memories ON agent_memories USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for agent_missions
ALTER TABLE IF EXISTS agent_missions ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS agent_missions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_missions ON agent_missions;
CREATE POLICY tenant_isolation_agent_missions ON agent_missions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for agent_status
ALTER TABLE IF EXISTS agent_status ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS agent_status ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_status ON agent_status;
CREATE POLICY tenant_isolation_agent_status ON agent_status USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for agent_violations
ALTER TABLE IF EXISTS agent_violations ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS agent_violations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_violations ON agent_violations;
CREATE POLICY tenant_isolation_agent_violations ON agent_violations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for agents
ALTER TABLE IF EXISTS agents ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS agents ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agents ON agents;
CREATE POLICY tenant_isolation_agents ON agents USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for ai_memories
ALTER TABLE IF EXISTS ai_memories ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS ai_memories ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ai_memories ON ai_memories;
CREATE POLICY tenant_isolation_ai_memories ON ai_memories USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for applied_client_mutations
ALTER TABLE IF EXISTS applied_client_mutations ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS applied_client_mutations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_applied_client_mutations ON applied_client_mutations;
CREATE POLICY tenant_isolation_applied_client_mutations ON applied_client_mutations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for appointments
ALTER TABLE IF EXISTS appointments ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS appointments ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_appointments ON appointments;
CREATE POLICY tenant_isolation_appointments ON appointments USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for assistant_artifacts
ALTER TABLE IF EXISTS assistant_artifacts ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS assistant_artifacts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_artifacts ON assistant_artifacts;
CREATE POLICY tenant_isolation_assistant_artifacts ON assistant_artifacts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for assistant_connectors
ALTER TABLE IF EXISTS assistant_connectors ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS assistant_connectors ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_connectors ON assistant_connectors;
CREATE POLICY tenant_isolation_assistant_connectors ON assistant_connectors USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for assistant_file_changes
ALTER TABLE IF EXISTS assistant_file_changes ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS assistant_file_changes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_file_changes ON assistant_file_changes;
CREATE POLICY tenant_isolation_assistant_file_changes ON assistant_file_changes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for assistant_memory_records
ALTER TABLE IF EXISTS assistant_memory_records ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS assistant_memory_records ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_memory_records ON assistant_memory_records;
CREATE POLICY tenant_isolation_assistant_memory_records ON assistant_memory_records USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for assistant_messages
ALTER TABLE IF EXISTS assistant_messages ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS assistant_messages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_messages ON assistant_messages;
CREATE POLICY tenant_isolation_assistant_messages ON assistant_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for assistant_skills
ALTER TABLE IF EXISTS assistant_skills ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS assistant_skills ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_skills ON assistant_skills;
CREATE POLICY tenant_isolation_assistant_skills ON assistant_skills USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for assistant_tasks
ALTER TABLE IF EXISTS assistant_tasks ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS assistant_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_tasks ON assistant_tasks;
CREATE POLICY tenant_isolation_assistant_tasks ON assistant_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for assistant_workspaces
ALTER TABLE IF EXISTS assistant_workspaces ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS assistant_workspaces ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_workspaces ON assistant_workspaces;
CREATE POLICY tenant_isolation_assistant_workspaces ON assistant_workspaces USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for auto_reply_policies
ALTER TABLE IF EXISTS auto_reply_policies ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS auto_reply_policies ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_auto_reply_policies ON auto_reply_policies;
CREATE POLICY tenant_isolation_auto_reply_policies ON auto_reply_policies USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for autodream_memories
ALTER TABLE IF EXISTS autodream_memories ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS autodream_memories ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_autodream_memories ON autodream_memories;
CREATE POLICY tenant_isolation_autodream_memories ON autodream_memories USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for availability_blocks
ALTER TABLE IF EXISTS availability_blocks ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS availability_blocks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_availability_blocks ON availability_blocks;
CREATE POLICY tenant_isolation_availability_blocks ON availability_blocks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for availability_ledger
ALTER TABLE IF EXISTS availability_ledger ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS availability_ledger ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_availability_ledger ON availability_ledger;
CREATE POLICY tenant_isolation_availability_ledger ON availability_ledger USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for availability_schedules
ALTER TABLE IF EXISTS availability_schedules ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS availability_schedules ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_availability_schedules ON availability_schedules;
CREATE POLICY tenant_isolation_availability_schedules ON availability_schedules USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for bom_items
ALTER TABLE IF EXISTS bom_items ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS bom_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_bom_items ON bom_items;
CREATE POLICY tenant_isolation_bom_items ON bom_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for booking_resource_reservations
ALTER TABLE IF EXISTS booking_resource_reservations ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS booking_resource_reservations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_booking_resource_reservations ON booking_resource_reservations;
CREATE POLICY tenant_isolation_booking_resource_reservations ON booking_resource_reservations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for booking_resources
ALTER TABLE IF EXISTS booking_resources ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS booking_resources ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_booking_resources ON booking_resources;
CREATE POLICY tenant_isolation_booking_resources ON booking_resources USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for booking_slots
ALTER TABLE IF EXISTS booking_slots ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS booking_slots ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_booking_slots ON booking_slots;
CREATE POLICY tenant_isolation_booking_slots ON booking_slots USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for bookings
ALTER TABLE IF EXISTS bookings ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS bookings ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_bookings ON bookings;
CREATE POLICY tenant_isolation_bookings ON bookings USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for builder_blocks
ALTER TABLE IF EXISTS builder_blocks ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS builder_blocks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_builder_blocks ON builder_blocks;
CREATE POLICY tenant_isolation_builder_blocks ON builder_blocks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for builder_brand_toolboxes
ALTER TABLE IF EXISTS builder_brand_toolboxes ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS builder_brand_toolboxes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_builder_brand_toolboxes ON builder_brand_toolboxes;
CREATE POLICY tenant_isolation_builder_brand_toolboxes ON builder_brand_toolboxes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for builder_pages
ALTER TABLE IF EXISTS builder_pages ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS builder_pages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_builder_pages ON builder_pages;
CREATE POLICY tenant_isolation_builder_pages ON builder_pages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for builder_sites
ALTER TABLE IF EXISTS builder_sites ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS builder_sites ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_builder_sites ON builder_sites;
CREATE POLICY tenant_isolation_builder_sites ON builder_sites USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for business_milestones
ALTER TABLE IF EXISTS business_milestones ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS business_milestones ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_business_milestones ON business_milestones;
CREATE POLICY tenant_isolation_business_milestones ON business_milestones USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for businesses
ALTER TABLE IF EXISTS businesses ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS businesses ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_businesses ON businesses;
CREATE POLICY tenant_isolation_businesses ON businesses USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for calendar_integrations
ALTER TABLE IF EXISTS calendar_integrations ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS calendar_integrations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_calendar_integrations ON calendar_integrations;
CREATE POLICY tenant_isolation_calendar_integrations ON calendar_integrations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for campaign_assets
ALTER TABLE IF EXISTS campaign_assets ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS campaign_assets ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_campaign_assets ON campaign_assets;
CREATE POLICY tenant_isolation_campaign_assets ON campaign_assets USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for campaigns
ALTER TABLE IF EXISTS campaigns ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS campaigns ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_campaigns ON campaigns;
CREATE POLICY tenant_isolation_campaigns ON campaigns USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for cart_items
ALTER TABLE IF EXISTS cart_items ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS cart_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_cart_items ON cart_items;
CREATE POLICY tenant_isolation_cart_items ON cart_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for carts
ALTER TABLE IF EXISTS carts ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS carts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_carts ON carts;
CREATE POLICY tenant_isolation_carts ON carts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for cash_ledger_entries
ALTER TABLE IF EXISTS cash_ledger_entries ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS cash_ledger_entries ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_cash_ledger_entries ON cash_ledger_entries;
CREATE POLICY tenant_isolation_cash_ledger_entries ON cash_ledger_entries USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for channel_executions
ALTER TABLE IF EXISTS channel_executions ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS channel_executions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_channel_executions ON channel_executions;
CREATE POLICY tenant_isolation_channel_executions ON channel_executions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for competitor_metrics
ALTER TABLE IF EXISTS competitor_metrics ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS competitor_metrics ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_competitor_metrics ON competitor_metrics;
CREATE POLICY tenant_isolation_competitor_metrics ON competitor_metrics USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for conflict_queue
ALTER TABLE IF EXISTS conflict_queue ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS conflict_queue ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_conflict_queue ON conflict_queue;
CREATE POLICY tenant_isolation_conflict_queue ON conflict_queue USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for consolidated_memory
ALTER TABLE IF EXISTS consolidated_memory ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS consolidated_memory ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_consolidated_memory ON consolidated_memory;
CREATE POLICY tenant_isolation_consolidated_memory ON consolidated_memory USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for context_snippets
ALTER TABLE IF EXISTS context_snippets ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS context_snippets ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_context_snippets ON context_snippets;
CREATE POLICY tenant_isolation_context_snippets ON context_snippets USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for conversational_checkout_sessions
ALTER TABLE IF EXISTS conversational_checkout_sessions ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS conversational_checkout_sessions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_conversational_checkout_sessions ON conversational_checkout_sessions;
CREATE POLICY tenant_isolation_conversational_checkout_sessions ON conversational_checkout_sessions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for conversational_intakes
ALTER TABLE IF EXISTS conversational_intakes ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS conversational_intakes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_conversational_intakes ON conversational_intakes;
CREATE POLICY tenant_isolation_conversational_intakes ON conversational_intakes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for crdt_deltas
ALTER TABLE IF EXISTS crdt_deltas ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS crdt_deltas ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_crdt_deltas ON crdt_deltas;
CREATE POLICY tenant_isolation_crdt_deltas ON crdt_deltas USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for customer360
ALTER TABLE IF EXISTS customer360 ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS customer360 ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_customer360 ON customer360;
CREATE POLICY tenant_isolation_customer360 ON customer360 USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for customer_identities
ALTER TABLE IF EXISTS customer_identities ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS customer_identities ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_customer_identities ON customer_identities;
CREATE POLICY tenant_isolation_customer_identities ON customer_identities USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for customer_loyalty_accounts
ALTER TABLE IF EXISTS customer_loyalty_accounts ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS customer_loyalty_accounts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_customer_loyalty_accounts ON customer_loyalty_accounts;
CREATE POLICY tenant_isolation_customer_loyalty_accounts ON customer_loyalty_accounts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for customer_timeline
ALTER TABLE IF EXISTS customer_timeline ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS customer_timeline ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_customer_timeline ON customer_timeline;
CREATE POLICY tenant_isolation_customer_timeline ON customer_timeline USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for customers
ALTER TABLE IF EXISTS customers ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS customers ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_customers ON customers;
CREATE POLICY tenant_isolation_customers ON customers USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for daily_work_items
ALTER TABLE IF EXISTS daily_work_items ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS daily_work_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_daily_work_items ON daily_work_items;
CREATE POLICY tenant_isolation_daily_work_items ON daily_work_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for delivery_tasks
ALTER TABLE IF EXISTS delivery_tasks ADD COLUMN IF NOT EXISTS organization_id VARCHAR;
ALTER TABLE IF EXISTS delivery_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_delivery_tasks ON delivery_tasks;
CREATE POLICY tenant_isolation_delivery_tasks ON delivery_tasks USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for delivery_zones
ALTER TABLE IF EXISTS delivery_zones ADD COLUMN IF NOT EXISTS organization_id VARCHAR;
ALTER TABLE IF EXISTS delivery_zones ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_delivery_zones ON delivery_zones;
CREATE POLICY tenant_isolation_delivery_zones ON delivery_zones USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for department_dead_letters
ALTER TABLE IF EXISTS department_dead_letters ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS department_dead_letters ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_department_dead_letters ON department_dead_letters;
CREATE POLICY tenant_isolation_department_dead_letters ON department_dead_letters USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for department_tasks
ALTER TABLE IF EXISTS department_tasks ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS department_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_department_tasks ON department_tasks;
CREATE POLICY tenant_isolation_department_tasks ON department_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for depletion_logs
ALTER TABLE IF EXISTS depletion_logs ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS depletion_logs ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_depletion_logs ON depletion_logs;
CREATE POLICY tenant_isolation_depletion_logs ON depletion_logs USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for deposit_requirements
ALTER TABLE IF EXISTS deposit_requirements ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS deposit_requirements ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_deposit_requirements ON deposit_requirements;
CREATE POLICY tenant_isolation_deposit_requirements ON deposit_requirements USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for entity_versions
ALTER TABLE IF EXISTS entity_versions ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS entity_versions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_entity_versions ON entity_versions;
CREATE POLICY tenant_isolation_entity_versions ON entity_versions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for escalations
ALTER TABLE IF EXISTS escalations ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS escalations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_escalations ON escalations;
CREATE POLICY tenant_isolation_escalations ON escalations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for estimates
ALTER TABLE IF EXISTS estimates ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS estimates ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_estimates ON estimates;
CREATE POLICY tenant_isolation_estimates ON estimates USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for fulfillment_batches
ALTER TABLE IF EXISTS fulfillment_batches ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS fulfillment_batches ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_fulfillment_batches ON fulfillment_batches;
CREATE POLICY tenant_isolation_fulfillment_batches ON fulfillment_batches USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for help_articles
ALTER TABLE IF EXISTS help_articles ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS help_articles ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_help_articles ON help_articles;
CREATE POLICY tenant_isolation_help_articles ON help_articles USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for hybrid_fs_sync_queue
ALTER TABLE IF EXISTS hybrid_fs_sync_queue ADD COLUMN IF NOT EXISTS organization_id VARCHAR;
ALTER TABLE IF EXISTS hybrid_fs_sync_queue ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_hybrid_fs_sync_queue ON hybrid_fs_sync_queue;
CREATE POLICY tenant_isolation_hybrid_fs_sync_queue ON hybrid_fs_sync_queue USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for inbound_signals
ALTER TABLE IF EXISTS inbound_signals ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS inbound_signals ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_inbound_signals ON inbound_signals;
CREATE POLICY tenant_isolation_inbound_signals ON inbound_signals USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for inbox_messages
ALTER TABLE IF EXISTS inbox_messages ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS inbox_messages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_inbox_messages ON inbox_messages;
CREATE POLICY tenant_isolation_inbox_messages ON inbox_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for incidents
ALTER TABLE IF EXISTS incidents ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS incidents ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_incidents ON incidents;
CREATE POLICY tenant_isolation_incidents ON incidents USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for integration_credentials
ALTER TABLE IF EXISTS integration_credentials ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS integration_credentials ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_integration_credentials ON integration_credentials;
CREATE POLICY tenant_isolation_integration_credentials ON integration_credentials USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for interaction_event_jobs
ALTER TABLE IF EXISTS interaction_event_jobs ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS interaction_event_jobs ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_interaction_event_jobs ON interaction_event_jobs;
CREATE POLICY tenant_isolation_interaction_event_jobs ON interaction_event_jobs USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for interaction_events
ALTER TABLE IF EXISTS interaction_events ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS interaction_events ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_interaction_events ON interaction_events;
CREATE POLICY tenant_isolation_interaction_events ON interaction_events USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for interactions
ALTER TABLE IF EXISTS interactions ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS interactions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_interactions ON interactions;
CREATE POLICY tenant_isolation_interactions ON interactions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for interactive_proposals
ALTER TABLE IF EXISTS interactive_proposals ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS interactive_proposals ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_interactive_proposals ON interactive_proposals;
CREATE POLICY tenant_isolation_interactive_proposals ON interactive_proposals USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for inventory_levels
ALTER TABLE IF EXISTS inventory_levels ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS inventory_levels ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_inventory_levels ON inventory_levels;
CREATE POLICY tenant_isolation_inventory_levels ON inventory_levels USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for inventory_predictions
ALTER TABLE IF EXISTS inventory_predictions ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS inventory_predictions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_inventory_predictions ON inventory_predictions;
CREATE POLICY tenant_isolation_inventory_predictions ON inventory_predictions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for invoice_communication_events
ALTER TABLE IF EXISTS invoice_communication_events ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS invoice_communication_events ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_invoice_communication_events ON invoice_communication_events;
CREATE POLICY tenant_isolation_invoice_communication_events ON invoice_communication_events USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for invoice_line_items
ALTER TABLE IF EXISTS invoice_line_items ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS invoice_line_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_invoice_line_items ON invoice_line_items;
CREATE POLICY tenant_isolation_invoice_line_items ON invoice_line_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for invoices
ALTER TABLE IF EXISTS invoices ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS invoices ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_invoices ON invoices;
CREATE POLICY tenant_isolation_invoices ON invoices USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for job_locations
ALTER TABLE IF EXISTS job_locations ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS job_locations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_job_locations ON job_locations;
CREATE POLICY tenant_isolation_job_locations ON job_locations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for job_templates
ALTER TABLE IF EXISTS job_templates ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS job_templates ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_job_templates ON job_templates;
CREATE POLICY tenant_isolation_job_templates ON job_templates USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for knowledge_embeddings
ALTER TABLE IF EXISTS knowledge_embeddings ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS knowledge_embeddings ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_knowledge_embeddings ON knowledge_embeddings;
CREATE POLICY tenant_isolation_knowledge_embeddings ON knowledge_embeddings USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for lead_gen_campaigns
ALTER TABLE IF EXISTS lead_gen_campaigns ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS lead_gen_campaigns ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_lead_gen_campaigns ON lead_gen_campaigns;
CREATE POLICY tenant_isolation_lead_gen_campaigns ON lead_gen_campaigns USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for leads
ALTER TABLE IF EXISTS leads ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS leads ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_leads ON leads;
CREATE POLICY tenant_isolation_leads ON leads USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for ledger_accounts
ALTER TABLE IF EXISTS ledger_accounts ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS ledger_accounts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ledger_accounts ON ledger_accounts;
CREATE POLICY tenant_isolation_ledger_accounts ON ledger_accounts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for ledger_entries
ALTER TABLE IF EXISTS ledger_entries ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS ledger_entries ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ledger_entries ON ledger_entries;
CREATE POLICY tenant_isolation_ledger_entries ON ledger_entries USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for ledger_transactions
ALTER TABLE IF EXISTS ledger_transactions ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS ledger_transactions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ledger_transactions ON ledger_transactions;
CREATE POLICY tenant_isolation_ledger_transactions ON ledger_transactions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for locations
ALTER TABLE IF EXISTS locations ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS locations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_locations ON locations;
CREATE POLICY tenant_isolation_locations ON locations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for loyalty_ledger
ALTER TABLE IF EXISTS loyalty_ledger ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS loyalty_ledger ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_loyalty_ledger ON loyalty_ledger;
CREATE POLICY tenant_isolation_loyalty_ledger ON loyalty_ledger USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for loyalty_programs
ALTER TABLE IF EXISTS loyalty_programs ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS loyalty_programs ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_loyalty_programs ON loyalty_programs;
CREATE POLICY tenant_isolation_loyalty_programs ON loyalty_programs USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for loyalty_rewards
ALTER TABLE IF EXISTS loyalty_rewards ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS loyalty_rewards ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_loyalty_rewards ON loyalty_rewards;
CREATE POLICY tenant_isolation_loyalty_rewards ON loyalty_rewards USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for loyalty_transactions
ALTER TABLE IF EXISTS loyalty_transactions ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS loyalty_transactions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_loyalty_transactions ON loyalty_transactions;
CREATE POLICY tenant_isolation_loyalty_transactions ON loyalty_transactions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for mcp_config_sync_log
ALTER TABLE IF EXISTS mcp_config_sync_log ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS mcp_config_sync_log ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_mcp_config_sync_log ON mcp_config_sync_log;
CREATE POLICY tenant_isolation_mcp_config_sync_log ON mcp_config_sync_log USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for mcp_servers
ALTER TABLE IF EXISTS mcp_servers ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS mcp_servers ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_mcp_servers ON mcp_servers;
CREATE POLICY tenant_isolation_mcp_servers ON mcp_servers USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for meeting_rooms
ALTER TABLE IF EXISTS meeting_rooms ADD COLUMN IF NOT EXISTS organization_id VARCHAR;
ALTER TABLE IF EXISTS meeting_rooms ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_meeting_rooms ON meeting_rooms;
CREATE POLICY tenant_isolation_meeting_rooms ON meeting_rooms USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for meeting_transcripts
ALTER TABLE IF EXISTS meeting_transcripts ADD COLUMN IF NOT EXISTS organization_id VARCHAR;
ALTER TABLE IF EXISTS meeting_transcripts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_meeting_transcripts ON meeting_transcripts;
CREATE POLICY tenant_isolation_meeting_transcripts ON meeting_transcripts USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for memories
ALTER TABLE IF EXISTS memories ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS memories ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_memories ON memories;
CREATE POLICY tenant_isolation_memories ON memories USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for migration_jobs
ALTER TABLE IF EXISTS migration_jobs ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS migration_jobs ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_migration_jobs ON migration_jobs;
CREATE POLICY tenant_isolation_migration_jobs ON migration_jobs USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for multi_party_split_ledgers
ALTER TABLE IF EXISTS multi_party_split_ledgers ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS multi_party_split_ledgers ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_multi_party_split_ledgers ON multi_party_split_ledgers;
CREATE POLICY tenant_isolation_multi_party_split_ledgers ON multi_party_split_ledgers USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for multi_party_splits
ALTER TABLE IF EXISTS multi_party_splits ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS multi_party_splits ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_multi_party_splits ON multi_party_splits;
CREATE POLICY tenant_isolation_multi_party_splits ON multi_party_splits USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for mutation_queue
ALTER TABLE IF EXISTS mutation_queue ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS mutation_queue ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_mutation_queue ON mutation_queue;
CREATE POLICY tenant_isolation_mutation_queue ON mutation_queue USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for newsletter_drafts
ALTER TABLE IF EXISTS newsletter_drafts ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS newsletter_drafts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_newsletter_drafts ON newsletter_drafts;
CREATE POLICY tenant_isolation_newsletter_drafts ON newsletter_drafts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for ohc_collective
ALTER TABLE IF EXISTS ohc_collective ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS ohc_collective ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_collective ON ohc_collective;
CREATE POLICY tenant_isolation_ohc_collective ON ohc_collective USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for ohc_collective_loyalty_balance
ALTER TABLE IF EXISTS ohc_collective_loyalty_balance ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS ohc_collective_loyalty_balance ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_collective_loyalty_balance ON ohc_collective_loyalty_balance;
CREATE POLICY tenant_isolation_ohc_collective_loyalty_balance ON ohc_collective_loyalty_balance USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for ohc_collective_member
ALTER TABLE IF EXISTS ohc_collective_member ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS ohc_collective_member ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_collective_member ON ohc_collective_member;
CREATE POLICY tenant_isolation_ohc_collective_member ON ohc_collective_member USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for ohc_i18n_strings
ALTER TABLE IF EXISTS ohc_i18n_strings ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS ohc_i18n_strings ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_i18n_strings ON ohc_i18n_strings;
CREATE POLICY tenant_isolation_ohc_i18n_strings ON ohc_i18n_strings USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for ohc_job_queue
ALTER TABLE IF EXISTS ohc_job_queue ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS ohc_job_queue ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_job_queue ON ohc_job_queue;
CREATE POLICY tenant_isolation_ohc_job_queue ON ohc_job_queue USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for ohc_multi_currency_ledger
ALTER TABLE IF EXISTS ohc_multi_currency_ledger ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS ohc_multi_currency_ledger ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_multi_currency_ledger ON ohc_multi_currency_ledger;
CREATE POLICY tenant_isolation_ohc_multi_currency_ledger ON ohc_multi_currency_ledger USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for ohc_shared_offer
ALTER TABLE IF EXISTS ohc_shared_offer ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS ohc_shared_offer ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_shared_offer ON ohc_shared_offer;
CREATE POLICY tenant_isolation_ohc_shared_offer ON ohc_shared_offer USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for ohc_staff_member
ALTER TABLE IF EXISTS ohc_staff_member ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS ohc_staff_member ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_staff_member ON ohc_staff_member;
CREATE POLICY tenant_isolation_ohc_staff_member ON ohc_staff_member USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for ohc_timecard_event
ALTER TABLE IF EXISTS ohc_timecard_event ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS ohc_timecard_event ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_timecard_event ON ohc_timecard_event;
CREATE POLICY tenant_isolation_ohc_timecard_event ON ohc_timecard_event USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for ohc_translation_preferences
ALTER TABLE IF EXISTS ohc_translation_preferences ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS ohc_translation_preferences ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_translation_preferences ON ohc_translation_preferences;
CREATE POLICY tenant_isolation_ohc_translation_preferences ON ohc_translation_preferences USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for ohc_universal_ledger
ALTER TABLE IF EXISTS ohc_universal_ledger ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS ohc_universal_ledger ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_universal_ledger ON ohc_universal_ledger;
CREATE POLICY tenant_isolation_ohc_universal_ledger ON ohc_universal_ledger USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for omni_inbox_messages
ALTER TABLE IF EXISTS omni_inbox_messages ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS omni_inbox_messages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_omni_inbox_messages ON omni_inbox_messages;
CREATE POLICY tenant_isolation_omni_inbox_messages ON omni_inbox_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for onboarding_state
ALTER TABLE IF EXISTS onboarding_state ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS onboarding_state ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_onboarding_state ON onboarding_state;
CREATE POLICY tenant_isolation_onboarding_state ON onboarding_state USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for opportunities
ALTER TABLE IF EXISTS opportunities ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS opportunities ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_opportunities ON opportunities;
CREATE POLICY tenant_isolation_opportunities ON opportunities USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for order_items
ALTER TABLE IF EXISTS order_items ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS order_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_order_items ON order_items;
CREATE POLICY tenant_isolation_order_items ON order_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for order_line_items
ALTER TABLE IF EXISTS order_line_items ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS order_line_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_order_line_items ON order_line_items;
CREATE POLICY tenant_isolation_order_line_items ON order_line_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for orders
ALTER TABLE IF EXISTS orders ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS orders ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_orders ON orders;
CREATE POLICY tenant_isolation_orders ON orders USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for pages
ALTER TABLE IF EXISTS pages ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS pages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_pages ON pages;
CREATE POLICY tenant_isolation_pages ON pages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for payment_events
ALTER TABLE IF EXISTS payment_events ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS payment_events ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_payment_events ON payment_events;
CREATE POLICY tenant_isolation_payment_events ON payment_events USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for payment_intents
ALTER TABLE IF EXISTS payment_intents ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS payment_intents ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_payment_intents ON payment_intents;
CREATE POLICY tenant_isolation_payment_intents ON payment_intents USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for po_line_items
ALTER TABLE IF EXISTS po_line_items ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS po_line_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_po_line_items ON po_line_items;
CREATE POLICY tenant_isolation_po_line_items ON po_line_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for pos_offline_transactions
ALTER TABLE IF EXISTS pos_offline_transactions ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS pos_offline_transactions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_pos_offline_transactions ON pos_offline_transactions;
CREATE POLICY tenant_isolation_pos_offline_transactions ON pos_offline_transactions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for pos_terminal_sessions
ALTER TABLE IF EXISTS pos_terminal_sessions ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS pos_terminal_sessions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_pos_terminal_sessions ON pos_terminal_sessions;
CREATE POLICY tenant_isolation_pos_terminal_sessions ON pos_terminal_sessions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for pre_order_entries
ALTER TABLE IF EXISTS pre_order_entries ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS pre_order_entries ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_pre_order_entries ON pre_order_entries;
CREATE POLICY tenant_isolation_pre_order_entries ON pre_order_entries USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for price_history
ALTER TABLE IF EXISTS price_history ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS price_history ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_price_history ON price_history;
CREATE POLICY tenant_isolation_price_history ON price_history USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for pricing_heuristics
ALTER TABLE IF EXISTS pricing_heuristics ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS pricing_heuristics ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_pricing_heuristics ON pricing_heuristics;
CREATE POLICY tenant_isolation_pricing_heuristics ON pricing_heuristics USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for pricing_rules
ALTER TABLE IF EXISTS pricing_rules ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS pricing_rules ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_pricing_rules ON pricing_rules;
CREATE POLICY tenant_isolation_pricing_rules ON pricing_rules USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for product_variants
ALTER TABLE IF EXISTS product_variants ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS product_variants ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_product_variants ON product_variants;
CREATE POLICY tenant_isolation_product_variants ON product_variants USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for products
ALTER TABLE IF EXISTS products ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS products ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_products ON products;
CREATE POLICY tenant_isolation_products ON products USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for project_tasks
ALTER TABLE IF EXISTS project_tasks ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS project_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_project_tasks ON project_tasks;
CREATE POLICY tenant_isolation_project_tasks ON project_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for projects
ALTER TABLE IF EXISTS projects ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS projects ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_projects ON projects;
CREATE POLICY tenant_isolation_projects ON projects USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for promotion_codes
ALTER TABLE IF EXISTS promotion_codes ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS promotion_codes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_promotion_codes ON promotion_codes;
CREATE POLICY tenant_isolation_promotion_codes ON promotion_codes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for proposals
ALTER TABLE IF EXISTS proposals ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS proposals ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_proposals ON proposals;
CREATE POLICY tenant_isolation_proposals ON proposals USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for purchase_orders
ALTER TABLE IF EXISTS purchase_orders ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS purchase_orders ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_purchase_orders ON purchase_orders;
CREATE POLICY tenant_isolation_purchase_orders ON purchase_orders USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for quote_requests
ALTER TABLE IF EXISTS quote_requests ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS quote_requests ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_quote_requests ON quote_requests;
CREATE POLICY tenant_isolation_quote_requests ON quote_requests USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for quotes
ALTER TABLE IF EXISTS quotes ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS quotes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_quotes ON quotes;
CREATE POLICY tenant_isolation_quotes ON quotes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for raw_materials
ALTER TABLE IF EXISTS raw_materials ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS raw_materials ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_raw_materials ON raw_materials;
CREATE POLICY tenant_isolation_raw_materials ON raw_materials USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for recovery_attempts
ALTER TABLE IF EXISTS recovery_attempts ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS recovery_attempts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_recovery_attempts ON recovery_attempts;
CREATE POLICY tenant_isolation_recovery_attempts ON recovery_attempts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for recovery_campaigns
ALTER TABLE IF EXISTS recovery_campaigns ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS recovery_campaigns ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_recovery_campaigns ON recovery_campaigns;
CREATE POLICY tenant_isolation_recovery_campaigns ON recovery_campaigns USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for referral_codes
ALTER TABLE IF EXISTS referral_codes ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS referral_codes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_referral_codes ON referral_codes;
CREATE POLICY tenant_isolation_referral_codes ON referral_codes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for referrals
ALTER TABLE IF EXISTS referrals ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS referrals ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_referrals ON referrals;
CREATE POLICY tenant_isolation_referrals ON referrals USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for reputation_profiles
ALTER TABLE IF EXISTS reputation_profiles ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS reputation_profiles ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_reputation_profiles ON reputation_profiles;
CREATE POLICY tenant_isolation_reputation_profiles ON reputation_profiles USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for reviews
ALTER TABLE IF EXISTS reviews ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS reviews ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_reviews ON reviews;
CREATE POLICY tenant_isolation_reviews ON reviews USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for revoked_tokens
ALTER TABLE IF EXISTS revoked_tokens ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS revoked_tokens ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_revoked_tokens ON revoked_tokens;
CREATE POLICY tenant_isolation_revoked_tokens ON revoked_tokens USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for role_assignments
ALTER TABLE IF EXISTS role_assignments ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS role_assignments ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_role_assignments ON role_assignments;
CREATE POLICY tenant_isolation_role_assignments ON role_assignments USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for roles
ALTER TABLE IF EXISTS roles ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS roles ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_roles ON roles;
CREATE POLICY tenant_isolation_roles ON roles USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for route_plans
ALTER TABLE IF EXISTS route_plans ADD COLUMN IF NOT EXISTS organization_id VARCHAR;
ALTER TABLE IF EXISTS route_plans ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_route_plans ON route_plans;
CREATE POLICY tenant_isolation_route_plans ON route_plans USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for route_stops
ALTER TABLE IF EXISTS route_stops ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS route_stops ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_route_stops ON route_stops;
CREATE POLICY tenant_isolation_route_stops ON route_stops USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for seo_discovery_reports
ALTER TABLE IF EXISTS seo_discovery_reports ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS seo_discovery_reports ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_seo_discovery_reports ON seo_discovery_reports;
CREATE POLICY tenant_isolation_seo_discovery_reports ON seo_discovery_reports USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for service_leads
ALTER TABLE IF EXISTS service_leads ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS service_leads ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_service_leads ON service_leads;
CREATE POLICY tenant_isolation_service_leads ON service_leads USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for service_requests
ALTER TABLE IF EXISTS service_requests ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS service_requests ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_service_requests ON service_requests;
CREATE POLICY tenant_isolation_service_requests ON service_requests USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for service_resource_requirements
ALTER TABLE IF EXISTS service_resource_requirements ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS service_resource_requirements ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_service_resource_requirements ON service_resource_requirements;
CREATE POLICY tenant_isolation_service_resource_requirements ON service_resource_requirements USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for service_routes
ALTER TABLE IF EXISTS service_routes ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS service_routes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_service_routes ON service_routes;
CREATE POLICY tenant_isolation_service_routes ON service_routes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for services
ALTER TABLE IF EXISTS services ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS services ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_services ON services;
CREATE POLICY tenant_isolation_services ON services USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for shared_tasks
ALTER TABLE IF EXISTS shared_tasks ADD COLUMN IF NOT EXISTS organization_id VARCHAR;
ALTER TABLE IF EXISTS shared_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shared_tasks ON shared_tasks;
CREATE POLICY tenant_isolation_shared_tasks ON shared_tasks USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for shared_tasks_decomposition
ALTER TABLE IF EXISTS shared_tasks_decomposition ADD COLUMN IF NOT EXISTS organization_id VARCHAR;
ALTER TABLE IF EXISTS shared_tasks_decomposition ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition;
CREATE POLICY tenant_isolation_shared_tasks_decomposition ON shared_tasks_decomposition USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for shared_tasks_v4
ALTER TABLE IF EXISTS shared_tasks_v4 ADD COLUMN IF NOT EXISTS organization_id VARCHAR;
ALTER TABLE IF EXISTS shared_tasks_v4 ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shared_tasks_v4 ON shared_tasks_v4;
CREATE POLICY tenant_isolation_shared_tasks_v4 ON shared_tasks_v4 USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for shifts
ALTER TABLE IF EXISTS shifts ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS shifts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shifts ON shifts;
CREATE POLICY tenant_isolation_shifts ON shifts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for smart_pricing_policies
ALTER TABLE IF EXISTS smart_pricing_policies ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS smart_pricing_policies ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_smart_pricing_policies ON smart_pricing_policies;
CREATE POLICY tenant_isolation_smart_pricing_policies ON smart_pricing_policies USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for staff_availability
ALTER TABLE IF EXISTS staff_availability ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS staff_availability ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_staff_availability ON staff_availability;
CREATE POLICY tenant_isolation_staff_availability ON staff_availability USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for staff_profiles
ALTER TABLE IF EXISTS staff_profiles ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS staff_profiles ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_staff_profiles ON staff_profiles;
CREATE POLICY tenant_isolation_staff_profiles ON staff_profiles USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for state_machine_transitions
ALTER TABLE IF EXISTS state_machine_transitions ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS state_machine_transitions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_state_machine_transitions ON state_machine_transitions;
CREATE POLICY tenant_isolation_state_machine_transitions ON state_machine_transitions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for sub_agent_queue
ALTER TABLE IF EXISTS sub_agent_queue ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS sub_agent_queue ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_sub_agent_queue ON sub_agent_queue;
CREATE POLICY tenant_isolation_sub_agent_queue ON sub_agent_queue USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for subscribers
ALTER TABLE IF EXISTS subscribers ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS subscribers ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_subscribers ON subscribers;
CREATE POLICY tenant_isolation_subscribers ON subscribers USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for subscription_plans
ALTER TABLE IF EXISTS subscription_plans ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS subscription_plans ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_subscription_plans ON subscription_plans;
CREATE POLICY tenant_isolation_subscription_plans ON subscription_plans USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for subscriptions
ALTER TABLE IF EXISTS subscriptions ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS subscriptions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_subscriptions ON subscriptions;
CREATE POLICY tenant_isolation_subscriptions ON subscriptions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for swarm_tasks
ALTER TABLE IF EXISTS swarm_tasks ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS swarm_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_swarm_tasks ON swarm_tasks;
CREATE POLICY tenant_isolation_swarm_tasks ON swarm_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for sync_conflict_queue
ALTER TABLE IF EXISTS sync_conflict_queue ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS sync_conflict_queue ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_sync_conflict_queue ON sync_conflict_queue;
CREATE POLICY tenant_isolation_sync_conflict_queue ON sync_conflict_queue USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for sync_events
ALTER TABLE IF EXISTS sync_events ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS sync_events ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_sync_events ON sync_events;
CREATE POLICY tenant_isolation_sync_events ON sync_events USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for task_dependencies
ALTER TABLE IF EXISTS task_dependencies ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS task_dependencies ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_task_dependencies ON task_dependencies;
CREATE POLICY tenant_isolation_task_dependencies ON task_dependencies USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for tasks
ALTER TABLE IF EXISTS tasks ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_tasks ON tasks;
CREATE POLICY tenant_isolation_tasks ON tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for team_invites
ALTER TABLE IF EXISTS team_invites ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS team_invites ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_team_invites ON team_invites;
CREATE POLICY tenant_isolation_team_invites ON team_invites USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for telemetry_buffer
ALTER TABLE IF EXISTS telemetry_buffer ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS telemetry_buffer ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_telemetry_buffer ON telemetry_buffer;
CREATE POLICY tenant_isolation_telemetry_buffer ON telemetry_buffer USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for tenant_ai_budgets
ALTER TABLE IF EXISTS tenant_ai_budgets ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS tenant_ai_budgets ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_tenant_ai_budgets ON tenant_ai_budgets;
CREATE POLICY tenant_isolation_tenant_ai_budgets ON tenant_ai_budgets USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for tenant_feed_items
ALTER TABLE IF EXISTS tenant_feed_items ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS tenant_feed_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_tenant_feed_items ON tenant_feed_items;
CREATE POLICY tenant_isolation_tenant_feed_items ON tenant_feed_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for test_sync_entities
ALTER TABLE IF EXISTS test_sync_entities ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS test_sync_entities ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_test_sync_entities ON test_sync_entities;
CREATE POLICY tenant_isolation_test_sync_entities ON test_sync_entities USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for tool_integrations
ALTER TABLE IF EXISTS tool_integrations ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS tool_integrations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_tool_integrations ON tool_integrations;
CREATE POLICY tenant_isolation_tool_integrations ON tool_integrations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for tooltips
ALTER TABLE IF EXISTS tooltips ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS tooltips ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_tooltips ON tooltips;
CREATE POLICY tenant_isolation_tooltips ON tooltips USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for travel_buffers
ALTER TABLE IF EXISTS travel_buffers ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS travel_buffers ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_travel_buffers ON travel_buffers;
CREATE POLICY tenant_isolation_travel_buffers ON travel_buffers USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for triage_items
ALTER TABLE IF EXISTS triage_items ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS triage_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_triage_items ON triage_items;
CREATE POLICY tenant_isolation_triage_items ON triage_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for triage_proposed_actions
ALTER TABLE IF EXISTS triage_proposed_actions ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS triage_proposed_actions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_triage_proposed_actions ON triage_proposed_actions;
CREATE POLICY tenant_isolation_triage_proposed_actions ON triage_proposed_actions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for unified_messages
ALTER TABLE IF EXISTS unified_messages ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS unified_messages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_unified_messages ON unified_messages;
CREATE POLICY tenant_isolation_unified_messages ON unified_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for unified_threads
ALTER TABLE IF EXISTS unified_threads ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS unified_threads ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_unified_threads ON unified_threads;
CREATE POLICY tenant_isolation_unified_threads ON unified_threads USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for unified_triage_actions
ALTER TABLE IF EXISTS unified_triage_actions ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS unified_triage_actions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_unified_triage_actions ON unified_triage_actions;
CREATE POLICY tenant_isolation_unified_triage_actions ON unified_triage_actions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for users
ALTER TABLE IF EXISTS users ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS users ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_users ON users;
CREATE POLICY tenant_isolation_users ON users USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for vendors
ALTER TABLE IF EXISTS vendors ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS vendors ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_vendors ON vendors;
CREATE POLICY tenant_isolation_vendors ON vendors USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for video_tutorials
ALTER TABLE IF EXISTS video_tutorials ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS video_tutorials ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_video_tutorials ON video_tutorials;
CREATE POLICY tenant_isolation_video_tutorials ON video_tutorials USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for waitlist_campaigns
ALTER TABLE IF EXISTS waitlist_campaigns ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS waitlist_campaigns ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_waitlist_campaigns ON waitlist_campaigns;
CREATE POLICY tenant_isolation_waitlist_campaigns ON waitlist_campaigns USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- Adding RLS for walkthrough_steps
ALTER TABLE IF EXISTS walkthrough_steps ADD COLUMN IF NOT EXISTS tenant_id VARCHAR;
ALTER TABLE IF EXISTS walkthrough_steps ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_walkthrough_steps ON walkthrough_steps;
CREATE POLICY tenant_isolation_walkthrough_steps ON walkthrough_steps USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
