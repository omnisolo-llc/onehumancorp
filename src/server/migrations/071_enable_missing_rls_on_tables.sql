-- Enable Row Level Security (RLS) on tables containing multi-tenant data

-- Found in 014_shared_tasks.sql
ALTER TABLE IF EXISTS shared_tasks ENABLE ROW LEVEL SECURITY;
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE schemaname = current_schema()
            AND tablename = 'shared_tasks'
            AND policyname = 'tenant_isolation_shared_tasks'
    ) THEN
        CREATE POLICY "tenant_isolation_shared_tasks" ON shared_tasks FOR ALL USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- Found in 017_business_milestones.sql
ALTER TABLE IF EXISTS business_milestones ENABLE ROW LEVEL SECURITY;
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE schemaname = current_schema()
            AND tablename = 'business_milestones'
            AND policyname = 'tenant_isolation_business_milestones'
    ) THEN
        CREATE POLICY "tenant_isolation_business_milestones" ON business_milestones FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- Found in 058_shared_tasks_decomposition_table.sql
ALTER TABLE IF EXISTS shared_tasks_decomposition ENABLE ROW LEVEL SECURITY;
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE schemaname = current_schema()
            AND tablename = 'shared_tasks_decomposition'
            AND policyname = 'tenant_isolation_shared_tasks_decomposition'
    ) THEN
        CREATE POLICY "tenant_isolation_shared_tasks_decomposition" ON shared_tasks_decomposition FOR ALL USING (organization_id::text = current_setting('app.current_tenant', true)) WITH CHECK (organization_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- Found in 008_data_model_architecture.sql
ALTER TABLE IF EXISTS customers ENABLE ROW LEVEL SECURITY;
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE schemaname = current_schema()
            AND tablename = 'customers'
            AND policyname = 'tenant_isolation_customers'
    ) THEN
        CREATE POLICY "tenant_isolation_customers" ON customers FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

ALTER TABLE IF EXISTS products ENABLE ROW LEVEL SECURITY;
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE schemaname = current_schema()
            AND tablename = 'products'
            AND policyname = 'tenant_isolation_products'
    ) THEN
        CREATE POLICY "tenant_isolation_products" ON products FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

ALTER TABLE IF EXISTS services ENABLE ROW LEVEL SECURITY;
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE schemaname = current_schema()
            AND tablename = 'services'
            AND policyname = 'tenant_isolation_services'
    ) THEN
        CREATE POLICY "tenant_isolation_services" ON services FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

ALTER TABLE IF EXISTS orders ENABLE ROW LEVEL SECURITY;
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE schemaname = current_schema()
            AND tablename = 'orders'
            AND policyname = 'tenant_isolation_orders'
    ) THEN
        CREATE POLICY "tenant_isolation_orders" ON orders FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

ALTER TABLE IF EXISTS order_line_items ENABLE ROW LEVEL SECURITY;
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE schemaname = current_schema()
            AND tablename = 'order_line_items'
            AND policyname = 'tenant_isolation_order_line_items'
    ) THEN
        CREATE POLICY "tenant_isolation_order_line_items" ON order_line_items FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

ALTER TABLE IF EXISTS bookings ENABLE ROW LEVEL SECURITY;
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE schemaname = current_schema()
            AND tablename = 'bookings'
            AND policyname = 'tenant_isolation_bookings'
    ) THEN
        CREATE POLICY "tenant_isolation_bookings" ON bookings FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

ALTER TABLE IF EXISTS ai_memories ENABLE ROW LEVEL SECURITY;
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE schemaname = current_schema()
            AND tablename = 'ai_memories'
            AND policyname = 'tenant_isolation_ai_memories'
    ) THEN
        CREATE POLICY "tenant_isolation_ai_memories" ON ai_memories FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

-- Found in 022_supply_chain.sql
ALTER TABLE IF EXISTS vendors ENABLE ROW LEVEL SECURITY;
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE schemaname = current_schema()
            AND tablename = 'vendors'
            AND policyname = 'tenant_isolation_vendors'
    ) THEN
        CREATE POLICY "tenant_isolation_vendors" ON vendors FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

ALTER TABLE IF EXISTS raw_materials ENABLE ROW LEVEL SECURITY;
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE schemaname = current_schema()
            AND tablename = 'raw_materials'
            AND policyname = 'tenant_isolation_raw_materials'
    ) THEN
        CREATE POLICY "tenant_isolation_raw_materials" ON raw_materials FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

ALTER TABLE IF EXISTS bom_items ENABLE ROW LEVEL SECURITY;
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE schemaname = current_schema()
            AND tablename = 'bom_items'
            AND policyname = 'tenant_isolation_bom_items'
    ) THEN
        CREATE POLICY "tenant_isolation_bom_items" ON bom_items FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

ALTER TABLE IF EXISTS purchase_orders ENABLE ROW LEVEL SECURITY;
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE schemaname = current_schema()
            AND tablename = 'purchase_orders'
            AND policyname = 'tenant_isolation_purchase_orders'
    ) THEN
        CREATE POLICY "tenant_isolation_purchase_orders" ON purchase_orders FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

ALTER TABLE IF EXISTS po_line_items ENABLE ROW LEVEL SECURITY;
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE schemaname = current_schema()
            AND tablename = 'po_line_items'
            AND policyname = 'tenant_isolation_po_line_items'
    ) THEN
        CREATE POLICY "tenant_isolation_po_line_items" ON po_line_items FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;

ALTER TABLE IF EXISTS depletion_logs ENABLE ROW LEVEL SECURITY;
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_policies
        WHERE schemaname = current_schema()
            AND tablename = 'depletion_logs'
            AND policyname = 'tenant_isolation_depletion_logs'
    ) THEN
        CREATE POLICY "tenant_isolation_depletion_logs" ON depletion_logs FOR ALL USING (tenant_id::text = current_setting('app.current_tenant', true)) WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
    END IF;
END
$$;
