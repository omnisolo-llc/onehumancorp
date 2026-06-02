issue_title: "[Architecture] Multi-Tenant Agentic Context Memory Layer"
issue_description: |
  # Multi-Tenant Agentic Context Memory Layer (pgvector)

  ## Problem Statement
  Currently, OHC’s AI departments (Operations, Marketing, Sales, etc.) operate effectively but lack a unified, scalable, and isolated long-term memory system across tenant boundaries. As Maya (the home baker) or Carlos (the handyman) interact with different AI agents, context is lost between sessions. They need the "Operations Agent" to remember an interaction that the "Customer Success Agent" had with a customer last week. The absence of a shared, tenant-isolated vector memory store limits the autonomous agents' ability to synthesize cross-department insights securely without leaking data between tenants.

  ## Research Report
  **Findings & Data:**
  - **SMB Pain Points:** SMB owners complain that AI tools "forget" past instructions or context (a top frustration from Reddit `r/smallbusiness`).
  - **Competitor Analysis:**
    - **Shopify Sidekick:** Offers session-based context but struggles with deep, historical merchant memory across different functional domains.
    - **Wix/Squarespace:** Lack true autonomous agents; basic chatbots do not share memory with backend operations.
  - **Technology Landscape:** `pgvector` inside PostgreSQL allows us to keep vector embeddings in the exact same database as our relational business data (tenants, orders, products). This is crucial for Row-Level Security (RLS) enforcement.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      Client[Mobile/Web App] --> API[OHC API Gateway];
      API --> AgentOrchestrator[Agent Orchestrator];
      AgentOrchestrator --> DepartmentA[Operations Agent];
      AgentOrchestrator --> DepartmentB[Customer Success Agent];

      DepartmentA --> EmbeddingService[Embedding & Context Service];
      DepartmentB --> EmbeddingService;

      EmbeddingService --> PG[PostgreSQL Database];

      subgraph PG[PostgreSQL Database with pgvector & RLS]
          TenantRLS[Tenant RLS Policy]
          TenantRLS --> Vectors[pgvector: agent_memory table]
          TenantRLS --> BusinessData[orders, products, users]
      end
  ```

  ### UI Wireframes / Mobile UX Flow (375px first)
  *   **Settings > AI Memory (Mobile View):** A simple list view detailing "What your agents know."
  *   **Glassmorphism Cards:** Each memory cluster (e.g., "Customer Preferences", "Brand Voice") is displayed as a translucent card (`backdrop-filter: blur(20px)`).
  *   **Interaction:** Users can swipe to "forget" a memory or tap to edit a specific learned fact. No vector math or technical terms are exposed.

  ### AI Agent Integration Points
  *   **Memory Ingestion:** Every time a department completes a job (e.g., resolving a customer complaint), it generates a short natural language summary, creates an embedding, and stores it in the `agent_memory` table.
  *   **Memory Retrieval:** Before a department acts, it queries the `agent_memory` table using a cosine similarity search constrained strictly by the `tenant_id` via PostgreSQL RLS.

  ### Key Design Decisions
  1.  **Unified Storage (pgvector over Pinecone/Milvus):** To guarantee multi-tenant security and minimize operational complexity, vectors are stored in the same PostgreSQL DB as application data, utilizing RLS for zero-trust isolation.
  2.  **Abstracted UI:** The user sees "Learned Facts" instead of "Embeddings".
  3.  **Department-Agnostic Context:** Memories are tagged by department but queryable globally within the tenant to allow cross-department synergy (e.g., Marketing learns from Customer Success).

  ## Implementation Prompt
  **For Implementer Agent:**
  Implement the Multi-Tenant Agentic Context Memory Layer.
  1.  **Database:** Create a new table `agent_memory` with columns: `id`, `tenant_id`, `department`, `content` (text), and `embedding` (vector). Apply PostgreSQL RLS policies to restrict read/write access exclusively to the current `tenant_id`.
  2.  **API/Service:** Develop a Rust service layer module (`src/server/domain/memory/`) with methods for `ingest_memory(tenant_id, department, text)` and `recall_memory(tenant_id, query_embedding, limit)`.
  3.  **Security:** Ensure that the API strictly injects `tenant_id` from the authenticated context (SPIFFE/OIDC).
  4.  **Acceptance Criteria:** E2E tests must verify that an agent in Tenant A cannot retrieve vectors from Tenant B. The API must respond within <150ms for memory retrieval.
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
