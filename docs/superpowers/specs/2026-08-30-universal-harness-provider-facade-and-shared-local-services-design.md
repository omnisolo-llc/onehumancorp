# Universal Harness Provider Facade and Shared Local Services Design

**Status:** Approved design; pending written-spec review.

**Extends:**
`2026-08-22-omnisolo-harness-middleware-design.md` and
`2026-08-23-openai-compatible-harness-model-routing-design.md`.

## Objective

Finish the eight harness integrations already advertised by OmniSolo, then add
a finite pinned set of four additional open-source harnesses through one
OpenAI-compatible provider facade. Every supported harness must retain access
to the local systems that OmniSolo already owns, including memory, MCP/tools,
workspace, artifacts, browser sessions, cache, integrations, and persistence.

The facade broadens model-provider compatibility. It does not make a harness a
native implementation of another harness protocol. Native protocol support and
OpenAI-compatible shim support remain separately reported capabilities.

The existing local implementations remain authoritative. This design adds a
shared binding and gateway contract around them; it does not replace the
SQLite, Postgres, Redis, file-backed, vector, MCP, workspace, or artifact
systems already used by OmniSolo.

## Scope

### Existing native harnesses

Stage one covers the eight entries currently advertised in the compatibility
inventory:

| Harness | Native transport | Worker image |
| --- | --- | --- |
| OmniSolo | In-process | `omnisolo/harness-worker:0.1.0` |
| Codex | App-server v2 JSON-RPC | `omnisolo/harness-worker-codex:0.149.0` |
| OpenCode | Native HTTP/SSE | `omnisolo/harness-worker-opencode:1.18.15` |
| DeepSeek Harness | Native JSON-RPC | `omnisolo/harness-worker-deepseek:0.1.1-rc.2` |
| Pi | Native RPC JSONL | `omnisolo/harness-worker-pi:0.73.1` |
| Kimi | ACP v1 JSON-RPC | `omnisolo/harness-worker-kimi:1.49.0` |
| OpenHands | Agent Server HTTP | `omnisolo/harness-worker-openhands:1.43.1` |
| AgentBoardTT OpenHarness | SDK sidecar JSONL | `omnisolo/harness-worker-openharness:0.6.0` |

The exact versions, source revisions, capabilities, and deployment targets are
owned by the pinned compatibility manifest and the deployment contract test.
An entry is not supported merely because it exists in the registry.

### Additional shim harnesses

Stage two adds only this finite set:

| Harness | Pin | Integration mode | Worker image |
| --- | --- | --- | --- |
| Aider | `v0.86.0` | OpenAI-compatible shim | `omnisolo/harness-worker-aider:0.86.0` |
| Goose | `v1.33.1` | OpenAI-compatible shim | `omnisolo/harness-worker-goose:1.33.1` |
| Open Interpreter | `v0.4.2` | OpenAI-compatible shim | `omnisolo/harness-worker-open-interpreter:0.4.2` |
| Plandex CLI | `cli/v2.2.1` | OpenAI-compatible shim | `omnisolo/harness-worker-plandex:2.2.1` |

