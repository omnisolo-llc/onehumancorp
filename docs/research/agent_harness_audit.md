<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# AI Agent Harness & Sandbox Architecture Report: Claude Code vs. OHC

## 1. Executive Summary

This report analyzes the Agent Harness and Sandbox implementations of industry-leading AI tools, specifically focusing on Anthropic's **Claude Code** (v2.1.88) and contrasting it with One Human Corp's (OHC) current Hybrid OS Architecture. The goal is to define actionable feature-gap missions that will elevate OHC's execution environments to state-of-the-art standards.

## 2. Competitive Architectural Breakdown: Claude Code's "Sandbox Runtime"

Claude Code utilizes a highly sophisticated execution harness called `@anthropic-ai/sandbox-runtime`, designed to run AI-generated commands safely on a user's machine.

### 2.1 Core Isolation Mechanisms

*   **Linux Namespace Sandboxing (Bubblewrap / `bwrap`)**: Claude relies heavily on `bwrap` to create unprivileged Linux containers.
    *   **PID Namespace Isolation (`--unshare-pid`)**: Prevents the agent from seeing or signaling host processes.
    *   **Network Isolation (`--unshare-net`)**: Blocks direct outbound internet access.
    *   **Filesystem Restrictions**: Uses `--ro-bind` for read-only access and explicitly binds only allowed directories. Denied paths are cleverly mounted as empty directories or `/dev/null` to prevent the agent from recreating them.
*   **Seccomp BPF Filtering**: Employs a custom `apply-seccomp` C binary to restrict syscalls. Crucially, it blocks UNIX domain socket creation (`socket(AF_UNIX)`) to prevent agents from communicating with host daemons (like Docker or X11) through default socket locations, unless explicitly allowed.
*   **Network Bridging (Socat)**: To provide controlled internet access despite `--unshare-net`, Claude sets up a proxy bridge using `socat`. The agent communicates with an internal listener, which forwards traffic via a UNIX socket to a proxy server running on the host, ensuring all network traffic is interceptable and configurable.
*   **Mac OS Strategy (`sandbox-exec`)**: On macOS, it relies on Apple's native `sandbox-exec` utility (Seatbelt profiles) to enforce similar filesystem and network restrictions.

### 2.2 Telemetry and I/O Instrumentation

*   **Cost Tracking**: Claude incorporates a `cost-tracker.ts` component that meticulously monitors token usage and API costs associated with the agent's actions within the harness.
*   **Violation Store**: Sandboxing events and policy violations are recorded in a `sandbox-violation-store.js`, providing an audit trail of attempted escapes or unauthorized accesses.

## 3. OHC Architecture Comparison

| Feature | Claude Code (Anthropic) | OHC Hybrid Architecture (Current) | Gap / Opportunity |
| :--- | :--- | :--- | :--- |
| **Execution Engine** | Local unprivileged namespaces (`bwrap` / `sandbox-exec`) | K8s Pods (Cloud) / Bazel OCI / Native Shell (Standalone) | OHC lacks fine-grained, localized process isolation *within* the Standalone Desktop client without relying on full Docker/K8s overhead. |
| **Network Control** | Proxied Bridge (socat) + isolated netns | Cloud: NetworkPolicies. Local: Unrestricted | Standalone mode agents have unrestricted host network access. |
| **Filesystem Safety** | Explicit bind mounts + `/dev/null` blackholes | Cloud: Ephemeral Volumes. Local: SQLite / Host FS | Standalone mode risks agent modifying unintended host files. |
| **Syscall Filtering** | Seccomp BPF (Unix Socket Blocking) | Cloud: Basic K8s seccomp. Local: None | High risk of local privilege escalation or daemon abuse in Standalone. |
| **State Persistence** | Transient within execution | Distributed AutoDream (pgvector/Pinecone) & Central DB | OHC excels at multi-agent shared state, but needs tighter sandbox integration with this state. |

## 4. Architectural Synthesis & Recommendations for OHC

To maintain "Absolute Autonomy" while ensuring "Zero Secrets" and local safety, OHC must develop a **Next-Generation Agent Harness Sandbox**. This harness must bridge the gap between heavy Cloud-Native K8s isolation and unsafe Standalone native execution.

**The OHC Unified Sandbox Adapter Pattern:**
We need an abstraction layer that dynamically chooses the best isolation strategy based on the host environment (K8s, Docker, or native Desktop), providing a uniform interface for the orchestrator.

## 5. Actionable Roadmap & Feature Gap Missions

The following critical missions will be generated as GitHub issues for the Swarm to execute.

1.  **[backend] Implement Native Bubblewrap Isolation Layer for Standalone Agents**
    *   **Goal**: Replicate Claude's `bwrap` strategy for Linux desktop users, allowing the Standalone OS to run untrusted agent code without full Docker overhead.
    *   **Priority**: P0
2.  **[security] Develop Seccomp BPF Filter for Agent Process Spawning**
    *   **Goal**: Create a native Go utility to apply strict seccomp profiles to agent sub-processes, specifically blocking unauthorized socket creation.
    *   **Priority**: P1
3.  **[observability] Integrate Sandbox Violation Telemetry into OpenTelemetry**
    *   **Goal**: Ensure any blocked syscalls or filesystem access attempts by the harness are exported via OTLP to our Grafana dashboards.
    *   **Priority**: P2

```mermaid
graph TD
    A[OHC Orchestrator] -->|Dispatch Task| B(Unified Sandbox Adapter)
    B -->|Cloud Mode| C[K8s Ephemeral Pod]
    B -->|Standalone (Linux)| D[Bubblewrap Namespace]
    B -->|Standalone (macOS)| E[sandbox-exec Profile]
    D -.->|Syscall Block| F[Seccomp Filter]
    D -.->|Network Route| G[Socat Proxy Bridge]
    C --> H[(OHC-SIP Central DB)]
    D --> H
    E --> H
```

</div>
