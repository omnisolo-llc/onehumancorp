<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 2rem; font-family: 'Outfit', 'Inter', sans-serif;">

# Title: [integrations] Hybrid System Resource Observer MCP

## Problem Statement
OHC's "Elastic Swarm Bursting" capability requires agents to autonomously decide when to migrate intensive tasks from a local Standalone environment to the Cloud-Native API. Currently, agents lack a unified, real-time mechanism to observe system resources (CPU, RAM, GPU, Disk) that adapts to the deployment footprint. Without this "Resource Observer," agents cannot intelligently trigger handoffs when local compute is saturated, leading to performance degradation and a suboptimal "Hybrid OS" experience.

## Research Report
Market analysis of agentic frameworks shows a lack of "resource-aware orchestration."
- **Claude Code**: Has limited visibility into local system state and no native "cloud-bursting" protocol.
- **OHC Advantage**: OHC-HA's multi-modal design allows for seamless task migration. By integrating a Hybrid Resource Observer MCP, we enable the swarm to monitor its own "physical" health and make data-driven decisions about task placement.
- **Technical Reference**: Utilization of `gopsutil` for local metrics and Kubernetes Metrics API for cloud-native visibility ensures 100% coverage across the hybrid spectrum.

## Design Doc
**Architecture:**
- Create a new package `srcs/server/lib/integrations/resource_observer/`.
- Introduce a `ResourceObserverManager` implementing the MCP Tool interface.
- Dynamically route metric collection based on `os.Getenv("OHC_MULTITENANT") == "true"`.
- **Cloud Mode**: Fetch metrics from Prometheus or the Kubernetes Metrics API (via `k8s.io/metrics`).
- **Standalone Mode**: Utilize `github.com/shirou/gopsutil` to gather host-level statistics.

**API Contracts:**
- `GetSystemLoad(ctx context.Context) (ResourceStats, error)`
- `CheckBurstEligibility(ctx context.Context, threshold float64) (bool, error)`

**Security:**
- Apply `RedactInterfacePII` to process lists or sensitive system metadata before returning to the agent.
- Ensure strict organization isolation when querying cloud-wide metrics.

## Implementation Prompt
"Implement the Hybrid System Resource Observer MCP tool in `srcs/server/lib/integrations/resource_observer/`.
1. Create `observer.go` defining the `ResourceObserverManager` and its MCP capabilities (`GetSystemLoad`, `CheckBurstEligibility`).
2. Implement mode detection: if `OHC_MULTITENANT=true`, use the Kubernetes Metrics client; otherwise, use `gopsutil` for local host metrics.
3. For Standalone mode, ensure the tool can report CPU usage per core and available VRAM (if applicable).
4. For Cloud mode, ensure metrics are scoped to the agent's specific pod/namespace.
5. Apply PII redaction to any process-specific data.
6. Create comprehensive tests in `observer_test.go` mocking both the K8s API and local system calls.
7. Update `BUILD.bazel` to include the new files and dependencies (`github.com/shirou/gopsutil/v3`, `k8s.io/metrics`)."

## Priority
P1

## Estimated Scope
Medium
</div>
