-- +goose Up
-- +goose StatementBegin
CREATE TABLE IF NOT EXISTS ultraplan_proposals (
    id TEXT PRIMARY KEY,
    plan_id TEXT NOT NULL,
    status TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS ultraplan_votes (
    plan_id TEXT NOT NULL,
    agent_id TEXT NOT NULL,
    vote TEXT NOT NULL
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE IF EXISTS ultraplan_votes;
DROP TABLE IF EXISTS ultraplan_proposals;
-- +goose StatementEnd
