# Release Notes

## v0.3.6 (Cloud) / v0.3.6+1 (Standalone)
- Scaling (Cloud): Implemented storage compression and token budget management tools for cost optimization in Kubernetes deployments.
- Privacy/Offline (Standalone): Enabled offline-compatible storage compression reducing local disk footprint for Standalone environments.

## v0.3.5 (Cloud) / v0.3.5+1 (Standalone)
- Scaling (Cloud): Enhanced Teammate Mesh APIs and AutoDream Worker logic for more scalable Kubernetes pod communications.
- Privacy/Offline (Standalone): Continued stabilization of the offline KAIROS state machine functionality via SQLite fallbacks.

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