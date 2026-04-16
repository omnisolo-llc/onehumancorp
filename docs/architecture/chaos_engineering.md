<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# OHC Chaos Engineering Resilience Architecture

**Overview**
The OHC Hybrid Agentic OS guarantees stability through relentless failure simulation. Our chaos engineering strategy systematically injects faults to ensure absolute autonomy and resilience across Cloud, Standalone, and Thin Client modes. This architecture is designed to continuously inject controlled failures into the Agent Harness and cluster orchestrator, allowing the system to self-heal and adapt to market reality.

## Chaos Injection Framework

The Chaos Injector (`lib/resilience/chaos/chaos.go`) is responsible for surfacing latent vulnerabilities within the Agent Harness by orchestrating controlled failure modes. Failure injection is orchestrated via a dedicated sub-swarm. This swarm safely injects faults across nodes, tests our graceful degradation in Standalone Mode, and verifies the "Zero Secrets" SPIFFE/SPIRE identity architecture under stress.

### Specific Failure Modes

*   **`CorruptAgentLock`:** Simulates race conditions and split-brain scenarios by forcibly invalidating distributed Redis locks (Cloud) or SQLite transaction locks (Standalone) while an agent is executing a task. The orchestrator must detect the corrupted lock, safely fence off the affected agent, and transparently failover the operation without losing any durable state or violating the Swarm Intelligence principles.
*   **`DropMeshSync`:** Artificially severs the connection between Standalone mode and the Cloud sync protocol, testing local metric buffering and eventual consistency upon reconnection.
*   **`SimulatedSandboxViolation`:** Triggers a synthetic AST validation failure to ensure the Harness Interceptor Engine accurately blocks unsafe execution and increments `ohc_sandbox_violation_total`.

## Interpreting Chaos Metrics

The **Chaos Resilience** dashboard (`monitoring/grafana/dashboards/chaos_resilience.json`) visualizes the swarm's recovery capabilities. It is your primary lens into the swarm's health during chaos events.

### Key Panels & Interpretation

*   **Injected Failures vs. Recovery Time:** Correlates `ohc_chaos_injected_total` with `ohc_task_recovery_time_ms`. A spike in injection should be followed by a controlled, bounded increase in recovery time without escalating error rates. Visualizes the time taken for the system to recover from an injected failure. A spike indicates a resilience gap.
*   **Environment Mode Comparison:** Utilizes the `EnvMode` tag to compare recovery characteristics. Cloud mode should rely on pod restarts and K8s orchestration, while Standalone mode should demonstrate robust local backoff and retry mechanisms.
*   **Sandbox Violation Tracking:** Monitors `ohc_sandbox_violation_total` during synthetic security breaches to confirm the AST Validator correctly intercepts all malicious inputs.
*   **Lock Contention Metrics:** Pay special attention to the `CorruptAgentLock` panels. They display the rate of lock corruption events and the percentage of successful automated recoveries.
*   **Token Burn Rate:** Chaos events should not cause uncontrolled token burn. Monitor the extrapolated 24h burn rate forecast to ensure budget compliance during stress tests.

</div>