The pins are based on the projects' official repositories and documented
provider configuration: [Aider](https://github.com/Aider-AI/aider),
[Goose](https://github.com/aaif-goose/goose),
[Open Interpreter](https://github.com/OpenInterpreter/open-interpreter), and
[Plandex](https://github.com/plandex-ai/plandex). Updating a pin is a new
compatibility change, not an implicit image rebuild.

This is a bounded compatibility set. Arbitrary user-supplied executables,
unreviewed rolling tags, and harnesses that require a different provider
protocol are outside this change. They may be added later only as explicit
pinned entries with their own adapter and acceptance row.

## Design Principles

1. **Native and shim modes are truthful.** A native adapter speaks the
   harness's declared protocol. A shim adapter only promises the capabilities
   it can provide through the common provider and service surfaces.
2. **One provider boundary.** Harnesses never need direct knowledge of the
   upstream provider credential or provider-specific deployment topology.
3. **One local-service boundary.** Durable local systems are resolved by
   OmniSolo and exposed through scoped service bindings, rather than copied
   into harness homes or mounted directly into child containers.
4. **Portable references, ephemeral authority.** Capsules may carry stable
   service references and non-secret policy metadata. They never carry tokens,
   credentials, process handles, raw sockets, or live leases.
5. **Existing stores remain valid.** Memory backend selection and existing MCP,
   workspace, artifact, cache, and persistence behavior remain available
   behind the new boundary.
6. **Capabilities are explicit.** The registry, live matrix, and API describe
   native protocol capabilities, provider compatibility, and local-service
   capabilities separately.
7. **Fail closed on authority.** Cross-harness handoff transfers observable
   state and policy references, not approval grants, database authority,
   browser process ownership, or provider secrets.

## System Architecture

```text
Client
  |
OmniSolo gateway and control plane
  |-- authentication, tenant policy, routing
  |-- model selection and attempt binding
  |-- LocalServiceRegistry
  |-- ProviderFacade
  |-- canonical event and capsule persistence
  |
  +-- Harness worker
       |-- native adapter or OpenAI-compatible shim adapter
       |-- per-attempt child process/server
       |-- scoped provider token and service bindings
       |
       +-- ProviderFacade routes to the configured upstream model API
       +-- LocalServiceGateway routes to existing OmniSolo local systems
            |-- memory backends, including a future Mem0 adapter
            |-- MCP/tool registry and reverse tunnel
            |-- workspace and artifact stores
            |-- browser/session service
            |-- cache, integrations, and persistence services
```

The provider facade and local-service gateway are internal control-plane
services. Their child-facing endpoints are allowlisted, authenticated, and
scoped to one attempt. They are not general-purpose network proxies.

The worker remains responsible for the native protocol lifecycle and canonical
event conversion. The gateway remains responsible for durable state,
authorization, provider credentials, service policy, and lease revocation.

## Provider Facade

### Request flow

1. The gateway resolves the requested model and reasoning effort into an
   immutable `ModelBinding` for the attempt.
2. The worker asks the facade for an attempt route. The route records the
   attempt ID, harness ID, model binding revision, API dialect, limits, and
   correlation namespace.
3. The facade returns an internal endpoint and short-lived opaque token. A
   process harness receives that endpoint and token through generated
   environment/configuration. It does not receive the upstream provider key.
4. The child may call only the configured health/model and inference paths,
   initially `/models`, `/responses`, and `/chat/completions`. The facade
   validates token audience, attempt, model, request limits, and correlation
   before forwarding upstream.
5. Upstream streaming chunks, usage, finish state, and provider errors are
   normalized by the facade. The native adapter converts them into canonical
   OmniSolo events and attaches the immutable model binding.
6. On cancellation, timeout, fencing, or deletion, the facade aborts the
   upstream request and revokes the attempt route.

The external deployment contract remains:

```text
OPENAI_API_KEY
OPENAI_API_BASE_URL
OPENAI_MODEL
OPENAI_REASONING_EFFORT
```

In facade mode, `OPENAI_API_KEY` inside a child is an opaque facade token with
the same variable name for upstream compatibility. The resolved upstream key
is held only by the protected facade/provider boundary. For the in-process
OmniSolo adapter, an authenticated internal client is used instead of placing
the token in a child environment.

The facade enforces one provider route per attempt. It does not allow a child
to change provider, model, base URL, reasoning effort, or arbitrary headers.
Provider-specific fields are preserved only when the selected adapter and
policy explicitly allow them.

### Model and reasoning semantics

Requested model intent and realized routing stay distinct:

- `RuntimeConfigSnapshot.requested_model` stores portable intent.
- `ModelBinding` stores the concrete model, provider route, limits, dialect,
  capability snapshot, and revision used by one attempt.
- `SessionCapsule` stores intent and historical non-secret bindings, never
  credentials or live route tokens.

The selection precedence remains task snapshot, session snapshot, then worker
defaults. `OPENAI_MODEL` defaults to `gpt-5.6-luna` and
`OPENAI_REASONING_EFFORT` defaults to `max` in the live OmniSolo deployment.

Every adapter must translate the reasoning value, reject it, or emit an
explicit typed capability downgrade. Silent loss of `max` is a failure.

## Shared Local-Service Plane

### Registry and binding contract

The control plane introduces a typed registry concept with these non-secret
records:

```text
LocalServiceDescriptor
  service_id
  kind
  implementation_id and version
  state_locality
  scope_model
  capabilities
  configuration_digest

LocalServiceBinding
  binding_id
  service_id
  tenant_id and project/workspace scope
  session/task/attempt association
  granted capabilities
  generation and lease metadata
  required/optional policy
```

The registry resolves a `LocalServiceBundle` when an attempt is admitted. A
binding is accepted only when the tenant, workspace, harness capability
snapshot, and policy all agree. Service-to-service credentials are never
forwarded to a harness.

The gateway maps typed service operations to existing implementations. MCP
continues to be the tool/resource protocol; not every local service is
converted into an MCP tool. Memory, workspace, artifact, browser, cache, and
integration APIs retain typed semantics, while the existing MCP registry and
reverse tunnel are reused for tool operations.

### Sharing semantics

| Service class | Durable owner | Default sharing scope | Authority rule |
| --- | --- | --- | --- |
| Memory, including existing stores and a future Mem0 adapter | OmniSolo memory service | Tenant/project/workspace; session records remain session-scoped | Every read/write is namespace- and tenant-checked |
| MCP server and tool catalog | OmniSolo MCP service | Project/workspace descriptor; invocation is attempt-scoped | Tool calls pass policy, approval, and correlation checks |
| Workspace and artifacts | Workspace/artifact service | Workspace/project snapshot and artifact IDs | Harnesses receive snapshot or artifact capabilities, never unrestricted host paths |
| Browser sessions | Browser service | Explicit session or task lease | Browser process and cookies remain service-owned |
| Cache and integration clients | Corresponding OmniSolo service | Tenant/project according to service policy | Child receives an operation capability, never a raw client credential |
| Provider facade | Provider control plane | Attempt route | Upstream key and provider authority remain gateway-owned |

The service plane makes state shareable without making all process state
shareable. For example, two harnesses can read and write the same memory
namespace or workspace snapshot, but they do not attach to the same browser
process, child stdin/stdout, database connection, or harness home.

Existing memory initialization and precedence remain intact. The registry
wraps the selected local backend rather than introducing a second competing
memory store. A Mem0 implementation, when configured, is another backend of
the memory service and must obey the same namespace, retention, redaction, and
cross-harness contract.

### Harness exposure

Each adapter maps the service bundle into the native surface available to its
harness:

- native MCP/tool protocols use the existing MCP client, registry, and reverse
  tunnel with attempt-scoped authorization;
- HTTP or SDK harnesses receive an internal service endpoint and scoped token;
- JSON-RPC/JSONL harnesses receive generated service configuration or a
  declared tool bridge;
- the in-process adapter uses typed service traits with the same policy checks;
- the shim adapters receive the same provider and service bundle, with only
  the surfaces supported by the pinned harness exposed.

If an upstream harness cannot consume a service surface, its descriptor must
say so and the adapter must return a typed unsupported-capability result. The
service is still preserved by OmniSolo and remains available to another
harness; it is not silently omitted or falsely advertised.

## Capsules, Handoff, and Lifecycle

Portable capsules contain stable service IDs, binding references, scope,
generation, capability snapshots, and configuration digests. They exclude:

- provider or service credentials;
- facade tokens and authorization headers;
- database URLs with embedded credentials;
- browser cookies, process IDs, and socket paths;
- raw harness home paths or host mounts;
- live leases and open stream handles.

On resume or cross-harness handoff, the target worker revalidates the capsule,
checks tenant and workspace policy, and requests fresh bindings. A service
generation or configuration digest mismatch produces an explicit migration or
stale-binding error; it does not silently attach to a different namespace.

The lifecycle is:

```text
unbound -> resolving -> bound -> active -> quiescing -> revoked -> closed
```

Cancellation revokes child tokens, aborts provider and service streams, releases
browser leases, and fences the attempt. Deletion removes ephemeral child homes,
generated configuration, route tokens, and attempt-owned leases. Durable memory,
workspace, artifacts, and project-scoped integrations follow their existing
retention policy.

Required service failures block admission or produce a structured terminal
error. Optional service failures produce an explicit capability-unavailable
event and do not change the selected provider or memory namespace.

## Security and Isolation

- Every service request carries an attempt and binding identity, either in the
  authenticated token or an equivalent internal credential context.
- The gateway checks tenant, project, workspace, session/task association,
  capability, generation, expiry, and correlation before dispatch.
- Child containers remain read-only with isolated temporary homes and no direct
  database, provider, browser, or host filesystem access.
- Generated configs and environment values are redacted in debug output.
- Secrets are excluded from events, capsules, model descriptors, service
  descriptors, logs, process arguments, and test artifacts.
- Approval history is portable; approval authority and active grants are
  re-evaluated for the target attempt.
- A child cannot use the provider facade as an arbitrary HTTP proxy or use a
  service token to mint another service token.
- Service calls are audited with stable service, binding, harness, attempt,
  and correlation IDs, but request payload redaction follows the existing
  secret and privacy policy.

## Rollout

### Stage 1: complete the existing eight

1. Preserve and tighten the current deterministic adapter, worker, deployment,
   and redaction tests.
2. Add the provider facade route to all eight native adapters while retaining
   their native session and event protocols.
3. Attach the shared local-service bundle to each native attempt.
4. Run the real provider matrix for all eight harnesses. Every row must perform
   model discovery, one real turn, native lifecycle validation, model/usage
   provenance, reasoning translation, and cleanup.
5. Run deterministic cross-harness service conformance. Each native adapter
   must use the same namespace and prove memory/artifact visibility across a
   different harness, plus MCP authorization and workspace isolation.

No shim harness is considered successful until all eight native rows pass.

### Stage 2: add the four pinned shims

1. Add one pinned image, registry preset, process contract, and adapter fixture
   for each additional harness.
2. Route every shim through the same provider facade and local-service bundle.
3. Report `openai_compatible` integration mode separately from native protocol
   kind and expose only measured capabilities.
4. Run the same provider, shared-service, security, cleanup, and deployment
   contract tests for all four rows.

The final matrix has twelve rows. A native row cannot be replaced by a shim row
to compensate for a failure.

## Test Strategy and Acceptance

Implementation follows test-driven development: each new behavior first gets a
failing deterministic test that demonstrates the intended failure.

### Deterministic tests

- Provider facade URL/path allowlisting, token audience, model binding, stream
  normalization, usage, cancellation, timeout, and error mapping.
- Native and shim environment/config generation with upstream secret
  redaction.
- Local service registry scope resolution, required/optional handling,
  generation checks, capability denial, lease revocation, and capsule
  round-trip without secrets.
- Shared memory read/write across two harness adapters using SQLite and the
  configured durable backends; the same contract covers a Mem0 backend when
  enabled.
- Cross-harness workspace snapshot and artifact visibility without raw path
  access.
- MCP/tool catalog visibility, per-attempt authorization, approval fencing,
  reverse-tunnel correlation, and cleanup.
- Browser lease ownership and non-transfer of browser process authority.
- Worker gRPC, native protocol, provider, local-service, deployment, and
  secret-scan suites for every registry entry.

### Real provider matrix

The opt-in live script keeps the existing credential behavior: the host may
provide `SUB2API_API_KEY`, which is mapped to the protected provider boundary
and never emitted in child artifacts. It checks the selected model at
`/models`, executes one no-tool turn through each named harness, validates
canonical correlation and terminal events, records model/reasoning/usage
provenance, and deletes the native session or child state.

The live matrix is emitted as machine-readable evidence with separate fields
for `native_protocol`, `integration_mode`, `provider_turn`,
`shared_service_probe`, `reasoning_translation`, and `cleanup`. Missing
credentials or an unset live-test flag means the suite is skipped; an enabled
row that fails is nonzero. Unsupported is not equivalent to passed.

### Completion criteria

This design is implemented only when:

1. All eight existing native registry entries pass deterministic and enabled
   live provider tests.
2. All eight native entries use the provider facade without bypassing their
   native protocol adapter.
3. All eight can resolve the shared local-service contract, and cross-harness
   tests prove namespace-safe memory and artifact sharing.
4. MCP, workspace, browser, cache, integration, and persistence services remain
   reachable through their declared scoped surfaces.
5. Capsules and event/log/config artifacts contain no provider or service
   credentials.
6. The four pinned shim harnesses pass the same provider, service, security,
   and cleanup contracts.
7. Deployment tests build or validate all twelve pinned worker targets without
   rolling tags or shared host homes.
8. The compatibility inventory reports native protocol support, provider
   compatibility, service capabilities, and reasoning downgrades separately.

## Proposed Implementation Boundaries

The implementation plan should extend the existing boundaries rather than
introduce a second execution system:

- harness middleware: registry mode, provider facade client/route, local
  service descriptors/bindings, capsule serialization, and capability reports;
- worker runtime: per-attempt route/bootstrap, service bundle admission,
  cancellation, and cleanup;
- existing agent memory/MCP/workspace/artifact services: typed gateway adapters
  that preserve current backend selection and persistence;
- deployment and inventory: one pinned manifest, twelve worker targets, and
  secret-safe environment/config generation;
- tests: native eight-stage gate, shim matrix, shared-service conformance,
  redaction, and live evidence.

The implementation must not make the harness worker a database client, mount
the host's harness homes, copy memory into portable capsules, or turn the
provider facade into an unrestricted proxy.
