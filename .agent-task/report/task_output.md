issue_title: "Omnichannel Native Rust Chat System Design (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OHC requires a native omnichannel chat system inside its Rust monolith, as Chatwoot is being 100% retired. Small business owners (like Maya the baker and Carlos the handyman) need a unified inbox that aggregates DMs (Instagram, WhatsApp, Email, Web Chat) into actionable work threads. The current setup relies on external services which breaks tenant isolation, increases latency, and makes AI agent context-sharing difficult. We need a high-performance, strictly tenant-isolated, native Rust implementation that replicates Chatwoot's core capabilities (Inboxes, Channels, Contacts, Conversations, Messages) while integrating deeply with OHC's AI triage agents.

  ## Research Report
  - **Source Code Audit (Chatwoot)**: Reviewed Chatwoot's core data models (`Conversation`, `Message`, `Inbox`, `Contact`, `Channel`, `AgentBot`). Chatwoot uses polymorphic associations for channels and sender types, which provides flexibility but can complicate strict tenant isolation if not modeled carefully in a monolithic Rust app.
  - **Current OHC State**: The repository has basic skeleton models in `src/server/services/chat/models.rs` and `service.rs`, but they lack the depth for true omnichannel support, real-time WebSocket capabilities, agent-bot handoffs, and AI triage integration. `src/server/services/inbox` exists but needs to be unified or clearly separated from the underlying chat engine.
  - **Competitive Analysis**: Platforms like Shopify Inbox and Wix Inbox natively integrate chat with store context. Our native solution must do the same: every conversation should have direct context to the customer's orders, bookings, and AI agent history.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL : configures
      INBOX ||--o{ CONVERSATION : contains
      TENANT ||--o{ CONTACT : owns
      CONTACT ||--o{ CONVERSATION : participates_in
      CONVERSATION ||--o{ MESSAGE : contains
      MESSAGE ||--o| ATTACHMENT : has

      TENANT {
          uuid id PK
          string name
      }
      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
          boolean auto_assignment
      }
      CHANNEL {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          string channel_type "e.g., WebWidget, Instagram, WhatsApp, Email"
          jsonb config
      }
      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string name
          string email
          string phone
          jsonb custom_attributes
      }
      CONVERSATION {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          uuid assignee_id FK "Optional (Agent or Human)"
          string status "open, snoozed, resolved"
          datetime last_activity_at
      }
      MESSAGE {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          string sender_type "Contact, Agent, Bot, System"
          uuid sender_id "nullable"
          string content
          string message_type "incoming, outgoing, template"
          boolean is_private "True for internal notes"
      }
  ```

  ### Core Capabilities & Flow
  1. **Omnichannel Ingestion**: Webhooks from Instagram/WhatsApp hit `ChannelAdapter` services. These adapters normalize payloads into internal `Message` creation requests.
  2. **AI Triage Integration**: When a new `Conversation` or `Message` is created, a background job is dispatched to the AI Triage Agent. The agent can update conversation status, draft a reply, or assign it to a human.
  3. **Real-time Updates**: A WebSocket service subscribes to Redis pub/sub channels (`tenant:{tenant_id}:inbox:{inbox_id}`). When a message is saved, an event is broadcasted.
  4. **Strict Isolation**: All queries MUST include `tenant_id` in the WHERE clause, enforced at the repository/service layer.

  ### Mobile UX Flow (375px)
  - **Inbox List View**: A unified feed of all active conversations. Badges for unread messages and AI-drafted replies pending approval.
  - **Conversation View**: Native chat UI. Sticky input bar at the bottom. Messages color-coded (Contact vs Human vs AI Bot). A distinct toggle to switch between "Reply to Customer" and "Internal Note".
  - **Context Panel**: (Accessible via a swipe or a top-right info button) Shows customer details, recent orders/bookings, and quick-action buttons (e.g., "Create Quote").

  ## Implementation Prompt
  **Goal:** Implement the core data layer, service logic, and WebSocket foundation for the native Rust Omnichannel Chat engine, replacing the need for Chatwoot.
  **Persona:** Maya (Baker) needs to see Instagram DMs, WhatsApp messages, and website chats in one single screen on her iPhone, with AI agent drafts ready for review.
  **Tasks:**
  1. Update DB migrations in `src/server/migrations` to reflect the ER diagram above (Inboxes, Channels, Contacts, Conversations, Messages). Ensure `tenant_id` is present on all tables and composite indexes include it.
  2. Implement the `ChatService` in `src/server/services/chat/service.rs` with methods to handle message ingestion, conversation creation, and status updates. Ensure all methods enforce `tenant_id` authorization.
  3. Set up a basic Redis pub/sub mechanism in `ChatService` to broadcast `message.created` and `conversation.updated` events, laying the groundwork for the WebSocket API.
  4. Implement Playwright E2E tests simulating a customer sending a message (via a mock webhook adapter) and the business owner seeing the message in the unified inbox UI.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
