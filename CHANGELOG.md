# OHC Hybrid Agentic OS - Changelog

## v0.4.25 (Cloud) / v0.4.25+1 (Standalone)
### Cloud Scaling Improvements
- Update Grafana dashboards for PostgreSQL and Redis metrics (#10982)
- Fix IpcTransport cross-mode duplicate delivery and checkpoint races (#734b)
- Fix gRPC calls with App UI Interceptor (#9cb3)
- Final MaintenanceWorker implementation and Build Fix (#33a8)
- Interop improvements: Refactor mesh handoff protocol and comm layer cross-mode sync (#8a65, #9537)
- Fix server_test timeout issues in sandboxed environment (#b320)

### Privacy/Offline Improvements
- Shield Sentry: Enhanced Chaos Resilience & Mode Parity (#a137)
- Fix intentional memory leak and Box::leak usages in Slint UI instantiations (#e295)
- Implement automated checks for PII leakage (#e930)
- SQLite memory consolidation schema parity (#edc2)
- Add UI cards for missing tool integrations in Slint dashboard (#5748)

## v0.4.24 (Cloud) / v0.4.24+1 (Standalone)
### Cloud Scaling Improvements
- docs: update outdated Go/Flutter references to Rust/Slint (#9661)

### Privacy/Offline Improvements
- docs: update outdated Go/Flutter references to Rust/Slint (#9661)

## v0.4.23 (Cloud) / v0.4.23+1 (Standalone)
### UI & UX Improvements
- Fix onboarding confetti state transition and wire checklist navigation (#9418)

## v0.4.22 (Cloud) / v0.4.22+1 (Standalone)
### Cloud Scaling Improvements
- chore: migrate protobufs to bazel and fix tests (#9343)

### Privacy/Offline Improvements
- chore: migrate protobufs to bazel and fix tests (#9343)

## v0.4.21 (Cloud) / v0.4.21+1 (Standalone)
### Cloud Scaling Improvements
- 🤖 Implementer: Harness Upgrade - [CrewAI Role-based architecture] (#9107)

### Privacy/Offline Improvements
- 🤖 Implementer: Harness Upgrade - [CrewAI Role-based architecture] (#9107)

## v0.4.20 (Cloud) / v0.4.20+1 (Standalone)
### Cloud Scaling Improvements
- 🤖 Implementer: Harness Upgrade - Anthropic 3-Stage Tool Gating (#8965)

### Privacy/Offline Improvements
- 🤖 Implementer: Harness Upgrade - Anthropic 3-Stage Tool Gating (#8965)

## v0.4.19 (Cloud) / v0.4.19+1 (Standalone)
### Cloud Scaling Improvements
- Fix Bazel test execution time warning for `server_test` (#8941)

### Privacy/Offline Improvements
- Fix Bazel test execution time warning for `server_test` (#8941)

## v0.4.18 (Cloud) / v0.4.18+1 (Standalone)
### Cloud Scaling Improvements
- 🔗 Link: Share Teammate Mesh and enable in-process builtin agent (#8924)

### Privacy/Offline Improvements
- 🔗 Link: Share Teammate Mesh and enable in-process builtin agent (#8924)

## v0.4.17 (Cloud) / v0.4.17+1 (Standalone)
### Cloud Scaling Improvements
- 🎥 Lens Audit: Softer Draft Wording (#8901)

### Privacy/Offline Improvements
- 🎥 Lens Audit: Softer Draft Wording (#8901)

## v0.4.16 (Cloud) / v0.4.16+1 (Standalone)
### Cloud Scaling Improvements
- [Hybrid Security Fix] Enforced SPIFFE ID headers for Authentication and multi-tenancy (#8891)

### Privacy/Offline Improvements
- [Hybrid Security Fix] Enforced SPIFFE ID headers for Authentication and multi-tenancy (#8891)

## v0.4.15 (Cloud) / v0.4.15+1 (Standalone)
### Cloud Scaling Improvements
- Implement Harness Upgrade - Subagent Orchestration: Worktree pattern. SubagentExecutor spawns a git worktree with an isolated branch (`subagent-<task_id>`) for secure local subagent executions.

### Privacy/Offline Improvements
- Implemented secure git worktree sandbox isolation to prevent parent directory access overrides when executing local tools.

## v0.4.14 (Cloud) / v0.4.14+1 (Standalone)
### Cloud Scaling Improvements
- Refactored Teammate Mesh transport to use Protobuf serialization for interop improvements.

## v0.4.13 (Cloud) / v0.4.13+1 (Standalone)
### Cloud Scaling Improvements
- Implemented Hybrid Distributed MeshLock Implementation
- Abstracted SKIP LOCKED for sqlite and added chaos tests
- Implemented observability metrics for AutoDream and Task Claim Contention
- Implemented OS-level SandboxManager using bwrap and sandbox-exec

### Privacy/Offline Improvements
- Added viral storefront link growth feature
- Added referral loop to User Management
- Implemented AI agent department draft-for-review approval workflow
- Hardened application for multi-tenant and local standalone
- Implemented high-fidelity 11-step onboarding wizard
- Removed tooltips from the app to improve reliability

## v0.4.12 (Cloud) / v0.4.12+1 (Standalone)
### Privacy/Offline Improvements
- Applied Glassmorphism UI tokens to dashboard, plan, cost, and walkthrough components.

## v0.4.11 (Cloud) / v0.4.11+1 (Standalone)
### Cloud Scaling Improvements
- Implemented JSON minification and Gzip compression middleware to optimize server HTTP payloads.

### Privacy/Offline Improvements
- Fixed TooltipRegistry namespace issue in Slint UI and improved dashboard accessibility.

## v0.4.10 (Cloud) / v0.4.10+1 (Standalone)
### Cloud Scaling Improvements
- Creates formal SQL definitions for `crdt_deltas` and `local_mcp_rag_tasks` to prepare them for RLS.
- Enforces `ENABLE ROW LEVEL SECURITY` across 18 tenant data tables and provisions them with default `CREATE POLICY` statements binding queries to the current authenticated database context.
- Hardens the `SyncMcpDeltas` RPC handler to extract its `tenant_id` securely from the authenticated SPIFFE ID metadata.

### Privacy/Offline Improvements
- Removes an insecure `println!` from the `power_sync_push` service handler to prevent request payload leakage into application logs.


## v0.4.9 (Cloud) / v0.4.9+1 (Standalone)
### Cloud Scaling Improvements
- Implement Harness Upgrade: Guardrails & Safety, Agent Loop Capabilities, Concurrent Tool Execution, Granular Capability ACLs, and 4-types of Error Handling with Compounding Error Prevention.
- Add hybrid latency benchmarks for cloud and standalone mode.
- Implement Omni-Context Sub-agent Routing in Rust.
- Restore MCP execution logic and tool tests in server_old.
- Fix RBAC for promtail and revert aggressive resource limits.
- Fix broken bazel targets to use src/server_old paths.
- Configure cli_test with hermetic vitest runner.

### Privacy/Offline Improvements
- Consolidate API Playbook and fix link references (#8645).
- Add architecture design for multi-tenant SaaS tier.
- Add Business Journey Architecture research report.
- Simplify language in Walkthrough.
- Mobile-first and glassmorphism redesign for Dashboard and Business Setup.


## v0.4.8 (Cloud) / v0.4.8+1 (Standalone)
### Cloud Scaling Improvements
- Implement Harness Upgrade with 4-types of Error Handling and Compounding Error Prevention.
- Add cost dashboard and plan UI features.
- Enforce premium dashboard aesthetics and optimize K8s autoscaling.
- Implement Business Setup Wizard UI with progressive disclosure.

### Privacy/Offline Improvements
- No specific privacy/offline changes in this release.


## v0.4.7 (Cloud) / v0.4.7+1 (Standalone)
### Cloud Scaling Improvements
- Implement Business Setup and Website Builder wizard UI scaffolding (#86b87bbd).
- Implement onboarding wizard cross-device state management (#c58aec3b).
- Add Business Share & Embed widget for viral storefront growth (#8c6d046a).

### Privacy/Offline Improvements
- Conduct Hybrid Privacy Audit and implement PII guardrails for telemetry (#16cac).
- Implement in-app mobile-first Help Center and Registry-based tooltips (#2c67d2aa).
- Implement Welcome Checklist post-onboarding widget on Dashboard (#ad60208b).

## v0.4.6 (Cloud) / v0.4.6+1 (Standalone)
### Cloud Scaling Improvements
- Implement Sub-Agent Orchestration Queue for KAIROS (#7696).
- Implement Realtime Teammate Mesh APIs (#7687).
- Implement Draft-for-Review AI Action Approval Workflow in KAIROS (#7676).
- Enhance test coverage for Agent Execution State Transition Latency Telemetry (#7692).
- Add test coverage for CapabilityAuthorizer (#7707).

### Privacy/Offline Improvements
- Simplify dashboard UI for non-technical users (#7691).
- Resolve memoryLock TOCTOU race conditions and expand lock suite (#7679).
- Add missing rows.Err() checks after db row iteration (#7678).
- Generate business journey architecture report (#7664).

## v0.4.5 (Cloud) / v0.4.5+1 (Standalone)
### Cloud Scaling Improvements
- Implement Phase 3: AutoDream Vector Data Pipelines.

### Privacy/Offline Improvements
- Enhance AutoDream Vector Data Pipelines for standalone privacy.

## v0.4.4 (Cloud) / v0.4.4+1 (Standalone)
### Cloud Scaling Improvements
- Parallel execution for SyncMissions and SyncContextSync.
- Optimize OHC-SIP synchronization latency.

### Privacy/Offline Improvements
- Add Agent Harness architecture research report.
- Add Telemetry for SQLite lock contention.

## v0.4.3 (Cloud) / v0.4.3+1 (Standalone)
### Cloud Scaling Improvements
- Optimise Orchestration & Observability.

### Privacy/Offline Improvements
- Add AutoDream Sync Walkthrough.


## v0.4.2 (Cloud) / v0.4.2+1 (Standalone)
### Cloud Scaling Improvements
- Implement Local Embedding Caching for Cost Optimization.

### Privacy/Offline Improvements
- Enhance standalone performance via Local Embedding Caching.

## v0.4.1 (Cloud) / v0.4.1+1 (Standalone)
### Cloud Scaling Improvements
- Architect Cross-Mode Database Schema Syncer via MCP.
- Optimize PopMessages latency.

### Privacy/Offline Improvements
- Fix PII redaction for nested slog groups.
- Bulk Team Invite UI in Flutter App.


## v0.3.11 (Cloud) / v0.3.11+1 (Standalone)
### Cloud Scaling Improvements
- Implement Bubblewrap Sandbox Telemetry.
- Implement Hybrid Parity Stress Tests.

### Privacy/Offline Improvements
- Clean up dead Linear integration and harden standalone wrapper.

## v0.3.9 (Cloud) / v0.3.9+1 (Standalone)
### Cloud Scaling Improvements
- Integrated Telemetry-MCP Bridge securely via docker-compose and added a thread-safe registration implementation for the KAIROS orchestrator.

### Privacy/Offline Improvements
- Maintained Standalone telemetry consistency across single-user environments for the new MCP Bridge integration.

## v0.3.8 (Cloud) / v0.3.8+1 (Standalone)
### Cloud Scaling Improvements
- Added MissionIngestionWorker to seamlessly vectorize mission artifacts into AutoDream long-term memory.

### Privacy/Offline Improvements
- Enhanced standalone AutoDream memory consistency using SQL-backed idempotent ingestion for mission artifacts.


## v0.3.7 (Cloud) / v0.3.7+1 (Standalone)
### Cloud Scaling Improvements
- Implemented Centrifuge WebSockets integration by hooking up CentrifugeNode to MeshTransport interface logic in orchestration.

### Privacy/Offline Improvements
- Added interop.ValidateSPIFFEID validation to all realtime mesh broadcast and capability advertisements ensuring compliance with OHC Hybrid Architecture protocols.

## v0.3.6 (Cloud) / v0.3.6+1 (Standalone)
### Cloud Scaling Improvements
- Implemented storage compression and token budget management tools for cost optimization in Kubernetes deployments.

### Privacy/Offline Improvements
- Enabled offline-compatible storage compression reducing local disk footprint for Standalone environments.



## v0.3.5 (Cloud) / v0.3.5+1 (Standalone)
### Cloud Scaling Improvements
- Enhanced Teammate Mesh APIs and AutoDream Worker logic for more scalable Kubernetes pod communications.

### Privacy/Offline Improvements
- Continued stabilization of the offline KAIROS state machine functionality via SQLite fallbacks.

## v0.3.4 (Cloud) / v0.3.4+1 (Standalone)
### Cloud Scaling Improvements
- Enhanced Cloud multi-tenant architecture and Hybrid Teammate Mesh APIs for improved coordination across Kubernetes pods.

### Privacy/Offline Improvements
- Implemented fully offline-capable KAIROS state machine via SQLite with safe fallbacks.

## v0.3.3 (Cloud) / v0.3.3+1 (Standalone)
### Cloud Scaling Improvements
- Enhanced Cloud multi-tenant architecture with robust onboarding tests and removed obsolete test files for cleaner CI/CD execution.

### Privacy/Offline Improvements
- Improved standalone offline test parity by ensuring onboarding integration tests run smoothly in isolated local environments without heavy Cloud dependencies.
