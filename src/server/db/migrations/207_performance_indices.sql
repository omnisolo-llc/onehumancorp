-- +goose Up

CREATE INDEX IF NOT EXISTS idx_daily_work_items_tenant_status_created_at ON daily_work_items(tenant_id, status, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_orders_tenant_created_at ON orders(tenant_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_agent_feed_items_tenant ON agent_feed_items(tenant_id);
CREATE INDEX IF NOT EXISTS idx_agent_action_requests_tenant ON agent_action_requests(tenant_id);

-- +goose Down
DROP INDEX IF EXISTS idx_daily_work_items_tenant_status_created_at;
DROP INDEX IF EXISTS idx_orders_tenant_created_at;
DROP INDEX IF EXISTS idx_agent_feed_items_tenant;
DROP INDEX IF EXISTS idx_agent_action_requests_tenant;
