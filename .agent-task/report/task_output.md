issue_title: "[Omnichannel Chat] Native Rust Implementation of Chat system Core Entities and Messaging"
issue_description: |
  # Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales.

  OHC previously relied on Chat system as an external dependency for omnichannel messaging. However, Chat system as an external service is 100% RETIRED. OHC must implement its own high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust inside `onehumancorp/mono` to achieve feature parity, absolute control over data tenancy, and seamless integration with our AI agents.

  # Research Report
  **Findings & Competitive Analysis:**
  - **Chat system Source Code Audit:** Chat system uses a robust core model consisting of Accounts (Tenants), Inboxes, Channels, Contacts, ContactInboxes, Conversations, and Messages. It heavily utilizes WebSockets for real-time updates and relies on background workers for integrations.
  - **Shopify/Wix Inboxes:** Often lack deep, native agentic integration (proactive drafting) and are limited in their external channel support compared to dedicated tools.
  - **OHC Native Rust Approach:** By building this natively in Rust, we can guarantee strong multi-tenant isolation at the DB and application level, extremely low latency for real-time messaging, and tight integration with `The Ambassador` AI agent for proactive context-aware reply drafting.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[External Webhooks: IG/WhatsApp/Email] -->|Gateway| B(Rust Channel Adapters)
      B --> C{Conversation Manager - Rust}
      C --> D[(PostgreSQL Unified DB)]
      C --> E[Redis / NATS for PubSub]
      E --> F[WebSocket Server - Rust]
      F --> G[OHC Flutter App 375px]
      C --> H[Event Bus]
      H --> I[AI Agent Coordinator]
      I -->|Drafts Reply| C
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Unified Inbox List:** A clean list view of active conversations, showing the channel icon (WhatsApp, IG), customer name, last message preview, and an indicator if an AI draft is pending.
  - **Conversation View:** Standard chat interface. If an AI draft exists, it appears above the input bar as a "Suggested Reply" card with "Send" and "Edit" buttons.
  - **Customer Context Panel:** A swipeable/expandable drawer showing the contact's past orders and tags.
  - **Visual Design:** Adheres to OHC Premium Token library. Clean, fast, native-feeling interactions.

  ### AI Agent Integration Points
  - **Native Event Emitting:** The Rust backend emits events for new incoming messages.
  - **The Ambassador:** Subscribes to these events, reads the context from the native DB, and writes back a proposed message draft linked to the conversation.

  ### Key Design Decisions
  - **Data Model Migration:** Replicate the core schema: `Tenants` (Accounts), `Channels` (polymorphic or specific tables per type), `Inboxes`, `Contacts`, `Conversations`, and `Messages`.
  - **Rust Backend:** Use Actix/Axum for HTTP and WebSockets. Tokio for async processing. Diesel or SQLx for DB interaction with strict tenant-ID enforcing macros/scopes.
  - **Real-time:** Implement a robust WebSocket handler in Rust that pushes updates to connected clients based on Redis Pub/Sub events.

  # Implementation Prompt
  **User-Facing Outcome:** The owner opens their OHC app and sees a unified feed of messages from WhatsApp and Instagram. They can read the history, see AI-drafted replies, and respond instantly, all powered by our lightning-fast native backend without relying on external third-party chat software.

  **CUJ & Acceptance Criteria:**
  1. Define the core Rust structs and database schema (PostgreSQL) for `Inbox`, `Channel`, `Contact`, `Conversation`, and `Message`, ensuring strict multi-tenant (`tenant_id`) isolation.
  2. Implement REST API endpoints in Rust to create and fetch Conversations and Messages for a specific tenant.
  3. Implement a basic WebSocket server in Rust that clients can connect to, authenticating and subscribing to updates for their tenant's inboxes.
  4. Write comprehensive unit tests in Rust for the core logic and database interactions.
  5. Provide an E2E Playwright test where a mocked incoming webhook creates a message, and the UI (mocking the WebSocket client or polling) correctly displays the new message in the conversation view.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
