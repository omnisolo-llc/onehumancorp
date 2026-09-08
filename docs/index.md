# OmniSolo Documentation

Welcome to the documentation for **OmniSolo** (formerly One Human Corp / OHC), the open-source **Hybrid Agentic OS** designed to empower a single operator to run an entire enterprise using multi-agent swarms, low-code orchestration, and rich integrations.

This site is the canonical documentation root for the repository. It follows a markdown-first workflow: source content lives in `docs/`, primary site navigation is declared in `mkdocs.yml`, and the rendered documentation website is built at runtime.

## Platform Architecture Highlights

- **Hybrid Operating Modes**:
  - **Cloud-Native Shared Service**: Axum HTTP/gRPC server backed by PostgreSQL, with `OHC_MULTITENANT=true` for tenant isolation and optional Valkey / PowerSync sync.
  - **Standalone Desktop Mode**: Local desktop environment using Tauri v2, local SQLite SIPDB (`ohc-standalone.db`), and IPC communication.
  - **Headless Cloud API**: Backend API (`OHC_HEADLESS=true`) providing auth, health probes, metrics, and orchestration for thin/mobile clients.
- **Universal Multi-Harness Middleware**:
  - Exposes session/task middleware supporting Codex app-server v2, OpenCode, DeepSeek Harness, Pi, Kimi ACP, OpenHands Agent Server, and AgentBoardTT OpenHarness.
  - OpenAI-compatible worker contract with support for `OPENAI_API_KEY`, MiniMax, Anthropic, Gemini Pro, and local Ollama models.
- **Swarm Orchestration & Low-Code Workflows**:
  - Builtin agent runtime (`src/agents/builtin/`), Scout agent (`src/agents/scout/`), and Ralph Loop autonomous execution.
  - Block-based Visual Workflows (`visual_workflow.rs`) with parallel fork/join and dynamic DAG execution (`/api/v1/workflow/run`).
  - Distributed State Machine and Sub-Agent Queues with Postgres row locks or SQLite IPC locks.
- **Episodic & Vector Memory (AutoDream)**:
  - AutoDream memory consolidation pipelines converting session interactions into embedded vector truth.
- **55+ Business Integrations & MCP Mesh**:
  - Integrated POS (Stripe Terminal, Square), E-commerce (Shopify), Communications (Twilio, WhatsApp, Slack, Zoom), Accounting (QuickBooks, Xero), Shipping (Shippo, ShipEngine), and MCP dynamic tool execution.

## Start Here

- **[Architecture Hub](technical/architecture/architecture-overview.md)**: Deep dives into KAIROS, Hybrid OS, Sub-Agent Queues, and Mesh Sync.
- **[Developer Setup & Guide](technical/developer/setup.md)**: Day-one onboarding, master CLI (`ohc_hybrid_cli.sh`), and Bazel build instructions.
- **[Multi-Harness Compatibility](omnisolo-harness-compatibility-inventory.md)**: Compatibility contracts, model bindings, and session capsule migration.
- **[Walkthroughs](walkthroughs/index.md)**: Guided tutorials for agent lifecycles, AutoDream CLI, Teammate Mesh, and onboarding.
- **[User Guide](user_guide.md)**: End-user handbook for setting up stores, hiring AI departments, and managing day-to-day operations.
- **[API Reference](technical/api/api-reference.md)** & **[Interactive API Playbook](api/playbook.md)**: Complete REST and gRPC API specifications.

## Conventions

- Markdown is the authoritative source format under `docs/`.
- All first-party backend and agent source code lives under `src/`.
- Primary desktop application is under `src/ui/tauri/`.
- Tasks and active roadmaps are tracked in GitHub issues.
