issue_title: "[KAIROS Orchestrator] Distributed Task Graph & Long-Term Episodic Memory"
issue_description: |
  # Research Report

  ## Problem Statement
  The KAIROS Orchestrator currently struggles with managing complex, cyclic workflows spanning multiple agents and ensuring long-term memory across disjointed sessions. As OHC scales to handle sophisticated workflows for personas like Maya (baker) and Carlos (handyman)—which involve quoting, scheduling, invoicing, and customer success follow-ups—agents suffer from context amnesia and blocking operations. The lack of a durable, distributed state machine and long-term episodic memory limits our ability to deliver the "Hybrid Agentic OS" vision effectively.

  ## Research & Findings
  Based on our analysis of the ecosystem and internal strategy documents, the top capability gaps are:
  1.  **Agent Memory (Short/Long-Term):** Reliance on ephemeral context limits cross-session recall.
  2.  **Stateful Execution Graph:** Inability to handle cyclic workflows and task dependencies gracefully.

  **OHC Advantage:** We can solve this by implementing the KAIROS Triad:
  *   **Shared Task List:** A durable, distributed Directed Acyclic Graph (DAG) for task decomposition and execution.
  *   **AutoDream:** A pipeline to consolidate episodic memory.
  *   **Teammate Mesh:** Realtime communication for agent coordination.

  ## Design Doc

  ### Architecture Blueprint (KAIROS Triad)

  ```mermaid
  graph TD;
      subgraph Master Orchestrator
          TaskGraph[Shared Task Graph DAG]
          Mesh[Teammate Mesh]
      end

      subgraph Execution Nodes
          AgentOps[Ops Agent]
          AgentFin[Finance Agent]
          AgentCS[CS Agent]
      end

      subgraph Memory Subsystem
          ShortTerm[Short-Term Context]
          AutoDream[AutoDream Pipeline]
          LongTerm[Long-Term Vector Memory]
      end

      TaskGraph -->|Delegate| Mesh
      Mesh <-->|Coordinate| AgentOps
      Mesh <-->|Coordinate| AgentFin
      Mesh <-->|Coordinate| AgentCS

      AgentOps -->|Write State| ShortTerm
      AgentFin -->|Write State| ShortTerm

      ShortTerm -.->|Consolidate & Embed| AutoDream
      AutoDream -->|Store Embeddings| LongTerm

      AgentCS -->|Semantic Search RAG| LongTerm
  ```

  ### Data Model (ER Diagram)
  ```mermaid
  erDiagram
      TASK-GRAPH ||--o{ TASK-NODE : contains
      TASK-NODE ||--o{ TASK-DEPENDENCY : depends_on
      TASK-NODE {
          string id
          string status
          string assigned_agent
          jsonb payload
      }
      MEMORY-SESSION ||--o{ EPISODE : records
      EPISODE {
          string id
          string session_id
          vector embedding
          jsonb context
      }
  ```

  ### Key Design Decisions
  1.  **Shared Task List (DAG):** Implement a database-backed graph for tasks using robust locking mechanisms to ensure exactly-once execution and handle dependencies.
  2.  **AutoDream Pipeline:** An asynchronous background worker that watches the short-term memory, chunks interactions, generates embeddings, and stores them in long-term memory.
  3.  **Teammate Mesh:** A realtime communication layer for agents to signal task completion and share intermediate state.
  4.  **Premium Design Elements:** Ensure the resulting UI for viewing task states is "Glassmorphism" styled, obscuring database and execution graph terminology behind a toggle.

  ### Mobile UX Flow (375px First)
  1. **Goal Setting:** Carlos asks the app to "Follow up on the Johnson sink quote and schedule a visit."
  2. **Task Graph Creation:** KAIROS silently decomposes this into a graph: [1. Check quote status] -> [2. Send SMS reminder if not accepted] -> [3. Propose times from calendar if accepted].
  3. **Execution View:** The mobile UI shows a clean, elegant card listing the goal: "Following up with Johnson...", removing the complexities of execution paths from the visible layer.
  4. **Episodic Recall:** When Carlos talks to the app next week, it uses the AutoDream index to recall the sink fix without being explicitly told.

  ### Performance & Offline Targets
  *   **Latency Target:** UI state updates for task generation and transitions must render under 50ms locally.
  *   **Offline Capability:** The task graph must be fully cacheable using local CRDT stores, allowing users like Carlos to view, modify, or add new high-level goals while disconnected from the network.
  *   **Payload Constraints:** Inter-agent messaging payloads on the Teammate Mesh must be delta-compressed to minimize bandwidth.

  ### Zero Trust & Security
  *   Multi-tenant isolation and secure identity (SPIFFE/SPIRE) must be guaranteed for all read/write paths in the AutoDream and Task Graph modules. Each tenant's episodic memory is cryptographically segmented to prevent data leakage.

  ### AI Agent Integration Points
  *   **Ops Agent:** Picks up sub-tasks for calendar checks.
  *   **Customer Success Agent:** Retrieves long-term memory to craft personalized SMS follow-ups.

  ## Implementation Prompt
  Implement the core components of the KAIROS Orchestrator to address task management and episodic memory:
  *   **User-Facing Outcome:** Agents can autonomously decompose complex goals into dependent tasks, coordinate execution without race conditions, and recall past interactions across disjointed sessions seamlessly. All without exposing database terminology to the non-technical small business owner.
  *   **CUJ (Critical User Journey):**
      1. User instructs the app with a complex multi-step goal (e.g. "Prepare an invoice for Maya and email it").
      2. The system translates the goal into a DAG of tasks stored via the Shared Task List.
      3. The Teammate Mesh coordinates execution amongst sub-agents.
      4. AutoDream persists the outcome in episodic memory so it can be recalled a month later.
  *   **Acceptance Criteria:**
      *   Create the backing store for the Shared Task List, supporting DAG dependencies and distributed locking.
      *   Implement the AutoDream pipeline to extract episodic data and insert it into a persistent vector memory store.
      *   Implement the Realtime Teammate Mesh APIs to handle messaging over a reliable pub/sub or in-memory channel mechanism.
      *   Ensure Zero-Trust Multi-Tenancy applies to all data access.

  ## Priority
  P0

  ## Estimated Scope
  Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
