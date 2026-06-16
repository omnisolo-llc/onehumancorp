issue_title: "Refactor Agent Feed Execution to a Dynamic Action Handler Protocol (Operations Manager)"
issue_description: |
  ## Problem Statement
  Currently, when a user (e.g., Maya the Baker or Carlos the Handyman) approves an AI-drafted action in the Agent Feed (e.g., a drafted social post, a quote, or an ambassador reply), the execution logic is hardcoded via `if feature_type == "..."` blocks that run raw SQL queries directly in the API handler (`src/server/api/agent_feed.rs`). This is unscalable, brittle, and prevents us from dynamically adding new AI agent capabilities. Non-technical owners rely on the OHC AI to execute complex, multi-step tasks (e.g., fulfilling subscriptions, adjusting inventory, routing localized shipping). This requires a robust, extensible execution engine—a true "Operations Manager"—rather than hardcoded API routes.

  ## Research Report
  - **Codebase Audit**: A review of `src/server/api/agent_feed.rs` reveals tightly coupled execution logic. When an item is marked as `APPROVED`, the handler manually checks `feature_type` (e.g., `incident_resolution`, `social_post_draft`, `ambassador_reply`, `quote_draft`) and executes raw `sqlx::query` statements. This violates separation of concerns and bypasses domain-layer validations.
  - **Competitor Analysis**: Traditional platforms (like Shopify or Wix) use structured APIs and webhooks to execute actions via third-party apps. For an AI-native platform like OHC, the AI should generate a standardized `ActionIntent` payload. When the owner taps "Approve" on their mobile feed, the system should route this intent to a registered, domain-specific handler (e.g., `QuoteHandler`, `InboxHandler`), ensuring Zero-Trust security, strict multi-tenant isolation, and proper audit logging.
  - **Owner/Operator Impact**: To achieve our vision of "Invisible AI Automation," the system must support hundreds of specialized agent actions. Without a dynamic action execution protocol, every new AI capability requires manual API endpoint changes, slowing down feature velocity and risking database corruption during concurrent mobile approvals.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Owner as Mobile App (375px)
      participant API as Agent Feed API
      participant ActionRouter as Operations Manager (Router)
      participant Handler as Domain Handler (e.g., Quote, Inbox)
      participant DB as PostgreSQL (Central Ledger)

      Owner->>API: POST /feed/{id}/state (state: APPROVED)
      API->>DB: Update feed item state
      API->>ActionRouter: Dispatch ActionIntent (Payload)
      ActionRouter->>ActionRouter: Lookup registered handler by feature_type
      ActionRouter->>Handler: execute(tenant_id, payload)
      Handler->>DB: Apply domain-specific business logic
      Handler-->>ActionRouter: Result (Success/Fail)
      ActionRouter-->>API: Acknowledgment
      API-->>Owner: 200 OK + UI Optimistic Update
  ```

  ### Key Design Decisions
  1. **Dynamic Action Router**: Introduce a registry pattern in the backend where different domain modules (e.g., CRM, Sales, Fulfillment) can register their action handlers.
  2. **Standardized ActionIntent**: All agent proposals must include a standardized JSON payload defining the target resource and mutation (e.g., `{"action": "mark_replied", "resource_type": "inbox_message", "resource_id": "123"}`).
  3. **Multi-Tenant Security**: Every handler must receive and strictly enforce the `tenant_id` context. No raw SQL updates should bypass the application's row-level security constraints.

  ### Mobile UX Flow (375px first)
  1. Maya opens the OHC app and sees a feed card: "Drafted reply to Sarah's Instagram DM."
  2. The card displays the drafted text and a prominent "Approve & Send" button (≥ 44x44px touch target).
  3. Maya taps "Approve & Send". The button immediately switches to a truthful loading state (e.g., a spinner).
  4. The frontend sends the `APPROVED` state to the API. The Action Router safely executes the domain logic.
  5. Upon success, the card gracefully transitions to a "Sent" state, removing itself from the active feed without requiring a full page reload.

  ### AI Agent Integration Points
  - **Generation**: When the LLM (e.g., The Ambassador) generates a draft, it must attach a strongly typed `context_payload` that conforms to the new ActionIntent schema.
  - **Execution**: The Action Router serves as the bridge between the AI's intent and the system's strict database mutations.

  ## Implementation Prompt
  **User-Facing Outcome**: As an owner, when I approve an agent's suggestion in my feed, the system executes it reliably and instantly without backend errors, paving the way for hundreds of new AI capabilities.
  **CUJ & Acceptance Criteria**:
  1. Refactor `update_feed_item_state` in `src/server/api/agent_feed.rs` to remove hardcoded SQL queries.
  2. Implement an `ActionRouter` (or similar dispatcher) that maps `feature_type` strings to specific handler functions (e.g., routing `quote_draft` to a function in `src/server/domain/sales.rs` or `quotes.rs`).
  3. Move the existing raw SQL updates (for incidents, inbox messages, quotes) into their respective domain modules.
  4. Write comprehensive unit tests for the new dispatcher to ensure unsupported `feature_type`s are handled gracefully without crashing.
  5. E2E Requirement: Implement a Playwright test simulating an owner approving an item in the feed and verify that the target table (e.g., `omni_inbox_messages`) is correctly updated via the new domain handler.

  ## Priority & Scope
  - **Priority**: P0
  - **Estimated Scope**: Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
