<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 2rem; border-radius: 12px; border: 1px solid rgba(255,255,255,0.1);">

# Title
[backend] Implement Bubblewrap Sandboxing & Native MCP Integration

# Problem Statement
Competitors like Claude Code employ robust Agent Harness environments using native OS-level sandboxing (`bwrap`), explicit `allowRead` and `denyWrite` capabilities, and native MCP integration via `@anthropic-ai/sandbox-runtime`. OHC currently lacks this granular isolation, exposing vulnerabilities, especially in our Standalone Mode, and relies on generic OS boundaries rather than strict, scoped policies.

# Research Report
An analysis of the leaked `claude-code (v2.1.88)` source code, Hermes Agent, and OpenClaw projects indicates:
- **Bubblewrap (`bwrap`) Sandboxing:** Agents execute inside isolated namespaces on Linux. The runtime dynamically mounts permitted paths (`allowRead`) and enforces `denyWrite` to ensure agents cannot modify unauthorized host files. It also uses bind-mounts over `/dev/null` for denied paths.
- **Native MCP Integration:** The harness runs natively with Model Context Protocol (MCP), dynamically loading external capabilities.
- **Memory Directory Pattern:** Local state is handled via a dedicated Memory Directory pattern (`memdir.ts`), enabling efficient state sharing in localized environments.

### Competitive Analysis: OHC vs Market

| Feature | OHC (Current State) | Market Standard (Claude Code) | Gap Resolution |
| :--- | :--- | :--- | :--- |
| **Sandboxing Isolation** | OS-level generic boundaries | Strict FS/Network per-command rules (`bwrap`) | Integrate scoped sandbox policies with `bwrap` |
| **Tool Integration** | Bespoke Interfaces | Native MCP (Model Context Protocol) | Implement native MCP support in Agent Harness |
| **State Sharing** | Centralized OHC-SIP | File-based memory directory (`memdir.ts`) | Add local memory directory fallback for Standalone Mode |

# Design Doc
The **Hybrid Agent Harness** requires major implementations:
1. **`bwrap` Wrapper:** Implement an OS-level wrapper around agent execution (on Linux) enforcing `allowRead` and `denyWrite` rules dynamically for each command via Bubblewrap (`bwrap`).
2. **Native MCP Support:** Implement an MCP manager that registers the harness as an MCP server and loads capabilities dynamically.
3. **Memory Directory Setup:** Automatically provision scoped memory directories for Standalone Mode.

```mermaid
graph TD;
    A[KAIROS Orchestrator] -->|Dispatch Task| B(Hybrid Agent Harness);
    B -->|Enforce read/write/network rules| C{Bubblewrap Sandbox};
    B -->|Load External Capabilities| E[Native MCP Integration];
    B -->|Provision State| F[Local Memory Directory];
    C --> D[Sub-Agent Execution];

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,C,D,E,F premium;
```

# Implementation Prompt
- **[backend] Implement bwrap Execution Wrapper:** Build functionality in the Agent Harness handling `bwrap` rules (`allowRead`/`denyWrite`). Ensure tests use mocking or skip if `bwrap` is absent.
- **[backend] Implement Native MCP Integration:** Build native Model Context Protocol support into the Agent Harness for seamless tool loading.
- **[backend] Implement Local Memory Directory Fallback:** Create memory directories and update system prompts to enforce usage of this directory for localized state.

# Priority
P0

# Estimated Scope
Large

</div>
