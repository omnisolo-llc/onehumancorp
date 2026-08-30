# Universal Harness Services Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox syntax for tracking.

**Goal:** Finish the eight native harness integrations behind a common provider facade and scoped local-service contract, then add the four pinned OpenAI-compatible shim harnesses without losing shared memory, tools, workspace, artifacts, browser, cache, integration, or persistence behavior.

**Architecture:** Keep the existing HarnessAdapter and native protocol runtimes as the canonical execution boundary. Add an internal provider facade that exposes an allowlisted OpenAI-compatible loopback route with an attempt-scoped token, and add typed LocalServiceBinding records that point to existing OmniSolo services without carrying authority. The eight native harnesses are the first acceptance gate; Aider, Goose, Open Interpreter, and Plandex use a separate shim mode and cannot mask native failures.

**Tech Stack:** Rust 2024, Tokio, Axum 0.8, Reqwest 0.12, Tonic/protobuf, Serde/JSON, existing harness middleware and worker images, Python sidecar fixtures, Docker Compose, Helm, and the existing ignored real-provider matrix.

---

## Scope and File Map

The design spans two dependent subsystems. Group A produces a complete,
testable native/shared-services gate. Group B consumes Group A's route and
binding contracts to add the finite shim set.

### Group A: native facade and local-service plane

- Create src/server/harness/middleware/provider_facade.rs: allowlisted
  OpenAI-compatible HTTP route, opaque token validation, upstream forwarding,
  response/error redaction, and route shutdown.
- Create src/server/harness/middleware/local_services.rs: service kinds,
  scope context, descriptors, bindings, bundle resolution, capability checks,
  lease state, and portable serialization rules.
- Modify src/server/harness/middleware/mod.rs to export the new modules.
- Modify src/server/harness/middleware/harness.rs for integration mode,
  provider route rewriting, service request context, and descriptors.
- Modify src/server/harness/middleware/types.rs for portable service bundles.
- Modify src/server/harness/middleware/capsule.rs for stable service references.
- Modify src/server/harness/middleware/grpc.rs for context validation and the
  local_services.bound event.
- Modify src/server/harness_worker/lib.rs to start the worker facade, replace
  child provider values with loopback route/token values, and shut it down.
- Modify src/server/harness/Cargo.toml and src/server/harness/BUILD.bazel for
  Axum and the new sources.
- Add focused provider_facade.rs and local_services.rs test targets.
- Extend external_adapters.rs, worker_grpc.rs, and worker_e2e.rs.

### Group B: finite shim expansion

- Modify harness.rs for OpenAiCompatibleShim, four pinned presets, truthful
  capability metadata, and shim process behavior.
- Create src/server/harness/sidecars/openai_compatible_shim.py, a deterministic
  JSONL wrapper for the four pinned CLI commands.
- Add deploy/tests/openai_compatible_shim_test.py.
- Modify Dockerfile.harness-worker, Compose, Helm values/templates, deployment
  contract tests, the live matrix, and the compatibility inventory.

## Task 1: Explicit Integration Modes and Pinned Registry Entries

**Files:**
- Modify: src/server/harness/middleware/harness.rs
- Test: src/server/harness/tests/external_adapters.rs
- Test: src/server/harness_worker/lib.rs

- [ ] Step 1: Write the failing registry tests.

Add tests asserting that HarnessRegistry::with_defaults has exactly twelve
descriptors and contains:

~~~rust
for id in [
    "omnisolo", "codex", "opencode", "deepseek", "pi", "kimi",
    "openhands", "openharness", "aider", "goose", "open-interpreter",
    "plandex",
] {
    assert!(ids.contains(&id), "missing harness {id}");
}
assert_eq!(
    registry.descriptor("aider").unwrap().metadata["integration_mode"],
    "openai_compatible"
);
assert_eq!(
    registry.descriptor("codex").unwrap().metadata["integration_mode"],
    "native"
);
~~~

Add a second test asserting these exact shim rows:

