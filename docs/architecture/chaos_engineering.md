<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# OHC Chaos Engineering Resilience Architecture

**Overview**
The OHC Hybrid Agentic OS guarantees stability through relentless failure simulation. Our chaos engineering strategy systematically injects faults to ensure graceful degradation and robust recovery across both Cloud-Native and Standalone execution modes.

## Chaos Injection Framework

The Chaos Injector (`lib/resilience/chaos/chaos.go`) is responsible for surfacing latent vulnerabilities within the Agent Harness by orchestrating controlled failure modes.

### Specific Failure Modes

*   **`CorruptAgentLock`:** Simulates race conditions and split-brain scenarios by forcibly invalidating distributed Redis locks (Cloud) or SQLite transaction locks (Standalone) while an agent is executing a task.
*   **`DropMeshSync`:** Artificially severs the connection between Standalone mode and the Cloud sync protocol, testing local metric buffering and eventual consistency upon reconnection.
*   **`SimulatedSandboxViolation`:** Triggers a synthetic AST validation failure to ensure the Harness Interceptor Engine accurately blocks unsafe execution and increments `ohc_sandbox_violation_total`.

## Interpreting Chaos Metrics

The **Chaos Resilience** dashboard (`monitoring/grafana/dashboards/chaos_resilience.json`) visualizes the swarm's recovery capabilities.

### Key Panels & Interpretation

*   **Injected Failures vs. Recovery Time:** Correlates `ohc_chaos_injected_total` with `ohc_task_recovery_time_ms`. A spike in injection should be followed by a controlled, bounded increase in recovery time without escalating error rates.
*   **Environment Mode Comparison:** Utilizes the `EnvMode` tag to compare recovery characteristics. Cloud mode should rely on pod restarts and K8s orchestration, while Standalone mode should demonstrate robust local backoff and retry mechanisms.
*   **Sandbox Violation Tracking:** Monitors `ohc_sandbox_violation_total` during synthetic security breaches to confirm the AST Validator correctly intercepts all malicious inputs.

</div>
