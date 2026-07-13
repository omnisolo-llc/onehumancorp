# Verified gRPC Identity and Agent Tenant Remediation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace spoofable gRPC identity metadata with verified cloud mTLS identity and enforce organization ownership on every agent-manager operation.

**Architecture:** Cloud gRPC startup requires a server certificate, key, and client CA, and the interceptor derives the SPIFFE URI from the verified peer certificate rather than trusting request metadata. Standalone gRPC binds only to loopback and retains strict metadata parsing for local compatibility. Agent-manager state is keyed by authenticated organization, and mutations verify resource ownership before calling global Hub methods.

**Tech Stack:** Rust 2024, Tonic/rustls mTLS, `x509-parser`, SQL-independent in-memory service tests, Cargo and Bazel.

---

### Task 1: Make SPIFFE syntax and trust-domain validation strict

**Files:**
- Modify: `src/server/auth/mod.rs:768`
- Test: `src/server/auth/grpc.rs`

- [ ] **Step 1: Write failing parser regressions**

Add tests that require `parse_spiffe_id` to reject empty organization/agent segments, encoded slashes, path traversal, extra path segments, and domains outside `onehumancorp.io`, `ohc.local`, `ohc.os`, or subdomains of `ohc.global`:

```rust
#[test]
fn parse_spiffe_id_rejects_empty_and_untrusted_identities() {
    for id in [
        "spiffe://evil.example/org/acme/agent/a1",
        "spiffe://onehumancorp.io/org//agent/a1",
        "spiffe://onehumancorp.io/org/acme/agent/",
        "spiffe://onehumancorp.io/org/acme/agent/a1/extra",
        "spiffe://onehumancorp.io/org/acme%2Fother/agent/a1",
    ] {
        assert!(super::parse_spiffe_id(id).is_err(), "accepted {id}");
    }
}
```

- [ ] **Step 2: Run the parser regression and verify red**

Run: `cargo test -p server_auth parse_spiffe_id_rejects_empty_and_untrusted_identities`
Expected: FAIL because the current slash-position parser accepts at least the untrusted and empty identities.

- [ ] **Step 3: Reuse the strict gRPC validator and require an exact path**

Implement `parse_spiffe_id` as:

```rust
pub fn parse_spiffe_id(spiffe_id: &str) -> Result<(String, String), Status> {
    grpc::validate_spiffe_id(spiffe_id)?;
    let path = spiffe_id
        .strip_prefix("spiffe://")
        .ok_or_else(|| Status::unauthenticated("invalid SPIFFE scheme"))?;
    let parts: Vec<&str> = path.split('/').collect();
    if parts.len() != 5 || parts[1] != "org" || parts[3] != "agent" {
        return Err(Status::unauthenticated("invalid SPIFFE identity path"));
    }
    Ok((parts[2].to_string(), parts[4].to_string()))
}
```

- [ ] **Step 4: Run the server-auth suite**

Run: `cargo test -p server_auth`
Expected: PASS; Postgres tests may explicitly report an environment skip but syntax tests must execute.

- [ ] **Step 5: Commit strict identity parsing**

```bash
git add src/server/auth/mod.rs src/server/auth/grpc.rs
git commit -m "security: validate server SPIFFE identities strictly"
```

### Task 2: Extract SPIFFE URI SANs from verified peer certificates

**Files:**
- Create: `src/server/auth/peer_identity.rs`
- Modify: `src/server/auth/mod.rs`
- Modify: `src/server/auth/Cargo.toml`
- Modify: `Cargo.lock`
- Modify: `MODULE.bazel.lock`
- Modify: `src/server/auth/BUILD.bazel`

- [ ] **Step 1: Add a failing certificate identity test**

Create `peer_identity.rs` with tests using `rcgen` to make one certificate with a valid URI SAN and one without it. The public interface is:

```rust
pub fn spiffe_id_from_certificate_der(der: &[u8]) -> Result<String, tonic::Status>;
```

The valid test must assert the exact URI; the missing-SAN and malformed-DER tests must return `Unauthenticated`.

- [ ] **Step 2: Run the focused test and verify red**

Run: `cargo test -p server_auth peer_identity`
Expected: FAIL until DER parsing is implemented.

- [ ] **Step 3: Parse only URI SANs and apply strict SPIFFE validation**

Use `x509_parser::parse_x509_certificate`, locate `ParsedExtension::SubjectAlternativeName`, select exactly one `GeneralName::URI` beginning with `spiffe://`, and pass it through `parse_spiffe_id`. Reject zero or multiple SPIFFE URIs.

Add `x509-parser = "0.18"` to dependencies and `rcgen = "0.14"` to dev-dependencies. Add the generated Bazel crate label and `peer_identity.rs` source.

- [ ] **Step 4: Run Cargo and Bazel auth tests**

Run:

```bash
cargo test -p server_auth peer_identity
bazel test //src/server/auth:server_auth_unit_test
```

Expected: PASS.

- [ ] **Step 5: Commit peer identity extraction**

```bash
git add Cargo.lock MODULE.bazel.lock src/server/auth
git commit -m "security: extract SPIFFE identity from peer certificates"
```

### Task 3: Require mTLS in cloud and constrain standalone gRPC to loopback

