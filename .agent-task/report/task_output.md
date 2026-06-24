issue_title: "Implement unified multi-tenant authentication and authorization"
issue_description: |
  # Unified Multi-Tenant Authentication and Authorization

  ## Problem Statement
  Currently, the multi-tenant architecture is implemented with row-level security in PostgreSQL using `tenant_id`, but the authentication and authorization flow is scattered and not uniformly enforced across all gRPC and REST entry points. The owner/operator (Maya, Carlos, Priya) needs to log in once, switch between their businesses securely, and trust that employees (e.g., Jun) only see what they are authorized to see. The current system relies on ad-hoc token checks, making it error-prone as the product scales to support teams and agencies.

  ## Research Report
  - **Codebase Findings:** The `README.md` states `OHC_MULTITENANT=true` enforces isolation through authenticated `organization_id` claims, but the actual implementation in `src/server/services` and `src/server/api` is inconsistent. We need a unified auth interceptor for gRPC and a middleware for Axum HTTP routes.
  - **Competitor Systems:** Shopify and Wix handle multi-tenancy by linking a single user identity to multiple stores/tenants, with role-based access control (RBAC) per store. Stripe uses a similar account-switching model.
  - **Missing Capability:** We lack a centralized session manager that validates SPIFFE/SPIRE identities (as mandated by Zero Trust constraints) and issues short-lived JWTs containing the active `tenant_id` and user role.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant User (Mobile/Web)
      participant API Gateway (Axum)
      participant AuthMiddleware
      participant SessionService
      participant Database (RLS)

      User->>API Gateway: Login (Credentials)
      API Gateway->>SessionService: Validate & Issue Token
      SessionService-->>API Gateway: JWT (Identity + TenantContext)
      API Gateway-->>User: Auth Token

      User->>API Gateway: Request Resource (Header: Bearer Token)
      API Gateway->>AuthMiddleware: Intercept
      AuthMiddleware->>AuthMiddleware: Verify JWT & Extract TenantID
      AuthMiddleware->>API Gateway: Pass Request + Context
      API Gateway->>Database: Query (Set local session tenant_id)
      Database-->>API Gateway: RLS-Filtered Data
      API Gateway-->>User: Response
  ```

  ### UI/UX Flow (375px First)
  1. **Login Screen:** Clean, single-column layout. Input fields for email/phone and password. Large touch target for "Sign In".
  2. **Workspace Switcher:** If the user belongs to multiple tenants (e.g., Nora runs two agencies), a bottom sheet slides up allowing them to select the active workspace.
  3. **Visual Cues:** The active workspace name is subtly displayed in the top app bar to ensure the owner always knows context.

  ### AI Agent Integration Points
  - Agents must inherit the `tenant_id` context of the user triggering them or the background job they are processing.
  - The `AuthMiddleware` must validate agent-to-agent communication via SPIFFE/SPIRE certificates.

  ## Implementation Prompt
  **Outcome:** Implement a centralized `AuthMiddleware` for Axum and a gRPC Interceptor for Tonic that uniformly validates JWTs, extracts the `tenant_id` and user role, and injects them into the request context. Ensure all database queries leverage this context to respect PostgreSQL Row Level Security.
  **Acceptance Criteria:**
  - 100% unit test coverage for the new middleware/interceptor.
  - E2E Playwright tests verifying cross-tenant data isolation (User A cannot see User B's data).
  - UI implements the Workspace Switcher bottom sheet if multiple tenants exist.
  - No plain-text secrets; all auth uses secure JWTs and SPIRE identities.

  ## Priority
  P0 (Critical for SaaS multi-tenancy and data security)

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
