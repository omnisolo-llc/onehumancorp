<div markdown="1" style="backdrop-filter: blur(20px) saturate(1.213); background: rgba(255, 255, 255, 0.05); border: 1px solid rgba(255, 255, 255, 0.08); font-family: 'Outfit', 'Inter', sans-serif; color: #ffffff; border-radius: 12px; padding: 24px; box-shadow: 0 8px 32px 0 rgba(0, 0, 0, 0.3);">

# Market Audit: Agent Harness Network Isolation Strategy

**Author**: Principal Product Researcher & Oracle (L7)
**Date**: 2026-04-16

## Executive Summary
This report details the network isolation strategies utilized by top market competitors (notably Claude Code) for their Agent Harness environments. The goal is to provide a strategic blueprint for OHC Standalone Desktop Mode to securely isolate agent network I/O.

## Competitive Analysis: Network Sandboxing

### Claude Code (Anthropic Sandbox Runtime)
Our deep audit of the `@anthropic-ai/sandbox-runtime` implementation reveals a sophisticated approach to network isolation on Linux:
- **Namespace Isolation**: Execution begins with `bwrap --unshare-net`, entirely removing access to the host's network interfaces.
- **Unix Socket Bridge**: Instead of raw network access, `socat` is used to create Unix sockets (`/tmp/http.sock`, `/tmp/socks.sock`) on the host. These are bind-mounted into the `bwrap` environment (`--bind /tmp/http.sock /tmp/http.sock`).
- **Proxy Environment**: Internal `socat` processes forward ports like `3128` (HTTP) and `1080` (SOCKS) to these Unix sockets. Environment variables (`HTTP_PROXY`, `ALL_PROXY`) are injected so all standard agent network libraries automatically route through the bridge.
- **Host Validation**: The host proxy receives traffic via the Unix socket, allowing it to perform allowlist/blocklist validation before sending requests out to the internet.

## The OHC "Blue Ocean" Advantage

| Feature Area | OHC Current State | Market Standard | **OHC Hybrid Vision** |
| :--- | :--- | :--- | :--- |
| **Network Access** | Host Networking | `bwrap --unshare-net` | **Zero-Trust K8s & bwrap Network Namespaces** |
| **Outbound Routing** | Direct | `socat` Unix Bridges | **Encrypted SPIFFE-Aware Local Proxies** |
| **Observability** | None | Limited logging | **OpenTelemetry Spans per Request** |

## Architectural Blueprint
To achieve state-of-the-art security, OHC must adopt a bridged network architecture for local agent execution.

```mermaid
graph TD
    A[Sub-Agent Execution bwrap] -->|HTTP_PROXY| B(Internal socat listener)
    B -->|Unix Socket /tmp/ohc.sock| C(Host-side proxy / Telemetry Engine)
    C -->|Allowed Request| D[Internet]
    C -->|Blocked / Logged| E[(pgvector AutoDream / Prometheus)]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B premium;
    class C,D,E premium;
```

</div>
