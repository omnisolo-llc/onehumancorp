issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  # Native Rust Omnichannel Chat System Architecture

  ## Problem Statement
  OneHumanCorp (OHC) currently relies on Chatwoot as an external dependency for managing omnichannel customer support. This creates a critical architectural vulnerability: external dependencies introduce latency, complicate multi-tenant data isolation, break the "zero-trust" internal security boundary, and require operators to manage third-party system integrations. Maya (the home baker) and Carlos (the field service owner) need a seamless, instantly responsive, native inbox where AI agents can draft replies to Instagram DMs and Web Chats without passing data to external vendors.

  ## Research Report
  Our codebase audit confirms that Chatwoot is being retired. We benchmarked the `https://github.com/chatwoot/chatwoot` source code, examining its:
  - **Data Models**: `Account` (Tenant), `Inbox`, `Conversation`, `Message`, `Contact`, `Channel`.
  - **Real-time WebSockets**: ActionCable-based pub/sub for instant message delivery.
  - **Agent Routing**: Round-robin and SLA-based assignment.
  - **Omnichannel Adapters**: Abstractions for Web Widget, Email, WhatsApp, Instagram, etc.

  Competitor analysis of Shopify Sidekick and Wix Inbox shows that tight native integration with the commerce ledger and CRM is essential. A native Rust implementation using our existing `sqlx` Postgres multi-tenant schema and Tokio-based WebSockets will provide sub-millisecond local latency, strong isolation, and direct access to OHC's AI drafting agents without external API hops.

  ## Design Doc

  ### Architecture
  We will implement a Native Rust Chat microservice inside `onehumancorp/mono` replacing Chatwoot features.

  **Core Components:**
  1.  **OmniInbox Server (Rust/Tokio/Axum)**: Handles REST APIs for message fetching and WebSocket connections for real-time delivery.
  2.  **Channel Adapters (Rust)**: Traits and implementations for `WebWidget`, `Email`, and future social channels, parsing incoming webhooks and standardizing them into `OmniMessage` structs.
  3.  **PostgreSQL Multi-Tenant Storage**: Strict row-level security based on `tenant_id`. Tables: `omni_inboxes`, `omni_conversations`, `omni_messages`, `omni_contacts`.
  4.  **Redis Pub/Sub**: For horizontal scaling of WebSocket nodes, broadcasting new messages to connected clients.
  5.  **AI Department Integration**: The Customer Service AI agent listens to the `omni_messages` event stream. When a customer messages, the agent retrieves context from the `omni_contacts` table and the commerce ledger, then drafts a reply for the owner's review.

  ### Mobile UX Flow (375px First)
  - **Unified Inbox View**: A bottom navigation tab leads to the Inbox. The list shows conversations ordered by latest activity, with clear avatars (Web, IG, Email icons).
  - **Conversation View**: Tapping a conversation opens a standard chat UI. A sticky bottom input area contains native mobile keyboard support and an "AI Draft" button prominently displayed.
  - **Translucent Glass UI**: Unread badges and header bars use our macOS-style translucent materials and UniFi-style spacing.
  - **Offline Resilience**: Messages are stored locally in the PWA/Flutter SQLite cache. Sending a message while offline queues it, showing a translucent "Sending..." state until network is restored.

  ### Implementation Prompt
  **Implementer Agent Task**: Build the foundational Native Rust Omnichannel Chat API and Database Schema.
  1.  **Database**: Create `sqlx` migrations for `omni_inboxes`, `omni_conversations`, `omni_messages`, and `omni_contacts` with strict `tenant_id` foreign keys and RLS.
  2.  **API**: Implement Axum REST endpoints in `src/server/ohc/inbox.rs` (or similar) to list inboxes, fetch conversations, and send messages.
  3.  **WebSocket**: Implement a basic Tokio/Axum WebSocket route that allows a frontend client to subscribe to a specific `tenant_id` and `inbox_id` to receive real-time JSON message payloads.
  4.  **AI Hook**: Emit an async event (e.g., via NATS or internal channel) when a new customer message arrives, allowing the `Customer Assistant` agent to intercept and generate a draft.
  5.  **Testing**: Achieve 100% unit test coverage for the API and write a Playwright E2E test verifying a user can open the inbox, see a message, and send a reply via the UI.

  ## Priority: P0 (Critical - Blocks Chatwoot Retirement)
  ## Estimated Scope: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
