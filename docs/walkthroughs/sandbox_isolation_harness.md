# OS-Level Sandbox Isolation Harness

<div style="background: rgba(255, 255, 255, 0.05); backdrop-filter: blur(20px) saturate(200%); border: 1px solid rgba(255, 255, 255, 0.1); border-radius: 12px; padding: 24px; margin-bottom: 24px;">
  <strong>Security Note:</strong> This feature ensures zero-trust execution by wrapping agent processes in OS-level isolation (Bubblewrap for Linux, sandbox-exec for macOS).
</div>

## Architecture

The OS-Level Sandbox Isolation Harness provides strict, system-level containment for untrusted agent workflows. By utilizing native isolation mechanisms (Bubblewrap/sandbox-exec), OHC ensures that processes cannot access unauthorized network endpoints or read sensitive local files outside their designated space.

## Configuration

Sandbox profiles are configured to strictly limit agent capabilities, ensuring isolation from the host environment.

## Security Guarantees

The sandbox provides robust security guarantees against unauthorized access and privilege escalation.

## Execution Flow

```mermaid
sequenceDiagram
    participant O as Orchestrator
    participant B as Bwrap/sandbox-exec
    participant A as Agent Process
    participant N as Network Proxy
    participant T as Telemetry Mesh

    O->>B: Spawn Sandbox with strict profile
    B->>A: Execute workload in isolation
    A->>N: Network request (intercepted)
    N-->>T: Log network telemetry
    N-->>A: Allowed Response / Denied
    A-->>B: Process Exit
    B-->>O: Return Exit Code & Metrics
```