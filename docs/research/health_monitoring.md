# Title
## Cross-Mode Health Monitoring Architecture

### Problem Statement
The swarm relies on an intelligent orchestration layer (KAIROS) that dispatches tasks to agents and synchronizes state between Cloud (Redis/Postgres) and Standalone (Local IPC/SQLite) deployments. Currently, the health monitor operates identically across modes without distinction, relying on the transport layer to list active agents. If an agent goes offline in Standalone mode, or if a network partition affects Cloud mode, the health monitor simply fires agents that aren't reporting. A more robust, mode-aware Cross-Mode Health Monitor is required to handle failovers correctly.

### Architecture & Design
The Health Monitor `run_health_monitor` accepts a mode parameter (`is_cloud`) and an explicit heartbeat tick duration (`tick_duration`).
- **Cloud Mode**: Relies on Redis TTLs. If Redis is partitioned, it should tolerate network jitter. The monitor logic tracks missed heartbeats in a state dictionary and only issues agent firing commands after two consecutive missed ticks, shielding the system from transient outages.
- **Standalone Mode**: Runs on local SQLite. Connectivity is guaranteed by localhost. The health check simply verifies the IPC ping without network jitter backoff and fires missing agents immediately on the first missed heartbeat.
- **Protocol**: The health check loop polls agents via `monitor_transport.get_active_agents()`. If an agent misses a heartbeat:
  - In Standalone: Immediately fire.
  - In Cloud: Record in `pending_fires` map and retry next tick.

### Implementation Prompt
1. Modified `src/server/orchestration/health.rs` to support `is_cloud` and `tick_duration`.
2. Built a robust map-based retry fallback for cloud mode.
3. Updated unit tests for strict 100% Rust code coverage.
4. Delivered a 5-scenario E2E UI verification suite validating task reassignment and offline capabilities.
