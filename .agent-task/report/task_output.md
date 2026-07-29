issue_title: "Native Rust Omnichannel Chat: Inbox, Conversations, and Messaging Pipeline"
issue_description: |
  ## Problem Statement
  We have a mandatory directive to retire Chatwoot as an external third-party service. OHC must own its complete omnichannel chat engine natively in Rust. The current product relies on piecemeal chat modules without a unified data model that correctly implements the abstractions Chatwoot pioneered: Inboxes, Channels, Conversations, Messages, and polymorphic Senders. Our business owners (like Maya the Baker or Carlos the Handyman) need a central "Inbox" view on their phones where DMs from Instagram, texts from Twilio, emails, and web widgets all flow in, get triaged, and have AI agents auto-reply as part of their workflow—without the complexity of navigating a separate chat app.

  ## Research Report
  I audited the Chatwoot source code repository (`https://github.com/chatwoot/chatwoot`) to understand their core architecture. Chatwoot relies on the following key domain abstractions:
  1.  **Account (`account_id`)**: The tenant (multi-tenancy root).
  2.  **Inbox (`inboxes`)**: Represents a specific pipeline/number/handle (e.g., "Maya's Bakery IG"). Belongs to a channel type and has config for auto-assignment, CSAT, working hours, and greeting messages.
  3.  **Channel (`channel_id`, polymorphic)**: The provider adapter holding configuration. There are specific models for WebWidget, Email, Instagram, FacebookPage, TwilioSms, WhatsApp, API, etc.
  4.  **Contact (`contacts`)**: The end-user communicating with the owner.
  5.  **Conversation (`conversations`)**: The thread. Has statuses (`open`, `resolved`, `snoozed`, `bot`), priority, and pointers to the contact, the inbox, the assignee agent, and SLA policies.
  6.  **Message (`messages`)**: The individual payload. Can be `incoming`, `outgoing`, or `template`. Has rich `content_type`s (text, image, card), sentiment fields, and external provider IDs (`source_id`). Crucially, messages have a `private` boolean for internal notes, and `sender_type`/`sender_id` which can point to a human Contact, a human Agent (owner), or a bot.

  These models map perfectly to our `onehumancorp/mono` architecture if implemented natively in Rust as `src/server/services/chat`.

  ## Design Doc
  **Architecture Overview**
  - **Data Models (Postgres via SeaORM or sqlx in Rust)**:
    - Create strongly typed tables for `chat_inboxes`, `chat_channels` (JSONB config or STI pattern), `chat_contacts`, `chat_conversations`, and `chat_messages`.
    - Every table MUST have `tenant_id` for row-level security.
  - **Services**:
    - `ChatInboxService`: manages inbox lifecycle and channels.
    - `ConversationService`: handles assignments, status changes, and priority.
    - `MessageService`: normalizes incoming webhooks to `chat_messages` and dispatches outgoing messages through the appropriate channel integration.
  - **AI Agent Integration**:
    - The "AI Agent" is just a system `sender` in a conversation. When a conversation is assigned to the "bot" status, incoming messages trigger an AI job in the Postgres SKIP LOCKED queue to generate and send a draft reply.
  - **Mobile UX Flow (375px)**:
    - The "Messages" tab shows a unified list of recent `Conversations` sorted by `last_activity_at`.
    - Tapping a thread opens the chat view with native mobile keyboard. The message composer includes an AI "Draft Reply" button right next to the Send button.
    - Avatars indicate the channel (Instagram icon vs SMS bubble).

  ## Implementation Prompt
  Implement the core Rust domain structs, Postgres schema migrations, and SeaORM/sqlx entities for the Native Omnichannel Chat engine.
  1. Create a DB migration for `chat_inboxes`, `chat_conversations`, and `chat_messages` with strict `tenant_id` multi-tenancy columns.
  2. Implement the Rust models and repository layer in `src/server/services/chat/repository`.
  3. Implement the `MessageService::send_message` method which saves the message to the DB and publishes a pubsub event (using the existing pubsub service or NATS) so real-time clients can be updated.
  4. Create a basic gRPC or REST endpoint for the mobile app to fetch conversations and messages.
  Ensure full unit test coverage and no reliance on external Chatwoot.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
