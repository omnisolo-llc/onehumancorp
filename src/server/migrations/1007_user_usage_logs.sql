CREATE TABLE IF NOT EXISTS user_usage_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    organization_id VARCHAR(128) NOT NULL,
    feature VARCHAR(128) NOT NULL,
    tokens_used INTEGER NOT NULL DEFAULT 0,
    computed_cost NUMERIC(10, 4) NOT NULL DEFAULT 0.0000,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_user_usage_logs_user ON user_usage_logs(user_id);