~~~rust
[
    ("aider", "0.86.0", "omnisolo/harness-worker-aider:0.86.0"),
    ("goose", "1.33.1", "omnisolo/harness-worker-goose:1.33.1"),
    (
        "open-interpreter",
        "0.4.2",
        "omnisolo/harness-worker-open-interpreter:0.4.2",
    ),
    ("plandex", "cli/v2.2.1", "omnisolo/harness-worker-plandex:2.2.1"),
]
~~~

Each row must report HarnessProtocolKind::OpenAiCompatibleShim and
integration_mode=openai_compatible.

- [ ] Step 2: Run the focused tests and verify the intended failure.

Run:

~~~bash
cargo test -p server_harness --test external_adapters default_registry_contains_eight_native_and_four_openai_shim_harnesses -- --exact
cargo test -p server_harness --test external_adapters shim_presets_have_exact_pins_and_explicit_protocol_kind -- --exact
~~~

Expected: compile or test failure because the new protocol kind and registry
entries do not exist.

- [ ] Step 3: Implement the smallest registry change.

Add OpenAiCompatibleShim with runtime family LegacyJsonLines. Add
HarnessIntegrationMode with Native and OpenAiCompatible variants, defaulting
legacy descriptors to Native. Add four ExternalHarnessPreset variants with
the exact pins, image names, executable omnisolo-openai-shim, and metadata
integration_mode=openai_compatible. Keep all eight existing protocol kinds,
aliases, versions, images, and capabilities unchanged. Map the four new IDs in
protocol_for_harness.

- [ ] Step 4: Run the focused tests and existing adapter suite.

~~~bash
cargo test -p server_harness --test external_adapters default_registry_contains_eight_native_and_four_openai_shim_harnesses -- --exact
cargo test -p server_harness --test external_adapters shim_presets_have_exact_pins_and_explicit_protocol_kind -- --exact
cargo test -p server_harness --test external_adapters
~~~

Expected: all pass.

- [ ] Step 5: Commit the registry change.

~~~bash
git add src/server/harness/middleware/harness.rs src/server/harness/tests/external_adapters.rs
git commit -m "feat(harness): register explicit shim integration modes"
~~~

## Task 2: Scoped Local-Service Contract

**Files:**
- Create: src/server/harness/middleware/local_services.rs
- Modify: src/server/harness/middleware/mod.rs
- Modify: src/server/harness/middleware/harness.rs
- Modify: src/server/harness/middleware/types.rs
- Test: src/server/harness/tests/local_services.rs

- [ ] Step 1: Write failing scope and serialization tests.

Create local_services.rs as an integration target and test that a default
bundle resolves exactly eight kinds:

~~~rust
let registry = LocalServiceRegistry::with_defaults();
let bundle = registry.resolve(LocalServiceScopeContext::for_attempt(
    "tenant-a",
    Some("project-a"),
    Some("workspace-a"),
    Uuid::from_u128(1),
    Some(Uuid::from_u128(2)),
    Some(Uuid::from_u128(3)),
)).unwrap();
assert_eq!(bundle.bindings.len(), 8);
let encoded = serde_json::to_string(&bundle).unwrap();
for forbidden in ["token", "secret", "authorization", "password", "cookie"] {
    assert!(!encoded.to_ascii_lowercase().contains(forbidden));
}
~~~

Add authorization assertions that tenant-b cannot use a tenant-a binding, that
browser capabilities are rejected on a memory binding, and that memory.read is
accepted for the tenant-a memory binding.

- [ ] Step 2: Run the tests and observe the missing-module failure.

~~~bash
cargo test -p server_harness --test local_services
~~~

Expected: failure because the local service types do not exist.

- [ ] Step 3: Implement typed descriptors, bindings, and authorization.

Add these public types:

~~~rust
pub const LOCAL_SERVICE_BUNDLE_SCHEMA: &str =
    "omnisolo.local_service_bundle.v1";

