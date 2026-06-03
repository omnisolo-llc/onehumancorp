-- +goose Up
-- +goose StatementBegin
CREATE TABLE events (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    start_time TIMESTAMP WITH TIME ZONE NOT NULL,
    end_time TIMESTAMP WITH TIME ZONE NOT NULL,
    location VARCHAR(255),
    capacity INT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE tickets (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    event_id UUID NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    customer_id UUID NOT NULL,
    qr_code_jwt TEXT NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'VALID',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE attendee_check_ins (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL,
    event_id UUID NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    ticket_id UUID NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
    check_in_time TIMESTAMP WITH TIME ZONE NOT NULL,
    is_offline_sync BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_events_tenant_id ON events(tenant_id);
CREATE INDEX idx_tickets_tenant_event ON tickets(tenant_id, event_id);
CREATE INDEX idx_tickets_jwt ON tickets(qr_code_jwt);
CREATE INDEX idx_check_ins_tenant_event ON attendee_check_ins(tenant_id, event_id);

-- Enable RLS for all tables
ALTER TABLE events ENABLE ROW LEVEL SECURITY;
ALTER TABLE tickets ENABLE ROW LEVEL SECURITY;
ALTER TABLE attendee_check_ins ENABLE ROW LEVEL SECURITY;

-- Create RLS policies
CREATE POLICY events_tenant_policy ON events
    FOR ALL USING (tenant_id = current_setting('app.current_tenant')::uuid);

CREATE POLICY tickets_tenant_policy ON tickets
    FOR ALL USING (tenant_id = current_setting('app.current_tenant')::uuid);

CREATE POLICY check_ins_tenant_policy ON attendee_check_ins
    FOR ALL USING (tenant_id = current_setting('app.current_tenant')::uuid);

-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS attendee_check_ins CASCADE;
DROP TABLE IF EXISTS tickets CASCADE;
DROP TABLE IF EXISTS events CASCADE;
-- +goose StatementEnd
