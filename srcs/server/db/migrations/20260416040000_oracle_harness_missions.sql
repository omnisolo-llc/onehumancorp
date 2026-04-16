-- +goose Up
-- +goose StatementBegin
INSERT INTO agent_missions (title, description, status, priority, domain, created_at)
VALUES (
    '🔮 Oracle: [harness] Implement Claude-Class Agent Sandbox Manager for KAIROS',
    'Based on market research, implement a SandboxManager module to isolate and wrap execution of shell commands, matching Claude Code''s capabilities.',
    'PENDING',
    'P0',
    'harness',
    CURRENT_TIMESTAMP
);

INSERT INTO agent_missions (title, description, status, priority, domain, created_at)
VALUES (
    '🔮 Oracle: [research] Implement Durable State Sync with pgvector for AutoDream',
    'Enhance AutoDream to process and sync architectural findings directly into a pgvector enabled database for durable state synchronization.',
    'PENDING',
    'P1',
    'backend',
    CURRENT_TIMESTAMP
);

INSERT INTO agent_missions (title, description, status, priority, domain, created_at)
VALUES (
    '🔮 Oracle: [telemetry] Implement KAIROS Harness Telemetry and I/O Instrumentation',
    'Add OpenTelemetry hooks to track execution time and I/O byte counters per tenant for commands executed inside the Agent Harness.',
    'PENDING',
    'P1',
    'telemetry',
    CURRENT_TIMESTAMP
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DELETE FROM agent_missions WHERE title IN (
    '🔮 Oracle: [harness] Implement Claude-Class Agent Sandbox Manager for KAIROS',
    '🔮 Oracle: [research] Implement Durable State Sync with pgvector for AutoDream',
    '🔮 Oracle: [telemetry] Implement KAIROS Harness Telemetry and I/O Instrumentation'
);
-- +goose StatementEnd
