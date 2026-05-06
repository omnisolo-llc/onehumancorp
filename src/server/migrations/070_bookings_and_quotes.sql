CREATE TABLE IF NOT EXISTS quotes (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    amount_cents BIGINT NOT NULL,
    description TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at_unix BIGINT NOT NULL
);

CREATE TABLE IF NOT EXISTS bookings (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    customer_id TEXT NOT NULL,
    quote_id TEXT,
    start_time_unix BIGINT NOT NULL,
    end_time_unix BIGINT NOT NULL,
    status TEXT NOT NULL,
    payment_link TEXT
);

ALTER TABLE quotes ENABLE ROW LEVEL SECURITY;
ALTER TABLE bookings ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation_quotes ON quotes USING (organization_id::text = current_setting('app.current_tenant', true));
CREATE POLICY tenant_isolation_bookings ON bookings USING (organization_id::text = current_setting('app.current_tenant', true));
