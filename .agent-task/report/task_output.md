issue_title: "[Platform Architecture] Design Native Rust Omnichannel Customer Support & Chat Engine (Replacing Chatwoot)"
issue_description: |
  # Problem Statement
  Small business owners like Carlos, Maya, and Fatima receive customer inquiries across fragmented channels (Instagram DMs, WhatsApp, SMS, Web Chat, Email). To handle this currently, operators rely on disjointed third-party solutions or complex platforms.

  OHC previously relied on Chatwoot as an external dependency for omnichannel inboxes. The new OHC engineering mandate explicitly requires **complete retirement of Chatwoot** as a third-party service, to be replaced by a native, high-performance, multi-tenant Rust architecture inside `onehumancorp/mono`.

  An operator needs to open the OHC app, see all customer messages unified in one thread per customer, with full business context (previous orders, reservations), and have AI draft the replies proactively. This cannot happen efficiently or securely if customer chat data is siloed in a 3rd-party Ruby-on-Rails application.

  # Research Report
  **Findings & Competitive Analysis:**
  - **Chatwoot Source Code Benchmarking:** Audited the Chatwoot Ruby on Rails codebase (`https://github.com/chatwoot/chatwoot`). Key architectural components identified: `Conversation`, `Message`, `Inbox`, `Channel` (with polymorphic adapters for WhatsApp, SMS, WebWidget, etc.), and a WebSocket real-time delivery system via ActionCable.
  - **The Native Advantage:** Porting this model natively to Rust via our internal gRPC/API framework guarantees:
    - strict multi-tenant data isolation (row-level security via PostgreSQL),
    - zero-trust authentication via SPIFFE/SPIRE,
    - direct integration with our internal AI agent mesh (e.g., The Ambassador) without webhooks crossing external boundaries,
    - drastically reduced latency, avoiding Ruby/Rails overhead.
  - **Market Gap:** Existing platforms (Shopify Inbox, Wix Inbox) do not natively embed AI proactive drafting based on the *whole* identity graph. By bringing the chat architecture in-house, OHC can instantly merge chat history with the ledger and task systems.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Channels: WhatsApp / IG / Web] -->|External Webhooks / WS| B(Channel Adapters - Rust)
      B --> C[Unified Inbox Service - Rust gRPC]
      C --> D[(PostgreSQL - Multi-Tenant)]
      C --> E[Redis Pub/Sub & WebSockets]
      E --> F[Flutter Web / Mobile App - 375px]
      C --> G[AI Ambassador Agent]
      G -->|Reads context & drafts reply| C
      C -->|Draft Action Required| F
  ```

  ### Mobile UX Flow (375px First)
  - **Inbox List View:** A clean, Unifi/Apple-style list of active conversations. Each row shows the customer name, latest message snippet, channel icon (e.g., IG, Web), and an "AI Draft Ready" status pill.
  - **Conversation View:**
    - Top half: Translucent glass header showing customer profile summary (e.g., "Maya's Bakery - Last order 3 days ago").
    - Middle: Scrollable chat history.
    - Bottom: The AI-drafted reply is pre-filled in the text area. The operator can hit "Send Draft" with one tap, or tap the text area to edit it natively before sending.

  ### AI Agent Integration Points
  - **The Ambassador (Customer Success Agent):** Hooks into the `MessageCreated` domain event published by the native Rust Inbox Service. It reads the new message, fetches the Customer's order history from the same DB, drafts a response, and writes a `MessageDraft` entity back to the Conversation.

  ### Key Design Decisions
  - **Polymorphic Channels in Rust:** Instead of Chatwoot's Rails STI (Single Table Inheritance), use Rust Enums or distinct trait objects to represent `Channel::WhatsApp`, `Channel::WebWidget`, `Channel::Email`.
  - **WebSocket Real-time Sync:** The backend must expose a WebSocket server (using axum/tokio-tungstenite) to stream new messages and AI drafts to the Flutter client instantly.
  - **Strict Multi-Tenancy:** The database schema (`inboxes`, `conversations`, `messages`, `contacts`) MUST include `tenant_id` on every table with Row-Level Security enabled.

  # Implementation Prompt
  **User-Facing Outcome:** An operator opens the OHC mobile app and sees a unified inbox of all customer communications. When an Instagram DM arrives, the app updates instantly via WebSockets, and an AI-drafted reply appears immediately, ready to be sent with one tap.
  **CUJ & Acceptance Criteria:**
  1. **Schema & DB:** Implement the native Rust models/Protobufs and PostgreSQL migrations for `Inbox`, `Conversation`, `Message`, and `Contact` matching Chatwoot's core capabilities, ensuring strict `tenant_id` isolation.
  2. **Channel Adapters:** Implement a base channel trait and at least one concrete adapter (e.g., a dummy/mock WebWidget channel) capable of receiving a message and routing it to the correct `Inbox`.
  3. **WebSocket Delivery:** Implement a WebSocket endpoint that a Flutter client can subscribe to, which broadcasts `MessageCreated` events for a specific tenant/conversation.
  4. **AI Drafting Hook:** Emit a system event when a customer message is saved, which triggers a basic mock "Ambassador" agent that automatically appends a drafted response to the conversation.
  5. **Automated Verification:** Write Playwright E2E tests and Rust unit tests verifying that a message submitted to the system creates the correct database records and broadcasts the event over WebSockets. Run `bazel test //...` to ensure 100% pass rate.

  # Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
