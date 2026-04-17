# OHC Hybrid Agentic OS - Changelog

## v0.4.3 (Cloud) / v0.4.3+1 (Standalone)
### Cloud Scaling Improvements
- Implemented Dynamic Viral Quota limits and added reverse indexing to Redis for efficient referral lookups.
- Modified QuotaService to grant a +50 usage limit bonus for successful team referrals and exposed the /accept endpoint.

### Privacy/Offline Improvements
- Verified graceful degradation of the dynamic viral quota tracking for Standalone configurations.

## v0.4.2 (Cloud) / v0.4.2+1 (Standalone)

### Cloud Scaling Improvements
- Implement QueueOrchestrator for Shared Task List (KAIROS Phase 1).
- Finalize Phase 4 Sub-Agent Queue and update Epic.
- Architect KAIROS Orchestration & AutoDream Foundation.

### Privacy/Offline Improvements
- Architect Hybrid MCP Tool for Blob Storage Sync.
- Fix flaky tests and nested group redaction.

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
