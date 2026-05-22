# One Human Corp

> [!IMPORTANT]
> This repository is auto-maintained and developed with AI bots. No human is interacting with issues or pull requests in this repository. If you have a question, start a Discussion instead.

## Getting Started (Day One Onboarding)

To begin your onboarding journey, we provide a **unified Master CLI** that handles all developer setup, environment configuration, and agent provisioning in a single interactive experience.

From the root of the repository, you must explicitly run the onboarding CLI:

```bash
./deploy/scripts/ohc_hybrid_cli.sh
```

**What this does:**
- 🚀 Guides you through the **Developer Setup**
- ⚙️ Configures your **Environment Variables**
- 🩺 Runs deep system **Diagnostics**
- 🔄 Allows seamless switching between `cloud`, `standalone`, and `headless` modes

This premium onboarding flow eliminates friction and ensures maximum developer velocity for Day One setup.

## Identity

One Human Corp employs the **OHC-HA Hybrid Architecture** for its identity and security framework, ensuring zero-trust verification seamlessly across both localized and cloud-native deployments.

The platform implements a hybrid identity model:
- **Agent Identity**: Relies on SPIFFE/SPIRE for universal workload identity, ensuring every inter-agent communication and tool call is cryptographically signed and mTLS validated.
- **Human Identity**: Utilizes OIDC (OpenID Connect) for human users, mapping human authentication directly into the internal SPIFFE trust domain.

## Product Vision & Market Strategy

One Human Corp (OHC) is the world's first **Hybrid Agentic OS**. For a deep dive into our competitive advantages and "Unfair Advantage" against Claude Code and Replit Agent, see the **[OHC Market Strategy](docs/vision/market_strategy.md)**.

## Architecture

The platform supports four operating modes:

| Mode | Local footprint | Remote footprint | Notes |
|------|-----------------|------------------|-------|
| **Cloud-native shared service** | Tauri v2 desktop client | Rust API server, Postgres, agents, optional Redis and Chatwoot | Set `OHC_MULTITENANT=true`. Scale stateless API pods horizontally while Postgres remains the consistency boundary. |
| **Headless cloud API** | Tauri desktop client | API-only Rust server | Set `OHC_HEADLESS=true` when the backend should expose APIs, health probes, metrics, and auth without serving the web UI. |
| **Desktop standalone** | Tauri v2 desktop shell plus local Rust backend and SQLite-backed SIPDB | Optional public SaaS integrations only | Optimized for local resource usage; Redis and Chatwoot are not required for the standalone wrapper flow. |
| **Single-machine integration stack** | Full local Docker Compose stack | None | Useful for development, demos, and end-to-end verification on one machine. |

```mermaid
graph TD;
    DesktopClient[Tauri v2 Desktop App\nStandalone or Remote] --> API[Rust Server / API];
    LegacyWeb[Legacy Next.js Prototype] -.-> API;
    API --> Orchestration[Orchestration Hub];
    API --> Auth[JWT / OIDC Auth];
    Orchestration --> Agents[AI Agents];
    API --> Postgres[(Postgres)];
    API --> SQLite[(Local SQLite SIPDB)];
    API --> Integrations[Public Integrations\nGoogle Chat / Issue Trackers / MCP];
```

### Source layout

| Directory | Language | Purpose |
|-----------|----------|---------|
| `src/ui/tauri/` | **Rust/HTML/JSON** | Canonical Tauri v2 desktop UI and packaged static frontend |
| `src/ui/next/` | **React/TypeScript** | Legacy/prototype Next.js web client retained until route and asset references are fully audited |
| `src/app/` | **Rust** | Backend agent client (formerly Slint UI, deprecated) |
| `src/server/` | **Rust** | API server, auth, dashboard handlers, integrations, billing, and runtime wiring |
| `src/agents/` | **Rust** | Built-in agent implementations |
| `src/proto/` | **Protobuf** | gRPC service definitions |
| `src/tests/` | **Python / Shell** | Repository-local validation helpers and test utilities |
| `src/e2e/` | **TypeScript** | Playwright E2E tests |
| `deploy/` | **YAML / Shell** | Docker Compose, Helm charts, and deployment helpers |
| `docs/` | **Markdown** | Architecture, roadmap, feature specs, and developer documentation |

### KAIROS Orchestration Documentation

The Swarm is powered by the KAIROS engine which maintains stability via three core pillars. For deep architectural dives into these systems, consult the feature documentation:
- **[Distributed State Machine](docs/features/kairos/distributed_state_machine.md):** Learn how agent transitions are rigorously tracked to prevent deadlocks.
- **[Sub-Agent Queue](docs/features/kairos/sub_agent_queue.md):** Learn how vast amounts of agent tasks are routed securely in the background.
- **[AutoDream Pipeline](docs/features/kairos/autodream_pipelines.md):** Learn how episodic memory is intelligently converted to long-term embedded vector truth.

