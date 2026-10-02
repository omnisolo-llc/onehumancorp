-- Explicit standalone initialization. Reads never create catalogue content.
CREATE TABLE IF NOT EXISTS agent_definition_authorities(
 authority_key TEXT PRIMARY KEY NOT NULL, eligible BOOLEAN NOT NULL CHECK(eligible IN (0,1)),
 FOREIGN KEY(authority_key,eligible) REFERENCES users(marketplace_authority_key,marketplace_eligible) ON UPDATE CASCADE ON DELETE CASCADE
);
CREATE UNIQUE INDEX IF NOT EXISTS agent_definition_user_identity ON users(id,tenant_id);
CREATE TABLE IF NOT EXISTS agent_definitions (
    id TEXT NOT NULL, version INTEGER NOT NULL CHECK(version > 0),
    digest TEXT NOT NULL CHECK(length(digest) = 64), document TEXT NOT NULL,
    search_text TEXT NOT NULL, source TEXT NOT NULL DEFAULT 'community' CHECK(source IN ('first_party','community')), authority_key TEXT,
    CHECK((source='first_party' AND authority_key IS NULL) OR (source='community' AND authority_key IS NOT NULL)), PRIMARY KEY(id, version)
);
CREATE TABLE IF NOT EXISTS agent_definition_publishers (
    definition_id TEXT NOT NULL, version INTEGER NOT NULL,
    tenant_id TEXT NOT NULL, user_id TEXT NOT NULL,
    PRIMARY KEY(definition_id,version),
    FOREIGN KEY(user_id,tenant_id) REFERENCES users(id,tenant_id) ON DELETE CASCADE,
    FOREIGN KEY(definition_id,version) REFERENCES agent_definitions(id,version)
);
CREATE TABLE IF NOT EXISTS agent_definition_installations (
    id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, user_id TEXT NOT NULL,
    definition_id TEXT NOT NULL, version INTEGER NOT NULL, document TEXT NOT NULL,
    UNIQUE(tenant_id,user_id,definition_id,version),
    FOREIGN KEY(user_id,tenant_id) REFERENCES users(id,tenant_id) ON DELETE CASCADE,
    FOREIGN KEY(definition_id,version) REFERENCES agent_definitions(id,version)
);
CREATE TABLE IF NOT EXISTS agent_definition_operations (
    tenant_id TEXT NOT NULL, user_id TEXT NOT NULL, request_id TEXT NOT NULL,
    fingerprint TEXT NOT NULL, receipt TEXT NOT NULL,
    PRIMARY KEY(tenant_id,user_id,request_id),
    FOREIGN KEY(user_id,tenant_id) REFERENCES users(id,tenant_id) ON DELETE CASCADE
);
CREATE TRIGGER IF NOT EXISTS immutable_agent_definitions_update BEFORE UPDATE ON agent_definitions BEGIN SELECT RAISE(ABORT,'definition_is_immutable'); END;
CREATE TRIGGER IF NOT EXISTS immutable_agent_definitions_delete BEFORE DELETE ON agent_definitions BEGIN SELECT RAISE(ABORT,'definition_is_immutable'); END;
CREATE TRIGGER IF NOT EXISTS immutable_agent_installations_update BEFORE UPDATE ON agent_definition_installations BEGIN SELECT RAISE(ABORT,'installation_is_immutable'); END;
CREATE TRIGGER IF NOT EXISTS immutable_agent_installations_delete BEFORE DELETE ON agent_definition_installations WHEN EXISTS(SELECT 1 FROM users WHERE id=OLD.user_id AND tenant_id=OLD.tenant_id) BEGIN SELECT RAISE(ABORT,'installation_is_immutable'); END;
CREATE TRIGGER IF NOT EXISTS immutable_agent_operations_update BEFORE UPDATE ON agent_definition_operations BEGIN SELECT RAISE(ABORT,'operation_is_immutable'); END;
CREATE TRIGGER IF NOT EXISTS immutable_agent_operations_delete BEFORE DELETE ON agent_definition_operations WHEN EXISTS(SELECT 1 FROM users WHERE id=OLD.user_id AND tenant_id=OLD.tenant_id) BEGIN SELECT RAISE(ABORT,'operation_is_immutable'); END;
-- Explicit first-party definitions. These are inert templates, never runtime agents.
INSERT INTO agent_definitions(id,version,digest,document,search_text,source) VALUES('00000000-0000-4000-8000-000000000001',1,'7740d03402f2f06f3a7ad5c6fabcb524d89783be735cd65149c60ee6ec29b36f','{"id":"00000000-0000-4000-8000-000000000001","version":1,"digest":"7740d03402f2f06f3a7ad5c6fabcb524d89783be735cd65149c60ee6ec29b36f","name":"Senior Rust Developer","description":"Prepare Rust changes with tests and explain their verification.","role":"Rust Developer","system_prompt":"Help prepare Rust code changes from the reviewed task and available source. Preserve authorization boundaries, add meaningful tests, and report unverified behavior. This saved definition grants no tools or execution authority.","visibility":"public","source":"first_party"}','senior rust developer prepare rust changes with tests and explain their verification. rust developer','first_party') ON CONFLICT(id,version) DO NOTHING;
INSERT INTO agent_definitions(id,version,digest,document,search_text,source) VALUES('00000000-0000-4000-8000-000000000002',1,'a398aa94e0f030ce0d9e6b52ce2fef76363a71cad217a55678ea2775af8be03a','{"id":"00000000-0000-4000-8000-000000000002","version":1,"digest":"a398aa94e0f030ce0d9e6b52ce2fef76363a71cad217a55678ea2775af8be03a","name":"Technical Writer","description":"Prepare clear technical documents from verified source material.","role":"Technical Writer","system_prompt":"Help draft clear technical documentation from the reviewed request and authorized source material. Distinguish evidence from assumptions and preserve private information. This saved definition grants no tools or execution authority.","visibility":"public","source":"first_party"}','technical writer prepare clear technical documents from verified source material. technical writer','first_party') ON CONFLICT(id,version) DO NOTHING;
CREATE TRIGGER IF NOT EXISTS immutable_agent_publishers_update BEFORE UPDATE ON agent_definition_publishers BEGIN SELECT RAISE(ABORT,'publisher_is_immutable'); END;
CREATE TRIGGER IF NOT EXISTS immutable_agent_publishers_delete BEFORE DELETE ON agent_definition_publishers WHEN EXISTS(SELECT 1 FROM users WHERE id=OLD.user_id AND tenant_id=OLD.tenant_id) BEGIN SELECT RAISE(ABORT,'publisher_is_immutable'); END;
