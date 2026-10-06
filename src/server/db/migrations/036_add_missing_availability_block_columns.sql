ALTER TABLE availability_blocks
ADD COLUMN IF NOT EXISTS resource_id TEXT,
ADD COLUMN IF NOT EXISTS is_recurring BOOLEAN DEFAULT FALSE,
ADD COLUMN IF NOT EXISTS recurrence_rule TEXT;

ALTER TABLE availability_blocks
DROP CONSTRAINT IF EXISTS availability_blocks_resource_id_fkey;

ALTER TABLE availability_blocks
ADD CONSTRAINT availability_blocks_resource_id_fkey
FOREIGN KEY (resource_id) REFERENCES booking_resources(id) ON DELETE CASCADE;

-- Also add an index for resource queries
CREATE INDEX IF NOT EXISTS idx_availability_blocks_tenant_resource_start
    ON availability_blocks (tenant_id, resource_id, start_time)
    WHERE is_available;
