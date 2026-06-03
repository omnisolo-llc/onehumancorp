-- Enable DoorDash settings per organization
CREATE TABLE IF NOT EXISTS doordash_delivery_settings (
    organization_id TEXT PRIMARY KEY,
    enabled BOOLEAN DEFAULT FALSE,
    radius_miles DOUBLE PRECISION DEFAULT 5.0,
    flat_fee_cents INTEGER DEFAULT 850,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_doordash_settings_modtime
    BEFORE UPDATE ON doordash_delivery_settings
    FOR EACH ROW
    EXECUTE FUNCTION update_modified_column();