### Remote clients and standalone mode

The Tauri v2 desktop app supports a configurable Backend URL and a standalone-mode toggle. In standalone mode the desktop app manages a local backend lifecycle. In remote-client mode the same app acts as a pure UI and talks to a cloud-hosted OHC server over the API.

Headless server deployments keep the API, auth, health probes, and metrics online while skipping static UI serving. That is the intended mode for mobile clients and desktop clients that should connect to cloud-hosted services instead of running a local backend.

### Multi-tenancy

In cloud-native mode (`OHC_MULTITENANT=true`) the `TenantRegistry` in
`src/server/dashboard/tenant.go` lazily provisions an organisation-scoped
`Server` per `organization_id` and routes authenticated requests to the correct
tenant handler. Dashboard snapshots, meetings, agent operations, approvals,
handoffs, and other server-local state are isolated per tenant handler, and the
HTTP layer filters org-visible data when shared persistence is used.

Shared-database persistence hardening is still ongoing, so the repo should not
yet claim perfect end-to-end schema-level tenant isolation for every future
query path. The runtime now supports the correct routing model and org-scoped
API surface for shared-service deployments.

New organisations are provisioned via:
```
POST /api/orgs/register   { "id": "acme", "name": "Acme Corp", "domain": "acme.com" }
```
After provisioning, users whose JWT includes `"organization_id": "acme"` are
routed exclusively to the Acme tenant.

## Quick Start

### Docker (single-machine deployment)

```bash
bazelisk run //:deploy_dev
```

Services:
| Service | Port | Description |
|---------|------|-------------|
| `server` | 8080 | Rust API server, auth endpoints, and optional embedded UI |
| `postgres` | 5432 | PostgreSQL |
| `redis` | 6379 | Redis |
| `chatwoot` | 3002 | Chat platform |
| `prometheus` | 9090 | Metrics |
| `grafana` | 3000 | Dashboards |

When the backend starts with an empty workforce, it now bootstraps an **internal default agent** backed by the built-in provider so a single-container deployment has an immediately available agent runtime.

For API-only remote-client deployments, set `OHC_HEADLESS=true` on the server.

### Bazel (full build + test)

```bash
# Build & Test the full system
bazelisk build //...
bazelisk test //...

# Run E2E tests (requires Docker for postgres/redis)
bazelisk test //src/e2e:playwright

# Quick Local Dev (run these in separate terminals)
bazelisk run //src/server:server
bazelisk run //src/ui/tauri:app
```

### E2E Tests with Playwright

The `//src/e2e:playwright` target runs the Playwright E2E suite against the real server, real browser UI, Postgres, and Redis. The suite currently discovers 250 tests across 61 spec files.

E2E tests follow a strict no-substitution contract:
- Test data is seeded only through the database, using `src/e2e/e2e-seed.sql`.
- The Playwright global setup in `src/e2e/global-setup.ts` waits for the real app, seeds Postgres, and signs in through the visible login UI.
- Every browser spec imports `src/e2e/fixtures.ts`, which logs in as the seeded admin user before each test.
- A seeded regular team member is also available through the `memberPage` fixture, so role-sensitive tests can verify the real member experience.
- Playwright network substitution is blocked in the shared fixture. Tests should exercise product UI and backend behavior, not intercepted API responses.

Seeded E2E users:

| Role | Email / username | Password |
|------|------------------|----------|
| Admin | `test@example.com` | `password123` |
| Team member | `member@example.com` | `MemberPass123!` |

Run the full Bazel-managed suite:

```bash
bazelisk test //src/e2e:playwright
```

Run the local Playwright suite against an already running app:

```bash
DATABASE_URL=postgres://ohc:ohc@localhost:5432/ohc \
REDIS_URL=redis://localhost:6379 \
MINIMAX_API_KEY=... \
npx playwright test
```

AI-generating tests may call the real MiniMax API. When a test validates generated output, it must use the AI judge helper in `src/e2e/ai-judge.ts`; the helper asks MiniMax to score the output from 0 to 10 and the test only passes when the score is greater than 9.

Tests capture screenshots on every page to `test-results/screenshots/` and explicit `page.screenshot()` calls save to `test-results/*.png`.

### Tauri v2 Desktop App

```bash
bazelisk run //src/ui/tauri:app
```

The app connects to the server at `http://127.0.0.1:18789` by default.

### Legacy Web UI (Next.js Prototype)

```bash
cd src/ui/next && npm run dev
```

This starts the legacy Next.js development server on `http://localhost:3000`. Do not add new provider flows here; build new user-facing UI in Tauri.

### Server binary

```bash
bazelisk run //src/server:server
```

## Configuration