pub enum LocalServiceKind {
    Memory, Mcp, Workspace, Artifact, Browser, Cache,
    Integration, ProviderFacade,
}

pub enum LocalServiceScope {
    Tenant, Project, Workspace, Session, Task, Attempt,
}

pub struct LocalServiceDescriptor {
    pub service_id: String,
    pub kind: LocalServiceKind,
    pub implementation_id: String,
    pub implementation_version: String,
    pub state_locality: String,
    pub scope_model: LocalServiceScope,
    pub capabilities: BTreeSet<String>,
    pub configuration_digest: String,
}

pub struct LocalServiceBinding {
    pub binding_id: Uuid,
    pub service_id: String,
    pub kind: LocalServiceKind,
    pub scope: LocalServiceScope,
    pub tenant_id: String,
    pub project_id: Option<String>,
    pub workspace_id: Option<String>,
    pub session_id: Uuid,
    pub task_id: Option<Uuid>,
    pub attempt_id: Option<Uuid>,
    pub granted_capabilities: BTreeSet<String>,
    pub generation: i64,
    pub configuration_digest: String,
}

pub struct LocalServiceBundle {
    pub schema: String,
    pub bindings: Vec<LocalServiceBinding>,
}
~~~

Derive Clone, Debug, Deserialize, PartialEq, and Serialize for the portable
types, with ordering derives on enums. Implement
LocalServiceRegistry::with_defaults with exactly Memory, Mcp, Workspace,
Artifact, Browser, Cache, Integration, and ProviderFacade. Implement resolve,
binding(kind), validate, and authorize. Resolve rejects blank tenant/session
identity, missing project/workspace IDs for their scopes, duplicate kinds, and
non-positive generations. Authorize checks tenant equality, binding state,
generation, and capability membership. No portable type may contain a token,
credential, cookie, socket, process ID, or raw endpoint.

- [ ] Step 4: Attach service bundles to request and runtime context.

Add an optional local_service_bundle field to HarnessSessionRequest and
RuntimeConfigSnapshot with serde defaults. Add
HarnessSessionRequest::with_local_service_bundle. Include the bundle in
request_context under local_services, preserving empty/absent bundles for old
callers.

- [ ] Step 5: Run focused and existing serialization suites.

~~~bash
cargo test -p server_harness --test local_services
cargo test -p server_harness --test protocol
cargo test -p server_harness --test external_adapters
~~~

Expected: all pass.

- [ ] Step 6: Commit the local-service contract.

~~~bash
git add src/server/harness/middleware/local_services.rs src/server/harness/middleware/mod.rs src/server/harness/middleware/harness.rs src/server/harness/middleware/types.rs src/server/harness/tests/local_services.rs
git commit -m "feat(harness): add scoped local service bindings"
~~~

## Task 3: Portable Service References and Rebinding

**Files:**
- Modify: src/server/harness/middleware/capsule.rs
- Modify: src/server/harness/middleware/grpc.rs
- Modify: src/server/harness/middleware/harness.rs
- Test: src/server/harness/tests/schema_contract.rs
- Test: src/server/harness/tests/worker_grpc.rs

- [ ] Step 1: Write failing capsule and request-context tests.

Compile a session capsule with a local bundle and assert that binding IDs,
scope IDs, generations, and configuration digests survive round-trip. Assert
that the serialized capsule contains none of authorization, facade_token,
browser_cookie, or upstream key values. Add a gRPC context test supplying
context.local_services and asserting the resulting HarnessSessionRequest has
the same binding IDs and tenant/session identity.

- [ ] Step 2: Run the tests and verify the missing-field failure.

~~~bash
cargo test -p server_harness --test schema_contract
cargo test -p server_harness --test worker_grpc
~~~

Expected: compile or assertion failure because capsule and parser types do not
carry local-service bindings.

- [ ] Step 3: Add portable manifest fields and integrity validation.

