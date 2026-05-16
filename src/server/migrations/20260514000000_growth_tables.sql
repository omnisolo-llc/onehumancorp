-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS growth_events (
    event_id VARCHAR(255) PRIMARY KEY,
    tenant_id VARCHAR(255) NOT NULL,
    event_type VARCHAR(255) NOT NULL,
    entity_id VARCHAR(255) NOT NULL,
    metadata JSONB NOT NULL,
    timestamp BIGINT NOT NULL
);

CREATE TABLE IF NOT EXISTS referral_programs (
    id VARCHAR(255) PRIMARY KEY,
    tenant_id VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    reward_type VARCHAR(255) NOT NULL,
    reward_value DOUBLE PRECISION NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at BIGINT NOT NULL
);

CREATE TABLE IF NOT EXISTS notification_prefs (
    user_id VARCHAR(255) PRIMARY KEY,
    tenant_id VARCHAR(255) NOT NULL,
    notify_on_click BOOLEAN NOT NULL DEFAULT false,
    notify_on_convert BOOLEAN NOT NULL DEFAULT true,
    notify_on_invite_accept BOOLEAN NOT NULL DEFAULT true
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS growth_events;
DROP TABLE IF EXISTS referral_programs;
DROP TABLE IF EXISTS notification_prefs;
-- +goose StatementEnd
