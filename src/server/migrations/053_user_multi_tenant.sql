ALTER TABLE users DROP CONSTRAINT IF EXISTS users_username_key;
ALTER TABLE users DROP CONSTRAINT IF EXISTS users_email_key;
ALTER TABLE users DROP CONSTRAINT IF EXISTS users_oidc_subject_key;
ALTER TABLE users ADD CONSTRAINT users_username_tenant_id_key UNIQUE (username, tenant_id);
ALTER TABLE users ADD CONSTRAINT users_email_tenant_id_key UNIQUE (email, tenant_id);
ALTER TABLE users ADD CONSTRAINT users_oidc_subject_tenant_id_key UNIQUE (oidc_subject, tenant_id);

ALTER TABLE roles DROP CONSTRAINT IF EXISTS roles_name_key;
ALTER TABLE roles ADD CONSTRAINT roles_name_tenant_id_key UNIQUE (name, tenant_id);
