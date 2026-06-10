issue_title: "Architecture Design: Multi-Tenant Agentic Context & Memory Layer"
issue_description: |
  ## Problem Statement
  Currently, OneHumanCorp (OHC) handles multi-tenancy at the PostgreSQL row level via RLS. However, AI agents acting autonomously on behalf of the owner lack a durable, context-aware memory layer that strictly respects these tenant boundaries across distributed jobs. When an AI agent (e.g., The Promoter or The Operations Manager) wakes up to process an event, it must cold-boot its context by querying the database, leading to high latency and redundant token usage. Furthermore, cross-agent collaboration (e.g., Sales passing context to Support) is fragmented, causing a disjointed "WorkBuddy" experience for the owner. The platform needs a high-performance, strictly isolated memory layer to scale autonomous agentic workflows seamlessly.

  ## Research Report
  - **Shopify / Wix / GoDaddy**: These platforms rely on static webhook architectures or disparate third-party apps for automation (e.g., Klaviyo for marketing, Gorgias for support). There is no shared, cross-departmental "memory" or "brain."
  - **Tencent WorkBuddy / DingTalk**: These enterprise assistants maintain long-running context using embedded vectors and Redis-backed session stores to provide instant, unified replies across any business domain.
  - **OHC Gap Analysis**: OHC's current `autodream` pipeline processes memory into long-term embedded truth, but there is a missing mid-tier caching architecture (Redis/Valkey + Vector DB) designed specifically for cross-agent episodic memory sharing. The `Assistant Workstation` (`/assistant`) lacks real-time, context-rich streaming because the backend agents do not share a synchronized memory state.

  ## Design Doc
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD
      A[Agent Trigger Event] --> B{Tenant API Gateway}
      B -->|Authenticate & Inject Tenant ID| C[Agent Orchestrator]
      C --> D[(PostgreSQL: RLS Truth)]
      C <--> E[(Valkey/Redis: Episodic Memory Cache)]
      C <--> F[(Vector DB: Semantic Search)]

      E -->|Tenant Isolation Key Prefixing| E1[ohc:mem:tenant_id:session_id]
      F -->|Tenant Metadata Filtering| F1[Vectors + {tenant_id}]

      C --> G[Agent Capability: Sales]
      C --> H[Agent Capability: Ops]

      G <--> E
      H <--> E
  ```

  ### Mobile UX Flow
  1. **Owner Action**: Maya opens the OHC app (375px) and asks the Assistant, "Did the VIP customer reply?"
  2. **Orchestrator Wakes**: The Assistant queries the `Episodic Memory Cache` using `ohc:mem:{maya_tenant_id}:latest`.
  3. **Context Retrieval**: The cache instantly returns the ongoing conversation context from the Sales Agent without hitting Postgres.
  4. **Agent Response**: The Assistant streams back, "Yes, they confirmed the vegan cake. I've already updated the booking."

  ### AI Agent Integration Points
  - **Memory Injection**: Every AI job dequeue (via `SKIP LOCKED`) now injects a transient `MemoryClient` pre-configured with the specific `tenant_id`.
  - **Context Handoff**: When a Sales Agent finishes a task, it writes an episodic summary to the Valkey cache before handing off to Ops. The Ops Agent reads this summary to continue the workflow seamlessly.

  ### Key Design Decisions
  - **Strict Key Prefixing**: All Redis/Valkey keys MUST use the format `ohc:mem:{tenant_id}:{resource}` to guarantee Zero-Trust isolation.
  - **Ephemeral vs. Durable**: Episodic memory in Valkey expires after 7 days, after which it is either discarded or summarized into PostgreSQL/Vector DB via the `autodream` pipeline.
  - **Performance Targets**: Context retrieval must complete in < 50ms to ensure the Assistant UI feels instantaneous on mobile networks.

  ## Implementation Prompt
  Implement the `AgentMemoryService` backend capability. This service must abstract interactions with the Redis/Valkey cache and the embedded vector store. It should provide methods like `SaveEpisodicMemory(tenant_id, session_id, context)` and `RetrieveRecentMemory(tenant_id, session_id)`. Ensure all operations strictly enforce multi-tenant boundaries. Write a suite of unit tests validating that an agent from Tenant A cannot access the episodic memory of Tenant B. Do not implement the API or UI; focus entirely on the durable, isolated backend service layer. Acceptance Criteria: 100% unit test coverage demonstrating strict tenant isolation in memory retrieval.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
