issue_title: "[Research] Secure API and Job Queue Multi-Tenancy Architecture"
issue_description: |
  # Research Report: Secure API and Job Queue Multi-Tenancy

  ## Problem Statement
  Currently, the OneHumanCorp platform relies primarily on Row-Level Security (RLS) in the database to isolate tenant data. However, as we scale to serve personas like Nora (Agency Principal managing multiple clients and contractors) and Jun (Location Manager orchestrating staff and local feedback), we need absolute confidence that data cannot leak between tenants. Relying solely on RLS at the persistence layer is brittle; if a developer forgets to apply a filter or configure a session correctly, a data leak occurs. We need a robust, systemic guarantee of multi-tenant isolation at the application's entry points (API) and background execution boundaries (AI Job Queue).

  From the perspective of an owner like Nora, she needs absolute assurance that her agency's client proposals, financial data, and AI agent drafts are never accidentally exposed to a competitor using the same platform, even if the system is under heavy load or processing background tasks.

  ## Research Report
  - **Competitor Analysis:** Platforms like Shopify and Stripe implement strict multi-tenant boundaries early in the request lifecycle. They don't just rely on database filtering; they use tenant-aware routing, middleware injection, and often dedicated database shards to guarantee isolation.
  - **Current OHC State:** OHC utilizes a `tenant_id` pattern. However, the enforcement mechanism needs to be elevated to the middleware/interceptor level for API requests and strictly validated by job queue workers before processing any background task.
  - **AI Agent Context:** Background agents processing events (e.g., drafting a response to a customer) must be strictly initialized within a single tenant's context. A failure in context separation could lead to an agent hallucinatory data from Workspace A into Workspace B's customer reply.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Client
      participant APIGateway
      participant TenantMiddleware
      participant ApplicationService
      participant AIJobQueue
      participant Database

      Client->>APIGateway: Request with Auth Token
      APIGateway->>TenantMiddleware: Forward Request
      TenantMiddleware->>TenantMiddleware: Validate Token & Extract Tenant Context
      alt Invalid/Missing Context
          TenantMiddleware-->>Client: 401/403 Unauthorized
      else Valid Context
          TenantMiddleware->>ApplicationService: Request + Immutable Tenant Context
          ApplicationService->>Database: Query executed in Tenant Context (RLS enforced)
          ApplicationService->>AIJobQueue: Enqueue Job (includes Tenant Context)
          ApplicationService-->>Client: Success Response
      end

      AIJobQueue->>AIJobQueue: Dequeue Job
      AIJobQueue->>TenantMiddleware: Validate Job Tenant Context
      TenantMiddleware->>ApplicationService: Execute Job within Tenant Context
  ```

  ### Mobile UX Flow (375px First)
  *   **Observation:** This is primarily a backend infrastructural change and should be entirely invisible to the user.
  *   **UI Impact:** There should be zero changes to the visual layout on a 375px screen. The app should continue to feel fast and responsive.
  *   **Error States:** If a tenant boundary violation is detected (e.g., a user tries to access a resource belonging to another tenant via a shared link), the app should gracefully display a "Resource Not Found" or "Access Denied" translucent glass card, rather than crashing or exposing an internal system error.

  ### AI Agent Integration Points
  *   **Agent Context Initialization:** When an AI agent is invoked (e.g., the Customer Assistant drafting a reply), the job execution engine must definitively inject the tenant context into the agent's memory/system prompt environment.
  *   **Data Access Limits:** The agent must be mechanically restricted from querying any database record or invoking any tool that operates outside its injected tenant context.

  ### Key Design Decisions
  1.  **Fail-Closed Middleware:** The tenant identification middleware must be fail-closed. If a request lacks a valid tenant context, it must be rejected immediately before reaching any business logic.
  2.  **Immutable Context:** Once the tenant context is established for a request or job, it must be immutable for the lifecycle of that execution.
  3.  **Job Queue Enforcement:** The background job queue must treat the `tenant_id` as a primary partitioning key, ensuring workers only process jobs for tenants they are authorized to handle.

  ## Implementation Prompt
  **User Facing Outcome:** Owners like Nora and Jun experience a secure, reliable platform where their data is strictly isolated. AI agents operate flawlessly within the boundaries of their specific business, never cross-contaminating information.

  **Critical User Journey (CUJ):**
  1.  An owner (e.g., Nora) logs into the mobile app (375px viewport).
  2.  The app requests her agency's dashboard data.
  3.  The backend verifies her identity, extracts her tenant context, and securely retrieves only her data.
  4.  She triggers a background AI task (e.g., generating a proposal draft).
  5.  The background worker securely executes the task, strictly confined to her tenant's data.

  **Acceptance Criteria:**
  *   Implement an API-level interceptor/middleware that mandates and validates a tenant context for all authenticated routes. Unauthenticated routes must be explicitly whitelisted.
  *   Ensure the background job execution framework requires a valid tenant context before processing any job.
  *   Write comprehensive unit tests to verify that requests without a valid tenant context are rejected by the middleware.
  *   Write E2E tests simulating a user attempting to access data across tenant boundaries, verifying that access is denied.

  ## Priority: P0
  ## Estimated Scope: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
