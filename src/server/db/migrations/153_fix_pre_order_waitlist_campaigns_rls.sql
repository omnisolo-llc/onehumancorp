-- +goose Up
-- Fix incorrect RLS policy and column type for waitlist_campaigns and pre_order_entries

-- Fix waitlist_campaigns RLS policy (was using app.current_tenant_id which does not exist)
DROP POLICY IF EXISTS "Tenant isolation for waitlist_campaigns select" ON waitlist_campaigns;
DROP POLICY IF EXISTS "Tenant isolation for waitlist_campaigns insert" ON waitlist_campaigns;
DROP POLICY IF EXISTS "Tenant isolation for waitlist_campaigns update" ON waitlist_campaigns;
DROP POLICY IF EXISTS "Tenant isolation for waitlist_campaigns delete" ON waitlist_campaigns;

CREATE POLICY "Tenant isolation for waitlist_campaigns select"
    ON waitlist_campaigns FOR SELECT
    USING (tenant_id::text = current_setting('app.current_tenant', true));

CREATE POLICY "Tenant isolation for waitlist_campaigns insert"
    ON waitlist_campaigns FOR INSERT
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE POLICY "Tenant isolation for waitlist_campaigns update"
    ON waitlist_campaigns FOR UPDATE
    USING (tenant_id::text = current_setting('app.current_tenant', true));

CREATE POLICY "Tenant isolation for waitlist_campaigns delete"
    ON waitlist_campaigns FOR DELETE
    USING (tenant_id::text = current_setting('app.current_tenant', true));

-- Fix pre_order_entries RLS policy
DROP POLICY IF EXISTS "Tenant isolation for pre_order_entries select" ON pre_order_entries;
DROP POLICY IF EXISTS "Tenant isolation for pre_order_entries insert" ON pre_order_entries;
DROP POLICY IF EXISTS "Tenant isolation for pre_order_entries update" ON pre_order_entries;
DROP POLICY IF EXISTS "Tenant isolation for pre_order_entries delete" ON pre_order_entries;

CREATE POLICY "Tenant isolation for pre_order_entries select"
    ON pre_order_entries FOR SELECT
    USING (tenant_id::text = current_setting('app.current_tenant', true));

CREATE POLICY "Tenant isolation for pre_order_entries insert"
    ON pre_order_entries FOR INSERT
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

CREATE POLICY "Tenant isolation for pre_order_entries update"
    ON pre_order_entries FOR UPDATE
    USING (tenant_id::text = current_setting('app.current_tenant', true));

CREATE POLICY "Tenant isolation for pre_order_entries delete"
    ON pre_order_entries FOR DELETE
    USING (tenant_id::text = current_setting('app.current_tenant', true));

ALTER TABLE waitlist_campaigns ENABLE ROW LEVEL SECURITY;
ALTER TABLE pre_order_entries ENABLE ROW LEVEL SECURITY;

-- Change column type for tenant_id from UUID to TEXT to be consistent with rest of DB
ALTER TABLE waitlist_campaigns ALTER COLUMN tenant_id TYPE TEXT;
ALTER TABLE pre_order_entries ALTER COLUMN tenant_id TYPE TEXT;

-- +goose Down
-- Reverting RLS changes
