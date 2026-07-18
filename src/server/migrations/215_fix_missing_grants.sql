-- +goose Up
-- Migration 215: Add missing grants to ohc_bypassrls role for newly created tables

GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO ohc_bypassrls;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO ohc_bypassrls;

-- Ensure that future tables also have privileges granted automatically to ohc_bypassrls
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL PRIVILEGES ON TABLES TO ohc_bypassrls;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL PRIVILEGES ON SEQUENCES TO ohc_bypassrls;

-- +goose Down
-- Revert the default privileges change
ALTER DEFAULT PRIVILEGES IN SCHEMA public REVOKE ALL PRIVILEGES ON TABLES FROM ohc_bypassrls;
ALTER DEFAULT PRIVILEGES IN SCHEMA public REVOKE ALL PRIVILEGES ON SEQUENCES FROM ohc_bypassrls;
