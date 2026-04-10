# Release Notes

## v0.2.9 (Cloud) / v0.2.9+1 (Standalone)
- Scaling (Cloud): Enhanced scaling by securely decoupling and routing sync states using SPIFFE identity verification.
- Privacy/Offline (Standalone): Integrated Hybrid Local-to-Cloud State Sync MCP Proxy (`statesyncmcp`) for robust, offline-capable synchronization.

## v0.2.8 (Cloud) / v0.1.8+1 (Standalone)
- Scaling (Cloud): Stabilized hybrid test scenarios ensuring consistent PostgreSQL performance for multi-tenant deployments.
- Privacy/Offline (Standalone): Synchronized version configurations and finalized SQLite compatibility for offline standalone environments.

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