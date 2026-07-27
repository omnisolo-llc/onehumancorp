issue_title: "Native Rust Omnichannel Chat: Architecture & Design"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) recently retired an external third-party chat service, adopting a mandate to build a high-performance, native Rust omnichannel customer support and chat engine. Currently, OHC lacks the foundational database models, gRPC definitions, and business logic to replicate the external tool's core entities (Inboxes, Channels, Conversations, Messages, and Contacts) required to support our target owner/operator personas (Maya the Baker, Carlos the Handyman) who rely on unified messaging across Instagram, WhatsApp, SMS, and Email.

  ## Research Report
  Based on an audit of the external open-source omnichannel codebase and OHC's mandate, we need to design a system that supports:
  - **Inboxes**: The central collection point for conversations, associated with a specific channel.
  - **Channels**: The underlying medium (Web Widget, Email, API, WhatsApp, Instagram, SMS). In the legacy platform, these are polymorphic associations to the Inbox.
  - **Conversations**: A thread of messages between a contact and the tenant (agents/bots).
  - **Messages**: Individual text/media payloads within a conversation.
  - **Contacts**: The end-user communicating with the tenant.

  The legacy app relies heavily on ActiveRecord's polymorphic associations for channels. In Rust/PostgreSQL, we will implement this using concrete foreign keys or a single `Channel` table with a `type` enum and JSONB configuration, ensuring strict row-level security (`tenant_id`).

  ## Design Doc

  ### Architecture
  The native chat system will be integrated into the `ohc-mono` backend (Rust + PostgreSQL).
  - **Data Model (PostgreSQL):**
    - `contacts` (`id`, `tenant_id`, `name`, `email`, `phone_number`, `avatar_url`, `custom_attributes` (JSONB))
    - `inboxes` (`id`, `tenant_id`, `name`, `channel_type` (Enum), `channel_id` (UUID), `avatar_url`, `settings` (JSONB))
    - `channels` (`id`, `tenant_id`, `type` (Enum: WEB_WIDGET, WHATSAPP, INSTAGRAM, SMS, EMAIL, API), `config` (JSONB), `provider` (String))
    - `conversations` (`id`, `tenant_id`, `inbox_id`, `contact_id`, `assignee_id` (User), `status` (Enum: OPEN, RESOLVED, PENDING, SNOOZED), `custom_attributes` (JSONB))
    - `messages` (`id`, `tenant_id`, `conversation_id`, `sender_type` (Enum: CONTACT, USER, AGENT_BOT), `sender_id` (UUID), `content` (Text), `message_type` (Enum: INCOMING, OUTGOING, TEMPLATE, ACTIVITY), `content_type` (Enum: TEXT, IMAGE, AUDIO, VIDEO, FILE), `status` (Enum: SENT, DELIVERED, READ, FAILED), `metadata` (JSONB))

  - **API (gRPC):**
    - `InboxService`: `CreateInbox`, `GetInbox`, `ListInboxes`, `UpdateInbox`, `DeleteInbox`
    - `ConversationService`: `CreateConversation`, `GetConversation`, `ListConversations`, `UpdateConversationStatus`
    - `MessageService`: `SendMessage`, `ListMessages` (with pagination), `MarkAsRead`
    - `ContactService`: `CreateContact`, `GetContact`, `UpdateContact`, `ListContacts`

  - **AI Agent Integration:**
    - AI Agents (e.g., Customer Ambassador) will subscribe to new `Message` events (via Postgres NOTIFY/LISTEN or application-level events) on specific `Inboxes`.
    - Agents will use `SendMessage` (as `sender_type = AGENT_BOT`) to reply asynchronously.
    - AI Work Triage will monitor `Conversations` and suggest actions or draft replies.

  ### Mobile UX Flow (375px first)
  1. **Unified Inbox View**: A list of `Conversations` sorted by recent activity. Each row shows the `Contact` avatar, name, last message preview, and an icon indicating the `Channel` (e.g., WhatsApp icon).
  2. **Conversation View**: A standard chat interface. Messages from the contact on the left, responses on the right. A prominent "Agent Draft" UI if an AI has suggested a reply, with "Approve" and "Edit" actions.
  3. **Contact Context Sheet**: Tapping a contact's avatar opens a bottom sheet showing their details (`custom_attributes`), past order history, and notes.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Your task is to implement the foundational database schema and gRPC API layer for OHC's native omnichannel chat system, replacing the legacy external service.

  1. **Database Schema**: Create SQL migrations (using SeaORM or the existing DB migration tool) to define the following tables with strict `tenant_id` Row-Level Security (RLS):
     - `contacts`, `channels`, `inboxes`, `conversations`, `messages`.
     - Use appropriate Enums for status, channel types, message types.
     - Include JSONB columns for flexible configuration (`config` on channels, `metadata` on messages).
  2. **gRPC Definitions**: Create Protobuf definitions (`.proto`) for the `InboxService`, `ConversationService`, `MessageService`, and `ContactService`.
  3. **Rust Services**: Implement the gRPC service handlers in `src/server/services/`. Ensure all queries are scoped by `tenant_id` retrieved from the authenticated context (SPIFFE/SPIRE).
  4. **Testing**: Write comprehensive unit tests for the Rust service layer and repository layer, ensuring 100% coverage. Write an E2E test that creates an inbox, a contact, starts a conversation, and sends a message.
  5. **MANDATORY**: Follow all superpowers skills, ensure `bazel test //...` passes 100%, and verify there is ZERO mocked data in the implementation.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
