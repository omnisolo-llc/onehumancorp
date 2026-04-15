# OHC Hybrid Agentic OS - Changelog

## v0.3.8 (Cloud) / v0.3.8+1 (Standalone)
### Cloud Scaling Improvements
- Fully implemented KAIROS Master Orchestration interfaces, including Shared Task List, Sub-Agent Queues, and AutoDream pipelines for cross-pod coordination.
- Integrated Teammate Mesh v2 with WebSocket transport and SPIFFE mTLS validation for zero-trust agent communication.

### Privacy/Offline Improvements
- Enhanced Standalone reliability with KAIROS state machine SQLite fallbacks and proactive chaos engineering verification.
- Integrated SwarmContextOverlay with premium Glassmorphism aesthetics for improved local swarm observability.

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
