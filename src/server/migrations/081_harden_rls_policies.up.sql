DROP POLICY IF EXISTS tenant_isolation_products ON products;
CREATE POLICY tenant_isolation_products ON products
    USING (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_services ON services;
CREATE POLICY tenant_isolation_services ON services
    USING (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_customers ON customers;
CREATE POLICY tenant_isolation_customers ON customers
    USING (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_orders ON orders;
CREATE POLICY tenant_isolation_orders ON orders
    USING (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_order_items ON order_items;
CREATE POLICY tenant_isolation_order_items ON order_items
    USING (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_bookings ON bookings;
CREATE POLICY tenant_isolation_bookings ON bookings
    USING (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS tenant_isolation_agent_memories ON agent_memories;
CREATE POLICY tenant_isolation_agent_memories ON agent_memories
    USING (tenant_id::text = current_setting('app.current_tenant', true));
