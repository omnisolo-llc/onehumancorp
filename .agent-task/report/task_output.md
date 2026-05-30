issue_title: "[Architecture] Zero-Trust Multi-Tenant AI Memory Isolation Mesh"
issue_description: |
  ## Problem Statement
  As OneHumanCorp scales, our AI departments (The Operations Manager, The Salesperson, etc.) require access to deep historical business data across multiple channels (inbox, sales, inventory) to provide proactive, autonomous assistance. However, using a single vector database or shared memory context for all tenants introduces a catastrophic risk of cross-tenant data leakage (e.g., Priya's boutique AI hallucinating pricing data from Carlos's handyman business). Existing frameworks often treat memory globally or rely purely on application-layer filtering, which is prone to security bugs.

  ## Research Report
  ### Security & Compliance Landscape
  *   **Application-Level Filtering:** High risk of human error leading to multi-tenant leakage. Not sufficient for zero-trust architectures.
  *   **Physical DB Separation:** Prohibitively expensive and difficult to scale across millions of micro-merchants.
  *   **RLS (Row-Level Security) in Postgres/pgvector:** The industry gold standard for multi-tenant SaaS data isolation. Provides database-enforced boundaries that application code cannot bypass.

  ### Opportunity
  By designing a Zero-Trust AI Memory Isolation Mesh using `pgvector` and PostgreSQL Row-Level Security (RLS) integrated with SPIFFE/SPIRE for workload identity, we guarantee cryptographic isolation of AI memory. Every agent query acts strictly within the bounds of a single `tenant_id` at the database kernel level, eliminating the risk of data leakage while maintaining high performance and low cost.

  ## Design Doc
  ### Architecture Overview
  1.  **Identity Layer:** SPIFFE/SPIRE assigns a short-lived cryptographic identity to every AI worker pod.
  2.  **Access Control Layer:** The API Gateway/Router injects the verified `tenant_id` into the Postgres connection session variable (e.g., `SET app.current_tenant = 'tenant-xyz'`).
  3.  **Storage Layer:** A unified PostgreSQL cluster running `pgvector`. Every memory table (`agent_memories`, `conversation_history`) has a `tenant_id` column and `ENABLE ROW LEVEL SECURITY`.
  4.  **RLS Policies:** Policies restrict `SELECT`, `INSERT`, `UPDATE`, `DELETE` operations strictly to rows where `tenant_id = current_setting('app.current_tenant')`.

  ### Mermaid.js Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant App as Mobile App (User)
      participant Gateway as API Gateway (SPIFFE)
      participant AI_Worker as AI Agent Worker
      participant DB as PostgreSQL (pgvector + RLS)

      App->>Gateway: Request AI Assistance (Auth Token)
      Gateway->>Gateway: Validate Token & Extract tenant_id
      Gateway->>AI_Worker: Route Request (tenant_id context)
      AI_Worker->>DB: Connect & SET app.current_tenant = '123'
      AI_Worker->>DB: Query Vector Memory (SELECT * FROM agent_memories)
      DB-->>AI_Worker: Returns ONLY rows for tenant '123' (Enforced by RLS)
      AI_Worker-->>App: Secure, Tenant-Isolated AI Response
  ```

  ### Implementation Prompt
  **To the Implementer:**
  Implement the Zero-Trust Multi-Tenant AI Memory Isolation Mesh.
  1.  Set up PostgreSQL with the `pgvector` extension.
  2.  Create the necessary schemas for agent memory storage, ensuring every table includes a `tenant_id` column.
  3.  Implement and enforce PostgreSQL Row-Level Security (RLS) policies based on a session variable (e.g., `app.current_tenant`).
  4.  Implement the database connection lifecycle in the Go backend to ensure the `tenant_id` is securely set on the connection immediately after checkout from the pool and cleared before return.
  5.  Provide comprehensive unit and integration tests proving that queries attempting to access data outside the set `tenant_id` fail or return zero rows at the database level.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