| Variable | Description |
|----------|-------------|
| `GEMINI_API_KEY` | Google Gemini API key |
| `MINIMAX_API_KEY` | MiniMax API key used by real AI-generating E2E flows, AI judge scoring, and `OHC_LLM_PROVIDER=minimax` agent runs |
| `ANTHROPIC_API_KEY` | Anthropic API key |
| `OPENAI_API_KEY` | OpenAI API key |
| `OHC_LLM_PROVIDER` | Builtin agent provider: `openai`, `openai-compatible`, `minimax`, `anthropic`, or `ollama` |
| `OHC_LLM_MODEL` | Builtin agent model name. Defaults are provider-specific when unset |
| `OHC_LLM_API_KEY` | Generic API key for `openai-compatible` providers, or fallback key for OpenAI/MiniMax |
| `OHC_LLM_BASE_URL` | Generic OpenAI-compatible API root such as `https://api.example.com/v1`; endpoint URLs ending in `/chat/completions` are normalized |
| `OPENAI_BASE_URL` | Optional OpenAI-compatible API root for `OHC_LLM_PROVIDER=openai` |
| `MINIMAX_BASE_URL` | Optional MiniMax-compatible API root; defaults to `https://api.minimax.chat/v1` |
| `DATABASE_URL` | PostgreSQL DSN. When unset, the server falls back to in-memory repositories and local SQLite-backed SIPDB support |
| `OHC_MULTITENANT` | Set `true` for multi-tenant cloud-native mode |
| `OHC_HEADLESS` | Set `true` for API-only deployments that should not serve the web UI |
| `OHC_SERVE_UI` | Optional override to force UI serving on or off |
| `OHC_CORE_URL` | URL of the Rust `ohc-core` sidecar |
| `MCP_BUNDLE_DIR` | Directory for MCP bundles |
| `FRONTEND_STATIC_DIR` | Path to compiled frontend assets. Tauri currently packages `src/ui/tauri/next_out`; `src/ui/next/out` is legacy/prototype output |
| `OHC_BOOTSTRAP_ORG_ID` | Optional bootstrap tenant ID used to serve unauthenticated routes in multi-tenant mode |
| `OHC_BOOTSTRAP_ORG_NAME` | Optional bootstrap tenant display name |
| `OHC_BOOTSTRAP_CEO_NAME` | Optional bootstrap tenant CEO name |
| `OHC_DEFAULT_AGENT_NAME` | Optional display name for the bootstrapped internal default agent |
| `OHC_DEFAULT_AGENT_ROLE` | Optional role for the bootstrapped internal default agent |
| `OHC_DEFAULT_AGENT_REGION` | Optional region/runtime label for the bootstrapped internal default agent (defaults to `docker`) |
| `OHC_DEFAULT_TENANT_ID` | Default tenant used by local E2E login when the browser form does not submit an explicit organization ID; defaults to `e2e-tenant` in the test harness |

Kubernetes secrets are used to inject credentials at runtime without committing them to source.

## Developer Workflow

### Setup and Mode Switching (Manual)

We provide helper scripts in `deploy/scripts/` to smooth the friction of developing against multiple hybrid targets. For day one setup, we recommend using the unified Master CLI (`./deploy/scripts/ohc_hybrid_cli.sh`) from the repository root instead.

- **Initial Setup:** `./deploy/scripts/ohc-setup.sh` (Generates `.env`, verifies builds, and provisions the workspace)
- **Mode Switching:** `source deploy/scripts/ohc-mode.sh [cloud|standalone|headless]` (Configures environment variables for the current terminal session)

### Build and Test

- **Build all modules:** `bazelisk build //...`
- **Run all tests:** `bazelisk test //...`
- **Run E2E tests:** `bazelisk test //src/e2e:playwright`
- **Run the server:** `bazelisk run //src/server:server`
- **Launch the Tauri app:** `bazelisk run //src/ui/tauri:app`
- **Run Rust lint with warnings as errors:** `bazelisk run //:rust_lint`
- **Build the legacy Next.js prototype:** `cd src/ui/next && npm run build`
- **Build the docs site:** `bazelisk run //:docs_build`

## Deprecated

### Next.js Prototype (Legacy)

`src/ui/next/` remains in the repository while references to its routes and assets are audited. It is not the canonical UI, and new provider flows should be implemented in the Tauri app.

### Slint UI (Deprecated)

The `src/app/slint_deprecated/` directory contains 72 legacy `.slint` files that were part of the old Slint-based UI. These files are no longer used - the UI has been migrated to Tauri v2.

Historical Slint components included: dashboard, login, wizard flows, agents, chat, channels, integrations, security, meetings, logs, pricing, scaling, swarm memory, website builder, setup wizard, task list, help center, release notes, tutorials, and API docs.

resolves #7869
