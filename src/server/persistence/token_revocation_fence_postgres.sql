-- Receipt precommit takes a shared lock on this same immutable token identity.
-- A trigger covers every canonical writer, including direct SQL inserts.
CREATE OR REPLACE FUNCTION ohc_token_revocation_fence() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
 PERFORM pg_advisory_xact_lock(hashtextextended(jsonb_build_array('ohc-token-fence-v1',NEW.tenant_id::text,NEW.jti::text)::text,0));
 RETURN NEW;
END $$;
CREATE OR REPLACE TRIGGER ohc_token_revocation_fence
 BEFORE INSERT OR UPDATE ON auth_revoked_tokens
 FOR EACH ROW EXECUTE FUNCTION ohc_token_revocation_fence();
-- Resolve built-ins before user-controlled schemas or temporary objects.
DO $$ BEGIN
 EXECUTE format('ALTER FUNCTION %I.ohc_token_revocation_fence() SET search_path TO pg_catalog,%I,pg_temp',current_schema(),current_schema());
END $$;
