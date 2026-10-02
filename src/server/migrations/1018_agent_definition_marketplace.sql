-- Public content is separate from private publisher/installation/receipt ownership.
CREATE TABLE agent_definition_authorities (
    authority_key TEXT PRIMARY KEY, eligible BOOLEAN NOT NULL
);
ALTER TABLE agent_definition_authorities ENABLE ROW LEVEL SECURITY;
ALTER TABLE agent_definition_authorities FORCE ROW LEVEL SECURITY;
CREATE POLICY public_agent_authority_read ON agent_definition_authorities FOR SELECT USING(true);
-- The portable auth migration adds its private parent constraint and derived
-- writer after normalized roles exist. Until then this empty projection has no
-- application INSERT/UPDATE/DELETE policy.
CREATE UNIQUE INDEX IF NOT EXISTS agent_definition_user_identity ON users(id,tenant_id);
CREATE TABLE agent_definitions (
    id TEXT NOT NULL, version BIGINT NOT NULL CHECK(version > 0),
    digest TEXT NOT NULL CHECK(length(digest) = 64), document TEXT NOT NULL,
    search_text TEXT NOT NULL, source TEXT NOT NULL DEFAULT 'community' CHECK(source IN ('first_party','community')), authority_key TEXT,
    CHECK((source='first_party' AND authority_key IS NULL) OR (source='community' AND authority_key IS NOT NULL)), PRIMARY KEY(id,version)
);
CREATE TABLE agent_definition_publishers (
    definition_id TEXT NOT NULL, version BIGINT NOT NULL,
    tenant_id TEXT NOT NULL, user_id TEXT NOT NULL,
    PRIMARY KEY(definition_id,version),
    FOREIGN KEY(user_id,tenant_id) REFERENCES users(id,tenant_id) ON DELETE CASCADE,
    FOREIGN KEY(definition_id,version) REFERENCES agent_definitions(id,version)
);
CREATE TABLE agent_definition_installations (
    id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, user_id TEXT NOT NULL,
    definition_id TEXT NOT NULL, version BIGINT NOT NULL, document TEXT NOT NULL,
    UNIQUE(tenant_id,user_id,definition_id,version),
    FOREIGN KEY(user_id,tenant_id) REFERENCES users(id,tenant_id) ON DELETE CASCADE,
    FOREIGN KEY(definition_id,version) REFERENCES agent_definitions(id,version)
);
CREATE TABLE agent_definition_operations (
    tenant_id TEXT NOT NULL, user_id TEXT NOT NULL, request_id TEXT NOT NULL,
    fingerprint TEXT NOT NULL, receipt TEXT NOT NULL,
    PRIMARY KEY(tenant_id,user_id,request_id),
    FOREIGN KEY(user_id,tenant_id) REFERENCES users(id,tenant_id) ON DELETE CASCADE
);
-- Explicit first-party definitions. These are inert templates, never runtime agents.
INSERT INTO agent_definitions(id,version,digest,document,search_text,source) VALUES('00000000-0000-4000-8000-000000000001',1,'7740d03402f2f06f3a7ad5c6fabcb524d89783be735cd65149c60ee6ec29b36f','{"id":"00000000-0000-4000-8000-000000000001","version":1,"digest":"7740d03402f2f06f3a7ad5c6fabcb524d89783be735cd65149c60ee6ec29b36f","name":"Senior Rust Developer","description":"Prepare Rust changes with tests and explain their verification.","role":"Rust Developer","system_prompt":"Help prepare Rust code changes from the reviewed task and available source. Preserve authorization boundaries, add meaningful tests, and report unverified behavior. This saved definition grants no tools or execution authority.","visibility":"public","source":"first_party"}','senior rust developer prepare rust changes with tests and explain their verification. rust developer','first_party') ON CONFLICT(id,version) DO NOTHING;
INSERT INTO agent_definitions(id,version,digest,document,search_text,source) VALUES('00000000-0000-4000-8000-000000000002',1,'a398aa94e0f030ce0d9e6b52ce2fef76363a71cad217a55678ea2775af8be03a','{"id":"00000000-0000-4000-8000-000000000002","version":1,"digest":"a398aa94e0f030ce0d9e6b52ce2fef76363a71cad217a55678ea2775af8be03a","name":"Technical Writer","description":"Prepare clear technical documents from verified source material.","role":"Technical Writer","system_prompt":"Help draft clear technical documentation from the reviewed request and authorized source material. Distinguish evidence from assumptions and preserve private information. This saved definition grants no tools or execution authority.","visibility":"public","source":"first_party"}','technical writer prepare clear technical documents from verified source material. technical writer','first_party') ON CONFLICT(id,version) DO NOTHING;
ALTER TABLE agent_definitions ENABLE ROW LEVEL SECURITY;
ALTER TABLE agent_definitions FORCE ROW LEVEL SECURITY;
CREATE POLICY public_definition_read ON agent_definitions FOR SELECT USING (source='first_party' OR EXISTS(SELECT 1 FROM agent_definition_authorities a WHERE a.authority_key=agent_definitions.authority_key AND a.eligible));

