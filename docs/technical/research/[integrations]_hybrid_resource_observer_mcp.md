<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Title: [integrations] Hybrid System Resource Observer MCP

## Problem Statement
OHC agents currently lack high-fidelity visibility into the host machine's hardware state (CPU, RAM, GPU, Disk, Network) in both Cloud-native and Standalone modes. This makes "Elastic Swarm Bursting" decisions (e.g., deciding whether to run a local LLM or offload to the cloud) ad-hoc and inefficient. Agents cannot intelligently adapt their workload based on real-time resource availability.

## Research Report
The KAIROS "Hybrid Agentic OS" requires a "System Resource Observer" that provides a unified view of hardware metrics. In Cloud-native mode, this data is often sourced from Kubernetes/Prometheus metrics. In Standalone mode, it must be sourced directly from the host OS (Linux, macOS, Windows).

### Competitive Analysis
| Feature | Basic Agent Frameworks | Cloud-Only Monitoring | OHC Hybrid Resource Observer |
| :--- | :--- | :--- | :--- |
| **Real-time Local Stats** | ❌ No | ❌ No | ✅ Yes |
| **GPU Visibility** | ❌ No | ✅ Partial | ✅ Yes (NVIDIA/Metal) |
| **Swarm Integration** | ❌ No | ✅ High | ✅ High |

### Key Technologies
- **`shirou/gopsutil/v3`**: For cross-platform system metrics.
- **NVIDIA Management Library (NVML)**: For GPU monitoring.
- **OpenTelemetry**: For exporting resource state to the OHC telemetry mesh.

## Design Doc
**Architecture:**
- **Resource Observer MCP**: Implements the MCP Tool interface.
- **Cloud Mode**: Interfaces with the Kubernetes Metrics API or queries the internal Prometheus instance using the shared `http.Client`.
- **Standalone Mode**: Uses `gopsutil` to fetch real-time host metrics.
- **Integration**: Feeds data into the KAIROS state machine to enable cost-and-resource-aware task routing.

**API Contracts:**
- `GetSystemLoad() (LoadStats, error)`
- `GetMemoryUsage() (MemStats, error)`
- `GetGPUStatus() (GPUStats, error)`

**Security:**
- Read-only access to metrics.
- PII Redaction for any sensitive environment variables or process names.

## Implementation Prompt
"Implement the Hybrid System Resource Observer MCP tool in `srcs/server/lib/integrations/resource_observer/`.
1. Create `observer.go` defining the `ResourceObserver` MCP tool.
2. Integrate `github.com/shirou/gopsutil/v3` for cross-platform CPU and RAM metrics.
3. In Standalone mode, directly query the host OS.
4. In Cloud mode, implement a client that fetches metrics from the Kubernetes Metrics API (via `k8s.io/client-go`).
5. Provide MCP tools: `get_system_health` (returns CPU/RAM/GPU/Disk summary) and `is_resource_available` (takes threshold, returns boolean).
6. Ensure 100% test coverage with mocks for both the OS metrics and the K8s API.
7. Add an E2E test proving that an agent can query local CPU usage in Standalone mode."

## Priority
P1

## Estimated Scope
Medium

</div>
