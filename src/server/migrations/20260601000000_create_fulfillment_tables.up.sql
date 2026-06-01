CREATE TABLE IF NOT EXISTS couriers (
    id UUID PRIMARY KEY,
    tenant_id VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    method INT NOT NULL,
    contact_info VARCHAR(255) NOT NULL,
    cost_per_mile DOUBLE PRECISION NOT NULL,
    base_cost DOUBLE PRECISION NOT NULL,
    is_available BOOLEAN NOT NULL DEFAULT true
);

CREATE INDEX idx_couriers_tenant_id ON couriers (tenant_id);

CREATE TABLE IF NOT EXISTS fulfillment_orders (
    id UUID PRIMARY KEY,
    tenant_id VARCHAR(255) NOT NULL,
    order_id VARCHAR(255) NOT NULL,
    assigned_method INT NOT NULL,
    state INT NOT NULL,
    courier_id VARCHAR(255),
    origin_lat DOUBLE PRECISION,
    origin_lon DOUBLE PRECISION,
    origin_addr VARCHAR(255),
    dest_lat DOUBLE PRECISION,
    dest_lon DOUBLE PRECISION,
    dest_addr VARCHAR(255),
    estimated_prep_time_ms BIGINT NOT NULL,
    estimated_delivery_time_ms BIGINT NOT NULL,
    estimated_cost DOUBLE PRECISION NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE INDEX idx_fulfillment_orders_tenant_id ON fulfillment_orders (tenant_id);

-- Enable RLS
ALTER TABLE couriers ENABLE ROW LEVEL SECURITY;
ALTER TABLE fulfillment_orders ENABLE ROW LEVEL SECURITY;

-- Create policies for RLS
CREATE POLICY couriers_tenant_isolation_policy ON couriers
    USING (tenant_id = current_setting('app.current_tenant_id', true));

CREATE POLICY fulfillment_orders_tenant_isolation_policy ON fulfillment_orders
    USING (tenant_id = current_setting('app.current_tenant_id', true));
