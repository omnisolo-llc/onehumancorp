<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# AutoDream Pipeline Walkthrough

Welcome to the AutoDream Pipeline walkthrough. This document outlines the long-term memory system of the OHC Swarm.

## 1. Overview
The AutoDream background pipeline asynchronously vectorizes ephemeral findings into a durable pgvector store. This prevents context window overflows and ensures long-term coherence across the swarm.

## 2. Data Pipeline Architecture
1. **Source**: Local `.agent-task/memory/` YAML files.
2. **Ingestion Agent**: Reads files, generates chunked text.
3. **Embedding Generation**: Calls LLM provider (e.g., Anthropic/OpenAI/Minimax) to produce vectors.
4. **Storage (pgvector)**:
   - Data is stored in the `autodream_memories` table.

```mermaid
sequenceDiagram
    participant Agent as Swarm Agent
    participant File as .agent-task/memory/
    participant Ingestion as Ingestion Agent
    participant DB as pgvector

    Agent->>File: Write YAML Memory
    Ingestion->>File: Read files
    Ingestion->>Ingestion: Generate chunked text
    Ingestion->>DB: Store Vector Embedding (autodream_memories)
```

</div>
