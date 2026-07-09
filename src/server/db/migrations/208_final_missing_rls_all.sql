-- +goose Up
ALTER TABLE IF EXISTS active_discounts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_active_discounts ON active_discounts;
CREATE POLICY tenant_isolation_active_discounts ON active_discounts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS affiliate_ledgers ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_affiliate_ledgers ON affiliate_ledgers;
CREATE POLICY tenant_isolation_affiliate_ledgers ON affiliate_ledgers USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS affiliate_links ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_affiliate_links ON affiliate_links;
CREATE POLICY tenant_isolation_affiliate_links ON affiliate_links USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS affiliate_payouts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_affiliate_payouts ON affiliate_payouts;
CREATE POLICY tenant_isolation_affiliate_payouts ON affiliate_payouts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS agent_action_requests ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_action_requests ON agent_action_requests;
CREATE POLICY tenant_isolation_agent_action_requests ON agent_action_requests USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS agent_feed_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_feed_items ON agent_feed_items;
CREATE POLICY tenant_isolation_agent_feed_items ON agent_feed_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS agent_session_summaries ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_agent_session_summaries ON agent_session_summaries;
CREATE POLICY tenant_isolation_agent_session_summaries ON agent_session_summaries USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS applied_client_mutations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_applied_client_mutations ON applied_client_mutations;
CREATE POLICY tenant_isolation_applied_client_mutations ON applied_client_mutations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS appointments ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_appointments ON appointments;
CREATE POLICY tenant_isolation_appointments ON appointments USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS assistant_artifacts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_artifacts ON assistant_artifacts;
CREATE POLICY tenant_isolation_assistant_artifacts ON assistant_artifacts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS assistant_connectors ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_connectors ON assistant_connectors;
CREATE POLICY tenant_isolation_assistant_connectors ON assistant_connectors USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS assistant_file_changes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_file_changes ON assistant_file_changes;
CREATE POLICY tenant_isolation_assistant_file_changes ON assistant_file_changes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS assistant_memory_records ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_memory_records ON assistant_memory_records;
CREATE POLICY tenant_isolation_assistant_memory_records ON assistant_memory_records USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS assistant_messages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_messages ON assistant_messages;
CREATE POLICY tenant_isolation_assistant_messages ON assistant_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS assistant_skills ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_skills ON assistant_skills;
CREATE POLICY tenant_isolation_assistant_skills ON assistant_skills USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS assistant_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_tasks ON assistant_tasks;
CREATE POLICY tenant_isolation_assistant_tasks ON assistant_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS assistant_workspaces ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_assistant_workspaces ON assistant_workspaces;
CREATE POLICY tenant_isolation_assistant_workspaces ON assistant_workspaces USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS availability_blocks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_availability_blocks ON availability_blocks;
CREATE POLICY tenant_isolation_availability_blocks ON availability_blocks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS booking_resource_reservations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_booking_resource_reservations ON booking_resource_reservations;
CREATE POLICY tenant_isolation_booking_resource_reservations ON booking_resource_reservations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS booking_resources ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_booking_resources ON booking_resources;
CREATE POLICY tenant_isolation_booking_resources ON booking_resources USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS booking_slots ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_booking_slots ON booking_slots;
CREATE POLICY tenant_isolation_booking_slots ON booking_slots USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS bookings ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_bookings ON bookings;
CREATE POLICY tenant_isolation_bookings ON bookings USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS cash_ledger_entries ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_cash_ledger_entries ON cash_ledger_entries;
CREATE POLICY tenant_isolation_cash_ledger_entries ON cash_ledger_entries USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS checkout_sessions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_checkout_sessions ON checkout_sessions;
CREATE POLICY tenant_isolation_checkout_sessions ON checkout_sessions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS conflict_queue ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_conflict_queue ON conflict_queue;
CREATE POLICY tenant_isolation_conflict_queue ON conflict_queue USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS consolidated_memory ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_consolidated_memory ON consolidated_memory;
CREATE POLICY tenant_isolation_consolidated_memory ON consolidated_memory USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS conversational_intakes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_conversational_intakes ON conversational_intakes;
CREATE POLICY tenant_isolation_conversational_intakes ON conversational_intakes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS crdt_deltas ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_crdt_deltas ON crdt_deltas;
CREATE POLICY tenant_isolation_crdt_deltas ON crdt_deltas USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS customer_identities ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_customer_identities ON customer_identities;
CREATE POLICY tenant_isolation_customer_identities ON customer_identities USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS customer_profile ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_customer_profile ON customer_profile;
CREATE POLICY tenant_isolation_customer_profile ON customer_profile USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS customers ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_customers ON customers;
CREATE POLICY tenant_isolation_customers ON customers USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS daily_work_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_daily_work_items ON daily_work_items;
CREATE POLICY tenant_isolation_daily_work_items ON daily_work_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS deposit_requirements ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_deposit_requirements ON deposit_requirements;
CREATE POLICY tenant_isolation_deposit_requirements ON deposit_requirements USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS entity_versions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_entity_versions ON entity_versions;
CREATE POLICY tenant_isolation_entity_versions ON entity_versions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS escalations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_escalations ON escalations;
CREATE POLICY tenant_isolation_escalations ON escalations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS estimates ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_estimates ON estimates;
CREATE POLICY tenant_isolation_estimates ON estimates USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS fulfillment_batches ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_fulfillment_batches ON fulfillment_batches;
CREATE POLICY tenant_isolation_fulfillment_batches ON fulfillment_batches USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS help_articles ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_help_articles ON help_articles;
CREATE POLICY tenant_isolation_help_articles ON help_articles USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS inbound_signals ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_inbound_signals ON inbound_signals;
CREATE POLICY tenant_isolation_inbound_signals ON inbound_signals USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS integration_credentials ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_integration_credentials ON integration_credentials;
CREATE POLICY tenant_isolation_integration_credentials ON integration_credentials USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS interactive_proposals ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_interactive_proposals ON interactive_proposals;
CREATE POLICY tenant_isolation_interactive_proposals ON interactive_proposals USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS inventory_levels ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_inventory_levels ON inventory_levels;
CREATE POLICY tenant_isolation_inventory_levels ON inventory_levels USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS inventory_predictions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_inventory_predictions ON inventory_predictions;
CREATE POLICY tenant_isolation_inventory_predictions ON inventory_predictions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS invoice_communication_events ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_invoice_communication_events ON invoice_communication_events;
CREATE POLICY tenant_isolation_invoice_communication_events ON invoice_communication_events USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS invoice_line_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_invoice_line_items ON invoice_line_items;
CREATE POLICY tenant_isolation_invoice_line_items ON invoice_line_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS invoices ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_invoices ON invoices;
CREATE POLICY tenant_isolation_invoices ON invoices USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS job_locations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_job_locations ON job_locations;
CREATE POLICY tenant_isolation_job_locations ON job_locations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS job_templates ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_job_templates ON job_templates;
CREATE POLICY tenant_isolation_job_templates ON job_templates USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS lead_gen_campaigns ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_lead_gen_campaigns ON lead_gen_campaigns;
CREATE POLICY tenant_isolation_lead_gen_campaigns ON lead_gen_campaigns USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS leads ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_leads ON leads;
CREATE POLICY tenant_isolation_leads ON leads USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS ledger_reserves ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ledger_reserves ON ledger_reserves;
CREATE POLICY tenant_isolation_ledger_reserves ON ledger_reserves USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS locations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_locations ON locations;
CREATE POLICY tenant_isolation_locations ON locations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS loyalty_ledgers ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_loyalty_ledgers ON loyalty_ledgers;
CREATE POLICY tenant_isolation_loyalty_ledgers ON loyalty_ledgers USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS mcp_config_sync_log ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_mcp_config_sync_log ON mcp_config_sync_log;
CREATE POLICY tenant_isolation_mcp_config_sync_log ON mcp_config_sync_log USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS multi_party_split_ledgers ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_multi_party_split_ledgers ON multi_party_split_ledgers;
CREATE POLICY tenant_isolation_multi_party_split_ledgers ON multi_party_split_ledgers USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS multi_party_splits ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_multi_party_splits ON multi_party_splits;
CREATE POLICY tenant_isolation_multi_party_splits ON multi_party_splits USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS ohc_collective ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_collective ON ohc_collective;
CREATE POLICY tenant_isolation_ohc_collective ON ohc_collective USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS ohc_collective_loyalty_balance ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_collective_loyalty_balance ON ohc_collective_loyalty_balance;
CREATE POLICY tenant_isolation_ohc_collective_loyalty_balance ON ohc_collective_loyalty_balance USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS ohc_collective_member ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_collective_member ON ohc_collective_member;
CREATE POLICY tenant_isolation_ohc_collective_member ON ohc_collective_member USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS ohc_job_queue ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_job_queue ON ohc_job_queue;
CREATE POLICY tenant_isolation_ohc_job_queue ON ohc_job_queue USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS ohc_shared_offer ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_shared_offer ON ohc_shared_offer;
CREATE POLICY tenant_isolation_ohc_shared_offer ON ohc_shared_offer USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS ohc_staff_member ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_staff_member ON ohc_staff_member;
CREATE POLICY tenant_isolation_ohc_staff_member ON ohc_staff_member USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS ohc_timecard_event ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_timecard_event ON ohc_timecard_event;
CREATE POLICY tenant_isolation_ohc_timecard_event ON ohc_timecard_event USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS ohc_universal_ledger ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_ohc_universal_ledger ON ohc_universal_ledger;
CREATE POLICY tenant_isolation_ohc_universal_ledger ON ohc_universal_ledger USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS omni_inbox_messages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_omni_inbox_messages ON omni_inbox_messages;
CREATE POLICY tenant_isolation_omni_inbox_messages ON omni_inbox_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS onboarding_state ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_onboarding_state ON onboarding_state;
CREATE POLICY tenant_isolation_onboarding_state ON onboarding_state USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS operation_intents ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_operation_intents ON operation_intents;
CREATE POLICY tenant_isolation_operation_intents ON operation_intents USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS opportunities ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_opportunities ON opportunities;
CREATE POLICY tenant_isolation_opportunities ON opportunities USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS pos_offline_transactions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_pos_offline_transactions ON pos_offline_transactions;
CREATE POLICY tenant_isolation_pos_offline_transactions ON pos_offline_transactions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS pos_terminal_sessions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_pos_terminal_sessions ON pos_terminal_sessions;
CREATE POLICY tenant_isolation_pos_terminal_sessions ON pos_terminal_sessions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS pre_order_entries ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_pre_order_entries ON pre_order_entries;
CREATE POLICY tenant_isolation_pre_order_entries ON pre_order_entries USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS price_history ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_price_history ON price_history;
CREATE POLICY tenant_isolation_price_history ON price_history USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS pricing_heuristics ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_pricing_heuristics ON pricing_heuristics;
CREATE POLICY tenant_isolation_pricing_heuristics ON pricing_heuristics USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS pricing_rules ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_pricing_rules ON pricing_rules;
CREATE POLICY tenant_isolation_pricing_rules ON pricing_rules USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS project_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_project_tasks ON project_tasks;
CREATE POLICY tenant_isolation_project_tasks ON project_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS projects ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_projects ON projects;
CREATE POLICY tenant_isolation_projects ON projects USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS proposals ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_proposals ON proposals;
CREATE POLICY tenant_isolation_proposals ON proposals USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS proposed_bookings ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_proposed_bookings ON proposed_bookings;
CREATE POLICY tenant_isolation_proposed_bookings ON proposed_bookings USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS purchase_orders ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_purchase_orders ON purchase_orders;
CREATE POLICY tenant_isolation_purchase_orders ON purchase_orders USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS quote_requests ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_quote_requests ON quote_requests;
CREATE POLICY tenant_isolation_quote_requests ON quote_requests USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS quotes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_quotes ON quotes;
CREATE POLICY tenant_isolation_quotes ON quotes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS recovery_attempts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_recovery_attempts ON recovery_attempts;
CREATE POLICY tenant_isolation_recovery_attempts ON recovery_attempts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS recovery_campaigns ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_recovery_campaigns ON recovery_campaigns;
CREATE POLICY tenant_isolation_recovery_campaigns ON recovery_campaigns USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS referrals ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_referrals ON referrals;
CREATE POLICY tenant_isolation_referrals ON referrals USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS reward_claims ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_reward_claims ON reward_claims;
CREATE POLICY tenant_isolation_reward_claims ON reward_claims USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS role_assignments ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_role_assignments ON role_assignments;
CREATE POLICY tenant_isolation_role_assignments ON role_assignments USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS seo_discovery_reports ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_seo_discovery_reports ON seo_discovery_reports;
CREATE POLICY tenant_isolation_seo_discovery_reports ON seo_discovery_reports USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS service_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_service_items ON service_items;
CREATE POLICY tenant_isolation_service_items ON service_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS service_leads ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_service_leads ON service_leads;
CREATE POLICY tenant_isolation_service_leads ON service_leads USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS service_resource_requirements ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_service_resource_requirements ON service_resource_requirements;
CREATE POLICY tenant_isolation_service_resource_requirements ON service_resource_requirements USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS service_routes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_service_routes ON service_routes;
CREATE POLICY tenant_isolation_service_routes ON service_routes USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS services ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_services ON services;
CREATE POLICY tenant_isolation_services ON services USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS shift_swap_requests ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shift_swap_requests ON shift_swap_requests;
CREATE POLICY tenant_isolation_shift_swap_requests ON shift_swap_requests USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS shifts ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_shifts ON shifts;
CREATE POLICY tenant_isolation_shifts ON shifts USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS smart_pricing_policies ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_smart_pricing_policies ON smart_pricing_policies;
CREATE POLICY tenant_isolation_smart_pricing_policies ON smart_pricing_policies USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS staff_availability ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_staff_availability ON staff_availability;
CREATE POLICY tenant_isolation_staff_availability ON staff_availability USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS staff_profiles ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_staff_profiles ON staff_profiles;
CREATE POLICY tenant_isolation_staff_profiles ON staff_profiles USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS subscribers ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_subscribers ON subscribers;
CREATE POLICY tenant_isolation_subscribers ON subscribers USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS subscription_plans ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_subscription_plans ON subscription_plans;
CREATE POLICY tenant_isolation_subscription_plans ON subscription_plans USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS subscriptions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_subscriptions ON subscriptions;
CREATE POLICY tenant_isolation_subscriptions ON subscriptions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS swarm_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_swarm_tasks ON swarm_tasks;
CREATE POLICY tenant_isolation_swarm_tasks ON swarm_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS sync_events ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_sync_events ON sync_events;
CREATE POLICY tenant_isolation_sync_events ON sync_events USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS task_dependencies ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_task_dependencies ON task_dependencies;
CREATE POLICY tenant_isolation_task_dependencies ON task_dependencies USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS team_invites ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_team_invites ON team_invites;
CREATE POLICY tenant_isolation_team_invites ON team_invites USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS telemetry_buffer ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_telemetry_buffer ON telemetry_buffer;
CREATE POLICY tenant_isolation_telemetry_buffer ON telemetry_buffer USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS tenant_feed_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_tenant_feed_items ON tenant_feed_items;
CREATE POLICY tenant_isolation_tenant_feed_items ON tenant_feed_items USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS tool_integrations ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_tool_integrations ON tool_integrations;
CREATE POLICY tenant_isolation_tool_integrations ON tool_integrations USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS tooltips ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_tooltips ON tooltips;
CREATE POLICY tenant_isolation_tooltips ON tooltips USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS unified_messages ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_unified_messages ON unified_messages;
CREATE POLICY tenant_isolation_unified_messages ON unified_messages USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS unified_threads ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_unified_threads ON unified_threads;
CREATE POLICY tenant_isolation_unified_threads ON unified_threads USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS unified_triage_actions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_unified_triage_actions ON unified_triage_actions;
CREATE POLICY tenant_isolation_unified_triage_actions ON unified_triage_actions USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS vendors ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_vendors ON vendors;
CREATE POLICY tenant_isolation_vendors ON vendors USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS video_tutorials ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_video_tutorials ON video_tutorials;
CREATE POLICY tenant_isolation_video_tutorials ON video_tutorials USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS waitlist_campaigns ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_waitlist_campaigns ON waitlist_campaigns;
CREATE POLICY tenant_isolation_waitlist_campaigns ON waitlist_campaigns USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS walkthrough_steps ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_walkthrough_steps ON walkthrough_steps;
CREATE POLICY tenant_isolation_walkthrough_steps ON walkthrough_steps USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS work_intents ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_work_intents ON work_intents;
CREATE POLICY tenant_isolation_work_intents ON work_intents USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS work_item ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_work_item ON work_item;
CREATE POLICY tenant_isolation_work_item ON work_item USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE IF EXISTS work_tasks ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_work_tasks ON work_tasks;
CREATE POLICY tenant_isolation_work_tasks ON work_tasks USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

