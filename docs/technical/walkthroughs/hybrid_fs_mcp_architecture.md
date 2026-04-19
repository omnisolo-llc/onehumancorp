<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# Hybrid File System MCP Architecture: Visual Walkthrough

This document visualizes how the Machine Context Protocol (MCP) bridges the local Standalone execution environment with the Cloud-Native Postgres/Redis persistence layers.

## 1. Context Syncing Flow

Agents running locally (Zero WIP Standalone) stream their context to the Cloud MCP Router. This ensures high-fidelity observation and state-transfer during Cloud-Bursting.

```mermaid
sequenceDiagram
    participant LocalFS as Local Agent File System (.ohc/runtime/)
    participant Sync as MCP Local Daemon
    participant Router as Cloud MCP Router
    participant Vector as pgvector (AutoDream)
    participant CloudAgent as Cloud Worker Pod

    LocalFS->>Sync: File Change Detected (Inotify/FSEvents)
    Sync->>Router: Push Context Delta (gRPC/WebSockets)
    Router->>Vector: Queue for AutoDream Vectorization
    Router->>CloudAgent: Stream Context via Teammate Mesh
    CloudAgent->>CloudAgent: Rehydrate State
```

## 2. Fallback Mechanism

When the Cloud Router is unreachable, the MCP Local Daemon gracefully degrades, queuing deltas in a local SQLite buffer (`mcp_sync_queue`) until connectivity is restored.
</div>
