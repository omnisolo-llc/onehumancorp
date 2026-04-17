# OS-Level Sandbox Isolation Harness Walkthrough

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1); color: #fff;">

Welcome to the walkthrough for the **OS-Level Sandbox Isolation Harness**. This document provides an architectural overview of the secure execution environment used by sub-agents.

## Architecture & Flow

To guarantee safety and observe behavior, OHC restricts sub-agent capabilities. The orchestrator wraps sub-agent processes in a restricted shell (`bwrap` on Linux or `sandbox-exec` on macOS). All network egress routes through an internal proxy.

```mermaid
graph TD;
    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);

    O[Orchestrator]:::premium -->|Spawns with Harness| B[Bwrap/sandbox-exec]:::premium;
    B -->|Restricted File Access| FS[(Isolated File System)]:::premium;
    B -->|Proxied Requests| P[Network Proxy]:::premium;
    P -->|Allowed traffic| Inet[Internet]:::premium;
    P -->|Violations & Telemetry| TM[Telemetry Mesh]:::premium;
```

## Security Guarantees & Configuration

- **File System Protection:** The root file system is mounted read-only. A dedicated temporary directory (e.g., `/tmp`) is provided for intermediate scratchpad space.
- **Network Isolation:** Only whitelisted domains can be accessed by the agent. Egress attempts outside the allowed set are intercepted and blocked by the Network Proxy.
- **Observability:** Blocked network calls emit high-fidelity OpenTelemetry metrics via the Telemetry Mesh, allowing operators to detect anomalies.

## Debugging Violations

1. Navigate to the **Grafana Observability Dashboard**.
2. Filter the `telemetry.sandbox_violation_total` metric by `agent_id`.
3. Check the denied requests and verify the requested domains against the whitelist defined in `srcs/backend/harness/network_proxy.go`.
4. Adjust the sub-agent instructions or expand the proxy whitelist if the request is legitimate.

</div>
