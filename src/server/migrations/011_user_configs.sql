CREATE TABLE IF NOT EXISTS user_configs (
    spiffe_id VARCHAR PRIMARY KEY,
    config_json TEXT NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    hash VARCHAR NOT NULL
);
