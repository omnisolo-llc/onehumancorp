-- 001_users.sql
CREATE TABLE IF NOT EXISTS users (
    id              TEXT PRIMARY KEY,
    username        TEXT UNIQUE NOT NULL,
    email           TEXT UNIQUE NOT NULL,
    password_hash   TEXT NOT NULL DEFAULT '',
    roles           TEXT NOT NULL DEFAULT '[]', -- JSON array
    active          BOOLEAN NOT NULL DEFAULT 1,
    organization_id TEXT NOT NULL DEFAULT '',
    oidc_subject    TEXT UNIQUE DEFAULT NULL,
    created_at      DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_users_username ON users (username);
CREATE INDEX IF NOT EXISTS idx_users_email ON users (email);

CREATE TABLE IF NOT EXISTS roles (
    id          TEXT PRIMARY KEY,
    name        TEXT UNIQUE NOT NULL,
    permissions TEXT NOT NULL DEFAULT '[]', -- JSON array
    created_at  DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
INSERT INTO roles (id, name, permissions) VALUES
    ('admin',    'admin',    '["*"]'),
    ('operator', 'operator', '["read", "write"]'),
    ('viewer',   'viewer',   '["read"]')
ON CONFLICT (id) DO NOTHING;

CREATE TABLE IF NOT EXISTS revoked_tokens (
    jti        TEXT PRIMARY KEY,
    expires_at DATETIME NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_revoked_tokens_exp ON revoked_tokens (expires_at);
