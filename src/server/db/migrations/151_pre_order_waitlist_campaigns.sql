-- Migration: Pre-Order Waitlist Campaigns
-- Description: Creates the waitlist campaigns and entries tables for viral omnichannel pre-order functionality.

CREATE TABLE IF NOT EXISTS waitlist_campaigns (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    product_id UUID,
    name TEXT NOT NULL,
    offer_text TEXT,
    theme TEXT DEFAULT 'light',
    status TEXT NOT NULL DEFAULT 'ACTIVE',
    capacity_limit INTEGER,
    deposit_required BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE waitlist_campaigns ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Tenant isolation for waitlist_campaigns select"
    ON waitlist_campaigns FOR SELECT
    USING (tenant_id = current_setting('app.current_tenant', true)::uuid);

CREATE POLICY "Tenant isolation for waitlist_campaigns insert"
    ON waitlist_campaigns FOR INSERT
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true)::uuid);

CREATE POLICY "Tenant isolation for waitlist_campaigns update"
    ON waitlist_campaigns FOR UPDATE
    USING (tenant_id = current_setting('app.current_tenant', true)::uuid);

CREATE POLICY "Tenant isolation for waitlist_campaigns delete"
    ON waitlist_campaigns FOR DELETE
    USING (tenant_id = current_setting('app.current_tenant', true)::uuid);

CREATE TABLE IF NOT EXISTS pre_order_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    waitlist_campaign_id UUID NOT NULL REFERENCES waitlist_campaigns(id) ON DELETE CASCADE,
    customer_id UUID,
    email TEXT NOT NULL,
    channel TEXT NOT NULL DEFAULT 'WEB',
    status TEXT NOT NULL DEFAULT 'PENDING',
    deposit_amount DECIMAL(10, 2),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE pre_order_entries ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Tenant isolation for pre_order_entries select"
    ON pre_order_entries FOR SELECT
    USING (tenant_id = current_setting('app.current_tenant', true)::uuid);

CREATE POLICY "Tenant isolation for pre_order_entries insert"
    ON pre_order_entries FOR INSERT
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true)::uuid);

CREATE POLICY "Tenant isolation for pre_order_entries update"
    ON pre_order_entries FOR UPDATE
    USING (tenant_id = current_setting('app.current_tenant', true)::uuid);

CREATE POLICY "Tenant isolation for pre_order_entries delete"
    ON pre_order_entries FOR DELETE
    USING (tenant_id = current_setting('app.current_tenant', true)::uuid);

-- Triggers for updated_at
CREATE TRIGGER update_waitlist_campaigns_updated_at
    BEFORE UPDATE ON waitlist_campaigns
    FOR EACH ROW
    EXECUTE FUNCTION update_modified_column();

CREATE TRIGGER update_pre_order_entries_updated_at
    BEFORE UPDATE ON pre_order_entries
    FOR EACH ROW
    EXECUTE FUNCTION update_modified_column();
