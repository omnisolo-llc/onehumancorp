# Specification: Enterprise Auth, API Gateway, and Workspace Analytics

## 1. Sub-Project 1: Open User Registration & SSO Controls

### 1.1 Config Toggle
- We add `registration_enabled` (`bool`) to `AppConfig` in `src/server/config/mod.rs`.
- Defaults to `false` in default config.
- Can be overridden via env var `OHC_REGISTRATION_ENABLED=true` or configuration files.

### 1.2 HTTP Registration Endpoint
- We expose `POST /api/v1/auth/register` in `src/server/auth/http.rs`.
- Expected JSON Request Payload:
  ```json
  {
    "username": "newuser",
    "email": "user@example.com",
    "password": "strongpassword123",
    "invite_token": null
  }
  ```
- **Registration Flow Logic**:
  1. If `invite_token` is present:
     - Look up the active invite token in the database. If not found or expired, return `400 Bad Request`.
     - Otherwise, register the user under that existing `organization_id` with the invited role.
  2. If `invite_token` is omitted / null:
     - Check if `registration_enabled` in `AppConfig` is `true`.
     - If `false`, immediately return `403 Forbidden` with error `{"error": "registration closed"}`.
     - If `true`, generate a new `organization_id` (UUID) as the user's personal organization, create the user with `ROLE_ADMIN` role under this new organization, issue their session JWT, and return `201 Created` with the session token.

---

## 2. Sub-Project 2: API Keys & Workflow Gateway

### 2.1 Database Schema (`api_keys`)
We create a new table `api_keys` in PostgreSQL:
```sql
CREATE TABLE IF NOT EXISTS api_keys (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    key_hash VARCHAR(64) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    member_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    organization_id VARCHAR(128) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP WITH TIME ZONE
);
CREATE INDEX IF NOT EXISTS idx_api_keys_hash ON api_keys(key_hash);
```

### 2.2 API Key Generation API
- Expose `POST /api/v1/settings/keys` to generate a new API key for the logged-in user.
- Generate secure key material of 32 random bytes, base64url-encode it to return to the user (only shown once), hash it with SHA256, and save the SHA256 hash in `key_hash`.

### 2.3 API Key Authentication Middleware
- Implement Axum middleware `api_key_auth_middleware` applied under `/api/v1/gateway/*`.
- Parses `Authorization: Bearer <token>`, hashes it with SHA256, and matches it against `key_hash` in `api_keys` table.
- Sets the `AuthInfo` extension using the key's bound `member_id` as the acting `user_id` and `organization_id`.

### 2.4 Gateway run Endpoint
- Expose `POST /api/v1/gateway/run` mapped to the existing `start_onboarding` workflow or custom workflow runner. It executes the specified workflow using the member-specific key context.

---

## 3. Sub-Project 3: Enterprise Workspace Controls & Member Analytics

### 3.1 Database Schema (`user_usage_logs`)
We create a new table `user_usage_logs` in PostgreSQL:
```sql
CREATE TABLE IF NOT EXISTS user_usage_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    organization_id VARCHAR(128) NOT NULL,
    feature VARCHAR(128) NOT NULL,
    tokens_used INTEGER NOT NULL DEFAULT 0,
    computed_cost NUMERIC(10, 4) NOT NULL DEFAULT 0.0000,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_user_usage_logs_user ON user_usage_logs(user_id);
```

### 3.2 Usage Logging Interface
- Add a helper `record_usage(user_id, organization_id, feature, tokens, cost)` which inserts a non-blocking database log.
- Wrap LLM inference, agent, and gateway executions to write usage data.

### 3.3 Admin Workspace Portal UI & API
- Expose `GET /api/v1/ui/admin/usage` which aggregates total usage logs grouped by user and feature:
  ```json
  [
    {
      "username": "newuser",
      "feature": "workflow_run",
      "tokens_used": 15200,
      "computed_cost": 0.0304
    }
  ]
  ```
- Build a new tab "Member Analytics" inside the Admin Panel UI to view these aggregations.
