-- +goose Up

-- 1. booking_resources
ALTER TABLE IF EXISTS booking_resources ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_booking_resources ON booking_resources;
CREATE POLICY tenant_isolation_booking_resources ON booking_resources USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 2. conversational_intakes
ALTER TABLE IF EXISTS conversational_intakes ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_conversational_intakes ON conversational_intakes;
CREATE POLICY tenant_isolation_conversational_intakes ON conversational_intakes USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 3. customer_identities
ALTER TABLE IF EXISTS customer_identities ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_customer_identities ON customer_identities;
CREATE POLICY tenant_isolation_customer_identities ON customer_identities USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 4. help_articles
ALTER TABLE IF EXISTS help_articles ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_help_articles ON help_articles;
CREATE POLICY tenant_isolation_help_articles ON help_articles USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 5. interactive_proposal_line_items
ALTER TABLE IF EXISTS interactive_proposal_line_items ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_interactive_proposal_line_items ON interactive_proposal_line_items;
CREATE POLICY tenant_isolation_interactive_proposal_line_items ON interactive_proposal_line_items USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 6. pos_offline_transactions
ALTER TABLE IF EXISTS pos_offline_transactions ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_pos_offline_transactions ON pos_offline_transactions;
CREATE POLICY tenant_isolation_pos_offline_transactions ON pos_offline_transactions USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 7. pre_order_entries
ALTER TABLE IF EXISTS pre_order_entries ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_pre_order_entries ON pre_order_entries;
CREATE POLICY tenant_isolation_pre_order_entries ON pre_order_entries USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 8. referrals
ALTER TABLE IF EXISTS referrals ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_referrals ON referrals;
CREATE POLICY tenant_isolation_referrals ON referrals USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 9. service_resource_requirements
ALTER TABLE IF EXISTS service_resource_requirements ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_service_resource_requirements ON service_resource_requirements;
CREATE POLICY tenant_isolation_service_resource_requirements ON service_resource_requirements USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 10. tooltips
ALTER TABLE IF EXISTS tooltips ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_tooltips ON tooltips;
CREATE POLICY tenant_isolation_tooltips ON tooltips USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 11. video_tutorials
ALTER TABLE IF EXISTS video_tutorials ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_video_tutorials ON video_tutorials;
CREATE POLICY tenant_isolation_video_tutorials ON video_tutorials USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 12. waitlist_campaigns
ALTER TABLE IF EXISTS waitlist_campaigns ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_waitlist_campaigns ON waitlist_campaigns;
CREATE POLICY tenant_isolation_waitlist_campaigns ON waitlist_campaigns USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- 13. walkthrough_steps
ALTER TABLE IF EXISTS walkthrough_steps ENABLE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS tenant_isolation_walkthrough_steps ON walkthrough_steps;
CREATE POLICY tenant_isolation_walkthrough_steps ON walkthrough_steps USING (tenant_id = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id = current_setting('app.current_tenant', true));

-- +goose Down
-- Revert RLS
DROP POLICY IF EXISTS tenant_isolation_booking_resources ON booking_resources;
ALTER TABLE IF EXISTS booking_resources DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_conversational_intakes ON conversational_intakes;
ALTER TABLE IF EXISTS conversational_intakes DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_customer_identities ON customer_identities;
ALTER TABLE IF EXISTS customer_identities DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_help_articles ON help_articles;
ALTER TABLE IF EXISTS help_articles DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_interactive_proposal_line_items ON interactive_proposal_line_items;
ALTER TABLE IF EXISTS interactive_proposal_line_items DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_pos_offline_transactions ON pos_offline_transactions;
ALTER TABLE IF EXISTS pos_offline_transactions DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_pre_order_entries ON pre_order_entries;
ALTER TABLE IF EXISTS pre_order_entries DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_referrals ON referrals;
ALTER TABLE IF EXISTS referrals DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_service_resource_requirements ON service_resource_requirements;
ALTER TABLE IF EXISTS service_resource_requirements DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_tooltips ON tooltips;
ALTER TABLE IF EXISTS tooltips DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_video_tutorials ON video_tutorials;
ALTER TABLE IF EXISTS video_tutorials DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_waitlist_campaigns ON waitlist_campaigns;
ALTER TABLE IF EXISTS waitlist_campaigns DISABLE ROW LEVEL SECURITY;

DROP POLICY IF EXISTS tenant_isolation_walkthrough_steps ON walkthrough_steps;
ALTER TABLE IF EXISTS walkthrough_steps DISABLE ROW LEVEL SECURITY;
