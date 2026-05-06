## v0.4.26 (Cloud) / v0.4.26+1 (Standalone)
### Cloud Scaling Improvements
- Optimize parallel fetch latency in dashboard API endpoints (#4867)
- Implement Business Journey Architecture with state machine and RLS (#c0f0)
- Resolve unused variables and scope compilation errors in Rust backend (#f8e3)

### Privacy/Offline Improvements
- Implement Cost Transparency Dashboard UI and Backend Schema Fixes (#d0c1)
- Enhance Sentinel Hybrid Security Fix for Tenant Leakage in Background Workers (#ee05)
- Improve UI jargon and testing for Login screen and Walkthrough (#3dc2, #6107)
- Implement success milestones logic for offline tracking (#a755)


# Release Notes

## v0.4.25 (Cloud) / v0.4.25+1 (Standalone)
- Scaling (Cloud): Fix IpcTransport races, gRPC UI Interceptor, and MaintenanceWorker fixes (#734b, #9cb3, #33a8).
- Privacy/Offline (Standalone): Shield Sentry Chaos Resilience, Slint UI memory safety, PII leakage checks, and SQLite schema parity (#a137, #e295, #e930, #edc2).

## v0.4.24 (Cloud) / v0.4.24+1 (Standalone)
- Scaling (Cloud): docs: update outdated Go/Flutter references to Rust/Slint (#9661)
- Privacy/Offline (Standalone): docs: update outdated Go/Flutter references to Rust/Slint (#9661)

## v0.4.23 (Cloud) / v0.4.23+1 (Standalone)
- UI & UX (Cloud): Fix onboarding confetti state transition and wire checklist navigation (#9418)
- UI & UX (Standalone): Fix onboarding confetti state transition and wire checklist navigation (#9418)

## v0.4.22 (Cloud) / v0.4.22+1 (Standalone)
- Scaling (Cloud): chore: migrate protobufs to bazel and fix tests (#9343)
- Privacy/Offline (Standalone): chore: migrate protobufs to bazel and fix tests (#9343)

## v0.4.21 (Cloud) / v0.4.21+1 (Standalone)
- Scaling (Cloud): 🤖 Implementer: Harness Upgrade - [CrewAI Role-based architecture] (#9107)
- Privacy/Offline (Standalone): 🤖 Implementer: Harness Upgrade - [CrewAI Role-based architecture] (#9107)

## v0.4.20 (Cloud) / v0.4.20+1 (Standalone)
- Scaling (Cloud): 🤖 Implementer: Harness Upgrade - Anthropic 3-Stage Tool Gating (#8965)
- Privacy/Offline (Standalone): 🤖 Implementer: Harness Upgrade - Anthropic 3-Stage Tool Gating (#8965)

## v0.4.19 (Cloud) / v0.4.19+1 (Standalone)
- Scaling (Cloud): Fix Bazel test execution time warning for `server_test` (#8941)
- Privacy/Offline (Standalone): Fix Bazel test execution time warning for `server_test` (#8941)

## v0.4.18 (Cloud) / v0.4.18+1 (Standalone)
- Scaling (Cloud): 🔗 Link: Share Teammate Mesh and enable in-process builtin agent (#8924)
- Privacy/Offline (Standalone): 🔗 Link: Share Teammate Mesh and enable in-process builtin agent (#8924)

## v0.4.17 (Cloud) / v0.4.17+1 (Standalone)
- Scaling (Cloud): 🎥 Lens Audit: Softer Draft Wording (#8901)
- Privacy/Offline (Standalone): 🎥 Lens Audit: Softer Draft Wording (#8901)

## v0.4.16 (Cloud) / v0.4.16+1 (Standalone)
- Scaling (Cloud): [Hybrid Security Fix] Enforced SPIFFE ID headers for Authentication and multi-tenancy (#8891)
- Privacy/Offline (Standalone): [Hybrid Security Fix] Enforced SPIFFE ID headers for Authentication and multi-tenancy (#8891)

## v0.4.15 (Cloud) / v0.4.15+1 (Standalone)
- Scaling (Cloud): Implement Harness Upgrade - Subagent Orchestration: Worktree pattern. SubagentExecutor spawns a git worktree with an isolated branch (`subagent-<task_id>`) for secure local subagent executions.
- Privacy/Offline (Standalone): Implemented secure git worktree sandbox isolation to prevent parent directory access overrides when executing local tools.

## v0.4.14 (Cloud) / v0.4.14+1 (Standalone)
- Scaling (Cloud): Refactored Teammate Mesh transport to use Protobuf serialization for interop improvements.

## v0.4.13 (Cloud) / v0.4.13+1 (Standalone)
- Scaling (Cloud): Implemented Hybrid Distributed MeshLock Implementation, abstracted SKIP LOCKED for sqlite and added chaos tests, implemented observability metrics for AutoDream and Task Claim Contention, and implemented OS-level SandboxManager using bwrap and sandbox-exec.
- Privacy/Offline (Standalone): Added viral storefront link growth feature, added referral loop to User Management, implemented AI agent department draft-for-review approval workflow, hardened application for multi-tenant and local standalone, implemented high-fidelity 11-step onboarding wizard, and removed tooltips from the app to improve reliability.

## v0.4.12 (Cloud) / v0.4.12+1 (Standalone)
- Privacy/Offline (Standalone): Applied Glassmorphism UI tokens to dashboard, plan, cost, and walkthrough components.

## v0.4.11 (Cloud) / v0.4.11+1 (Standalone)
- Scaling (Cloud): Implemented JSON minification and Gzip compression middleware to optimize server HTTP payloads.
- Privacy/Offline (Standalone): Fixed TooltipRegistry namespace issue in Slint UI and improved dashboard accessibility.



## v0.4.10 (Cloud) / v0.4.10+1 (Standalone)
- Scaling (Cloud): Enforce `ENABLE ROW LEVEL SECURITY` across 18 tenant data tables, harden the `SyncMcpDeltas` RPC handler, and create formal SQL definitions for `crdt_deltas` and `local_mcp_rag_tasks`.
- Privacy/Offline (Standalone): Remove an insecure `println!` from the `power_sync_push` service handler to prevent payload leakage into application logs.


## v0.4.9 (Cloud) / v0.4.9+1 (Standalone)
- Scaling (Cloud): Implement Harness Upgrade (Guardrails & Safety, Agent Loop Capabilities, Concurrent Tool Execution, Granular Capability ACLs, and 4-types of Error Handling with Compounding Error Prevention), add hybrid latency benchmarks, implement Omni-Context Sub-agent Routing in Rust, restore MCP execution logic in server_old, fix RBAC for promtail, fix broken bazel targets, and configure cli_test with hermetic vitest runner.
- Privacy/Offline (Standalone): Consolidate API Playbook, add architecture design for multi-tenant SaaS tier, add Business Journey Architecture research report, simplify language in Walkthrough, and redesign Dashboard and Business Setup with Mobile-first and glassmorphism.


## v0.4.8 (Cloud) / v0.4.8+1 (Standalone)
- Scaling (Cloud): Implement Harness Upgrade with 4-types of Error Handling and Compounding Error Prevention, add cost dashboard and plan UI features, enforce premium dashboard aesthetics and optimize K8s autoscaling, and implement Business Setup Wizard UI with progressive disclosure.
- Privacy/Offline (Standalone): No specific privacy/offline changes in this release.



## v0.4.7 (Cloud) / v0.4.7+1 (Standalone)
- Scaling (Cloud): Implement Business Setup and Website Builder wizard UI scaffolding, onboarding cross-device state, and Business Share & Embed widget.
- Privacy/Offline (Standalone): Conduct Hybrid Privacy Audit with PII guardrails, implement in-app mobile-first Help Center with tooltips, and add Dashboard Welcome Checklist.

## v0.4.6 (Cloud) / v0.4.6+1 (Standalone)
- Scaling (Cloud): Implement Sub-Agent Orchestration Queue, Realtime Teammate Mesh APIs, Draft-for-Review workflow, and enhance testing.
- Privacy/Offline (Standalone): Simplify dashboard UI, resolve memoryLock TOCTOU race conditions, add missing rows.Err() checks, and generate architecture report.

## v0.4.5 (Cloud) / v0.4.5+1 (Standalone)
- Scaling (Cloud): Implement Phase 3: AutoDream Vector Data Pipelines.
- Privacy/Offline (Standalone): Enhance AutoDream Vector Data Pipelines for standalone privacy.

## v0.4.4 (Cloud) / v0.4.4+1 (Standalone)
- Scaling (Cloud): Parallel execution for SyncMissions and SyncContextSync, Optimize OHC-SIP synchronization latency.
- Privacy/Offline (Standalone): Add Agent Harness architecture research report, Add Telemetry for SQLite lock contention.

## v0.4.3 (Cloud) / v0.4.3+1 (Standalone)
- Scaling (Cloud): Optimise Orchestration & Observability.
- Privacy/Offline (Standalone): Add AutoDream Sync Walkthrough.

## v0.4.2 (Cloud) / v0.4.2+1 (Standalone)
- Scaling (Cloud): Implement Local Embedding Caching for Cost Optimization.
- Privacy/Offline (Standalone): Enhance standalone performance via Local Embedding Caching.
## v0.4.1 (Cloud) / v0.4.1+1 (Standalone)
- Scaling (Cloud): Architect Cross-Mode Database Schema Syncer via MCP, Optimize PopMessages latency.
- Privacy/Offline (Standalone): Fix PII redaction for nested slog groups, Bulk Team Invite UI in Flutter App.


## v0.3.11 (Cloud) / v0.3.11+1 (Standalone)
- Scaling (Cloud): Implement Bubblewrap Sandbox Telemetry and Implement Hybrid Parity Stress Tests.
- Privacy/Offline (Standalone): Clean up dead Linear integration and harden standalone wrapper.

## v0.3.9 (Cloud) / v0.3.9+1 (Standalone)
- Scaling (Cloud): Integrated Telemetry-MCP Bridge securely via docker-compose and added a thread-safe registration implementation for the KAIROS orchestrator.
- Privacy/Offline (Standalone): Maintained Standalone telemetry consistency across single-user environments for the new MCP Bridge integration.

## v0.3.8 (Cloud) / v0.3.8+1 (Standalone)
- Scaling (Cloud): Added MissionIngestionWorker to seamlessly vectorize mission artifacts into AutoDream long-term memory.
- Privacy/Offline (Standalone): Enhanced standalone AutoDream memory consistency using SQL-backed idempotent ingestion for mission artifacts.

## v0.3.7 (Cloud) / v0.3.7+1 (Standalone)
- Scaling (Cloud): Implemented Centrifuge WebSockets integration by hooking up CentrifugeNode to MeshTransport interface logic in orchestration.
- Privacy/Offline (Standalone): Added interop.ValidateSPIFFEID validation to all realtime mesh broadcast and capability advertisements ensuring compliance with OHC Hybrid Architecture protocols.
## v0.3.4 (Cloud) / v0.3.4+1 (Standalone)
- Scaling (Cloud): Formalized real-time Teammate Mesh APIs and KAIROS DAG orchestration for distributed pod execution.
- Privacy/Offline (Standalone): Ensured KAIROS orchestrator degrades gracefully into isolated SQLite single-user mode.
## v0.3.2 (Cloud) / v0.3.2+1 (Standalone)
- Scaling (Cloud): Enforced Tenant Data Isolation in Blob & FS Providers (#3933) to prevent cross-tenant data leakage.
- Privacy/Offline (Standalone): Visually integrated the new TaskListScreen into the DashboardScreen maintaining OHC-SIP aesthetic standards, and added the AutoDream Sync Daemon Walkthrough.

## v0.3.1 (Cloud) / v0.3.1+1 (Standalone)
- Scaling (Cloud): Implement Hybrid MCP RAG Protocol (Phases 1-3) for scalable knowledge retrieval.
- Privacy/Offline (Standalone): Enable local context integration through the Hybrid MCP RAG Protocol for standalone offline support.

## v0.3.0 (Cloud) / v0.3.0+1 (Standalone)
- Scaling (Cloud): Formalized the Realtime Teammate Mesh APIs using Redis Pub/Sub for horizontal scalability and broadcasting across pods.
- Privacy/Offline (Standalone): Implemented MemoryMeshTransport for the Teammate Mesh to run without external dependencies.
- Scaling (Cloud): Architected Shared Task List and OHC Core Systems for Agent coordination.

## v0.2.9 (Cloud) / v0.2.9+1 (Standalone)
- Scaling (Cloud): Enhanced Cloud multi-tenant architecture with robust tests and SPIFFE auth support for new MCP package.
- Privacy/Offline (Standalone): Implemented the `statesyncmcp` MCP package for seamless local-to-cloud synchronization.


## v0.2.9 (Cloud) / v0.1.9+1 (Standalone)
- Scaling (Cloud): Exposed HybridHealthProbe through the orchestrator Hub to check database availability and mesh channel connectivity.
- Privacy/Offline (Standalone): Strict enforcement of user data privacy in standalone mode by correctly handling telemetry opt-ins.

## v0.2.8 (Cloud) / v0.1.8+1 (Standalone)
- Scaling (Cloud): Ensured AutoDream pipeline >90% coverage for memory consolidation scaling.
- Privacy/Offline (Standalone): Enhanced hybrid-aware tests for robust standalone offline capabilities.

## v0.2.7 (Cloud) / v0.1.7+1 (Standalone)
- Scaling (Cloud): Optimized multi-tenant K8s resource consumption and improved Prometheus metrics scraping efficiency.
- Privacy/Offline (Standalone): Fixed concurrent database access panics and local data propagation using optimized SQLite throttle controls.

## v0.2.6 (Cloud) / v0.1.6+1 (Standalone)
- Scaling (Cloud): Integrated PowerSync Go backend service to enforce strict Tenant isolation.
- Privacy/Offline (Standalone): Integrated PowerSync Flutter SDK for seamless local-to-cloud synchronization.

## v0.2.5 (Cloud) / v0.1.5+1 (Standalone)
- Scaling (Cloud): Enhanced PostgreSQL multi-tenant synchronization and scaling via optimized pod concurrency configurations.
- Privacy/Offline (Standalone): Improved SQLite fallback and offline state syncing for absolute local sovereignty without external database dependencies.

## v0.2.4
- Scaling (Cloud): Enhanced PostgreSQL multi-tenant synchronization and scaling.
- Privacy/Offline (Standalone): Improved SQLite fallback and offline state syncing.

## v0.2.3
9242ea4d Fix Flutter Linux desktop support, test asset propagation, and e2e test flutter interception (#886)

## v0.2.2
3bd2195c fix(ui): improve accessibility and semantic colors in dashboard widgets (#856)

## v0.2.1
181cbe23 🎨 Palette: Enhance Accessibility and Micro-Interactions in Integrations Screen (#804)
