<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; padding: 2rem; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# 🛡️ OHC Agent Harness: OS-Level Sandboxing & MCP Integration Walkthrough

Welcome to the **Agent Harness OS-Level Sandboxing and MCP Integration** visual walkthrough. This document details the architectural components of OHC's execution sandboxing layer and its native integration with the Model Context Protocol (MCP).

## 1. The Core Sandboxing Architecture

OHC enforces absolute zero-trust autonomy via the `bwrap` OS-Level Sandboxing wrapper on Linux systems.

- **OS-Level Isolation (`bwrap`)**: Every Agent Harness runtime encapsulates execution in an isolated namespace.
- **Strict Capability ACLs**: Explicit `allowRead` and `denyWrite` directives prevent agents from reading or modifying unauthorized host files.
- **Native MCP Integration**: The harness natively runs as an MCP Server, exposing its sandboxed capabilities via standard Model Context Protocol schemas, bridging the gap between isolated execution and cloud-based reasoning.

## 2. Hybrid Agent Harness Flow

The diagram below illustrates the sandboxing and context sharing process:

```mermaid
graph TD;
    KAIROS[KAIROS Orchestrator] -->|Dispatch Command| Harness(Hybrid Agent Harness)
    Harness -->|Execute via| Bwrap{Bubblewrap Sandbox}
    Bwrap -->|Enforce Allow/Deny| FS[Isolated Filesystem]
    Bwrap -->|Intercept Tools| MCP(Native MCP Interface)
    MCP -->|Expose Capabilities| Ext(External Model APIs)

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class KAIROS,Harness,Bwrap,FS,MCP,Ext premium;
```

</div>
