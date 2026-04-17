<<<<<<< HEAD
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
=======
<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# OS-Level Sandbox Isolation Harness: Visual Walkthrough

This guide details the architecture, configuration, and security guarantees of the new OS-Level Sandbox Isolation Harness (bwrap/sandbox-exec) integration for One Human Corp. It provides a premium visual walkthrough of the security isolation mechanism and how to debug sandbox violations.

## 1. Architectural Flow

The Sandbox Isolation Harness wraps all agent executions in a restricted environment, ensuring that untrusted code cannot access the host file system or network without explicit permission.

```mermaid
graph TD
    A[Cloud Orchestrator] -->|Spawns Agent Task| B{Sandbox Harness}
    B -->|Bwrap / sandbox-exec| C[Agent Runtime Execution]
    C -->|Intercepted Network Requests| D[Network Proxy]
    D -->|Sanitized & Monitored| E(Telemetry Mesh)

    style A fill:#003366,stroke:#333,stroke-width:2px,color:#fff
    style B fill:#800000,stroke:#333,stroke-width:2px,color:#fff
    style C fill:#006699,stroke:#333,stroke-width:2px,color:#fff
    style D fill:#993300,stroke:#333,stroke-width:2px,color:#fff
    style E fill:#0099cc,stroke:#333,stroke-width:2px,color:#fff
```

## 2. Configuration & Security Guarantees

The integration utilizes either `bwrap` (Linux) or `sandbox-exec` (macOS) to enforce:
- **File System Isolation:** Read-only access to root, with a temporary in-memory filesystem (`tmpfs`) for the workspace.
- **Network Isolation:** All traffic is routed through our internal proxy, blocking unauthorized external domains.
- **Process Isolation:** The agent is given its own PID namespace.

## 3. Debugging Sandbox Violations

When an agent attempts a restricted action, a violation event is emitted to the Telemetry Mesh.
To debug:
1. Access the OHC Observability Dashboard.
2. Filter logs by `event_type: sandbox_violation`.
3. Inspect the attempted path or external domain.

</div>
>>>>>>> bca7df9d (✍️ Scribe: Implement documentation for OS-Level Sandbox Isolation Harness)
