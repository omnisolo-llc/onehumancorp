<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# KAIROS AutoDream CLI: Interactive Guide

Welcome to the AutoDream CLI interactive guide. This tool allows developers and administrators to interface with the AutoDream memory consolidation engine directly from the command line, enabling robust testing, debugging, and manual operations within the OHC ecosystem.

## Core Commands and Visual Walkthrough

This interactive guide outlines the primary CLI commands for interacting with the AutoDream pipeline.

```mermaid
graph TD
    CLI[CLI User] -->|autodream status| Status[Check Pipeline Status]
    CLI -->|autodream run --force| Run[Force Memory Consolidation]
    CLI -->|autodream query "agent config"| Query[Query Vector Memory]

    Status --> DB[(pgvector / SQLite)]
    Run --> LLM[Embedding Service]
    LLM --> DB
    Query --> DB
    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%),font-family:'Outfit',sans-serif;
    class CLI,Status,Run,Query,DB,LLM premium;
```

### 1. Checking Pipeline Status

To view the current status of the AutoDream memory consolidation pipeline:

```bash
$ autodream status
Status: Running
Last Consolidation: 5 mins ago
Pending Sessions: 2
```

### 2. Forcing Memory Consolidation

If you need to force an immediate embedding pass (e.g., after an important agent session):

```bash
$ autodream run --force
[INFO] Scanning for new session context...
[INFO] Found 2 pending sessions.
[INFO] Generating embeddings via Minimax...
[SUCCESS] 2 sessions vectorized and upserted to autodream_memories.
```

### 3. Querying Vector Memory

To interactively query the memory space to verify what the swarm currently "knows":

```bash
$ autodream query "KAIROS Master Architecture"
Top results:
- [0.92] KAIROS Master Design Doc (Session ID: abc-123)
- [0.89] Distributed State Machine (Session ID: xyz-789)
```

## Integrating with the Hybrid Architecture

The CLI automatically detects the running environment and degrades gracefully in Standalone Mode.
Learn more about the core pipeline in the [AutoDream Walkthrough](kairos_autodream_walkthrough.md).

</div>