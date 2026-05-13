-- 059_create_bypassrls_role.sql
-- Create a role with BYPASSRLS to allow elevated system queries without magic strings.

DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'ohc_bypassrls') THEN
        CREATE ROLE ohc_bypassrls WITH NOLOGIN BYPASSRLS;
    END IF;
END
$$;

-- Grant to the current user (the user running migrations and the app connection)
GRANT ohc_bypassrls TO CURRENT_USER;