Add local_service_bindings: Vec<LocalServiceBinding> to SessionManifest and
CapsuleCompileInput, defaulting older JSON to an empty vector. Copy the field
through clone and compile. During verify_integrity, validate every binding and
require its tenant and session to match the manifest. Authority-bearing values
remain impossible to serialize because they are absent from the type.

- [ ] Step 4: Parse, validate, and propagate bundles at the worker boundary.

In apply_request_context, parse context.local_services as a
LocalServiceBundle, validate it against request.tenant_id and
request.session_id, and return Status::invalid_argument for malformed or
cross-tenant bindings. Include it in every native and shim adapter request.
Emit local_services.bound before the adapter's first native event, containing
only service IDs, kinds, generations, scopes, and granted capabilities.

- [ ] Step 5: Run worker and conformance suites.

~~~bash
cargo test -p server_harness --test schema_contract
cargo test -p server_harness --test worker_grpc
cargo test -p server_harness --test middleware_conformance
~~~

Expected: all existing tests and new binding assertions pass.

- [ ] Step 6: Commit portable service references.

~~~bash
git add src/server/harness/middleware/capsule.rs src/server/harness/middleware/grpc.rs src/server/harness/middleware/harness.rs src/server/harness/tests/schema_contract.rs src/server/harness/tests/worker_grpc.rs
git commit -m "feat(harness): rebind scoped services across capsules"
~~~

## Task 4: OpenAI-Compatible Provider Facade

**Files:**
- Create: src/server/harness/middleware/provider_facade.rs
- Modify: src/server/harness/Cargo.toml
- Modify: src/server/harness/BUILD.bazel
- Test: src/server/harness/tests/provider_facade.rs

- [ ] Step 1: Write failing facade tests.

Use an in-process Axum upstream fixture that records authorization and returns
a valid Responses body. Add tests that:

~~~rust
let facade = ProviderFacade::start(
    upstream.url(),
    "upstream-secret",
    selection("gpt-5.6-luna"),
).await.unwrap();
let response = reqwest::Client::new()
    .post(format!("{}/responses", facade.route().base_url()))
    .bearer_auth(facade.route().token())
    .json(&json!({
        "model": "gpt-5.6-luna",
        "input": "hello",
        "stream": false
    }))
    .send().await.unwrap();
assert_eq!(response.status(), reqwest::StatusCode::OK);
assert_eq!(
    upstream.last_authorization().await,
    "Bearer upstream-secret"
);
~~~

Add a second test covering wrong token, wrong model, GET/POST mismatch, and
unallowlisted /admin path. Add a redaction assertion over Debug and error
strings.

- [ ] Step 2: Run the focused tests and observe the missing-type failure.

~~~bash
cargo test -p server_harness --test provider_facade
~~~

Expected: compile failure because ProviderFacade does not exist.

- [ ] Step 3: Implement route types and the Axum proxy.

Add ProviderFacadeRoute with base_url and token, ProviderFacadeConfig with
upstream URL/key and immutable ResolvedModelSelection, and ProviderFacade with
route, one-shot shutdown, and server task. Bind 127.0.0.1:0 and return a route
rooted at /v1. Accept only GET /v1/models, POST /v1/responses, and POST
/v1/chat/completions. Require exact Bearer token, reject model changes, strip
child authorization and arbitrary headers, add the protected upstream Bearer
header, and forward status/content type/body. Use bytes_stream for response
streaming. Redact upstream key from all errors and Debug output. Shutdown must
await the server task.

- [ ] Step 4: Run focused facade tests and dependency checks.

~~~bash
cargo test -p server_harness --test provider_facade
cargo test -p server_harness --lib provider_facade
~~~

Expected: route, rejection, streaming, and redaction tests pass.

- [ ] Step 5: Commit the provider facade.

~~~bash
git add src/server/harness/middleware/provider_facade.rs src/server/harness/Cargo.toml src/server/harness/BUILD.bazel src/server/harness/tests/provider_facade.rs
git commit -m "feat(harness): add scoped OpenAI provider facade"
~~~