ALTER TABLE agent_definition_publishers ENABLE ROW LEVEL SECURITY;
ALTER TABLE agent_definition_publishers FORCE ROW LEVEL SECURITY;
CREATE POLICY definition_publisher_owner ON agent_definition_publishers
USING (tenant_id=current_setting('app.current_tenant',true) AND user_id=current_setting('app.current_actor',true))
WITH CHECK (tenant_id=current_setting('app.current_tenant',true) AND user_id=current_setting('app.current_actor',true));
ALTER TABLE agent_definition_installations ENABLE ROW LEVEL SECURITY;
ALTER TABLE agent_definition_installations FORCE ROW LEVEL SECURITY;
CREATE POLICY definition_installation_owner ON agent_definition_installations
USING (tenant_id=current_setting('app.current_tenant',true) AND user_id=current_setting('app.current_actor',true))
WITH CHECK (tenant_id=current_setting('app.current_tenant',true) AND user_id=current_setting('app.current_actor',true));
ALTER TABLE agent_definition_operations ENABLE ROW LEVEL SECURITY;
ALTER TABLE agent_definition_operations FORCE ROW LEVEL SECURITY;
CREATE POLICY definition_operation_owner ON agent_definition_operations
USING (tenant_id=current_setting('app.current_tenant',true) AND user_id=current_setting('app.current_actor',true))
WITH CHECK (tenant_id=current_setting('app.current_tenant',true) AND user_id=current_setting('app.current_actor',true));
CREATE FUNCTION immutable_agent_definition_record() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN RAISE EXCEPTION 'agent_definition_record_is_immutable'; END $$;
CREATE FUNCTION immutable_private_agent_definition_record() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
 IF TG_OP='DELETE' AND NOT EXISTS(SELECT 1 FROM users WHERE id=OLD.user_id AND tenant_id=OLD.tenant_id) THEN RETURN OLD; END IF;
 RAISE EXCEPTION 'agent_definition_record_is_immutable';
END $$;
CREATE TRIGGER immutable_agent_definitions BEFORE UPDATE OR DELETE ON agent_definitions FOR EACH ROW EXECUTE FUNCTION immutable_agent_definition_record();
CREATE TRIGGER immutable_agent_installations BEFORE UPDATE OR DELETE ON agent_definition_installations FOR EACH ROW EXECUTE FUNCTION immutable_private_agent_definition_record();
CREATE TRIGGER immutable_agent_operations BEFORE UPDATE OR DELETE ON agent_definition_operations FOR EACH ROW EXECUTE FUNCTION immutable_private_agent_definition_record();
CREATE TRIGGER immutable_agent_publishers BEFORE UPDATE OR DELETE ON agent_definition_publishers FOR EACH ROW EXECUTE FUNCTION immutable_private_agent_definition_record();

DO $$ BEGIN EXECUTE format('ALTER FUNCTION %I.immutable_private_agent_definition_record() SET search_path TO pg_catalog,%I,pg_temp',current_schema(),current_schema()); END $$;