-- +goose Down
-- Revert missing RLS
DROP POLICY IF EXISTS tenant_isolation_active_discounts ON active_discounts;
ALTER TABLE IF EXISTS active_discounts DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_affiliate_ledgers ON affiliate_ledgers;
ALTER TABLE IF EXISTS affiliate_ledgers DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_affiliate_links ON affiliate_links;
ALTER TABLE IF EXISTS affiliate_links DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_affiliate_payouts ON affiliate_payouts;
ALTER TABLE IF EXISTS affiliate_payouts DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_agent_action_requests ON agent_action_requests;
ALTER TABLE IF EXISTS agent_action_requests DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_agent_feed_items ON agent_feed_items;
ALTER TABLE IF EXISTS agent_feed_items DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_agent_session_summaries ON agent_session_summaries;
ALTER TABLE IF EXISTS agent_session_summaries DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_applied_client_mutations ON applied_client_mutations;
ALTER TABLE IF EXISTS applied_client_mutations DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_appointments ON appointments;
ALTER TABLE IF EXISTS appointments DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_assistant_artifacts ON assistant_artifacts;
ALTER TABLE IF EXISTS assistant_artifacts DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_assistant_connectors ON assistant_connectors;
ALTER TABLE IF EXISTS assistant_connectors DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_assistant_file_changes ON assistant_file_changes;
ALTER TABLE IF EXISTS assistant_file_changes DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_assistant_memory_records ON assistant_memory_records;
ALTER TABLE IF EXISTS assistant_memory_records DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_assistant_messages ON assistant_messages;
ALTER TABLE IF EXISTS assistant_messages DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_assistant_skills ON assistant_skills;
ALTER TABLE IF EXISTS assistant_skills DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_assistant_tasks ON assistant_tasks;
ALTER TABLE IF EXISTS assistant_tasks DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_assistant_workspaces ON assistant_workspaces;
ALTER TABLE IF EXISTS assistant_workspaces DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_availability_blocks ON availability_blocks;
ALTER TABLE IF EXISTS availability_blocks DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_booking_resource_reservations ON booking_resource_reservations;
ALTER TABLE IF EXISTS booking_resource_reservations DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_booking_resources ON booking_resources;
ALTER TABLE IF EXISTS booking_resources DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_booking_slots ON booking_slots;
ALTER TABLE IF EXISTS booking_slots DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_bookings ON bookings;
ALTER TABLE IF EXISTS bookings DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_cash_ledger_entries ON cash_ledger_entries;
ALTER TABLE IF EXISTS cash_ledger_entries DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_checkout_sessions ON checkout_sessions;
ALTER TABLE IF EXISTS checkout_sessions DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_conflict_queue ON conflict_queue;
ALTER TABLE IF EXISTS conflict_queue DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_consolidated_memory ON consolidated_memory;
ALTER TABLE IF EXISTS consolidated_memory DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_conversational_intakes ON conversational_intakes;
ALTER TABLE IF EXISTS conversational_intakes DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_crdt_deltas ON crdt_deltas;
ALTER TABLE IF EXISTS crdt_deltas DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_customer_identities ON customer_identities;
ALTER TABLE IF EXISTS customer_identities DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_customer_profile ON customer_profile;
ALTER TABLE IF EXISTS customer_profile DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_customers ON customers;
ALTER TABLE IF EXISTS customers DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_daily_work_items ON daily_work_items;
ALTER TABLE IF EXISTS daily_work_items DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_deposit_requirements ON deposit_requirements;
ALTER TABLE IF EXISTS deposit_requirements DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_entity_versions ON entity_versions;
ALTER TABLE IF EXISTS entity_versions DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_escalations ON escalations;
ALTER TABLE IF EXISTS escalations DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_estimates ON estimates;
ALTER TABLE IF EXISTS estimates DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_fulfillment_batches ON fulfillment_batches;
ALTER TABLE IF EXISTS fulfillment_batches DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_help_articles ON help_articles;
ALTER TABLE IF EXISTS help_articles DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_inbound_signals ON inbound_signals;
ALTER TABLE IF EXISTS inbound_signals DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_integration_credentials ON integration_credentials;
ALTER TABLE IF EXISTS integration_credentials DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_interactive_proposals ON interactive_proposals;
ALTER TABLE IF EXISTS interactive_proposals DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_inventory_levels ON inventory_levels;
ALTER TABLE IF EXISTS inventory_levels DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_inventory_predictions ON inventory_predictions;
ALTER TABLE IF EXISTS inventory_predictions DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_invoice_communication_events ON invoice_communication_events;
ALTER TABLE IF EXISTS invoice_communication_events DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_invoice_line_items ON invoice_line_items;
ALTER TABLE IF EXISTS invoice_line_items DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_invoices ON invoices;
ALTER TABLE IF EXISTS invoices DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_job_locations ON job_locations;
ALTER TABLE IF EXISTS job_locations DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_job_templates ON job_templates;
ALTER TABLE IF EXISTS job_templates DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_lead_gen_campaigns ON lead_gen_campaigns;
ALTER TABLE IF EXISTS lead_gen_campaigns DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_leads ON leads;
ALTER TABLE IF EXISTS leads DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_ledger_reserves ON ledger_reserves;
ALTER TABLE IF EXISTS ledger_reserves DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_locations ON locations;
ALTER TABLE IF EXISTS locations DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_loyalty_ledgers ON loyalty_ledgers;
ALTER TABLE IF EXISTS loyalty_ledgers DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_mcp_config_sync_log ON mcp_config_sync_log;
ALTER TABLE IF EXISTS mcp_config_sync_log DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_multi_party_split_ledgers ON multi_party_split_ledgers;
ALTER TABLE IF EXISTS multi_party_split_ledgers DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_multi_party_splits ON multi_party_splits;
ALTER TABLE IF EXISTS multi_party_splits DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_ohc_collective ON ohc_collective;
ALTER TABLE IF EXISTS ohc_collective DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_ohc_collective_loyalty_balance ON ohc_collective_loyalty_balance;
ALTER TABLE IF EXISTS ohc_collective_loyalty_balance DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_ohc_collective_member ON ohc_collective_member;
ALTER TABLE IF EXISTS ohc_collective_member DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_ohc_job_queue ON ohc_job_queue;
ALTER TABLE IF EXISTS ohc_job_queue DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_ohc_shared_offer ON ohc_shared_offer;
ALTER TABLE IF EXISTS ohc_shared_offer DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_ohc_staff_member ON ohc_staff_member;
ALTER TABLE IF EXISTS ohc_staff_member DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_ohc_timecard_event ON ohc_timecard_event;
ALTER TABLE IF EXISTS ohc_timecard_event DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_ohc_universal_ledger ON ohc_universal_ledger;
ALTER TABLE IF EXISTS ohc_universal_ledger DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_omni_inbox_messages ON omni_inbox_messages;
ALTER TABLE IF EXISTS omni_inbox_messages DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_onboarding_state ON onboarding_state;
ALTER TABLE IF EXISTS onboarding_state DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_operation_intents ON operation_intents;
ALTER TABLE IF EXISTS operation_intents DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_opportunities ON opportunities;
ALTER TABLE IF EXISTS opportunities DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_pos_offline_transactions ON pos_offline_transactions;
ALTER TABLE IF EXISTS pos_offline_transactions DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_pos_terminal_sessions ON pos_terminal_sessions;
ALTER TABLE IF EXISTS pos_terminal_sessions DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_pre_order_entries ON pre_order_entries;
ALTER TABLE IF EXISTS pre_order_entries DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_price_history ON price_history;
ALTER TABLE IF EXISTS price_history DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_pricing_heuristics ON pricing_heuristics;
ALTER TABLE IF EXISTS pricing_heuristics DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_pricing_rules ON pricing_rules;
ALTER TABLE IF EXISTS pricing_rules DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_project_tasks ON project_tasks;
ALTER TABLE IF EXISTS project_tasks DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_projects ON projects;
ALTER TABLE IF EXISTS projects DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_proposals ON proposals;
ALTER TABLE IF EXISTS proposals DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_proposed_bookings ON proposed_bookings;
ALTER TABLE IF EXISTS proposed_bookings DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_purchase_orders ON purchase_orders;
ALTER TABLE IF EXISTS purchase_orders DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_quote_requests ON quote_requests;
ALTER TABLE IF EXISTS quote_requests DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_quotes ON quotes;
ALTER TABLE IF EXISTS quotes DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_recovery_attempts ON recovery_attempts;
ALTER TABLE IF EXISTS recovery_attempts DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_recovery_campaigns ON recovery_campaigns;
ALTER TABLE IF EXISTS recovery_campaigns DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_referrals ON referrals;
ALTER TABLE IF EXISTS referrals DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_reward_claims ON reward_claims;
ALTER TABLE IF EXISTS reward_claims DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_role_assignments ON role_assignments;
ALTER TABLE IF EXISTS role_assignments DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_seo_discovery_reports ON seo_discovery_reports;
ALTER TABLE IF EXISTS seo_discovery_reports DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_service_items ON service_items;
ALTER TABLE IF EXISTS service_items DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_service_leads ON service_leads;
ALTER TABLE IF EXISTS service_leads DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_service_resource_requirements ON service_resource_requirements;
ALTER TABLE IF EXISTS service_resource_requirements DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_service_routes ON service_routes;
ALTER TABLE IF EXISTS service_routes DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_services ON services;
ALTER TABLE IF EXISTS services DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_shift_swap_requests ON shift_swap_requests;
ALTER TABLE IF EXISTS shift_swap_requests DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_shifts ON shifts;
ALTER TABLE IF EXISTS shifts DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_smart_pricing_policies ON smart_pricing_policies;
ALTER TABLE IF EXISTS smart_pricing_policies DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_staff_availability ON staff_availability;
ALTER TABLE IF EXISTS staff_availability DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_staff_profiles ON staff_profiles;
ALTER TABLE IF EXISTS staff_profiles DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_subscribers ON subscribers;
ALTER TABLE IF EXISTS subscribers DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_subscription_plans ON subscription_plans;
ALTER TABLE IF EXISTS subscription_plans DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_subscriptions ON subscriptions;
ALTER TABLE IF EXISTS subscriptions DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_swarm_tasks ON swarm_tasks;
ALTER TABLE IF EXISTS swarm_tasks DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_sync_events ON sync_events;
ALTER TABLE IF EXISTS sync_events DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_task_dependencies ON task_dependencies;
ALTER TABLE IF EXISTS task_dependencies DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_team_invites ON team_invites;
ALTER TABLE IF EXISTS team_invites DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_telemetry_buffer ON telemetry_buffer;
ALTER TABLE IF EXISTS telemetry_buffer DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_tenant_feed_items ON tenant_feed_items;
ALTER TABLE IF EXISTS tenant_feed_items DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_tool_integrations ON tool_integrations;
ALTER TABLE IF EXISTS tool_integrations DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_tooltips ON tooltips;
ALTER TABLE IF EXISTS tooltips DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_unified_messages ON unified_messages;
ALTER TABLE IF EXISTS unified_messages DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_unified_threads ON unified_threads;
ALTER TABLE IF EXISTS unified_threads DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_unified_triage_actions ON unified_triage_actions;
ALTER TABLE IF EXISTS unified_triage_actions DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_vendors ON vendors;
ALTER TABLE IF EXISTS vendors DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_video_tutorials ON video_tutorials;
ALTER TABLE IF EXISTS video_tutorials DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_waitlist_campaigns ON waitlist_campaigns;
ALTER TABLE IF EXISTS waitlist_campaigns DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_walkthrough_steps ON walkthrough_steps;
ALTER TABLE IF EXISTS walkthrough_steps DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_work_intents ON work_intents;
ALTER TABLE IF EXISTS work_intents DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_work_item ON work_item;
ALTER TABLE IF EXISTS work_item DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_work_tasks ON work_tasks;
ALTER TABLE IF EXISTS work_tasks DISABLE ROW LEVEL SECURITY;
