<div style="background: rgba(255, 255, 255, 0.05); backdrop-filter: blur(20px) saturate(200%); border-radius: 12px; padding: 24px; font-family: 'Outfit', 'Inter', sans-serif; color: #fff;">

# Hybrid MCP Architecture Walkthrough

Welcome to the OHC Hybrid Model Context Protocol (MCP) Architecture visual walkthrough. This guide explains how OHC bridges local offline execution with multi-tenant cloud scaling.

## Core Concepts

| Architecture Mode | Data Storage | Scaling |
| --- | --- | --- |
| **Cloud-Native** | PostgreSQL (pgvector), Redis | Infinite / K8s |
| **Standalone** | SQLite (Local) | Low Resource |

## Workflow Diagram

```mermaid
graph TD
    A[Local Client] -->|SQLite| B(Standalone Mode)
    A -->|API| C(Cloud Mode)
    C --> D[PostgreSQL pgvector]
    B <-->|Sync| D
```

## Setup Instructions
1. Ensure your OHC client is configured for the desired mode.
2. Refer to the [API Playbook](../api_playbook.md) for endpoint details.
3. Verify connection stability.

</div>
