issue_title: "[Platform Architecture] Unified Authentication & Access Control (SPIFFE/OIDC Integration)"
issue_description: |
  # Unified Authentication & Access Control (SPIFFE/OIDC Integration)

  ## 1. Problem Statement
  OneHumanCorp currently lacks a robust, unified authentication and authorization mechanism that bridges the gap between human users (via OIDC) and internal system components (via SPIFFE/SPIRE). This missing infrastructure is critical for enforcing Zero-Trust security and row-level multi-tenancy in cloud deployments. Without it, Maya, Carlos, and Priya cannot securely operate their businesses while internal AI agents securely access their data on their behalf.

  ## 2. Research Report
  - **Context:** The product vision requires mapping human authentication (OIDC) into a SPIFFE trust domain.
  - **Gap:** There is no centralized AuthZ/AuthN service or unified middleware that intercepts requests, validates OIDC tokens, mints SPIFFE SVIDs for agents, and enforces tenant isolation (`tenant_id`) at the API and database levels.
  - **Market Standard:** Platforms like Stripe and Shopify utilize unified gateway services to enforce strict multi-tenancy and secure agent-to-service communication.

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      Client[Client (Flutter PWA/Mobile)] -->|OIDC Token| APIGateway[API Gateway / Auth Middleware (Go)]
      APIGateway -->|Validate Token| IdP[Identity Provider (OIDC)]
      APIGateway -->|Extract tenant_id| ServiceLayer[Service Layer (Go gRPC/REST)]
      ServiceLayer -->|SPIFFE mTLS| Agents[AI Agents]
      ServiceLayer -->|tenant_id context| DB[(PostgreSQL with RLS)]
  ```

  ### Mobile UX Flow
  - The authentication layer is invisible to the user post-login.
  - Maya logs in once on her 375px device (Flutter app) and her session securely scopes all her actions to her specific `tenant_id` seamlessly.

  ### AI Agent Integration Points
  - Agents must securely acquire short-lived SPIFFE SVIDs to authenticate with internal Go API services.
  - Agents must carry the `tenant_id` in their context to ensure they only access data belonging to the owner they are assisting.

  ### Key Design Decisions
  - **Zero-Trust:** All internal communication must require SPIFFE mTLS.
  - **Tenant Isolation:** Every request must carry a `tenant_id` context, injected by the Auth Middleware, which is then used to enforce PostgreSQL Row-Level Security (RLS).

  ## 4. Implementation Prompt
  **Goal:** Implement a unified authentication and authorization middleware in the Go backend that validates OIDC tokens for human users, issues and validates SPIFFE SVIDs for internal agents, and consistently injects a `tenant_id` context into all downstream service requests to enforce multi-tenancy.

  **Acceptance Criteria:**
  - Create a Go middleware that intercepts all incoming requests.
  - Validate incoming OIDC tokens and extract the human user's identity and `tenant_id`.
  - Implement SPIFFE/SPIRE integration to validate SVIDs for internal agent-to-service communication.
  - Ensure the `tenant_id` is passed down to the database layer to enforce RLS.
  - Add comprehensive unit tests (100% coverage) for the middleware.
  - Add at least 5 Playwright E2E tests simulating multi-tenant data access scenarios, ensuring a user from Tenant A cannot access data from Tenant B.

  ## 5. Priority & Scope
  - **Priority:** P0 (Critical path for secure multi-tenancy)
  - **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
