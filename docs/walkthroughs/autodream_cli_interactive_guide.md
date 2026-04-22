<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# AutoDream CLI: Interactive Guide

Welcome to the **AutoDream CLI Interactive Guide**. This walkthrough provides hands-on instructions for operating and monitoring the AutoDream memory consolidation pipeline via the `ohc-cli` utility.

## Introduction

AutoDream is the KAIROS memory consolidation engine. It transforms ephemeral agent session data into long-term semantic memory stored in a vector database. While it typically runs as a background daemon, the CLI provides powerful tools for manual intervention and observation.

## CLI Lifecycle and Pipeline Integration

```mermaid
graph TD
    subgraph CLI Interface
        C_Start[ohc-cli autodream start]
        C_Run[ohc-cli autodream run --force]
        C_Status[ohc-cli autodream status]
        C_Query[ohc-cli autodream query]
        C_Prune[ohc-cli autodream prune]
    end

    subgraph AutoDream Pipeline
        Daemon[AutoDream Daemon]
        Scanner[Session Scanner]
        Chunker[Chunking Engine]
        Embedder[Embedding API]
    end

    subgraph Persistence Layer
        SessionData[(Ephemeral Session Data)]
        VectorDB[(Vector Database)]
    end

    C_Start --> Daemon
    C_Run --> Scanner
    C_Status --> Daemon
    C_Query --> VectorDB
    C_Prune --> SessionData

    Daemon --> Scanner
    Scanner --> SessionData
    Scanner --> Chunker
    Chunker --> Embedder
    Embedder --> VectorDB

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class C_Start,C_Run,C_Status,C_Query,C_Prune,Daemon,Scanner,Chunker,Embedder,SessionData,VectorDB premium;
```

---

## 1. Initiating Consolidation

You can manually trigger the AutoDream consolidation process using either the `start` or `run` commands.

### Starting the Daemon
To start the AutoDream daemon in the foreground:
```bash
ohc-cli autodream start --mode cloud
```
*Use `--mode standalone` when running OHC in local desktop mode.*

### Forcing an Immediate Run
If you need to ensure recent session data is embedded immediately (e.g., before a complex multi-agent task):
```bash
ohc-cli autodream run --force
```

---

## 2. Monitoring Pipeline Status

Stay informed about the health and progress of your memory consolidation pipeline.

### Checking Status
```bash
ohc-cli autodream status
```

### Real-time Monitoring
To watch the pipeline progress in real-time as it chunks and embeds data:
```bash
ohc-cli autodream status --watch
```

---

## 3. Querying Vector Memory

Verify what your Swarm currently "knows" by querying the vector memory space directly.

```bash
ohc-cli autodream query "KAIROS Master Architecture"
```
**Example Output:**
```text
Top results:
- [0.92] KAIROS Master Design Doc (Session ID: abc-123)
- [0.89] Distributed State Machine (Session ID: xyz-789)
```

---

## 4. Maintenance and Hygiene

Enforce the "Zero-WIP" protocol by pruning stale ephemeral data after successful embedding.

```bash
ohc-cli autodream prune --max-age 2h
```
This command removes `agent_session_data` older than 2 hours, ensuring the local environment remains clean while long-term context is preserved in the Vector DB.

---

## Related Documentation
- [AutoDream Architectural Walkthrough](./kairos_autodream_walkthrough.md)
- [KAIROS Orchestrator CLI Guide](../technical/walkthroughs/kairos_orchestrator_cli.md)

</div>
