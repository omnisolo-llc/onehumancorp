<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif; color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# KAIROS AutoDream CLI: Interactive Guide

Welcome to the AutoDream CLI interactive guide. This tool allows developers and administrators to interface with the AutoDream memory consolidation engine directly from the command line, enabling robust testing, debugging, and manual operations within the OHC ecosystem.

## Core Commands and Visual Walkthrough

This interactive guide outlines the primary CLI commands for interacting with the AutoDream pipeline.

```mermaid
graph TD
    CLI[CLI User] -->|autodream status| Status[Check Pipeline Status]
    CLI -->|autodream start| Start[Start AutoDream Daemon]
    CLI -->|autodream run --force| Run[Force Memory Consolidation]
    CLI -->|autodream query "agent config"| Query[Query Vector Memory]
    CLI -->|autodream prune --max-age 2h| Prune[Prune Stale Data]

    Status --> DB[(pgvector / SQLite)]
    Start --> Daemon(AutoDream Daemon)
    Run --> LLM[Embedding Service]
    LLM --> DB
    Query --> DB
    Prune --> SessionData(agent_session_data)
```

### 1. Initiating the AutoDream Daemon

To manually invoke the AutoDream process from the OHC CLI, use the `start` command. This will trigger a sweep of recent `agent_session_data` and memory files.

```bash
$ ohc-cli autodream start --mode cloud
[INFO] Starting AutoDream Daemon in Cloud Mode...
[INFO] Scanning for session contexts...
```

> **Tip:** Use `--mode standalone` if you are operating on a local desktop to fallback to SQLite and local embedding generation.

### 2. Checking Pipeline Status

To view the current status of the AutoDream memory consolidation pipeline, including chunking progress and vector storage insertions:

```bash
$ ohc-cli autodream status --watch
Status: Running
Last Consolidation: 5 mins ago
Pending Sessions: 2
```

### 3. Forcing Memory Consolidation

If you need to force an immediate embedding pass (e.g., after an important agent session):

```bash
$ ohc-cli autodream run --force
[INFO] Scanning for new session context...
[INFO] Found 2 pending sessions.
[INFO] Generating embeddings via Minimax...
[SUCCESS] 2 sessions vectorized and upserted to autodream_memories.
```

### 4. Querying Vector Memory

To interactively query the memory space to verify what the swarm currently "knows":

```bash
$ ohc-cli autodream query "KAIROS Master Architecture"
Top results:
- [0.92] KAIROS Master Design Doc (Session ID: abc-123)
- [0.89] Distributed State Machine (Session ID: xyz-789)
```

### 5. Pruning Stale Data

To enforce the "Zero-WIP" cleanliness protocol, you can manually trigger a pruning cycle after embeddings are successfully stored.

```bash
$ ohc-cli autodream prune --max-age 2h
[INFO] Scanning for agent_session_data older than 2h...
[SUCCESS] Pruned 15 stale session files.
```

## Integrating with the Hybrid Architecture

The CLI automatically detects the running environment and degrades gracefully in Standalone Mode.
Learn more about the core pipeline in the [AutoDream Walkthrough](kairos_autodream_walkthrough.md).

</div>