## Task 5: Route Native Workers Through the Facade

**Files:**
- Modify: src/server/harness_worker/lib.rs
- Modify: src/server/harness/middleware/harness.rs
- Modify: src/server/harness/middleware/grpc.rs
- Test: src/server/harness_worker/tests/worker_e2e.rs
- Test: src/server/harness/tests/worker_grpc.rs

- [ ] Step 1: Write failing child-isolation tests.

Add a worker fixture whose child prints OPENAI_API_BASE_URL and
OPENAI_API_KEY, then calls the loopback /models route. Assert that the child
sees a 127.0.0.1 URL and a token different from the configured upstream key,
while the upstream fixture sees the configured key. Add a model-mismatch test
that proves the facade rejects before upstream invocation.

- [ ] Step 2: Run the tests and observe direct-key behavior.

~~~bash
cargo test -p omnisolo_harness_worker --test worker_e2e worker_child_receives_only_scoped_provider_route -- --exact
cargo test -p server_harness --test worker_grpc
~~~

Expected: the isolation assertion fails because current process specs inject
the upstream key directly.

- [ ] Step 3: Move upstream credentials to worker-owned facade state.

Add a redacted provider_api_key SecretValue to WorkerConfig for first-party and
process workers. Keep deprecated environment names as input aliases, but
remove the upstream value from child-facing ProcessHarnessSpec before the
service is constructed. When a key is configured, start ProviderFacade before
preflight, point the OmniSolo OpenAiResponsesClient at the facade route, and
call ProcessHarnessSpec::with_provider_facade_route for process branches.

That method must rewrite OPENAI_API_BASE_URL, OPENAI_BASE_URL, and Codex
provider argument values, and set the child API key to the opaque facade token.
When no key is configured, preserve existing fixture-only workers that do not
make provider calls. On startup failure or graceful shutdown, close the facade.

- [ ] Step 4: Bind local services for every attempt.

Require LocalServiceRegistry::validate for request bundles. Emit
local_services.bound before native first events. On cancellation, quiesce,
delete, process exit, and adapter drop, revoke the binding and close the
provider route while retaining durable memory, workspace, artifact, and
project-scoped local state.

- [ ] Step 5: Update secret expectations and run native suites.

Update existing secret tests to assert the upstream key remains only inside
worker-owned SecretValue. The child environment, generated arguments, events,
capsules, and debug output must contain only the facade token or no key.

~~~bash
cargo test -p server_harness --tests
cargo test -p omnisolo_harness_worker --tests --no-run
cargo test -p omnisolo_harness_worker --test worker_e2e
~~~

Expected: deterministic native suites pass and no secret-canary appears in
serialized requests, debug output, child artifacts, or event payloads.

- [ ] Step 6: Commit native facade integration.

~~~bash
git add src/server/harness_worker/lib.rs src/server/harness/middleware/harness.rs src/server/harness/middleware/grpc.rs src/server/harness_worker/tests/worker_e2e.rs src/server/harness/tests/worker_grpc.rs
git commit -m "feat(harness): route native workers through provider facade"
~~~

## Task 6: Generic OpenAI-Compatible Shim Runner

**Files:**
- Modify: src/server/harness/middleware/harness.rs
- Create: src/server/harness/sidecars/openai_compatible_shim.py
- Test: src/server/harness/tests/external_adapters.rs
- Test: deploy/tests/openai_compatible_shim_test.py

- [ ] Step 1: Write failing shim lifecycle tests.

Add tests proving that a shim creates a synthetic session ID, executes one
prompt through a fixture command, returns assistant.final and usage.recorded,
and deletes its process. Add a Python fixture with a fake command that prints
its environment and a deterministic marker.

Required behavior:

~~~text
stdin JSONL request -> one child command with prompt on stdin
child stdout -> assistant.final text
child exit 0 -> terminal success and usage event
child exit nonzero -> typed process failure
~~~

- [ ] Step 2: Run focused tests and observe missing shim behavior.

