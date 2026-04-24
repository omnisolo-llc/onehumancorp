-- +goose Up
-- +goose StatementBegin
ALTER TABLE shared_tasks ADD COLUMN action_risk TEXT NOT NULL DEFAULT 'ACTION_RISK_UNSPECIFIED';
ALTER TABLE shared_tasks_decomposition ADD COLUMN action_risk TEXT NOT NULL DEFAULT 'ACTION_RISK_UNSPECIFIED';
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE shared_tasks DROP COLUMN action_risk;
ALTER TABLE shared_tasks_decomposition DROP COLUMN action_risk;
-- +goose StatementEnd
