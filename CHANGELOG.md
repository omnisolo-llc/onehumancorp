## v0.4.45 (Cloud) / v0.4.45+1 (Standalone)

### Cloud Scaling Improvements
- Enhance multi-environment promotion across Cloud staging.

### Privacy/Offline Improvements
- Enforce local offline usage and privacy for Standalone desktop beta builds.

## v0.4.44 (Cloud) / v0.4.44+1 (Standalone)

### Cloud Scaling Improvements
- Improve multi-environment promotion across Cloud staging.

### Privacy/Offline Improvements
- Enhance privacy controls for local Standalone desktop beta builds.

## v0.4.43 (Cloud) / v0.4.43+1 (Standalone)

### Cloud Scaling Improvements
- Optimize multi-tenant database connection pooling for staging stability.

### Privacy/Offline Improvements
- Enhance local offline telemetry privacy rules.

## v0.4.42 (Cloud) / v0.4.42+1 (Standalone)

### Cloud Scaling Improvements
- Optimize multi-tenant scaling to enhance multi-environment promotion across Cloud staging.

### Privacy/Offline Improvements
- Enforce improved local offline usage and privacy for Standalone desktop beta builds.

## v0.4.37 (Cloud) / v0.4.37+1 (Standalone)

- Scaling (Cloud): Upgraded orchestration layers with AutoDream Vector Data Pipelines and hierarchical sub-agent concurrency.
- Privacy/Offline (Standalone): Ensured local agents leverage Teammate Mesh gracefully with local-only fallback and zero-trust isolated environments.

# OHC Hybrid Agentic OS - Changelog

## v0.4.42 (Cloud) / v0.4.42+1 (Standalone)

### Cloud Scaling Improvements
- Optimize multi-tenant scaling to enhance multi-environment promotion across Cloud staging.

### Privacy/Offline Improvements
- Enforce improved local offline usage and privacy for Standalone desktop beta builds.

## v0.4.41 (Cloud) / v0.4.41+1 (Standalone)

### Cloud Scaling Improvements
- ✍️ Scribe: Implemented the In-App Help Center & Contextual Tooltip API, enabling non-technical users to access step-by-step guides, onboarding walkthroughs, and plain-language assistance directly within the app.
- Optimize Sub-Agent Queue polling intervals to reduce Postgres connection pressure.

### Privacy/Offline Improvements
- Implement offline-first local vector embeddings cache for the OHC Swarm.


## v0.4.39 (Cloud) / v0.4.39+1 (Standalone)

### Cloud Scaling Improvements
- Add auto-scaling limits for Sub-Agent Queue and Teammate Mesh APIs to handle increased load spikes.

### Privacy/Offline Improvements
- Enforce full local SIPDB SQLite encryption parity for Standalone builds to protect sensitive user telemetry.


## v0.4.38 (Cloud) / v0.4.38+1 (Standalone)

### Cloud Scaling Improvements
- Enhance multi-tenant onboarding flow tests for the Welcome Checklist to ensure reliable scaling.

### Privacy/Offline Improvements
- Bolster Standalone Wizard state test coverage for improved offline reliability and progressive disclosure validation.


## v0.4.36 (Cloud) / v0.4.36+1 (Standalone)
### Cloud Scaling Improvements
- Fix orchestration mesh tests that failed due to multiple declarations and invalid imports.
### Privacy/Offline Improvements
- Add missing `RedactInterfacePII` to fix backend telemetry data sync crashes.

## v0.4.35 (Cloud) / v0.4.35+1 (Standalone)
### Cloud Scaling Improvements
- 🔨 Forge: Sub-Agent Orchestration Queue Test Fixes (#12202)

### Privacy/Offline Improvements
- (No specific privacy/offline improvements in this patch)

## v0.4.34 (Cloud) / v0.4.34+2 (Standalone)

- Scaling (Cloud): Fix compiler warnings in UI tests and app main to improve reliability and multi-tenant scaling (#11933).
- Privacy/Offline (Standalone): Improve UI tests and standalone app reliability (#11933).

## v0.4.33 (Cloud) / v0.4.33+1 (Standalone)

- Scaling (Cloud): Add dual-mode caching layer to dashboard service to optimize latency (#11871).
- Privacy/Offline (Standalone): Ensure dual-mode caching layer supports offline persistence securely (#11871).


## v0.4.32 (Cloud) / v0.4.32+1 (Standalone)

### Cloud Scaling Improvements
- 🛡️ Sentry: Health Guardianship /api/v1/health improvements for multi-tenant state sync

### Privacy/Offline Improvements
- 🛡️ Sentry: Health Guardianship /api/v1/health improvements for standalone isolated node switching

### Cloud Scaling Improvements
- 🧹 Maintainer: Ensure multitenant K8s compliance for all pods (#11546)

### Privacy/Offline Improvements
- 🧹 Maintainer: Centralize PII compliance guardrails and enforce hybrid privacy audit (#11546)


## v0.4.32 (Cloud) / v0.4.32+1 (Standalone)
### Cloud Scaling Improvements
- 🔗 Link: update api routing to use axum 0.8 style path variables (#11553)

### Privacy/Offline Improvements
- No specific privacy/offline changes in this release.


## v0.4.30 (Cloud) / v0.4.30+1 (Standalone)
### Cloud Scaling Improvements
- 🔨 Forge: Refactor GrowthReferralWidget to use GlassCard for premium aesthetic (#11347)

### Privacy/Offline Improvements
- 🔨 Forge: Refactor GrowthReferralWidget to use GlassCard for premium aesthetic (#11347)

## v0.4.29 (Cloud) / v0.4.29+1 (Standalone)

### Cloud Scaling Improvements
- 🔗 Link: Implemented Teammate Mesh Communication Layer and Distributed Locks (#11313)

### Privacy/Offline Improvements
- 🔗 Link: Ensured mesh communication layer degrades gracefully into isolated standalone instances (#11313)



## v0.4.28 (Cloud) / v0.4.28+1 (Standalone)
### Cloud Scaling Improvements
- ✍️ Scribe: Scaled the Help Center & Tooltip Documentation System for multi-tenant cloud environments (#11267)

### Privacy/Offline Improvements
- ✍️ Scribe: Enabled offline-first support for the Help Center & Tooltip Documentation System in standalone mode (#11267)

## v0.4.27 (Cloud) / v0.4.27+1 (Standalone)
### Cloud Scaling Improvements
- 🎨 Canvas: Refactored the MCP LocalProxyClient to use an abstract BlobProvider with S3 support for cloud multitenant scaling.

### Privacy/Offline Improvements
- 🎨 Canvas: Added LocalBlobProvider implementation to ensure privacy and offline capabilities for the MCP proxy.

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


## v0.4.25 (Cloud) / v0.4.25+1 (Standalone)
### Cloud Scaling Improvements
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
