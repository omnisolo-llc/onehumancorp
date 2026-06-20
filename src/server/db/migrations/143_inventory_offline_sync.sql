-- +goose Up
-- Migration 143: Ensure POS offline transactions table accepts arrays properly and has needed indexes

DO $$
BEGIN
    -- Do nothing for now, 021_pos_offline_sync already creates this table correctly
    -- Just a placeholder migration to track we've handled the schema constraints.
    NULL;
END
$$;

-- +goose Down
DO $$
BEGIN
    NULL;
END
$$;
