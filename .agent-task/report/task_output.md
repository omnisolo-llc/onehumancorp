issue_title: "Design and Implement Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  ### Title
  Build Native Rust Omnichannel Chat System (Chatwoot Replacement)

  ### Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels (Instagram DMs, WhatsApp, web widgets, SMS). Managing these manually or through disconnected third-party tools like Chatwoot leads to missed messages, slow response times, and lost sales. Currently, OHC relies on Chatwoot as an external service for omnichannel support, which creates data silos, adds latency, complicates deployment, and prevents seamless AI agent integration. A native, high-performance omnichannel inbox built in Rust is required to natively process incoming messages, maintain strict multi-tenant isolation, and instantly trigger OHC's internal AI agents (like The Ambassador) for autonomous customer support.

  ### Research Report
  **Findings & Chatwoot Source Code Audit:**
  - Analyzed the `chatwoot/app/models/` source code. Chatwoot relies on core entities such as `Account` (Tenant equivalent), `Inbox`, `Conversation`, `Message`, `Contact`, and `ContactInbox`.
  - Chatwoot abstracts integrations using polymorphic `Channel::*` models (e.g., `Channel::Whatsapp`, `Channel::WebWidget`, `Channel::FacebookPage`).
  - Real-time communication is handled via WebSockets (ActionCable in Rails) for the agent dashboard and web widgets.
  - **OHC Native Rust Opportunity:** We can implement a highly optimized version of this architecture in Rust within `onehumancorp/mono`. By leveraging PostgreSQL Row-Level Security (RLS) on `tenant_id`, we can guarantee strict multi-tenant isolation at the database level.
  - Using Rust's asynchronous capabilities (Tokio) and WebSocket libraries (e.g., `axum` or `tungstenite`), OHC can handle thousands of concurrent connections efficiently.
  - **Competitor Systems Audit:** Shopify Inbox and Wix Inbox aggregate messages but lack proactive AI. Stripe and Apple focus on high security and clean UX. A native Rust implementation allows us to build a seamless unified inbox that triggers our internal AI agents (operations, CS, etc.) natively without external API roundtrips.

  ### Design Doc

  **Architecture Diagram:**
  ```mermaid
  graph TD
      A[WhatsApp Webhook] -->|Ingest| B(Rust API Gateway)
      C[Instagram DM Webhook] -->|Ingest| B
      D[Web Chat Widget (WebSocket)] -->|Stream| B
      B --> E{Channel Adapter Router}
      E --> F[WhatsApp Channel Adapter]
      E --> G[Instagram Channel Adapter]
      E --> H[Web Widget Channel Adapter]
      F --> I[Omnichannel Core Service]
      G --> I
      H --> I
      I -->|Persist RLS isolated| J[(PostgreSQL: Conversations, Messages, Contacts)]
      I --> K[Event Bus / Redis PubSub]
      K --> L[The Ambassador AI Agent]
      K --> M[WebSocket Notifier - Owner App]
      L -->|Drafts Reply| I
      M --> N[Owner Mobile App 375px]
  ```

  **Mobile UX Flow (375px First):**
  - **Unified Inbox Feed:** The owner sees a clean, single stream of messages on their phone. Badges indicate the source (WhatsApp icon, Web icon).
  - **Message View:** Tapping a conversation opens a standard chat interface. The UI is built with translucent glass materials (macOS style).
  - **AI Proactive Draft:** If a customer asks a question, the system shows an "AI Drafted Reply" floating card above the composer. The owner can tap "Send" or "Edit". No need to switch contexts or manually type standard replies.
  - **Offline/Flaky Network:** The app queues outgoing messages locally. If the network drops, a truthful "Sending..." state is shown, utilizing a local SQLite cache until sync is restored.

  **AI Agent Integration Points:**
  - **Event Trigger:** Whenever a new `Message` is created by a customer, an event is published to the Redis queue.
  - **The Ambassador (CS Agent):** Consumes the event, retrieves the `Conversation` history, the `Contact`'s past orders, and drafts a reply.
  - **State Management:** The AI saves the draft to the database, pushing a WebSocket update to the owner's app to show "Draft Ready for Approval".

  **Key Design Decisions:**
  - **Zero Chatwoot Dependency:** 100% native Rust implementation. No external dependencies for the chat engine.
  - **Data Model:** Direct mappings for `conversations` (status, contact_id, tenant_id), `messages` (content, sender_type, tenant_id), and `inboxes` (channel_type, credentials). All tables mandate `tenant_id` for RLS.
  - **Extensibility:** The channel adapter pattern allows us to easily add SMS (Twilio), Email, or Telegram later.

  ### Implementation Prompt
  **User-Facing Outcome:** Maya, a baker, receives an Instagram DM. Her OHC app pings her with a drafted reply from her AI assistant. She taps "Approve" and the reply is sent back via Instagram. She does not know or care that Chatwoot is gone; her app is just faster and fully integrated with her store.

  **CUJ & Acceptance Criteria:**
  1. Define Rust structs, PostgreSQL migrations (with RLS), and API endpoints for `Conversations`, `Messages`, `Contacts`, and `Inboxes`.
  2. Implement a `ChannelAdapter` trait and concrete implementations for WhatsApp Webhooks and Web Widget WebSockets.
  3. Ensure a `POST /api/v1/webhooks/whatsapp` endpoint correctly parses incoming Meta webhook payloads, creates a `Message`, and associates it with the correct `Conversation` and `Tenant`.
  4. Ensure all database queries enforce `tenant_id` boundaries.
  5. Provide Playwright E2E tests: A mocked WhatsApp webhook hits the API. The owner logs in via the UI, sees the new message in the mobile inbox, and sends a reply which triggers the outbound WhatsApp adapter.

  ### Priority
  P0

  ### Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, p0-mandate]
assignees: []
