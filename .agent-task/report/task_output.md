issue_title: "[Architecture] Long-Term Episodic Memory & Context Rehydration Engine"
issue_description: |
  # Mission Queue Brief: Long-Term Episodic Memory Engine

  ## Problem Statement
  Currently, OneHumanCorp's (OHC) AI Agents suffer from "Amnesia" across disjointed work sessions. When an owner like Maya (the baker) asks the assistant about a customer who reached out three months ago, or when Carlos (the handyman) needs the assistant to recall that he prefers giving a 10% discount on large roof repair jobs, the assistant has no persistent memory.

  Passing full interaction histories into the prompt balloon token usage, drastically increase latency, and eventually hit hard context window limits. Small business owners expect their work assistant to have long-term episodic memory—remembering past interactions, successful tool applications, and user preferences—without requiring manual re-explanation every session.

  ## Research Report & Gap Analysis
  Based on an audit of leading AI agent frameworks (OpenClaw, CrewAI, AutoGen, Claude Code) and the specific needs of OHC's non-technical operators:
  *   **The Competitor Gap:** Frameworks typically rely on short-lived context arrays or inefficient, monolithic vector polling that slows down agent response times.
  *   **The OHC Advantage:** OHC can uniquely solve this using its Native K8s / LangGraph Control Plane. We can implement native K8s CSI Snapshotting paired with LangGraph checkpointers, backed by a scalable Vector Database (Redis/Pinecone).
  *   **Token Efficiency:** Context window hydration targets only the top `k` relevant memory chunks.

  ## Architecture Design

  **System Flow:**
  1.  **Stateful Nodes:** Each agent session maps to an isolated environment within the tenant.
  2.  **Short-Term LangGraph State:** The current conversation and immediate tool outputs pass through explicit LangGraph state transitions.
  3.  **Long-Term Semantic Chunking:** A dedicated K8s worker pool asynchronously processes historical LangGraph states. It summarizes the interactions and writes embeddings (e.g., to pgvector/Redis) tied to the `tenant_id` and `customer_id`.
  4.  **Context Rehydration (Pre-Flight):** When a new session begins or an implicit query requires past context, a pre-flight node performs a semantic search to retrieve the top `k` most relevant memory interactions. These are injected into the agent's System Prompt.

  **Data Architecture Invariants:**
  *   **Tenant Isolation:** All memories MUST be strictly isolated using Row-Level Security (RLS) on `tenant_id`. `ohc:lock:{tenant_id}:{customer_id}` patterns must be used to prevent race conditions during memory generation.
  *   **Structure:**
      ```json
      {
         "tenant_id": "<uuid>",
         "agent_id": "<spiffe_id_segment>",
         "session_id": "<uuid>",
         "turn_index": 4,
         "summary_embedding": "[...]",
         "raw_state": "{...}"
      }
      ```

  **AI Agent Integration:**
  The `Operations Agent` and `Customer Success Agent` will be the primary consumers. For example, when a DM comes in, the unified inbox triggers a memory rehydration pre-flight step before drafting the reply.

  **Mobile UX Flow:**
  1.  On the 375px mobile shell, the owner taps a customer's profile.
  2.  The UI displays an "Assistant Memory" card, synthesizing past interactions into a 2-sentence summary (e.g., "Always orders vegan. Prefers weekend delivery.").
  3.  The agent drafts a reply leveraging this hidden context.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Implement the backend data layer and LangGraph node for the Long-Term Episodic Memory Engine.
  1. Create the database schema/migrations required to store agent session summaries and their vector embeddings (using pgvector), strictly enforcing RLS by `tenant_id`.
  2. Implement a pre-flight LangGraph node (e.g., in the unified feed or chat service) that queries this memory store based on the current context/user input.
  3. Wire the retrieved context into the prompt hydration process for the main Gemini Pro generation call.
  4. Build a background job (or asynchronous step) that summarizes closed sessions and writes them to the memory store.
  5. Expose an API endpoint that allows the mobile UI to fetch the synthesized "Assistant Memory" for a specific customer or context.
  6. Ensure 100% unit test coverage for the memory retrieval and storage logic, and write a Playwright E2E test verifying that a simulated past interaction influences a subsequent agent response.

  ## Priority & Scope
  - **Priority:** P0 (Critical path for Agent intelligence)
  - **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []