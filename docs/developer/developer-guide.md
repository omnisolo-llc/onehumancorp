<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Developer Guide: One Human Corp

## Introduction
This guide is intended for engineers who want to contribute to the One Human Corp (OHC) platform. It covers everything from local setup to adding new features and deploying to Kubernetes.

## Operating Modes

The repo is intentionally built as a hybrid cloud-native and desktop product:

1. Cloud-native shared service: a horizontally scalable Go API tier backed by Postgres, with `OHC_MULTITENANT=true` enabling org-aware routing.
2. Headless API deployment: the same backend with `OHC_HEADLESS=true`, used by remote mobile or desktop clients that should not receive a hosted web UI.
3. Desktop standalone mode: the Flutter desktop app manages a local backend lifecycle and local SQLite-backed state.
4. Remote client mode: the Flutter app acts mainly as a UI, connects to a configured backend URL, and authenticates against a remote OHC deployment.

## Prerequisites
| Tool | Minimum Version | Install |
|------|----------------|---------|
| [Bazelisk](https://github.com/bazelbuild/bazelisk) | latest | `brew install bazelisk` or `go install github.com/bazelbuild/bazelisk@latest` |
| Go | 1.25 | managed by Bazel automatically |
| Node.js | 22 | only needed for IDE tooling; tests run inside Bazel |
| Docker | 24 | required for local Docker Compose and Kind e2e |
| [Kind](https://kind.sigs.k8s.io/) | 0.23+ | `brew install kind` |
| [Helm](https://helm.sh/) | 3.14+ | `brew install helm` |
| kubectl | 1.30+ | `brew install kubectl` |

## Setup
### 1. Clone the Repository
```bash
git clone https://github.com/onehumancorp/mono.git
cd mono
```

### 2. Configure Environment

We provide a specialized setup script that automatically checks prerequisites, writes a default `.env` file, and validates both **Cloud** and **Standalone** build targets before appending to the local runtime memory log (`OHC_MEMORY_DIR`, typically `.ohc/runtime/memory/`).

```bash
./deploy/scripts/ohc-setup.sh
```

### 3. Mode Switching CLI
The backend uses environment variables to dictate its operating mode. To quickly switch between `cloud`, `standalone`, and `headless` configurations in your shell, source the mode-switcher script:

```bash
source deploy/scripts/ohc-mode.sh standalone
# Configured for Standalone Desktop Mode.

source deploy/scripts/ohc-mode.sh cloud
# Configured for Cloud-Native Multi-Tenant Mode.
```

### 4. Local Development with Docker Compose
```bash
docker compose -f deploy/docker-compose.yml up --build
```
Navigate to `http://localhost:8080` to use the integrated API and UI stack. Set `OHC_HEADLESS=true` if you want the backend to run without serving the web client.

> All build and test commands below are issued as `bazel …` (via Bazelisk).

---

## Repository Layout

```
mono/
├── BUILD.bazel              Root build file
├── MODULE.bazel             Bazel module dependencies
├── WORKSPACE                Legacy WORKSPACE (kept for rules_go compat)
├── go.mod                   Go module
├── deploy/
│   ├── docker/              Dockerfiles (backend + frontend)
│   ├── docker-compose.yml   Local dev compose stack
│   ├── helm/ohc/            Helm chart (server, ui, Redis, CNPG)
│   └── tests/               Deploy artefact and Kind e2e tests
├── docs/
│   ├── features/            Feature-centric documentation (Design + CUJ + Guide)
│   ├── developer/           This guide
│   ├── templates/           Standard templates for docs
│   ├── system-design.md     Top-level architecture
│   └── roadmap.md           Strategic technical roadmap
└── srcs/
  ├── app/                 Flutter client for mobile, desktop, and web
  ├── proto/               Protobuf definitions
  └── server/              Go backend services and runtime entrypoint
      ├── agents/          Agent provider registry, workers, and MCP bundles
      ├── billing/         Usage tracking and model cost accounting
      ├── checkpointer/    LangGraph checkpoint persistence
      ├── dashboard/       HTTP handlers and UI-serving gateway
      ├── domain/          Domain model (Org / Dept / Role)
      ├── integrations/    External service integrations
      ├── orchestration/   Agent hub and meeting rooms
      └── tools/           Tooling dependency shims
```

---

## Available Bazel Commands

### Build

```bash
# Build everything
bazel build //...

# Build just the backend binary
bazel build //srcs/server:ohc

# Build the Flutter web app (via Bazel)
bazel build //srcs/app:app
```

### Test

```bash
# Run all tests
bazel test //...

# Run all Go unit tests
bazel test //srcs/server/...

# Run Flutter widget and service tests
bazel test //srcs/app/lib/...

# Run Flutter desktop e2e tests
bazel test //srcs/app:app_desktop_e2e_test

# Run Flutter web e2e tests
bazel test //srcs/app:app_web_e2e_test

# Run deploy artefact verification
bazel test //deploy:deploy_artifacts_test

# Run Kind cluster end-to-end smoke test
bazel test //deploy:kind_e2e_test

# Stream test output (useful for debugging)
bazel test //... --config=verbose

# Re-run tests even if cached
bazel test //... --cache_test_results=no

# Launch the local development environment (run these in separate terminals)
bazelisk run //srcs/server:ohc
bazelisk run //srcs/app:start

# Launch standalone desktop mode
bazelisk run //:desktop

# Build Linux package artifacts
bazelisk build //srcs/app:app_deb
# Requires rpmbuild on the host
bazelisk build //srcs/app:app_rpm
```

### Lint / Type-check

```bash
# Go vet (run via Bazel nogo)
bazel build //... --keep_going

# Flutter static analysis
cd srcs/app && flutter analyze
```

---

## Running Tests Locally

### Go Unit Tests

```bash
bazel test //srcs/server/...
```

### Flutter App Tests

```bash
bazel test //srcs/app/lib/...
bazel test //srcs/app:app_desktop_e2e_test
bazel test //srcs/app:app_web_e2e_test
```

The Bazel target `//srcs/app:app_web_e2e_test` starts the backend and Flutter web app automatically, then runs Playwright against the served build artifacts.

### Kind End-to-End Test

Requires `kind`, `helm`, `kubectl`, and `docker` on `$PATH`.

```bash
bazel test //deploy:kind_e2e_test
```

This test:
1. Creates a temporary Kind cluster
2. Builds and loads Docker images into Kind
3. Installs Redis (Bitnami) and CloudNative PG via Helm
4. Installs the OHC application chart
5. Waits for all pods to become `Ready`
6. Runs REST API smoke tests against the deployed service
7. Deletes the Kind cluster (cleanup on exit)

---

## Local Development with Docker Compose

Docker Compose is the fastest way to stand up the full stack for local testing.

### 1 — Build and start

```bash
docker compose -f deploy/docker-compose.yml up --build
```

Services:
| Service | Port | URL |
|---------|------|-----|
| Server | 8080 | http://localhost:8080 (API + optional UI) |
| Redis | 6379 | redis://localhost:6379 |
| PostgreSQL | 5432 | postgres://localhost:5432/ohc |
| Chatwoot | 3002 | http://localhost:3002 |

### 2 — Seed demo data

```bash
curl -s -X POST http://localhost:8080/api/dev/seed \
  -H 'Content-Type: application/json' \
  -d '{"scenario":"launch-readiness"}' | jq .
```

### 3 — Open the dashboard

Navigate to [http://localhost:8080](http://localhost:8080).

### 4 — Stop

```bash
docker compose -f deploy/docker-compose.yml down
```

### 5 — Stop and remove volumes (full reset)

```bash
docker compose -f deploy/docker-compose.yml down -v
```

---

## Environment Variables

### Backend (`srcs/server`)

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | `8080` | HTTP listen port |
| `DATABASE_URL` | *(empty)* | PostgreSQL DSN; falls back to in-memory store when unset |
| `REDIS_ADDR` | *(empty)* | Redis address e.g. `redis:6379`; pub-sub disabled when unset |
| `OHC_MULTITENANT` | `false` | Enables org-aware multi-tenant routing for shared-service deployments |
| `OHC_HEADLESS` | `false` | Disables static UI serving so the backend runs as an API-only service |
| `OHC_SERVE_UI` | `true` | Optional override for static UI serving |
| `GEMINI_API_KEY` | *(empty)* | Google Gemini API key for AI model calls |
| `LOG_LEVEL` | `info` | Structured log level (`debug`/`info`/`warn`/`error`) |

### Frontend assets

| Variable | Default | Description |
|----------|---------|-------------|
| `FRONTEND_STATIC_DIR` | `srcs/app/build/web` | Path to compiled Flutter artifacts |

---

## Adding a New API Endpoint

1. Add the handler function in `srcs/server/dashboard/server.go` or the relevant handler file in `srcs/server/dashboard/`
2. Register the route in `srcs/server/dashboard/server.go`
3. Add a unit test in `srcs/server/dashboard/server_test.go`
4. Update the proto if a new message type is needed (`srcs/proto/`)
5. Run `bazel test //srcs/server/...`

---

## Adding a New Agent Role

1. Define the role constant in `srcs/server/orchestration/service.go`
2. Add any role-specific behaviour to `Hub.HandleMessage`
3. Update the default `Catalog` in `srcs/server/billing/tracker.go` if the role uses a different model
4. Add the role to the Skill Pack defaults in `srcs/server/dashboard/server.go`

---

## CI Pipeline

All CI is driven by Bazel.  The GitHub Actions workflow runs:

```
bazel test //...
```

No raw `npm test`, `go test`, or shell scripts are invoked directly by CI.

---

## Protobuf Code Generation

```bash
bazel build //srcs/proto/...
```

Generated Go stubs land in `bazel-bin/srcs/proto/`.

---

## Troubleshooting

### Bazel sandbox permission errors

```bash
bazel clean --expunge
bazel test //...
```

### `go: module lookup disabled by GOFLAGS` in Bazel

Ensure `CGO_ENABLED=1` is set in `.bazelrc` (it already is).

### Kind cluster creation fails

Check Docker is running and has enough resources (≥ 4 GB RAM, 2 CPUs).

### Playwright browser not found

Install Playwright browsers:
```bash
npx playwright install --with-deps chromium
```

The Bazel `app_web_e2e_test` target handles this automatically.

</div>
