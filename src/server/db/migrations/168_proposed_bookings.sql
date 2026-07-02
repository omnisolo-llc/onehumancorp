CREATE TABLE IF NOT EXISTS proposed_bookings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    customer_id UUID NOT NULL,
    conversation_id UUID NOT NULL,
    requested_service TEXT NOT NULL,
    proposed_time TEXT NOT NULL,
    estimated_value DOUBLE PRECISION NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE proposed_bookings ENABLE ROW LEVEL SECURITY;
CREATE POLICY proposed_bookings_tenant_isolation ON proposed_bookings
    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);

CREATE TABLE IF NOT EXISTS work_tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL,
    booking_id UUID NOT NULL,
    description TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'open',
    scheduled_time TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE work_tasks ENABLE ROW LEVEL SECURITY;
CREATE POLICY work_tasks_tenant_isolation ON work_tasks
    USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
