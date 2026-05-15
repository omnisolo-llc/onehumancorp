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

## Advanced Observability Techniques

### 12. Synthetic Transaction Monitoring
We cannot rely solely on real user traffic to detect issues. OHC must deploy synthetic "bots" that continuously navigate the platform 24/7, creating test stores, processing fake payments, and triggering agent workflows. If a synthetic transaction fails, an alert is triggered immediately, often before a real user ever encounters the bug.

### 13. Distributed Tracing for LLM Latency
The biggest bottleneck in the agentic swarm is LLM inference latency. We must use distributed tracing (OpenTelemetry) to precisely measure the time spent in prompt compilation, the round-trip time to the LLM provider (OpenAI/Anthropic), and the time spent parsing the response. This data is critical for deciding when to route requests to faster, smaller models vs. slower, more capable models.

### 14. Anomaly Detection on Business Metrics
Health monitoring isn't just about server CPU; it's about business health. We must establish baseline metrics for key user actions (e.g., average time to first sale, number of Action Cards approved per day). If the system-wide average for "Action Cards Approved" drops by 20%, it likely indicates a UX bug or a degradation in agent proposal quality, even if all servers are technically "healthy."


### 15. Real User Monitoring (RUM) Integrations
Tracking core web vitals across the generated storefronts is paramount for SEO and user experience. We must deploy Real User Monitoring (RUM) agents on tenant sites to capture First Input Delay (FID), Largest Contentful Paint (LCP), and Cumulative Layout Shift (CLS) natively across the platform.

### 16. Error Budgeting
For SRE purposes, OHC needs to establish formal Error Budgets. When an error budget for a specific service or agent is exhausted, feature development should temporarily freeze, and engineering resources should pivot to reliability and stability improvements until the budget recovers.
### 17. Alert Fatigue Mitigation
### 18. SLO Definition per Agent
### 19. Node Starvation Monitoring
### 20. Network Egress Cost Monitoring
### 21. Alert Fatigue Mitigation
### 22. SLO Definition per Agent
### 23. Node Starvation Monitoring
### 24. Network Egress Cost Monitoring