**Files:**
- Modify: `Cargo.toml`
- Modify: `src/server/lib.rs:2467,418,7339`
- Test: `src/server/lib.rs`

- [ ] **Step 1: Add failing TLS configuration tests**

Extract and test:

```rust
fn grpc_bind_host(standalone: bool) -> &'static str;
fn grpc_tls_config_from_pem(
    standalone: bool,
    cert: Option<Vec<u8>>,
    key: Option<Vec<u8>>,
    client_ca: Option<Vec<u8>>,
) -> Result<Option<tonic::transport::ServerTlsConfig>, String>;
```

Assert standalone binds `127.0.0.1`, cloud binds `0.0.0.0`, standalone returns `None`, and cloud rejects every missing/empty PEM component.

- [ ] **Step 2: Run the tests and verify red**

Run: `cargo test -p ohc-mono --lib grpc_tls_config`
Expected: FAIL because the helpers do not exist.

- [ ] **Step 3: Enable Tonic TLS and configure the server**

Enable Tonic's `tls` feature. In cloud mode, read `OHC_GRPC_TLS_CERT_PATH`, `OHC_GRPC_TLS_KEY_PATH`, and `OHC_GRPC_CLIENT_CA_PATH` before database initialization, construct `Identity::from_pem` and `Certificate::from_pem`, then call:

```rust
ServerTlsConfig::new()
    .identity(identity)
    .client_ca_root(client_ca)
```

Apply it to `Server::builder().tls_config(config)?`. Missing cloud TLS configuration must abort startup. Build `addr` with `grpc_bind_host(is_standalone_runtime())`.

- [ ] **Step 4: Derive cloud metadata from `Request::peer_certs`**

Change `spiffe_interceptor` to accept a mutable request. In cloud mode require a non-empty peer certificate chain, parse the leaf with `spiffe_id_from_certificate_der`, replace any inbound `x-spiffe-id` value with the certificate-derived URI, and insert `server_auth::AuthInfo` into request extensions. In standalone mode strictly parse the metadata identity. Never compare or trust a cloud-supplied identity header.

- [ ] **Step 5: Verify root Cargo and Bazel builds**

Run:

```bash
cargo test -p ohc-mono --lib grpc_tls_config
cargo test -p ohc-mono --lib spiffe_interceptor
bazel build //src/server:server_lib
```

Expected: PASS.

- [ ] **Step 6: Commit mTLS startup enforcement**

```bash
git add Cargo.toml Cargo.lock MODULE.bazel.lock src/server/lib.rs
git commit -m "security: require verified gRPC peer identity in cloud"
```

### Task 4: Enforce organization ownership in the agent manager

**Files:**
- Modify: `src/server/services/agent/service.rs`
- Test: `src/server/services/agent/service.rs`

- [ ] **Step 1: Write cross-organization negative tests**

Create two organizations and assert:

```rust
assert_eq!(
    service.fire_agent(request_for("org-a", "org-b-agent")).await.unwrap_err().code(),
    tonic::Code::PermissionDenied,
);
assert_eq!(
    service.delegate_task(cross_org_request()).await.unwrap_err().code(),
    tonic::Code::PermissionDenied,
);
```

Also assert identities, skills, and snapshots created under `org-b` are absent from `org-a` responses.

- [ ] **Step 2: Run the negative tests and verify red**

Run: `cargo test -p ohc-mono --lib services::agent::service::tests::cross_org -- --nocapture`
Expected: FAIL because current vectors and Hub mutations are global.

- [ ] **Step 3: Centralize authenticated organization extraction**

Add:

```rust
fn authenticated_org<T>(request: &Request<T>) -> Result<String, Status> {
    let id = server_auth::extract_spiffe_id_from_metadata(request.metadata())
        .map_err(Status::unauthenticated)?;
    let (org, _) = server_auth::parse_spiffe_id(&id)?;
    if org.is_empty() {
        return Err(Status::unauthenticated("SPIFFE organization is empty"));
    }
    Ok(org)
}
```

Remove every default-tenant fallback from this cloud-intercepted service.

- [ ] **Step 4: Scope mutations and state**

Before `fire_agent`, fetch the agent and require `agent.organization_id == org`. Before `delegate_task`, require both sender and recipient organizations to match. Change `skills` and `snapshots` to `RwLock<HashMap<String, Vec<_>>>`, keyed by authenticated organization. Filter identities with `get_agents_by_org`. Require authenticated organization extraction in every state-reading or state-mutating method.

- [ ] **Step 5: Run service, auth, and server build verification**

Run:

```bash
cargo test -p ohc-mono --lib services::agent -- --nocapture
cargo test -p server_auth
bazel build //src/server:server_lib
git diff --check
```

Expected: PASS, with database-dependent tests explicitly classified.

- [ ] **Step 6: Commit tenant ownership enforcement**

```bash
git add src/server/services/agent/service.rs
git commit -m "security: scope agent management to organization"
```

## Plan self-review

- Coverage: F-01 and F-02 from the audit report are covered by Tasks 1–4.
- Isolation: certificate parsing lives in server-auth; transport setup remains in server startup; resource authorization remains in the agent service.
- Failure mode: cloud startup and cloud requests fail closed when verified peer identity is unavailable; standalone compatibility is limited to loopback.
- No placeholder implementation steps remain.
