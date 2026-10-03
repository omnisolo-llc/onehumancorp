-- Invoked by the existing portable migration authority after canonical roles
-- have been created/backfilled. No SECURITY DEFINER or runtime bypass role.
ALTER TABLE users ADD COLUMN IF NOT EXISTS marketplace_authority_key TEXT;
ALTER TABLE users ADD COLUMN IF NOT EXISTS marketplace_eligible BOOLEAN NOT NULL DEFAULT FALSE;
CREATE UNIQUE INDEX IF NOT EXISTS agent_authority_user_key ON users(marketplace_authority_key);
CREATE UNIQUE INDEX IF NOT EXISTS agent_authority_user_state ON users(marketplace_authority_key,marketplace_eligible);
CREATE TABLE IF NOT EXISTS agent_definition_authorities(authority_key TEXT PRIMARY KEY, eligible BOOLEAN NOT NULL);
DO $$ BEGIN
 IF NOT EXISTS(SELECT 1 FROM pg_constraint WHERE conrelid='agent_definition_authorities'::regclass AND conname='agent_authority_parent') THEN
  ALTER TABLE agent_definition_authorities ADD CONSTRAINT agent_authority_parent FOREIGN KEY(authority_key,eligible) REFERENCES users(marketplace_authority_key,marketplace_eligible) ON UPDATE CASCADE ON DELETE CASCADE;
 END IF;
END $$;


-- AUTHORITY_SCHEMA_END
UPDATE users SET marketplace_authority_key=replace(gen_random_uuid()::text,'-','') WHERE marketplace_authority_key IS NULL;
UPDATE users SET marketplace_eligible=(COALESCE(active,FALSE) AND EXISTS(
 SELECT 1 FROM identity_user_roles r WHERE r.user_id=users.id AND r.tenant_id=users.tenant_id
 AND (r.role_name ~ '^[Aa][Dd][Mm][Ii][Nn]$' OR r.role_name ~ '^[Oo][Ww][Nn][Ee][Rr]$')));
INSERT INTO agent_definition_authorities(authority_key,eligible)
 SELECT marketplace_authority_key,marketplace_eligible FROM users
 ON CONFLICT(authority_key) DO NOTHING;
-- AUTHORITY_BACKFILL_END
ALTER TABLE users ALTER COLUMN marketplace_authority_key SET NOT NULL;
-- Gate ordering is statement-local. Earlier caller locks can still deadlock;
-- callers must surface rollback/unknown outcomes, never replay external effects.
CREATE OR REPLACE FUNCTION agent_authority_statement_gate() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN PERFORM pg_advisory_xact_lock(57129048260865031); RETURN NULL; END $$;
CREATE OR REPLACE FUNCTION agent_user_derive_authority() RETURNS trigger LANGUAGE plpgsql AS $$
DECLARE derived BOOLEAN;
BEGIN
 IF TG_OP='INSERT' THEN
   IF NEW.marketplace_authority_key IS NOT NULL OR NEW.marketplace_eligible THEN
     RAISE EXCEPTION 'agent_authority_fields_are_derived';
   END IF;
   NEW.marketplace_authority_key:=replace(gen_random_uuid()::text,'-','');
 ELSE
   IF NEW.marketplace_authority_key IS DISTINCT FROM OLD.marketplace_authority_key
      OR NEW.id IS DISTINCT FROM OLD.id OR NEW.tenant_id IS DISTINCT FROM OLD.tenant_id THEN
     RAISE EXCEPTION 'agent_authority_key_is_immutable';
   END IF;
 END IF;
 derived:=COALESCE(NEW.active,FALSE) AND EXISTS(
   SELECT 1 FROM identity_user_roles r WHERE r.user_id=NEW.id AND r.tenant_id=NEW.tenant_id
   AND (r.role_name ~ '^[Aa][Dd][Mm][Ii][Nn]$' OR r.role_name ~ '^[Oo][Ww][Nn][Ee][Rr]$'));
 IF TG_OP='UPDATE' AND NEW.marketplace_eligible IS DISTINCT FROM OLD.marketplace_eligible
    AND NEW.marketplace_eligible IS DISTINCT FROM derived THEN
   RAISE EXCEPTION 'agent_authority_fields_are_derived';
 END IF;
 NEW.marketplace_eligible:=derived;
 RETURN NEW;
END $$;
CREATE OR REPLACE FUNCTION agent_user_create_authority() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
 INSERT INTO agent_definition_authorities(authority_key,eligible) VALUES(NEW.marketplace_authority_key,NEW.marketplace_eligible);
 RETURN NULL;
