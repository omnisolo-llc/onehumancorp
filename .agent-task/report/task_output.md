issue_title: "Implement Native Rust Omnichannel Chat (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  Chatwoot as an external third-party service and dependency has been completely retired from the OHC architecture. However, small business owners (like Maya the baker and Carlos the handyman) still need a unified inbox to manage customer inquiries across Instagram DMs, WhatsApp, SMS, and website chat. Without a native, multi-tenant Rust chat system, our operators lack the core work triage and customer relationship capabilities critical to their daily operations.

  ## Research Report
  ### Context & Gap Analysis
  - **Market Context:** Competitors like Shopify Sidekick and Wix Inbox integrate messaging directly into the commerce dashboard. A fragmented experience using external services causes context switching and loss of data isolation.
  - **Chatwoot Source Code Audit:**
    - Chatwoot handles omnichannel via `Channel Adapters` (e.g., `Channel::Whatsapp`, `Channel::Sms`, `Channel::Api`).
    - The core models include `Account` (Tenant), `Inbox`, `Conversation`, `Message`, and `Contact`.
    - Real-time updates are driven by ActionCable WebSockets.
  - **Proposed Replacement:** A native Rust implementation utilizing high-performance WebSockets (e.g., using `tokio` and `axum`) integrated tightly with OHC's existing PostgreSQL row-level security (RLS) multi-tenancy model.

  ## Design Doc
  ### Architecture & Data Model
  - **Core Entities:**
    - `Tenant` (Tenant ID enforced via RLS)
    - `Inbox` (Aggregates multiple channels)
    - `ChannelAdapter` (Rust traits for specific providers like Twilio, Meta API)
    - `Conversation` (Linked to a Contact and Inbox)
    - `Message` (The actual payload, supports text and media)
  - **Real-time Layer:** Rust `axum` WebSocket server using Redis Pub/Sub for horizontal scaling across nodes.
  - **Agent Integration:** AI Customer Assistant hooks into the event bus to automatically draft replies for `Conversation` events before the owner even sees the message.

  ### Mobile UX Flow (375px First)
  - **Unified Inbox Screen:** Sticky top navigation. Each conversation item shows a clear avatar, channel icon (e.g., WhatsApp), and unread indicator. Uses macOS-style Translucent Glass materials.
  - **Conversation View:** Full screen on mobile. Native keyboard support. Input area includes quick actions for "AI Draft", "Attach Offer", or "Request Payment".
  - **Zero-Trust Security:** API endpoints will require SPIFFE/SPIRE identity tokens and strictly enforce `tenant_id` in all SQL queries.

  ## Implementation Prompt
  **Goal:** Build the native Rust omnichannel chat system to replace Chatwoot.
  **CUJ:**
  1. Maya (Tenant) logs into the OHC Flutter app.
  2. A customer sends an Instagram DM asking about cake prices.
  3. The message arrives via a webhook, is processed by the new Rust `ChannelAdapter`, and saved to the `Message` table.
  4. The WebSocket server pushes the update to Maya's app instantly.
  5. The AI Assistant drafts a reply based on Maya's product catalog.
  6. Maya taps "Send" on the AI draft, dispatching the message back through the Rust API to Instagram.

  **Acceptance Criteria:**
  - Rust models, migrations, and WebSocket API for Conversations and Messages are implemented.
  - Strict multi-tenant isolation via RLS and SPIFFE/SPIRE authentication.
  - End-to-end integration tests mimicking a webhook ingestion, database insertion, and WebSocket push.
  - No external Chatwoot dependencies remain.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
