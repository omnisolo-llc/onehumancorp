issue_title: "[Architecture] Edge-Native Mobile Tap-To-Pay Identity Gateway"
issue_description: |
  ## Problem Statement
  Small business owners like Priya (Boutique Owner) and Fatima (Food Cart Operator) struggle with maintaining separate identities for their businesses and personal lives, especially when they need to securely log in from multiple devices without complex passwords. The current identity infrastructure forces them to manage separate credentials per tenant context, leading to friction during checkout flows for their customers and management flows for themselves.

  ## Research Report
  - **Status Quo:** Competitors like Shopify use Shop Pay to unify consumer identity, but merchant identity remains fragmented. Wix uses a monolithic login that struggles with multi-tenancy.
  - **OHC Architecture Gap:** OHC currently lacks an edge-native identity gateway that securely resolves multi-tenant boundaries (SPIFFE/SPIRE) at the load balancer layer without forcing a round-trip to the core database for every request.
  - **Goal:** Implement a Zero-Touch Multi-Tenant Edge Identity Gateway that can authenticate and route traffic based on JWT/SPIFFE headers with sub-10ms latency.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile App / PWA] -->|Requests with JWT| B[Edge Gateway Envoy]
      B -->|Validates Token & Resolves Tenant| C[SPIFFE / SPIRE Node]
      B -->|Routes Request + X-Tenant-ID| D[OHC Core API Services]
      D --> E[(Multi-Tenant PostgreSQL)]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  **Screen 1: Context Switcher Overlay**
  *   **Layout:** A glassmorphic bottom-sheet modal that appears when a user clicks their profile icon.
  *   **Content:** A list of the user's businesses ("Priya's Boutique", "Priya's Personal"). The current business is highlighted with a green checkmark.
  *   **Action:** Tapping another business instantly switches the `X-Tenant-ID` context via the Edge Gateway without requiring a re-login.

  **Screen 2: Tap-to-Pay Authentication**
  *   **Layout:** A translucent overlay over the checkout flow.
  *   **Content:** "Tap to Pay requires verifying your identity." (Only shown if edge context is missing or expired).
  *   **Action:** A simple "Verify with Face ID/Touch ID" button, initiating a quick SPIFFE/SPIRE token refresh.

  ### Mobile UX Flow
  1. The user logs into the OHC App once.
  2. The Edge Gateway assigns a SPIFFE-backed JWT identity token.
  3. During operation (e.g. Tap-to-Pay, adding inventory), the user's requests hit the Edge Gateway.
  4. The Edge Gateway resolves the multi-tenant boundary locally (<10ms) and routes the request securely.

  ### AI Agent Integration Points
  - **Security Agent (The Protector):** Monitors the Edge Gateway logs. If anomalous context-switching occurs (e.g., trying to access Priya's Boutique from an unknown IP), the agent automatically invalidates the edge session and triggers a re-authentication prompt.
  - **Operations Agent (The Manager):** Uses the resolved tenant identity from the Edge Gateway to correctly route incoming POS orders to the appropriate ledger.

  ### Key Design Decisions
  1. **Edge Resolution:** Validating tenant boundaries at the edge (via a sidecar or Envoy proxy) prevents the core API and PostgreSQL from being burdened with identity validation per-request.
  2. **Zero Trust (SPIFFE/SPIRE):** Secures the internal routing from the Gateway to the Core API.

  ## Implementation Prompt
  **To the Implementer:**
  Implement the Zero-Touch Multi-Tenant Edge Identity Gateway.
  - **Outcome:** A lightweight edge service (e.g., Envoy configuration or Rust proxy) that sits in front of the OHC Core API. It must extract identity tokens (JWT), securely validate them using SPIFFE/SPIRE, append the `X-Tenant-ID` header, and route the request to the backend.
  - **CUJ (Critical User Journey):**
    1. A mobile client sends an HTTP request with an authorization token.
    2. The Edge Gateway intercepts it.
    3. The Gateway validates the token using local SPIRE nodes (sub-10ms latency).
    4. The Gateway resolves the `tenant_id` and appends `X-Tenant-ID`.
    5. The OHC API processes the request, trusting the Gateway.
  - **Acceptance Criteria:**
    - Edge authentication latency must be strictly <10ms under load.
    - Invalid tokens must be rejected at the edge with a 401 Unauthorized before reaching the core API.
    - Zero Trust architecture must be maintained: the core API must only accept requests from the Edge Gateway.
    - The implementation must include 100% unit test coverage and pass all existing Playwright E2E tests for the frontend authentication flows.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
