issue_title: "[Architectural Design] Native Rust Omnichannel Inbox (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OHC currently relies on external systems (legacy Chatwoot dependencies) for omnichannel customer engagement. This introduces unnecessary latency, complicates multi-tenant isolation, and breaks the Zero Trust / SPIFFE security model required by our core architecture. To provide business owners (Maya, Carlos, Priya) with a true "OneHumanCorp" unified assistant experience, OHC must natively own the messaging infrastructure. We need a lightning-fast, native Rust omnichannel chat system that supports web widgets, WhatsApp Business, email, and social DMs in a unified inbox, seamlessly integrated with the OHC Agent Triage system.

  ## Research Report
  - **Chatwoot Codebase Audit:** An audit of `chatwoot/app/models` and `chatwoot/app/controllers/api` reveals a strong conceptual model that we can adapt natively. Key entities identified include:
    - `Account` (maps to OHC `Tenant`)
    - `Inbox` (the entry point for a specific channel)
    - `Conversation` (the threaded interaction)
    - `Message` (individual communication units)
    - `Contact` (maps to OHC `Customer`)
    - `Channel` (with specific adapters like WebWidget, Whatsapp, Email)
  - **Competitor Analysis:** Shopify Ping, Wix Inbox, and Stripe's customer communication tools centralize all messaging. However, OHC goes further by letting the AI Assistant draft replies and triage tasks directly from this data.
  - **Architecture Gap:** OHC's backend currently lacks the generalized event-driven message bus and multi-channel adapter patterns needed to ingest, normalize, and distribute real-time messages across web sockets (Web Widget) and webhooks (WhatsApp/Email) natively within our Rust stack.

  ## Design Doc
  ### Data Model & Invariants
  - All tables must enforce Row-Level Security (RLS) using `tenant_id`.
  - **Entities:**
    - `conversations` (id, tenant_id, inbox_id, contact_id, status)
    - `messages` (id, tenant_id, conversation_id, sender_type, sender_id, content, channel_message_id)
    - `inboxes` (id, tenant_id, name, channel_type, channel_config)
    - `contacts` (id, tenant_id, name, identifier)
  - **Multi-Tenancy:** PostgreSQL `ENABLE ROW LEVEL SECURITY` with tenant isolation.
  - **Real-Time Delivery:** Redis Pub/Sub will route incoming messages from the ingest API/webhooks to the correct active WebSocket connections (for the OHC owner dashboard and web widgets).

  ### System Architecture
  ```mermaid
  graph TD
      A[WhatsApp / Email / Web Widget] -->|Webhooks / API| B(Rust Ingest Service)
      B --> C{Channel Adapters}
      C -->|Normalize Message| D(Message Bus / Redis)
      D --> E[PostgreSQL DB with RLS]
      D --> F[WebSocket Manager]
      F --> G[OHC Owner Dashboard]
      F --> H[Live Web Widget]
      D --> I[AI Triage Agent]
      I -->|Draft Reply / Create Task| D
  ```

  ### Mobile UX Flow (375px)
  1. **Unified Inbox View:** A sticky bottom navigation bar or prominent home card showing "Unread Messages" with a badge.
  2. **Conversation Thread:** Clean, translucent chat bubbles. System events (e.g., "Agent drafted a reply", "Customer paid deposit") are interleaved in the chat feed as distinct cards.
  3. **Action Bar:** Persistent text input with quick-action buttons (Send AI Draft, Request Payment, Create Booking) easily tappable on mobile screens. Touch targets >= 44x44px.

  ### AI Agent Integration
  - **Work Triage:** Every incoming message triggers an async job to the AI Triage Agent via the PostgreSQL job queue. The agent reads the last 10 messages and the customer profile to determine the intent (e.g., inquiry, complaint, booking).
  - **Drafting:** The AI Assistant preemptively drafts a reply and saves it to the `messages` table with `status = drafted`. The owner sees this draft in the UI and can tap "Approve & Send" or edit it.

  ## Implementation Prompt
  Implement the native Rust omnichannel chat models, PostgreSQL schema with RLS, and the REST/WebSocket API layers to replace Chatwoot.
  1. Define Rust structs and Diesel/SQLx migrations for `Tenant`, `Inbox`, `Conversation`, `Message`, and `Contact`.
  2. Implement the `ChannelAdapter` trait and provide a concrete implementation for a `WebWidget` channel and a `WhatsApp` channel.
  3. Create the REST endpoints for ingesting messages and fetching conversation history.
  4. Implement a Redis-backed WebSocket manager that pushes real-time message events to connected 375px mobile UI clients.
  5. Ensure all database operations strictly enforce the `tenant_id` boundaries.
  Acceptance Criteria: A web client can connect via WebSocket, send a message to a tenant's inbox, the message is persisted with correct RLS, and the event is broadcasted back to the tenant's authenticated session.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
