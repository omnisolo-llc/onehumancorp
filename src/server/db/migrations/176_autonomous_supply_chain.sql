-- Create table for Bill of Materials (BOM) linking finished goods to raw materials
CREATE TABLE IF NOT EXISTS inventory_bom_ledger (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    finished_good_id UUID NOT NULL,
    raw_material_id UUID NOT NULL,
    quantity_required NUMERIC NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Enable Row Level Security
ALTER TABLE inventory_bom_ledger ENABLE ROW LEVEL SECURITY;

-- Add RLS policies for tenant isolation
CREATE POLICY "inventory_bom_ledger_tenant_isolation" ON inventory_bom_ledger
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id')::uuid)
    WITH CHECK (tenant_id = current_setting('app.current_tenant_id')::uuid);

-- Create table for Supplier Directory
CREATE TABLE IF NOT EXISTS supplier_directory (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    contact_email VARCHAR(255),
    contact_phone VARCHAR(50),
    is_local BOOLEAN DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE supplier_directory ENABLE ROW LEVEL SECURITY;

CREATE POLICY "supplier_directory_tenant_isolation" ON supplier_directory
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id')::uuid)
    WITH CHECK (tenant_id = current_setting('app.current_tenant_id')::uuid);

-- Create table for Purchase Orders (Procurement)
CREATE TABLE IF NOT EXISTS purchase_orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    supplier_id UUID NOT NULL REFERENCES supplier_directory(id),
    total_amount NUMERIC NOT NULL,
    status VARCHAR(50) DEFAULT 'draft', -- draft, pending_approval, approved, ordered, received
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE purchase_orders ENABLE ROW LEVEL SECURITY;

CREATE POLICY "purchase_orders_tenant_isolation" ON purchase_orders
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant_id')::uuid)
    WITH CHECK (tenant_id = current_setting('app.current_tenant_id')::uuid);
