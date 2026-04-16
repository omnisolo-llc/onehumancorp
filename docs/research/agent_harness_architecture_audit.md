# Competitive Architecture Research: Agent Harness and Telemetry

## 1. Abstract
This report details the architectural requirements for the OHC Hybrid Agent OS by auditing market-leading agent environments (OpenClaw, Hermes Agent, Gstack, and Claude Code's leaked runtime). The primary focus is the Agent Harness implementation: execution sandboxing, network isolation, filesystem protection, and observability.

## 2. Competitive Landscape

### OpenClaw
OpenClaw provides a highly capable agentic runtime but runs mostly in user-space without deep OS-level strict isolation.
*   **Harness Architecture**: Relies on specific tool whitelists.
*   **Weakness**: Runaway agents can execute unauthorized CLI commands if an exploit vector is found.

### Hermes Agent
Hermes utilizes Docker containers for isolation.
*   **Harness Architecture**: Strong isolation but high overhead.
*   **Weakness**: Unsuitable for the instantaneous sub-agent spawning required by OHC's Teammate Mesh due to TTFB (Time to First Byte) latency constraints.

### Claude Code Sandbox Runtime
The `@anthropic-ai/sandbox-runtime` package implements a mature, low-latency, highly secure Agent Harness.
*   **Harness Architecture**:
    *   **Bubblewrap (bwrap)**: Deep OS-level namespace sandboxing on Linux.
    *   **Network Namespace Bridging**: Uses `--unshare-net` to sever internet access, re-connecting via a strict `socat` proxy bridge on Unix sockets.
    *   **Pre-execution Git Scrubbing**: Actively prevents agents from breaking out of the sandbox via Git hooks by mounting `/dev/null` over planted `.git` internals (`HEAD`, `objects`, `refs`).
    *   **Token-Level AST Validation**: Validates shell commands using AST parsers (e.g. `tree-sitter-bash`) before execution to block subshell obfuscation (`echo "su"$(echo "do")`).

## 3. OHC Feature Gap Synthesis
To achieve market dominance, OHC must adopt a strict `bwrap` strategy.

<div class="glass-panel" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">
  <h3>Visual Architecture Compare</h3>

  <pre class="mermaid">
  graph TD
      subgraph Claude Code Sandbox
          CC_B[bwrap --unshare-net]
          CC_S[socat Unix Socket Proxy]
          CC_G[Git Scrubbing Filter]
          CC_T[AST Validation]
          CC_B --> CC_S
          CC_B --> CC_G
          CC_B --> CC_T
      end

      subgraph OHC Standalone Target
          OHC_B[bwrap OS Sandbox]
          OHC_P[Socat Network Bridge Proxy]
          OHC_G[Pre/Post Command Git Scrubbing]
          OHC_T[AST Validation for Terminal Execution]
          OHC_O[OpenTelemetry Metric & I/O Instrumentation]

          OHC_B --> OHC_P
          OHC_B --> OHC_G
          OHC_B --> OHC_T
          OHC_B --> OHC_O
      end

      classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff;
      class CC_B,CC_S,CC_G,CC_T,OHC_B,OHC_P,OHC_G,OHC_T,OHC_O premium;
  </pre>
</div>

| Feature Area | OpenClaw | Claude Code Sandbox | **OHC (Target)** |
| :--- | :--- | :--- | :--- |
| **OS Isolation** | No | Yes (bwrap) | **Yes (Strict bwrap)** |
| **Network Control** | Host Network | Proxy Bridge | **Socat Proxy Bridge** |
| **Git Vulnerability Protect** | No | Yes (Scrubbing) | **Yes (Pre/Post Scrubbing)** |
| **Command Validation** | Regex | AST Validation | **Token-Level AST** |
| **Telemetry** | Local Logs | Local Logs | **OpenTelemetry** |

## 4. Actionable Missions
The following GitHub Issues have been created for Implementer agents:
1.  **Issue #5182**: `[harness] Implement Socat Network Bridge Proxy for Desktop Mode Agent Harness`
2.  **Issue #5184**: `[harness] Implement Git Repository Scrubbing inside Local Harness`
3.  **Issue #5192**: `[harness] Implement Token-Level Command Validation for Terminal Execution`
4.  **Issue #5193**: `[harness] Emit Metric Telemetry and I/O Instrumentation for Harness Operations`
