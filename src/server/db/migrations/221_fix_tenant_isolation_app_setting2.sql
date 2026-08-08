-- +goose Up

-- Fix staff_shifts RLS policy
DROP POLICY IF EXISTS staff_shifts_tenant_isolation ON staff_shifts;
CREATE POLICY staff_shifts_tenant_isolation ON staff_shifts FOR ALL
    USING (tenant_id = current_setting('app.current_tenant', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true)::uuid);

-- Fix staff_members RLS policy
DROP POLICY IF EXISTS staff_members_tenant_isolation ON staff_members;
CREATE POLICY staff_members_tenant_isolation ON staff_members FOR ALL
    USING (tenant_id = current_setting('app.current_tenant', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true)::uuid);

-- Fix staff_tasks RLS policy
DROP POLICY IF EXISTS staff_tasks_tenant_isolation ON staff_tasks;
CREATE POLICY staff_tasks_tenant_isolation ON staff_tasks FOR ALL
    USING (tenant_id = current_setting('app.current_tenant', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true)::uuid);

-- Fix staff_task_assignments RLS policy
DROP POLICY IF EXISTS staff_task_assignments_tenant_isolation ON staff_task_assignments;
CREATE POLICY staff_task_assignments_tenant_isolation ON staff_task_assignments FOR ALL
    USING (tenant_id = current_setting('app.current_tenant', true)::uuid)
    WITH CHECK (tenant_id = current_setting('app.current_tenant', true)::uuid);

-- Fix service_bookings RLS policy
DROP POLICY IF EXISTS "Tenant isolation for service_bookings select" ON service_bookings;
CREATE POLICY "Tenant isolation for service_bookings select" ON service_bookings FOR SELECT
    USING (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS "Tenant isolation for service_bookings insert" ON service_bookings;
CREATE POLICY "Tenant isolation for service_bookings insert" ON service_bookings FOR INSERT
    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS "Tenant isolation for service_bookings update" ON service_bookings;
CREATE POLICY "Tenant isolation for service_bookings update" ON service_bookings FOR UPDATE
    USING (tenant_id::text = current_setting('app.current_tenant', true));

DROP POLICY IF EXISTS "Tenant isolation for service_bookings delete" ON service_bookings;
CREATE POLICY "Tenant isolation for service_bookings delete" ON service_bookings FOR DELETE
    USING (tenant_id::text = current_setting('app.current_tenant', true));


-- +goose Down
-- Intentionally blank
