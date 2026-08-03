issue_title: "Implement Native Rust Omnichannel Chat System for OHC"
issue_description: |
  # Native Rust Omnichannel Chat System for OHC

  ## Problem Statement
  As mandated by OHC Engineering Standards, the Chatwoot external third-party integration is 100% retired. OHC requires its own high-performance, multi-tenant omnichannel customer support & chat engine built natively in Rust. This ensures zero external dependencies for chat, tighter multi-tenant isolation, and perfect integration into the broader OHC operations capabilities (Operations Assistant, Customer Relationship Assistant). Maya (baker) and Carlos (handyman) need an immediate, reliable inbox to handle incoming queries from Instagram, SMS, WhatsApp, and their custom website widget, all aggregated in one 375px mobile view.

  ## Research Report
  Based on an architectural audit of the `chatwoot/chatwoot` repository, the core data domain centers around several critical entities:
  - **Account/Tenant:** The multi-tenant boundary. Every entity must strict row-level security using `account_id` (mapped to OHC's `tenant_id`).
  - **Inbox & Channel:** Inboxes route incoming communications from specific Channels (Web Widget, API, Email, Facebook, etc.).
  - **Contact & ContactInbox:** The `Contact` represents the end customer. The `ContactInbox` binds a contact to a specific source channel securely (using `hmac_verified` tokens and `source_id`).
  - **Conversation & Message:** Conversations group messages by contact and inbox. Messages handle polymorphic content types (text, templates, forms, attachments) and track read/delivery statuses.
  - **User & Agent:** Operators of the platform who respond to conversations.

  For OHC's Rust implementation, these models will be translated into a highly concurrent Rust service utilizing PostgreSQL for relational data and Redis for PubSub WebSocket propagation.

  ## Design Doc

  ### Architecture Overview
  ```mermaid
  graph TD
      subgraph OHC Native Rust Chat Services
          A[WebSocket Gateway] -->|PubSub| B(Redis Message Bus)
          C[Webhook Listener] -->|Ingest| D{Channel Router}
          D --> E[Conversations & Messages DB Layer]
          D --> F[AI Auto-Response Agent]
          E -->|Write| G[(PostgreSQL with RLS)]
          B <-->|Real-time Events| E
      end
      subgraph Client Side
          H[OHC Flutter Mobile/PWA] <-->|WSS & REST| A
          I[Customer Web Widget] <-->|WSS & REST| A
      end
  ```

  ### Data Model Invariants (Rust / Diesel/SQLx target)
  1. **Strict Multi-tenancy:** All tables (Inboxes, Conversations, Messages, Contacts) MUST include `tenant_id` for row-level security.
  2. **Conversations:** Needs `id`, `tenant_id`, `inbox_id`, `contact_id`, `status` (open, resolved, snoozed), `assignee_id`.
  3. **Messages:** Needs `id`, `tenant_id`, `conversation_id`, `sender_type` (Contact, Agent, AI), `sender_id`, `content_type`, `content`, `status`.
  4. **Contacts:** Needs `id`, `tenant_id`, `identifier` (email/phone), `name`.
  5. **ContactInbox:** Links a Contact to a specific Inbox with a unique channel-specific `source_id`.

  ### Mobile UX Flow (375px First)
  - **Home Screen:** Global "Unified Inbox" icon with an unread counter.
  - **Inbox List:** Vertically scrollable list of conversations. Each item shows the contact avatar, name, channel icon (e.g., WhatsApp, Web), and a snippet of the latest message.
  - **Conversation View:** Standard chat interface. Sticky header with contact name and status. Scrollable message history. Sticky input field at the bottom with quick replies (AI drafted) and an attachment button. Native keyboard triggers scroll-to-bottom.

  ### AI Agent Integration Points
  - **Work Triage:** AI reads incoming messages and tags them (e.g., "urgent", "quote request").
  - **Customer & Relationship Assistant:** AI drafts replies automatically based on previous conversations and the owner's knowledge base. Owners see the draft in the input box and tap to approve/send.

  ### Key Design Decisions
  - **Rust Native:** The chat engine is built in Rust using Tokio/Tonic/Axum for extreme high concurrency and low latency.
  - **Row Level Security (RLS):** Enforced at the PostgreSQL level for every query using `tenant_id` to prevent data leakage between owners.
  - **Event-Driven:** WebSockets via Redis PubSub ensure instant updates on the Flutter client.

  ## Implementation Prompt
  **To the Implementer:**
  Implement the backend core data models and service layer for the OHC native Rust omnichannel chat system.
  1. Create the database migrations (PostgreSQL) for `inboxes`, `contacts`, `contact_inboxes`, `conversations`, and `messages` using strict multi-tenant row-level security on `tenant_id`.
  2. Implement the Rust REST and WebSocket API endpoints to list conversations, send messages, and receive incoming webhook payloads from external channels.
  3. Ensure the AI Assistant can subscribe to new messages to generate automated draft replies.
  4. Build a Flutter UI for the Inbox and Conversation views targeting a 375px mobile screen.
  5. Verify the full CUJ using Playwright: An owner logs in, navigates to the unified inbox, reads an incoming message, views an AI-drafted reply, and clicks send.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []