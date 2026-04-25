# [Backend] OHC Agent Harness vs Market Standards

## Problem Statement
OHC currently lacks a robust execution harness for isolated agent actions, limiting our ability to safely execute terminal commands, modify arbitrary files, and manage background tasks without host interference or security risks. We need an advanced isolation and telemetry layer comparable to top tier AI agent frameworks.

## Research Report
### Market Standard Analysis: Claude Code (Leaked v2.1.88)
Based on analyzing the Claude Code source tree (`@anthropic-ai/sandbox-runtime` and `CC-Source`), their execution harness consists of:
1. **OS-Level Sandboxing**:
   - Uses `bwrap` (Bubblewrap) on Linux for complete namespace isolation.
   - Generates strict Seccomp BPF filters (`generate-seccomp-filter.js`).
   - Validates symlink traversal and blocks dangerous paths dynamically.
2. **Network Interception**:
   - Uses HTTP/SOCKS proxy injection to filter or monitor all outbound tool network requests (`http-proxy.js`, `socks-proxy.js`).
3. **Execution Control**:
   - `BashTool.tsx` interfaces with `SandboxManager`.
   - Distinguishes carefully between Read-only commands (e.g. `cat`, `grep`) and Write/Destructive commands (`rm`, `sed`).
   - Implements shell AST parsing (`utils/bash/ast.js`) to evaluate security constraints before execution.
4. **Telemetry & Instrumentation**:
   - `SandboxViolationStore` tracks every denied action for auditing.
   - Comprehensive cost tracking (`cost-tracker.ts`).
   - Local task management with backgrounding capabilities (`LocalShellTask.js`).

### OHC Gap Analysis
OHC currently runs commands natively without a protective virtualization layer or tight network control for AI agents.

## Design Doc
### Architecture
Introduce `ohc-harness` service module.
1. **Execution Engine**:
   - Wrap local tool commands using Linux Namespaces / MacOS Sandbox equivalent.
   - Integrate with `gvisor` or `bwrap` to prevent breakout.
2. **Network Proxy**:
   - Route all agent shell tasks through a local forward proxy that logs telemetry via OpenTelemetry.
3. **Command Semantics Parser**:
   - Implement an AST parser to pre-validate bash scripts before launching.

```mermaid
graph TD
    A[Agent Bash Tool] --> B[OHC Harness AST Parser]
    B -->|Validation Pass| C[bwrap Sandbox]
    B -->|Validation Fail| D[Violation Store / OHC-SIP]
    C --> E[Execution]
    C -.-> F[SOCKS Proxy Injector]
    F --> G[Network Control & OTEL]
    style A fill:rgba(255,255,255,0.03),stroke:#fff,stroke-width:1px
    style B fill:rgba(255,255,255,0.03),stroke:#fff,stroke-width:1px
    style C fill:rgba(255,255,255,0.03),stroke:#fff,stroke-width:1px
    style D fill:rgba(255,255,255,0.03),stroke:#fff,stroke-width:1px
    style E fill:rgba(255,255,255,0.03),stroke:#fff,stroke-width:1px
    style F fill:rgba(255,255,255,0.03),stroke:#fff,stroke-width:1px
    style G fill:rgba(255,255,255,0.03),stroke:#fff,stroke-width:1px
```

### API Contracts
```typescript
interface OHCSandboxConfig {
    readPaths: string[];
    writePaths: string[];
    networkPolicies: NetworkPolicy[];
}
```

<style>
/* Premium Feel OHC Tokens */
body {
    font-family: 'Outfit', 'Inter', sans-serif;
    backdrop-filter: blur(20px) saturate(200%);
    background: rgba(255, 255, 255, 0.03);
    color: #eee;
}
table {
    width: 100%;
    border-collapse: collapse;
}
th, td {
    border: 1px solid rgba(255,255,255,0.1);
    padding: 8px;
    text-align: left;
}
th {
    background: rgba(255, 255, 255, 0.05);
}
</style>

### Comparative Table
| Feature | OHC Current | Claude Code Harness | Target OHC Harness |
|---------|-------------|----------------------|--------------------|
| **Execution** | Native Shell | bwrap / MacOS Sandbox | bwrap / gVisor |
| **Network** | Unrestricted | Proxied & Filtered | SOCKS5 + OTEL |
| **Parsing** | None | Regex + AST Validation | Go AST Validator |

## Implementation Prompt
"Implement the `ohc-harness` execution environment in `src/server/harness/`. Utilize Bubblewrap (`bwrap`) via Go's `os/exec` for Linux isolation. Route network traffic through an injected SOCKS5 proxy to enforce `NetworkPolicy` restrictions. Add OpenTelemetry tracing to track command start, exit, stdout size, and stderr events. Achieve 100% test coverage."

## Priority
P0

## Estimated Scope
Large