~~~bash
cargo test -p server_harness --test external_adapters shim_adapter_executes_fixture_command -- --exact
python3 -m unittest discover -s deploy/tests -p 'openai_compatible_shim_test.py'
~~~

Expected: failures because the new protocol kind still reaches generic legacy
dispatch and the sidecar is absent.

- [ ] Step 3: Implement shim process behavior and sidecar.

Add uses_openai_compatible_shim to ProcessHarnessAdapter. In this mode,
create_session returns shim:<session UUID> without claiming native resume;
execute validates attempt/prompt, starts the isolated sidecar, writes the
prompt, reads bounded stdout/stderr, and returns final plus usage; resume
accepts only the active synthetic ID; fork creates a new ID; close/delete,
cancellation, and drop terminate the child.

The model binding event must include integration_mode=openai_compatible.
Service bundles are passed as redacted JSON configuration or a declared tool
bridge, never raw service credentials.

Create openai_compatible_shim.py with a fixed mapping for aider, goose,
open-interpreter, and plandex. Reject unknown IDs, shell metacharacters, and
unallowlisted arguments. Clear inherited environment, set only facade variables
and allowlisted workspace values, enforce a timeout, and redact the facade
token from diagnostics.

- [ ] Step 4: Run shim and native suites.

~~~bash
cargo test -p server_harness --test external_adapters
python3 -m unittest discover -s deploy/tests -p 'openai_compatible_shim_test.py'
cargo test -p server_harness --tests
~~~

Expected: four shim fixtures pass and all native suites remain green.

- [ ] Step 5: Commit the shim runner.

~~~bash
git add src/server/harness/middleware/harness.rs src/server/harness/sidecars/openai_compatible_shim.py src/server/harness/tests/external_adapters.rs deploy/tests/openai_compatible_shim_test.py
git commit -m "feat(harness): add pinned OpenAI-compatible shim runner"
~~~

## Task 7: Twelve-Row Deployment and Inventory Contracts

**Files:**
- Modify: deploy/docker/Dockerfile.harness-worker
- Modify: deploy/docker-compose.yml
- Modify: deploy/helm/ohc/values.yaml
- Modify: deploy/helm/ohc/templates/harness-workers.yaml
- Modify: deploy/helm/ohc/templates/harness-workers-hpa.yaml
- Modify: deploy/tests/harness_worker_deployment_contract_test.sh
- Modify: scripts/test-live-harness-matrix.sh
- Modify: docs/omnisolo-harness-compatibility-inventory.md

- [ ] Step 1: Write failing deployment assertions.

Extend the deployment contract test to require twelve exact image rows, harness
IDs, protocol kinds, pins, and ports. Require each shim target to retain the
native security profile: read-only root, no-new-privileges, all capabilities
dropped, isolated /tmp, /workspace, and /home/omnisolo. Reject latest in all
twelve worker rows.

- [ ] Step 2: Run the deployment contract and observe missing-target failures.

~~~bash
bash deploy/tests/harness_worker_deployment_contract_test.sh
~~~

Expected: failure listing the missing shim targets.

- [ ] Step 3: Add pinned image stages and deployment rows.

Add exact pinned stages or verified wrapper entrypoints for Aider 0.86.0,
Goose 1.33.1, Open Interpreter 0.4.2, and Plandex cli/v2.2.1. Wire each to
omnisolo-openai-shim with protocol openai_compatible_shim. Keep provider base
URL/model/reasoning non-secret and source the upstream key only through the
worker facade's protected secret input.

Add twelve matrix rows with separate integration_mode and
shared_service_probe fields. Keep native rows first and make the script exit
nonzero if any native row fails.

- [ ] Step 4: Run deployment rendering and contract checks.

~~~bash
bash deploy/tests/harness_worker_deployment_contract_test.sh
docker compose -f deploy/docker-compose.yml config
~~~

Expected: deployment checks pass and Compose renders without missing new
variables. If image builds are unavailable, record that as a live prerequisite.

