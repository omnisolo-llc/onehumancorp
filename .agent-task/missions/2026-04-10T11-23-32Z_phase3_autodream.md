---
title: "Phase 3: AutoDream Data Pipelines (pgvector/LLM embeddings)"
status: PENDING
agent: "KAIROS Orchestrator"
priority: P1
scope: Medium
---

<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Phase 3: AutoDream Data Pipelines (pgvector/LLM embeddings)

## Problem Statement
OHC's long-term memory consolidation system, "AutoDream", needs robust data pipelines to map vector embeddings natively to `[]byte` in Go structs, for both Postgres (pgvector) and SQLite compatibility, enabling agents to retrieve semantic context efficiently.

## Research Report
### Memory Consolidation
| Feature | Storage | Encoding |
| --- | --- | --- |
| Vectors | pgvector / SQLite | `[]byte` (not `[]float32`) |

### Mermaid Flow
```mermaid
graph LR;
    A[Raw Memory] -->|Encode to Embeddings| B(AutoDream Pipeline);
    B -->|Save as []byte| C[Vector DB];
```

## Design Doc
- **Module Path**: `srcs/server/autodream`
- **Architecture**: Define entity schemas mapping vector fields as `[]byte`. Build a nightly/background synchronization job (AutoDream process) to compress short-term facts into vector embeddings.

## Implementation Prompt
Architect the data pipelines for AutoDream. Implement Go structs mapping vectors to `[]byte`. Provide the interface and a concrete implementation to persist embeddings to PostgreSQL/SQLite transparently. Add corresponding unit tests.
</div>
