issue_title: "[Native Chat] Implement Rust Omnichannel Chat System to replace Chatwoot"
issue_description: |
  # Problem Statement

  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" simply aggregate messages without context.

  OHC previously relied on Chatwoot as an external dependency to handle this, but following our new OHC Engineering Standards, Chatwoot has been 100% RETIRED as an external service. We need a native Rust omnichannel customer support & chat engine built directly into OHC to replace it and achieve 100% feature parity, allowing our AI "Ambassador" agent to seamlessly intercept, draft, and reply to these cross-channel messages.

  # Research Report

  **Findings & Competitive Analysis:**
  - **Chatwoot Source Audit:** Chatwoot uses separate models for Inboxes, Channels, Contacts, Conversations, and Messages. Channels adapter pattern handles different incoming networks (Web, Email, SMS, WhatsApp, Twitter). It relies on WebSockets for real-time updates and Postgres for persistence.
  - **Current OHC State:** We have basic initial SQL migrations (`1009_native_omnichannel_chat.sql`) creating `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages` tables with tenant isolation. We have some initial Rust structs in `src/server/services/chat/models.rs`.
  - **Missing Capabilities:** We need the full service layer to handle CRUD operations on these entities, a WebSocket event dispatcher for real-time UI updates, specific channel adapters (starting with a generic Webhook adapter for Instagram/WhatsApp ingestion), and the GraphQL/REST API endpoints for the mobile client.

  # Design Doc

  ### Architecture Diagram

  ```mermaid
  graph TD
      A[External Webhooks] -->|Ingest| B(Webhook Receiver API)
      B --> C{Channel Adapter Registry}
      C -->|Normalize| D[Chat Service Layer]
      D --> E[Postgres DB]
      D --> F[Redis Pub/Sub Event Bus]
      F --> G[WebSocket Gateway]
      G --> H[Flutter Mobile Client 375px]
      D --> I[AI Job Queue]
      I --> J[The Ambassador Agent]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)

  - **Home Feed (Mobile):** Top card shows "1 New Message from Sarah (Insta DM)".
  - **Interaction:** Tapping the card opens the Unified Inbox view. Top half shows the customer context. Bottom half shows the conversation history and a composer.
  - **Action:** A prominent primary button "Send Draft" if the AI drafted a reply, or a standard text input field with native mobile keyboard to manually respond.
  - **Visual Design:** Glassmorphism cards, blurred background to maintain focus, clean Ubiquiti UniFi modular dashboard card layouts.

  ### AI Agent Integration Points

  - **The Ambassador Agent:** Subscribes to the `conversation.message_created` event on the internal job queue. Uses RAG against the tenant's context to draft a reply, inserting a new `ChatMessage` with `sender_type = 'bot_draft'`.

  ### Key Design Decisions

  - **Native Rust Implementation:** Eliminates the operational overhead of running Ruby/Rails (Chatwoot) and guarantees strict multi-tenant data isolation using our existing Postgres RLS policies.
  - **Event-Driven Architecture:** Every message insertion triggers an event on Redis Pub/Sub, fanning out to connected WebSockets and AI worker queues.

  # Implementation Prompt

  **User-Facing Outcome:** As an owner, I open the OHC app and see unified messages from my customers across different channels. I can read their history and reply instantly, all powered natively by OHC without relying on a third-party chat widget provider.

  **CUJ & Acceptance Criteria:**
  1. Implement the Rust service layer in `src/server/services/chat/` for Inboxes, Channels, Contacts, Conversations, and Messages. It must enforce multi-tenant isolation via the `tenant_id` on every query.
  2. Implement an API route (`src/server/api/chat.rs`) that exposes these operations for the Flutter client.
  3. Implement a webhook receiver endpoint that accepts incoming JSON payloads, maps them to a specific `chat_channel` based on the URL token, creates or finds the `chat_contact`, creates a `chat_conversation` if needed, and inserts the `chat_message`.
  4. Ensure 100% unit test coverage for the new Rust service logic.
  5. Provide Playwright E2E tests: A test user configures an Inbox, a mock webhook fires to simulate an incoming message, and the UI test asserts the message appears in the owner's inbox feed.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, chatwoot-replacement]
assignees: []
