# OHC Hybrid Architecture Research: Agent Harness vs Market Leaders

## Overview
This document outlines our research on the execution environment needed for OHC's Hybrid Agentic OS, specifically focusing on the Agent Harness. This research analyzes techniques used by leading agents, in particular the `AI coding assistant(2_1_88).tgz` codebase, to formulate a blueprint for our enterprise-grade isolation and bridging capabilities.

## Key Findings from AI Coding Assistant
Our synthesis reveals that robust, production-ready local agents rely on a defense-in-depth approach for execution:

### 1. OS-Level Isolation (`bwrap`)
The core sandbox uses `bwrap --unshare-net` to achieve deep OS-level isolation.
This prevents arbitrary network access and confines the process to a restricted filesystem view.

### 2. Network Bridging (`socat`)
To allow controlled network egress while maintaining the isolated namespace, a `socat` proxy bridge is employed.
This acts as a secure, monitored gateway for necessary outbound connections.

### 3. Git Repository Scrubbing
Before and after execution, strict Git repository scrubbing is implemented.
This critical step prevents sandbox escapes that might attempt to exploit filesystem hooks or lingering build artifacts.

### 4. AST Command Validation
Subshell evasion is thwarted by performing token-level AST command validation, utilizing tools like `tree-sitter-bash`.
Commands are parsed and analyzed before execution to guarantee they align with expected structures and do not contain malicious payloads.

### 5. Deep Instrumentation
Deep OpenTelemetry instrumentation is woven throughout the execution lifecycle.
This provides the necessary observability to monitor agent behavior, track resource usage, and quickly identify anomalies.

---

## Architecture Comparison

```mermaid
graph TD
    subgraph Market Leaders (e.g., OpenClaw, Hermes)
        A1[Container-based Sandbox]
        A2[Permissive Egress]
        A3[Regex Command Filters]
    end

    subgraph OHC Hybrid Agentic OS
        B1[bwrap --unshare-net]
        B2[socat Proxy Bridge]
        B3[tree-sitter AST Validation]
        B4[Git Hook Scrubbing]
        B5[OpenTelemetry Integration]
    end

    A1 -.->|Heavier, Slower Startup| B1
    A2 -.->|Higher Risk| B2
    A3 -.->|Easily Bypassed| B3
```

## OHC Premium UI Implementation Tokens
When surfacing this status or telemetry to users in the OHC app, adhere strictly to the OHC Premium Design Standards:

```css
/* Glassmorphism Panel for Agent Harness Status */
.agent-harness-panel {
    backdrop-filter: blur(20px) saturate(200%);
    background-color: rgba(255, 255, 255, 0.05); /* Semi-transparent */
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 16px;
    padding: 24px;
}

/* Typography Scale */
h2 {
    font-family: 'Outfit', sans-serif;
    font-weight: 600;
}
p {
    font-family: 'Inter', sans-serif;
    font-weight: 400;
}
```
