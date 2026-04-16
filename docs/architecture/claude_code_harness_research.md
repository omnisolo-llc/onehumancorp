<div markdown="1" style="backdrop-filter: blur(20px); background: rgba(255, 255, 255, 0.1); border-radius: 12px; padding: 24px; font-family: 'Outfit', 'Inter', sans-serif;">

# Research Report: Claude Code Agent Harness & Implementation Gaps

## 1. Executive Summary
This document analyzes the Agent Harness environment within Claude Code to extract architectural and operational capabilities, contrasting them against OHC Hybrid Architecture (OHC-HA).

## 2. Architecture Comparison

### Harness Telemetry & State Management

| Feature | Claude Code Harness | OHC-HA |
| --- | --- | --- |
| **Telemetry** | `AnalyticsMetadata`, specific background trackers, manual event logging. | OpenTelemetry & Prometheus mandatory everywhere. |
| **State Sharing** | File-based memory directory (`memdir.ts`), localized event history. | Centralized OHC-SIP (PostgreSQL/SQLite vector DBs). |
| **Permissions** | Granular bash/file permissions (`bashPermissions.ts`, `filesystem.ts`), regex rule enforcement. | SPIFFE/SPIRE for auth, Git-Lock coordinate. |

### Component Flow

```mermaid
graph TD
    A[User Request] -->|Claude Code| B[Agent Harness CLI]
    B --> C[BashTool / Sandbox]
    B --> D[FileReadTool / Write]
    C -.-> E[AST Parsers & Checkers]
    D -.-> F[Permission Checks]
    E -.-> G[Execution Environment]
    F -.-> G
```

## 3. Implementation Targets
We need to implement robust AST parsing and strict local directory write locks based on `bashPermissions.ts`, `filesystem.ts` context, and isolated background execution models mirroring `parseForSecurityFromAst` processes.

</div>
