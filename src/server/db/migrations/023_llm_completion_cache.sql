CREATE TABLE IF NOT EXISTS llm_completion_cache (
    request_hash VARCHAR(64) PRIMARY KEY,
    response_payload BYTEA NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