- [ ] Step 5: Commit deployment and inventory changes.

~~~bash
git add deploy/docker/Dockerfile.harness-worker deploy/docker-compose.yml deploy/helm/ohc/values.yaml deploy/helm/ohc/templates/harness-workers.yaml deploy/helm/ohc/templates/harness-workers-hpa.yaml deploy/tests/harness_worker_deployment_contract_test.sh scripts/test-live-harness-matrix.sh docs/omnisolo-harness-compatibility-inventory.md
git commit -m "feat(harness): deploy twelve pinned compatibility workers"
~~~

## Task 8: Shared-Service Conformance and Native-First Live Gate

**Files:**
- Modify: src/server/harness_worker/tests/live_harness_matrix.rs
- Modify: scripts/test-live-harness-matrix.sh
- Modify: src/server/harness/tests/middleware_conformance.rs

- [ ] Step 1: Write the shared-service conformance test.

Use one LocalServiceRegistry and one scope context to bind two different
harness adapters. Have adapter A write a namespaced memory/artifact sentinel
through the test gateway and adapter B read it. Assert visibility, reject a
tenant mismatch, verify MCP authorization, verify workspace snapshot IDs, and
assert that browser process identity is not transferred.

- [ ] Step 2: Run the conformance test and observe the missing wiring failure.

~~~bash
cargo test -p server_harness --test middleware_conformance shared_service_bindings_are_visible_across_harnesses -- --exact
~~~

Expected: failure until bundles are attached to both adapters and capability
checks are enforced.

- [ ] Step 3: Implement the deterministic cross-harness probe.

Run the probe for all eight native descriptors, using different writer/reader
pairs for each service class. Store IDs and digests only in events. Keep
existing memory backend selection authoritative; the probe must wrap the
configured SQLite/Postgres/Redis/file/vector backend. A Mem0 backend, when
enabled, uses the same namespace contract.

- [ ] Step 4: Extend live evidence with native-first ordering.

Each row must record:

~~~json
{
  "harness_id": "codex",
  "native_protocol": "codex_app_server",
  "integration_mode": "native",
  "provider_turn": "passed",
  "shared_service_probe": "passed",
  "reasoning_translation": "max",
  "cleanup": "passed"
}
~~~

Run all eight native rows first. Return nonzero for any native failure, missing
service probe, or secret artifact. Only after the native gate passes may the
four shim rows run.

- [ ] Step 5: Run the complete deterministic suite.

~~~bash
cargo test -p server_harness --tests
cargo test -p omnisolo_harness_worker --tests
bash deploy/tests/harness_worker_deployment_contract_test.sh
git diff --check
~~~

Expected: all deterministic tests pass, deployment checks pass, and no
whitespace errors are reported.

- [ ] Step 6: Run the opt-in live matrix when prerequisites exist.

~~~bash
OMNISOLO_LIVE_HARNESS_E2E=1 bash scripts/test-live-harness-matrix.sh
~~~

Expected: twelve machine-readable rows, with all eight native rows passing
before shim rows are accepted. If the live flag, key, images, or network is
unavailable, report the exact skipped prerequisite rather than claiming live
completion.

## Final Review and Handoff

- [ ] Step 1: Review the full change against the approved specification.

Map every completion criterion to a passing test or live-evidence field. Search
for authority leaks:

~~~bash
rg -n -i "facade_token|authorization|openai_api_key|password|cookie|secret" src/server/harness src/server/harness_worker deploy scripts
~~~

Every match must be an allowlisted constant, redaction test, or protected
secret-handling path.

- [ ] Step 2: Request code review before integration.

Use the requesting-code-review skill with the implementation commits and this
plan. Fix all Critical and Important findings before finishing.

- [ ] Step 3: Finish the development branch.

Use the finishing-a-development-branch skill. Preserve unrelated dirty files,
present the verified commit set, and report unavailable live prerequisites
separately from deterministic results.
