-- +goose Up
-- Migration 142: Fix pos offline transactions payload definition for items

DO $$
BEGIN
    -- No schema change strictly needed if pos_offline_transactions payload is already JSONB
    -- However, we can ensure the schema exists just in case
    NULL;
END
$$;

-- +goose Down
DO $$
BEGIN
    NULL;
END
$$;