END $$;
CREATE OR REPLACE FUNCTION agent_role_refresh_authority() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
 IF TG_OP<>'INSERT' THEN
  UPDATE users SET marketplace_eligible=marketplace_eligible WHERE id=OLD.user_id AND tenant_id=OLD.tenant_id;
 END IF;
 IF TG_OP<>'DELETE' THEN
  UPDATE users SET marketplace_eligible=marketplace_eligible WHERE id=NEW.user_id AND tenant_id=NEW.tenant_id;
 END IF;
 RETURN NULL;
END $$;
CREATE OR REPLACE FUNCTION agent_projection_immutable_key() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
 IF NEW.authority_key IS DISTINCT FROM OLD.authority_key THEN RAISE EXCEPTION 'agent_authority_key_is_immutable'; END IF;
 RETURN NEW;
END $$;
DROP TRIGGER IF EXISTS agent_users_statement_gate ON users;
CREATE TRIGGER agent_users_statement_gate BEFORE INSERT OR UPDATE OR DELETE ON users FOR EACH STATEMENT EXECUTE FUNCTION agent_authority_statement_gate();
DROP TRIGGER IF EXISTS agent_roles_statement_gate ON identity_user_roles;
CREATE TRIGGER agent_roles_statement_gate BEFORE INSERT OR UPDATE OR DELETE ON identity_user_roles FOR EACH STATEMENT EXECUTE FUNCTION agent_authority_statement_gate();
DROP TRIGGER IF EXISTS agent_user_authority ON users;
CREATE TRIGGER agent_user_authority BEFORE INSERT OR UPDATE ON users FOR EACH ROW EXECUTE FUNCTION agent_user_derive_authority();
DROP TRIGGER IF EXISTS agent_user_authority_insert ON users;
CREATE TRIGGER agent_user_authority_insert AFTER INSERT ON users FOR EACH ROW EXECUTE FUNCTION agent_user_create_authority();
DROP TRIGGER IF EXISTS agent_role_authority ON identity_user_roles;
CREATE TRIGGER agent_role_authority AFTER INSERT OR UPDATE OR DELETE ON identity_user_roles FOR EACH ROW EXECUTE FUNCTION agent_role_refresh_authority();
DROP TRIGGER IF EXISTS agent_projection_key ON agent_definition_authorities;
CREATE TRIGGER agent_projection_key BEFORE UPDATE ON agent_definition_authorities FOR EACH ROW EXECUTE FUNCTION agent_projection_immutable_key();
ALTER TABLE agent_definition_authorities ENABLE ROW LEVEL SECURITY;
ALTER TABLE agent_definition_authorities FORCE ROW LEVEL SECURITY;
DROP POLICY IF EXISTS public_agent_authority_read ON agent_definition_authorities;
CREATE POLICY public_agent_authority_read ON agent_definition_authorities FOR SELECT USING(true);
DROP POLICY IF EXISTS own_agent_authority_insert ON agent_definition_authorities;
CREATE POLICY own_agent_authority_insert ON agent_definition_authorities FOR INSERT WITH CHECK(EXISTS(
 SELECT 1 FROM users WHERE marketplace_authority_key=authority_key AND marketplace_eligible=eligible
 AND tenant_id=current_setting('app.current_tenant',true)));
-- No application UPDATE/DELETE policy. Standard FK cascades maintain the row.
-- The deployment's restricted runtime role needs only SELECT/INSERT here;
-- never grant it TRUNCATE, REFERENCES, TRIGGER or ownership of this table.
-- Pin all trigger relation lookups to the migration schema. pg_temp is explicit
-- and last so a caller-created temporary identity table cannot shadow auth data.
DO $$ DECLARE function_name TEXT; BEGIN
 FOREACH function_name IN ARRAY ARRAY['agent_authority_statement_gate','agent_user_derive_authority','agent_user_create_authority','agent_role_refresh_authority','agent_projection_immutable_key'] LOOP
  EXECUTE format('ALTER FUNCTION %I.%I() SET search_path TO pg_catalog,%I,pg_temp',current_schema(),function_name,current_schema());
 END LOOP;
END $$;

DROP POLICY IF EXISTS verified_definition_insert ON agent_definitions;
CREATE POLICY verified_definition_insert ON agent_definitions FOR INSERT WITH CHECK (
    source='community' AND EXISTS(SELECT 1 FROM users u WHERE u.id=current_setting('app.current_actor',true) AND u.tenant_id=current_setting('app.current_tenant',true) AND u.marketplace_authority_key=authority_key AND u.marketplace_eligible)
);
