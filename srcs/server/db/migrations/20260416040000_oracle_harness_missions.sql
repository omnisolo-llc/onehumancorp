-- +goose Up
-- +goose StatementBegin
INSERT INTO agent_missions (id, status, payload, created_at)
VALUES (
    'mission_oracle_harness_1',
    'PENDING',
    '{"title": "🔮 Oracle: [harness] Implement Claude-Class Agent Sandbox Manager for KAIROS", "description": "Based on market research, implement a SandboxManager module to isolate and wrap execution of shell commands, matching Claude Code''s capabilities.", "priority": "P0", "domain": "harness"}',
    CURRENT_TIMESTAMP
);

INSERT INTO agent_missions (id, status, payload, created_at)
VALUES (
    'mission_oracle_harness_2',
    'PENDING',
    '{"title": "🔮 Oracle: [research] Implement Durable State Sync with pgvector for AutoDream", "description": "Enhance AutoDream to process and sync architectural findings directly into a pgvector enabled database for durable state synchronization.", "priority": "P1", "domain": "backend"}',
    CURRENT_TIMESTAMP
);

INSERT INTO agent_missions (id, status, payload, created_at)
VALUES (
    'mission_oracle_harness_3',
    'PENDING',
    '{"title": "🔮 Oracle: [telemetry] Implement KAIROS Harness Telemetry and I/O Instrumentation", "description": "Add OpenTelemetry hooks to track execution time and I/O byte counters per tenant for commands executed inside the Agent Harness.", "priority": "P1", "domain": "telemetry"}',
    CURRENT_TIMESTAMP
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DELETE FROM agent_missions WHERE id IN (
    'mission_oracle_harness_1',
    'mission_oracle_harness_2',
    'mission_oracle_harness_3'
);
-- +goose StatementEnd
