<div style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; padding: 1.5rem; font-family: 'Outfit', 'Inter', sans-serif;">

# AutoDream Pipeline

The AutoDream Pipeline acts as the long-term memory consolidation mechanism for the OHC Swarm. It processes ephemeral agent context and transforms it into durable semantic knowledge.

## Mechanism

1. **Extraction**: A background worker (`AutoDreamWorker`) periodically scans ephemeral session contexts (`.agent-task/memory/*.yml` and session data).
2. **Compression & Embedding**: The pipeline uses LLMs (e.g., Minimax) to summarize these logs and generate high-dimensional vector embeddings.
3. **Persistence**:
    - **Cloud-Native Mode**: Embeddings are stored in PostgreSQL using the `pgvector` extension within the `autodream_memories` table, allowing for precise Nearest Neighbor (semantic) searches.
    - **Standalone Mode**: Embeddings and summaries are saved in the local SQLite database, supporting a fallback search mechanism.

## Semantic Retrieval

Agents can perform semantic searches against this database to retrieve historical context, previous solutions, and architectural decisions, thereby preventing the re-learning of information and adhering to the "Shared Memory" principle of the Swarm Intelligence Protocol (OHC-SIP).

</div>
