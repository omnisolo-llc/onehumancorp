# OHC Oracle Research Report: Agent Harness & Sandbox Architecture

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 2rem; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1); color: #fff;">

## Title
Implement OS-Level Namespace Sandboxing via Bubblewrap (`bwrap`)

## Problem Statement
OHC's current agent harness relies on unconstrained process execution, creating a massive security and stability vulnerability. Without robust OS-level isolation, agents can accidentally overwrite host system files, traverse outside their designated worktrees, and bind to arbitrary network ports. Competitors like Claude Code employ strict `bwrap` (Bubblewrap) namespaces to enforce read-only system mounts and restrict visibility purely to the workspace context. OHC needs this native isolation layer to achieve absolute autonomy safely.

## Research Report
### Claude Code (v2.1.88)
Claude Code leverages the internal `@anthropic-ai/sandbox-runtime` for robust, OS-level security.
*   **Isolation Strategy:** Uses `bwrap` (Bubblewrap) for creating unprivileged Linux namespaces, enforcing strict `--bind` and `--ro-bind` (read-only) paths dynamically.
*   **Network Governance:** Spawns local HTTP/SOCKS MITM proxies and intercepts traffic. Unrecognized hosts trigger a `SandboxAskCallback` for human approval.
*   **Execution Validations:** Includes AST-based Bash command validation (`BashTool`), blocking injection vectors, redirections, and Unix domain socket creations via dynamic `seccomp-bpf` filters.
*   **State & Memory:** Utilizes a structured Memory Directory pattern (`src/memdir/`) decoupled via a local CLI to remote Server-Sent Events (SSE) bridge architecture.

### OpenClaw
OpenClaw implements a modular Multi-Tiered Execution Container architecture.
*   **Isolation Strategy:** Uses Docker containers scoped per session to enforce isolation unless explicit host access is granted.
*   **Harness Registry:** Features a dynamic `AgentHarness` registry allowing fallback mechanisms (e.g., `pi-embedded-runner`).

### OHC vs Market Reality
| Feature Capability | OHC Current State | Market Standard (Claude/Claw) | Priority Gap |
| :--- | :--- | :--- | :--- |
| **Execution Sandboxing** | Direct Host execution | `bwrap` OS sandboxes / Docker | 🚨 Critical (P0) |
| **Network Control** | Full Host Network | Intercepting SOCKS/HTTP Proxies | 🟡 High (P1) |
| **AST Bash Verification** | Basic RegEx / Strings | AST Validation (`shell-quote`) | 🟡 High (P1) |

## Design Doc
### Architecture
```mermaid
graph TD
    A[KAIROS Orchestrator] --> B(OHC Hybrid Harness Engine)

    subgraph Secure Execution Sandbox
        B -->|Tool Exec| C{OS-Native Sandbox Manager}
        C -->|Linux| D[bwrap Namespace]
        C -->|Docker| F[Session Container]
    end

    subgraph Telemetry & Control
        D -.-> G[Local MITM Proxy]
        G -->|Allowed| H[Internet]
        G -->|Denied| I[(OHC Central Database / pgvector)]
        B -->|Semantic Check| J[AST Bash Policy Engine]
        J -->|Valid| C
    end

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,C,D,F,G,H,I,J premium;
```

### Components
1. **Harness API (Rust):** Expand `src/server/agents/harness/harness.rs` to support multiple backends (`LocalBwrap`, `Docker`).
2. **Bwrap Adapter:** Create `src/server/agents/harness/bwrap.rs` that implements the `Harness` interface. It must dynamically construct `bwrap` command-line arguments based on an `AgentHarnessPolicy`.
3. **AgentHarnessPolicy:** A struct defining `WorkspaceDir`, `ReadOnlyMounts`, `TmpDir`, and `NetworkEnabled`.

## Implementation Prompt
**Role:** Implementer Agent
**Task:** Implement the `bwrap` backend for the OHC Agent Harness.
1. Define the `AgentHarnessPolicy` struct in `src/server/agents/harness/policy.rs`.
2. Implement `bwrapHarness` in `src/server/agents/harness/bwrap.rs`.
3. The `Exec` method should construct a `bwrap` command. It MUST include: `--unshare-all`, `--share-net` (if network is enabled), `--ro-bind / /`, and `--bind <workspace> <workspace>`.
4. Ensure 100% unit test coverage for `bwrap.rs` using a mock exec interface.

## Priority
P0

## Estimated Scope
Medium
</div>
