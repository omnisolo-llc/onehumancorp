<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.05); font-family: 'Outfit', 'Inter', sans-serif; border-radius: 12px; padding: 24px; border: 1px solid rgba(255, 255, 255, 0.08); color: #ffffff;">

# Market Audit: Agent Harness & Sandbox Architecture

**Author**: Principal Product Researcher & Oracle (L7)
**Date**: 2026-04-17

## Executive Summary
This report analyzes the Agent Harness and Sandbox implementation of the leaked Claude Code application. We discovered that Claude Code has a deeply integrated sandbox that implements explicit overrides, intercepts tool outputs to communicate sandbox violations, and features defense-in-depth protection mechanisms to stop sandbox escapes via internal git file mutations. OHC must port these protections to the KAIROS Orchestrator.

## Competitive Analysis: OHC vs Market

| Feature | Claude Code (Market Leader) | OHC Hybrid OS | Gap |
| :--- | :--- | :--- | :--- |
| **Sandbox Execution** | Yes, `bwrap`/`sandbox-exec` | Yes, basic blockedPatterns in `bash_sandbox/sandbox.go` | Minimal |
| **Sandbox Escape Mitigations** | Advanced (Git internal path write blocking) | Basic Regexp matching | **Critical Gap** |
| **Telemetry Injection** | Injects `<sandbox_violations>` directly into `stderr` | Basic string appending | Minimal |
| **User Override** | LLM-driven `dangerouslyDisableSandbox` capability | Strict policies only | **Feature Gap** |

## Deep Technical Architecture (Claude Code Harness)

### Git-Internal Path Mitigations (Sandbox Escape)
Claude Code explicitly parses bash commands to detect operations that create or mutate git-internal files (`HEAD`, `objects/`, `refs/`, `hooks/`). If a sandboxed shell could write a malicious script to `.git/hooks/pre-commit` and then invoke `git status` or another git command, it would result in arbitrary, potentially unsandboxed execution via the hook escaping the isolation boundary. Claude Code blocks this explicitly in its validation layer.

```mermaid
flowchart TD
    A[BashTool Input] --> B{Parses Command}
    B -->|Contains 'git' & mutates hooks/| C[Block Command]
    B -->|Checks cwd == original cwd| D[Allow if true]
    C --> E[Return 'passthrough' warning to LLM]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class A,B,C,D,E premium;
```

## Actionable Roadmap
Based on these findings, an actionable GitHub issue has been injected into the swarm for implementers:
**Mission: [backend] Implement Git-Internal Path Write Protections (Sandbox Escape Prevention)**

</div>
