issue_title: "Implement Native Omnichannel Unified Chat Inbox (Phase 1: Foundation)"
issue_description: |
  # Problem Statement

  Small business owners (like Carlos the handyman, Maya the baker, or Priya the boutique owner) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, web chat, and email. Managing these manually leads to missed messages, slow response times, and lost sales. We recently removed Chatwoot (our external dependency for chat). We must now implement a **Native Omnichannel Unified Chat Inbox** directly in OHC (Rust backend, Flutter/React frontend). The system must centralize messages, provide a unified UI for the owner, and integrate deeply with OHC's customer identity graph and AI agents (like The Ambassador) for contextual auto-replies.

  # Research Report

  **Findings & Competitive Analysis:**

  - **Chatwoot Source Audit:** We audited `https://github.com/chatwoot/chatwoot`. Chatwoot uses a robust model consisting of Accounts (Tenants), Contacts, Inboxes, Channels, Conversations, and Messages. It heavily relies on PostgreSQL for relational data, Redis for queues (Sidekiq) and Pub/Sub, and ActionCable for WebSockets.
  - **Shopify Inbox & Wix Inbox:** Good aggregation, but AI features are mostly limited to "improving tone" or generating generic replies, not acting as an autonomous customer success agent.
  - **OHC Opportunity:** Leverage our "Teammate" AI philosophy. The Customer Success Agent (The Ambassador) doesn't just aggregate messages; it reads them, queries the customer's omnichannel identity graph (purchase history, past bookings, previous DMs), and proactively drafts a complete, accurate response.
  - **Architecture Gap:** We currently lack the core data models (Conversations, Messages, Contacts, Inboxes) and the backend Rust services to handle incoming multi-channel webhooks and real-time websocket delivery to the frontend unified inbox UI.

  # Design Doc

  ### Architecture Diagram

  ```mermaid
  graph TD
      A[Instagram DM] -->|Webhook| B(Omnichannel Gateway - Rust)
      C[WhatsApp] -->|Webhook| B
      D[Email] -->|Webhook| B
      B --> E{Customer Identity Resolution}
      E -->|Lookup/Create| F[PostgreSQL: Contacts, Conversations, Messages]
      F --> G[Event Mesh / NATS]
      G --> H[The Ambassador Agent]
      H -->|Query Context| F
      H -->|Draft Reply| I[Action Required Queue / DB]
      F -->|Real-time PubSub| J[WebSocket Server]
      J --> K[Mobile/Web App UI - Unified Inbox]
      I --> K
      K -->|1-Tap Approve/Reply| L[Omnichannel Dispatcher]
      L --> A/C/D
  ```

  ### Core Data Models (PostgreSQL / Rust Structs)

  We need to replicate the core Chatwoot models natively in Rust with strict OHC multi-tenant (`tenant_id`) isolation:

  - **`inboxes`**: Represents a channel endpoint (e.g., a specific WhatsApp number or FB Page).
    - `id`, `tenant_id`, `name`, `channel_type` (whatsapp, email, widget, etc.).
  - **`contacts`**: A unified customer profile.
    - `id`, `tenant_id`, `name`, `email`, `phone_number`, `avatar_url`.
  - **`contact_inboxes`**: Links a contact to a specific inbox identifier (e.g., their WhatsApp number).
    - `id`, `tenant_id`, `contact_id`, `inbox_id`, `source_id` (external ID).
  - **`conversations`**: A thread of messages between a Contact and the Business.
    - `id`, `tenant_id`, `contact_id`, `inbox_id`, `status` (open, resolved, snoozed), `assignee_id`.
  - **`messages`**: Individual messages in a conversation.
    - `id`, `tenant_id`, `conversation_id`, `sender_type` (contact, user, agent), `sender_id`, `content`, `message_type` (incoming, outgoing, template), `status` (sent, delivered, read, failed).

  ### Mobile UX Flow (375px First)

  - **Inbox List (Feed):** A unified list of open conversations. Each row shows the customer name, channel icon (WhatsApp, IG), preview of the last message, and a badge if there is an AI-drafted reply waiting for approval.
  - **Conversation View:** Standard chat interface. Messages from the customer on the left, owner/AI on the right.
  - **AI Integration (The Ambassador):** If the AI drafts a reply, it appears in a distinct glassmorphic "Draft" card at the bottom above the input bar, with primary "Approve & Send" and secondary "Edit" buttons.
  - **Customer Context Panel:** A swipe-from-right drawer (or top-level tabs on desktop) showing the customer's OHC history: past orders, upcoming bookings, total LTV.

  ### Key Design Decisions

  - **Zero Trust Multi-Tenancy:** Every table MUST have `tenant_id` and PostgreSQL Row Level Security (RLS) enabled.
  - **Real-time Engine:** Use Rust (e.g., Axum WebSockets + Redis/NATS PubSub) for pushing message updates to the client to ensure instant delivery.
  - **Proactive Drafting:** Move from read-reply to read-approve. The AI drafts the response before the user opens the app.
  - **Separation of Concerns:** Phase 1 focuses on the core CRUD APIs and data models. Phase 2 will focus on channel adapters (webhooks) and Phase 3 on the AI Ambassador integration.

  # Implementation Prompt

  **User-Facing Outcome:** The foundational database schema, Rust ORM models, and core gRPC/REST API endpoints for the Unified Inbox are implemented. A developer can create an inbox, a contact, start a conversation, and send/receive messages via the API, all strictly isolated by `tenant_id`.

  **CUJ & Acceptance Criteria (Phase 1):**
  1. Define the PostgreSQL schema (migrations) for `inboxes`, `contacts`, `contact_inboxes`, `conversations`, and `messages` ensuring `tenant_id` is present on all and RLS is configured.
  2. Implement the corresponding Rust models/entities (using the project's standard ORM/SQL builder, e.g., SQLx or SeaORM).
  3. Implement the core API service layer in Rust to support:
     - Creating/Listing Inboxes.
     - Creating/Resolving Contacts and ContactInboxes.
     - Creating/Listing Conversations.
     - Creating/Listing Messages within a Conversation.
  4. Ensure strict multi-tenant isolation; API calls must fail or filter correctly if accessing data across tenants.
  5. Provide exhaustive Unit Tests (100% coverage for new modules) and E2E API tests (e.g., using Bazel test targets) verifying CRUD operations and tenant isolation. Do NOT implement the frontend UI or external webhooks yet; focus purely on the robust native foundation.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
