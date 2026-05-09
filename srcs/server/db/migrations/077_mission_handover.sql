-- +goose Up
UPDATE agent_missions
SET status = 'blocked',
    mission_log = COALESCE(mission_log, '') || '
' || 'I cannot finish an OHC product mission. Queue is empty and PostgreSQL container failed to start.'
WHERE id = 'm_handoff';

-- +goose Down
