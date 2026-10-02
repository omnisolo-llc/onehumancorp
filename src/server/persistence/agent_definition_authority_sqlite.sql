-- Run once the canonical users/identity_user_roles tables and the two private
-- users columns exist, inside the existing portable migration transaction.
-- Every runtime SQLite connection must have PRAGMA foreign_keys=ON.
CREATE UNIQUE INDEX IF NOT EXISTS agent_authority_user_key ON users(marketplace_authority_key);
CREATE UNIQUE INDEX IF NOT EXISTS agent_authority_user_state ON users(marketplace_authority_key,marketplace_eligible);
UPDATE users SET marketplace_authority_key=lower(hex(randomblob(16))) WHERE marketplace_authority_key IS NULL;
UPDATE users SET marketplace_eligible=(active=1 AND EXISTS(
 SELECT 1 FROM identity_user_roles r WHERE r.user_id=users.id AND r.tenant_id=users.tenant_id
 AND (r.role_name GLOB '[Aa][Dd][Mm][Ii][Nn]' OR r.role_name GLOB '[Oo][Ww][Nn][Ee][Rr]')));
CREATE TABLE IF NOT EXISTS agent_definition_authorities(
 authority_key TEXT PRIMARY KEY NOT NULL,
 eligible BOOLEAN NOT NULL CHECK(eligible IN (0,1)),
 FOREIGN KEY(authority_key,eligible) REFERENCES users(marketplace_authority_key,marketplace_eligible)
 ON UPDATE CASCADE ON DELETE CASCADE
);
INSERT INTO agent_definition_authorities(authority_key,eligible)
 SELECT marketplace_authority_key,marketplace_eligible FROM users WHERE true
 ON CONFLICT(authority_key) DO NOTHING;

CREATE TRIGGER IF NOT EXISTS agent_user_key_insert BEFORE INSERT ON users BEGIN
 SELECT CASE WHEN NEW.marketplace_authority_key IS NOT NULL OR NEW.marketplace_eligible!=0
 THEN RAISE(ABORT,'agent_authority_fields_are_derived') END;
END;
CREATE TRIGGER IF NOT EXISTS agent_user_key_update BEFORE UPDATE ON users BEGIN
 SELECT CASE WHEN OLD.marketplace_authority_key IS NOT NULL AND
 (NEW.marketplace_authority_key IS NOT OLD.marketplace_authority_key OR NEW.id IS NOT OLD.id OR NEW.tenant_id IS NOT OLD.tenant_id)
 THEN RAISE(ABORT,'agent_authority_key_is_immutable') END;
 SELECT CASE WHEN NEW.marketplace_eligible IS NOT OLD.marketplace_eligible AND
 NEW.marketplace_eligible IS NOT (NEW.active=1 AND EXISTS(
 SELECT 1 FROM identity_user_roles r WHERE r.user_id=NEW.id AND r.tenant_id=NEW.tenant_id
 AND (r.role_name GLOB '[Aa][Dd][Mm][Ii][Nn]' OR r.role_name GLOB '[Oo][Ww][Nn][Ee][Rr]')))
 THEN RAISE(ABORT,'agent_authority_fields_are_derived') END;
END;
CREATE TRIGGER IF NOT EXISTS agent_user_authority_insert AFTER INSERT ON users BEGIN
 UPDATE users SET marketplace_authority_key=lower(hex(randomblob(16))) WHERE id=NEW.id;
 INSERT INTO agent_definition_authorities(authority_key,eligible)
 SELECT marketplace_authority_key,marketplace_eligible FROM users WHERE id=NEW.id;
END;
-- SQLite cannot assign NEW columns in a BEFORE trigger. Reject explicit forged
-- values before the write, then recompute every user write atomically afterward.
CREATE TRIGGER IF NOT EXISTS agent_user_authority_update AFTER UPDATE ON users
WHEN NEW.marketplace_eligible IS NOT (NEW.active=1 AND EXISTS(
 SELECT 1 FROM identity_user_roles r WHERE r.user_id=NEW.id AND r.tenant_id=NEW.tenant_id
 AND (r.role_name GLOB '[Aa][Dd][Mm][Ii][Nn]' OR r.role_name GLOB '[Oo][Ww][Nn][Ee][Rr]')))
BEGIN
 UPDATE users SET marketplace_eligible=(active=1 AND EXISTS(
 SELECT 1 FROM identity_user_roles r WHERE r.user_id=users.id AND r.tenant_id=users.tenant_id
 AND (r.role_name GLOB '[Aa][Dd][Mm][Ii][Nn]' OR r.role_name GLOB '[Oo][Ww][Nn][Ee][Rr]'))) WHERE id=NEW.id;
END;
CREATE TRIGGER IF NOT EXISTS agent_role_authority_insert AFTER INSERT ON identity_user_roles BEGIN
 UPDATE users SET marketplace_eligible=(active=1 AND EXISTS(
 SELECT 1 FROM identity_user_roles r WHERE r.user_id=users.id AND r.tenant_id=users.tenant_id
 AND (r.role_name GLOB '[Aa][Dd][Mm][Ii][Nn]' OR r.role_name GLOB '[Oo][Ww][Nn][Ee][Rr]'))) WHERE id=NEW.user_id;
END;
CREATE TRIGGER IF NOT EXISTS agent_role_authority_delete AFTER DELETE ON identity_user_roles BEGIN
 UPDATE users SET marketplace_eligible=(active=1 AND EXISTS(
 SELECT 1 FROM identity_user_roles r WHERE r.user_id=users.id AND r.tenant_id=users.tenant_id
 AND (r.role_name GLOB '[Aa][Dd][Mm][Ii][Nn]' OR r.role_name GLOB '[Oo][Ww][Nn][Ee][Rr]'))) WHERE id=OLD.user_id;
END;
CREATE TRIGGER IF NOT EXISTS agent_role_authority_update AFTER UPDATE ON identity_user_roles BEGIN
 UPDATE users SET marketplace_eligible=(active=1 AND EXISTS(
 SELECT 1 FROM identity_user_roles r WHERE r.user_id=users.id AND r.tenant_id=users.tenant_id
 AND (r.role_name GLOB '[Aa][Dd][Mm][Ii][Nn]' OR r.role_name GLOB '[Oo][Ww][Nn][Ee][Rr]'))) WHERE id IN(OLD.user_id,NEW.user_id);
END;
CREATE TRIGGER IF NOT EXISTS agent_projection_update BEFORE UPDATE ON agent_definition_authorities BEGIN
 SELECT CASE WHEN NEW.authority_key IS NOT OLD.authority_key THEN RAISE(ABORT,'agent_authority_key_is_immutable') END;
END;
CREATE TRIGGER IF NOT EXISTS agent_projection_delete BEFORE DELETE ON agent_definition_authorities
WHEN EXISTS(SELECT 1 FROM users WHERE marketplace_authority_key=OLD.authority_key)
BEGIN SELECT RAISE(ABORT,'agent_authority_projection_is_derived'); END;
