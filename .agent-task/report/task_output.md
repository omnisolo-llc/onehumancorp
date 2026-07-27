issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  # Problem Statement
  Small business owners need an omnichannel chat system to interact with their customers seamlessly. Currently, the platform might have been relying on external third-party services like Chatwoot, which is against the OHC standards. We need to implement a native, high-performance, multi-tenant omnichannel customer support & chat engine in Rust natively in `onehumancorp/mono`.

  # Research Report
  Based on our analysis of the codebase and the `chatwoot` source code:
  - Chatwoot handles omnichannel conversations (WhatsApp, email, web widget) using a unified Inbox model, storing `Contact`, `Conversation`, and `Message` entities.
  - OHC's current `omnichannel_repo.rs` has a simplified model for `CustomerProfile`, `WorkItem`, `Conversation`, and `Message`, but we need to expand this and create full service implementations mirroring Chatwoot's capabilities, especially around real-time messaging, webhook ingestions, and multi-tenancy.
  - The goal is to fully retire Chatwoot and use a native Rust implementation in OHC.

  # Design Doc
  ## Architecture Diagram
  ```mermaid
  graph TD
      A[WhatsApp/Email Webhook] -->|Ingest| B(Omnichannel Gateway - Rust)
      C[Web Chat Widget] -->|WebSocket| B
      B --> D{Identity Resolution Engine}
      D -->|Lookup/Create| E[(Unified Customer Graph DB)]
      B --> F[Conversation Manager]
      F --> G[(Omnichannel Messages DB)]
      G --> H[Action Required Queue]
      H --> I[Owner Mobile App Feed]
      I -->|Approve/Reply| J[Dispatcher]
      J --> A
      J --> C
  ```

  ## Mobile UX Flow (375px First)
  - The business owner opens the OHC app and sees a unified "Inbox" tab.
  - Tapping a conversation opens a standard chat UI, showing the customer's history across different channels.
  - The UI uses premium translucent glass materials and clear Ubiquiti-style layouts.

  ## AI Agent Integration Points
  - **The Ambassador (Customer Success Agent):** Automatically drafts replies based on the customer context and product catalog when a new message arrives.

  ## Key Design Decisions
  - **Native Rust Implementation:** High performance and tight integration with OHC's authentication (SPIFFE/SPIRE) and multi-tenant systems.
  - **Zero Trust & Security:** Strict row-level security and multi-tenant boundaries enforced at the repository level.

  # Implementation Prompt
  **User-Facing Outcome:** Business owners can view and reply to customer messages from any channel directly within the OHC app, with proactive AI assistance, without relying on any external chat platform like Chatwoot.

  **CUJ & Acceptance Criteria:**
  1. Set up Rust Axum/Tonic API endpoints for webhook ingestion (WhatsApp, Email) and WebSocket connections (Web Widget).
  2. Implement the `Contact`, `Inbox`, `Conversation`, and `Message` models in Rust, strictly enforcing `tenant_id` boundaries.
  3. Wire up the Omnichannel Gateway to handle incoming messages, create/update conversations, and trigger the Ambassador Agent for AI drafting.
  4. Ensure all database operations use the `ohc_universal_ledger` or `omnichannel_repo` with row-level security.
  5. Provide Playwright E2E tests: Simulate an incoming webhook, verify the conversation is created, and ensure the owner can view and reply to it in the UI.

  # Priority
  P0

  # Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
