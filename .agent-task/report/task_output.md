issue_title: "Implement Missing Memory Consolidation Worker"
issue_description: |
  # Research Report: Implement Missing Memory Consolidation Worker

  ## Problem Statement
  Small business owners using OneHumanCorp (OHC) need their agents to have long-term context of the business. However, as agent sessions conclude, ephemeral contexts are not properly pruned or consolidated in background, resulting in ballooning memory usage and contradictory states. The architectural design in `docs/features/kairos/memory_consolidation.md` specifies a Memory Consolidation System with a background worker responsible for detecting conflicts (overlapping embeddings), resolving conflicts (using owner_override, reliability_score, etc.), and pruning stale contexts (> 180 days). Currently, the architecture outlines this mechanism but the background worker component itself is incomplete or missing in the implementation of the `autodream_pipeline` and `VectorRepository`.

  ## Research Findings
  - **Codebase Audit:** The `VectorRepository` class in `src/agents/builtin/memory_store.rs` has logic for inserting (`upsert`) and querying (`semantic_search`) embeddings. It also has a function `determine_conflict_winner` which detects the winner between conflicting records. However, there is no background worker that periodically polls the database to prune stale memory or resolve conflicts.
  - **Competitor Systems Audit:** In leading agentic systems, context windows are limited. Memories must be periodically summarized, consolidated, and pruned. OHC uses a vector database (`pgvector`) for storage and retrieval, which works well, but lacks the background cleanup.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD;
      AgentSession[Agent Interaction Session] -->|Upserts Memory| MemoryStore[(Consolidated Memory Database)];
      BackgroundWorker[Memory Consolidation Worker] -->|Periodically Polls| MemoryStore;
      BackgroundWorker -->|1. Prunes Stale| Stale[Remove context > 180 days];
      BackgroundWorker -->|2. Detects Conflicts| Conflict[Identify overlapping embeddings < 0.05 distance];
      BackgroundWorker -->|3. Resolves Conflicts| Resolution[Keep highest reliability/override];
      MemoryStore -->|Semantic Search| CrossDept[Cross-Department Context Sharing];
  ```

  ### Mobile UX Flow
  This feature is entirely background-focused and doesn't introduce direct UI elements. The UX impact is that users (agents acting on their behalf) respond with more accurate, coherent, and relevant long-term memory. It should be invisible to the owner.

  ### AI Agent Integration Points
  - **Memory Store (`pgvector`):** The background worker operates directly on the `consolidated_memory` table.
  - **VectorRepository:** The background worker will utilize existing methods or new queries on the VectorRepository to identify stale and conflicting records.
  - **AutoDream Pipeline:** This worker should be integrated into the existing `AutoDream` pipeline or worker pool as a scheduled periodic task.

  ## Implementation Prompt
  **User-Facing Outcome:** The AI agents exhibit consistent, up-to-date long-term memory without hallucinating contradictory facts or suffering performance degradation from an unbound vector store. The owner can trust that the agent remembers their preferences correctly over months.

  **CUJ & Acceptance Criteria:**
  1. A background daemon/worker is implemented that periodically queries the `consolidated_memory` table.
  2. The worker must delete records where `last_referenced_at < 180 days ago` AND `owner_override = FALSE` AND `reference_count < 5` AND `source_type = 'TASK_SUMMARY'`.
  3. The worker must identify pairs of records with a semantic distance `< 0.05`.
  4. The worker must use `VectorRepository::determine_conflict_winner` to resolve conflicts, keeping the winner and deleting the loser.
  5. The logic must work correctly in both Postgres and SQLite standalone modes.

  **Instructions for Implementer:**
  Implement the `MemoryConsolidationWorker` as a recurring task within the `src/server/workers/` or `src/server/autodream_pipeline/` modules. Provide comprehensive unit tests proving pruning and conflict resolution logic on both Postgres and SQLite interfaces. Update the orchestrator to spawn this daemon alongside the other background workers.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
