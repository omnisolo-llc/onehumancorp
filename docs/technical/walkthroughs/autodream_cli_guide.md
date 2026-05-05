<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.05); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.15); box-shadow: 0 4px 30px rgba(0, 0, 0, 0.1); padding: 24px; border-radius: 16px; color: #fff;">

# Interactive CLI Guide for AutoDream

Welcome to the **AutoDream CLI Walkthrough**. This guide provides interactive, command-line instructions for operating the AutoDream pipeline—OHC's long-term memory consolidation engine.

## 1. Initiating the AutoDream Daemon

To manually invoke the AutoDream process from the OHC CLI, use the `start` command. This will trigger a sweep of recent `agent_session_data` and memory files.

```bash
ohc-cli autodream start --mode cloud
```

> **Tip:** Use `--mode standalone` if you are operating on a local desktop to fallback to SQLite and local embedding generation.

## 2. Monitoring Pipeline Status

You can check the current processing status, including chunking progress and vector storage insertions, with the `status` command.

```bash
ohc-cli autodream status --watch
```

### AutoDream Execution Flow

```mermaid
graph TD
    CLI[ohc-cli autodream start] -->|Triggers| Daemon(AutoDream Daemon)
    Daemon -->|Scans| TaskFiles(OHC_MEMORY_DIR/*.yml)
    Daemon -->|Scans| SessionData(agent_session_data)

    subgraph Processing Pipeline
        TaskFiles --> Chunker[Chunking & Tokenization]
        SessionData --> Chunker
        Chunker -->|API Call| EmbedAPI[Embedding API]
        EmbedAPI -->|Vectors| VDB[(Vector Database)]
    end

    CLI_Status[ohc-cli autodream status] -.->|Queries| Daemon

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class CLI,Daemon,TaskFiles,SessionData,Chunker,EmbedAPI,VDB,CLI_Status premium;
```

## 3. Pruning Stale Data

To enforce the "Zero-WIP" cleanliness protocol, you can manually trigger a pruning cycle after embeddings are successfully stored.

```bash
ohc-cli autodream prune --max-age 2h
```

This ensures that the ephemeral `agent_session_data` older than two hours is safely truncated, freeing up storage while relying on the Vector DB for long-term semantic recall.

---
*For a high-level architectural overview, see the [AutoDream Pipeline Walkthrough](./autodream_pipeline.md).*

</div>
