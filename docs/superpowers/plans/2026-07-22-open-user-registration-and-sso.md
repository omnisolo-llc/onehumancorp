# Open User Registration & SSO Controls Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the new `registration_enabled` server configuration parameter and expose the public `/api/v1/auth/register` HTTP registration endpoint with comprehensive validation, JWT issuance, and Keycloak integration compatibility.

**Architecture:** We add a configuration option to `AppConfig` and map a new Axum router endpoint under `/api/v1/auth/register` inside `http.rs`. On successful validation, it inserts a new user with `ROLE_ADMIN` under a newly generated organization, unless an active invitation is used.

**Tech Stack:** Rust, Axum, Serde, sqlx, Git, Bazel.

## Global Constraints
- Naming rules: Setting name `registration_enabled` (env OHC_REGISTRATION_ENABLED).
- Security rule: Enforce `registration_enabled` check strictly (return 403 Forbidden with `{"error": "registration closed"}` if disabled).
- Database constraint: Generate unique UUIDs for new organization IDs and user IDs.

---

### Task 1: Add `registration_enabled` to Server Configuration

**Files:**
- Modify: `src/server/config/mod.rs`
- Test: `src/server/config/mod.rs` (unit tests)

**Interfaces:**
- Produces: `AppConfig::registration_enabled` (`bool`)

- [ ] **Step 1: Declare the `registration_enabled` field inside `AppConfig`**

Modify `src/server/config/mod.rs` around line 12:
```rust
#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub listen_addr: String,
    pub grpc_addr: String,
    pub database_url: Option<String>,
    pub standalone: bool,
    pub sqlite_encryption_key: Option<String>,
    pub redis_url: Option<String>,
    pub multitenant: bool,
    pub headless: bool,
    pub registration_enabled: bool,
    // ... rest of fields
}
```

- [ ] **Step 2: Set default value in the builder**

Modify `src/server/config/mod.rs` around line 77:
```rust
        .set_default("headless", false)?
        .set_default("agent_auth_disabled", false)?
        .set_default("telemetry_enabled", false)?
        .set_default("registration_enabled", false)?
```

- [ ] **Step 3: Run existing config tests to verify it compiles**

Run: `bazelisk test //src/server/config:server_config_unit_test`
Expected: PASS

- [ ] **Step 4: Commit changes**

```bash
git add src/server/config/mod.rs
git commit -m "feat(config): add registration_enabled configuration parameter"
```

---

### Task 2: Implement `/api/v1/auth/register` HTTP Endpoint

**Files:**
- Modify: `src/server/auth/http.rs`
- Test: `src/server/auth/http.rs` (TDD unit tests)

**Interfaces:**
- Consumes: `AppConfig::registration_enabled`
- Produces: `POST /api/v1/auth/register` JSON HTTP API

- [ ] **Step 1: Write the failing TDD registration tests in `http.rs`**

Add these tests to the test module of `src/server/auth/http.rs`:
```rust
    #[tokio::test]
    async fn register_returns_forbidden_when_registration_disabled() {
        let store = Arc::new(Store::new());
        let state = HttpAuthState::new(store, HashSet::new());
        let app = router_with_state(state);

        let response = app.oneshot(json_request(
            "/api/v1/auth/register",
            r#"{"username":"testregister","email":"test@example.test","password":"correcthorsebatterystaple"}"#,
        )).await.unwrap();

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        let body = to_bytes(response.into_body(), 1024).await.unwrap();
        assert_eq!(body.as_ref(), br#"{"error":"registration closed"}"#);
    }
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `bazelisk test //src/server/auth:server_auth_unit_test`
Expected: FAIL (endpoint `/api/v1/auth/register` does not exist)

- [ ] **Step 3: Map the new registration route and implement the handler**

Register route inside `router_with_state` at `src/server/auth/http.rs`:
```rust
fn router_with_state(state: HttpAuthState) -> Router {
    Router::new()
        .route("/api/v1/auth/login", post(login))
        .route("/api/v1/auth/logout", post(logout))
        .route("/api/v1/auth/register", post(register))
        .with_state(state)
}
```

Implement the `register` handler:
```rust
#[derive(serde::Deserialize)]
struct RegisterRequest {
    username: String,
    email: String,
    password: String,
    invite_token: Option<String>,
}

async fn register(
    State(state): State<HttpAuthState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    request: axum::extract::Request,
) -> Response {
    if !::server_config::get().registration_enabled {
        return error(StatusCode::FORBIDDEN, "registration closed");
    }

    // Parse JSON payload and execute user creation logic
    // Assign ROLE_ADMIN and generate unique UUID for organization_id
    // Issue token and return 201 Created
    // ... (full code will be implemented in Step 3)
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `bazelisk test //src/server/auth:server_auth_unit_test`
Expected: PASS

- [ ] **Step 5: Commit changes**

```bash
git add src/server/auth/http.rs
git commit -m "feat(auth): implement public registration HTTP endpoint"
```
