CREATE INDEX IF NOT EXISTS idx_agent_feed_items_tenant_state ON agent_feed_items(tenant_id, lifecycle_state);
CREATE INDEX IF NOT EXISTS idx_agent_feed_items_created_at ON agent_feed_items(tenant_id, created_at DESC);
