issue_title: "Implement Custom Rust Omnichannel Chat System Parity with Chatwoot"
issue_description: |
  **Title**: Implement Custom Rust Omnichannel Chat System Parity with Chatwoot
  **Priority**: P0
  **Estimated Scope**: Large

  ## Problem Statement
  OneHumanCorp currently has a rudimentary Rust-based chat system (`src/server/services/chat/models.rs`, `src/server/services/chat/service.rs`). However, the product vision explicitly mandates the **100% retirement of external Chatwoot services** and requires OHC to implement its own high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust inside `onehumancorp/mono`.

  The current implementation is lacking a significant portion of Chatwoot's core functionalities. It only has basic Inbox, Channel, Contact, Conversation, and Message entities. It is missing critical features such as:
  - Diverse Channel Adapters (Email, WhatsApp, Facebook Page, Instagram, SMS, Telegram, Web Widget, Twitter, Line)
  - Canned Responses & Macros
  - Team & Agent Assignments (with advanced SLA policies and auto-assignment)
  - Labels & Custom Attributes
  - Automation Rules
  - Real-time WebSocket APIs & Presence tracking

  This gap prevents Maya (the baker using Instagram DMs), Carlos (the handyman using SMS), and other personas from effectively managing their customer inquiries natively within OHC.

  ## Research Report
  Based on an audit of the Chatwoot source code (`https://github.com/chatwoot/chatwoot/tree/master/app/models`), Chatwoot's architecture relies heavily on polymorphic associations (e.g., `messages` having a `message_type` and `sender_type`, `inboxes` containing various `channel_type` models).

  Our native Rust implementation must replicate these core concepts while adhering to our strict multi-tenant Row-Level Security (RLS) PostgreSQL architecture.

  Competitor platforms like Zendesk, Intercom, and Shopify Inbox all support deep omnichannel capabilities. To be a true "Workbuddy-like AI work assistant," OHC must not just relay messages but allow AI agents to interject, draft responses, and trigger automations based on real-time channel events.

  ## Design Doc
  ### Data Model & Invariants (Native Rust Omnichannel Chat System)
  We need to expand the existing schemas in `src/server/services/chat/models.rs` and the DB migrations.

  *   `ChatInbox`: Represents an entry point (e.g., "Customer Support", "Instagram DMs").
  *   `ChatChannel`: Needs to support specific configuration structures for `web_widget`, `api`, `email`, `facebook_page`, `twitter_profile`, `twilio_sms`, `whatsapp`, `instagram`, `line`, `telegram`. We should use structured JSONB or specific relational tables for channel credentials securely.
  *   `ChatContact`: Must support custom attributes, avatars, and contact merging.
  *   `ChatConversation`: Needs fields for `status` (open, resolved, snoozed), `snoozed_until`, `assignee_id`, `team_id`, `priority`, and `cached_label_list`.
  *   `ChatMessage`: Must support `message_type` (incoming, outgoing, template), `content_type` (text, image, interactive), `private` (for internal agent notes), and attachments.
  *   `ChatLabel` / `ChatConversationLabel`: For tagging conversations.
  *   `ChatCannedResponse`: For quick replies.
  *   `ChatMacro`: For automated action sequences.

  ### Architecture Diagram
  ```mermaid
  erDiagram
    TENANT ||--o{ CHAT_INBOX : owns
    CHAT_INBOX ||--o{ CHAT_CHANNEL : contains
    TENANT ||--o{ CHAT_CONTACT : manages
    CHAT_INBOX ||--o{ CHAT_CONVERSATION : receives
    CHAT_CONTACT ||--o{ CHAT_CONVERSATION : initiates
    CHAT_CONVERSATION ||--o{ CHAT_MESSAGE : has
    CHAT_CONVERSATION ||--o{ CHAT_CONVERSATION_LABEL : tagged_with
    TENANT ||--o{ CHAT_LABEL : defines
    TENANT ||--o{ CHAT_CANNED_RESPONSE : defines
    TENANT ||--o{ CHAT_MACRO : defines
  ```

  ### Multi-Tenant Isolation
  Every table MUST include `tenant_id` and have RLS policies enabled. Lock key patterns via Redis (`ohc:lock:{tenant_id}:conversation:{id}`) must be used when routing messages to avoid race conditions in auto-assignment.

  ### AI Agent Integration
  The system must emit pub/sub events (via NATS/Redis) on `message.created`. The `Customer Assistant` AI agent will subscribe to these events, analyze context, and either draft a reply (saving a `private=true` message or setting a pending state) or reply directly if authorized.

  ### Mobile UX Flow (375px First)
  - **Unified Inbox Screen:** A consolidated list of conversations. Swiping right resolves, swiping left snoozes.
  - **Conversation Screen:** Native feel, fast scrolling. Differentiates internal notes (yellow background) from public messages (blue/gray). Input area supports quick-insert of canned responses (via `/` command) and attachments.

  ## Implementation Prompt
  **Goal:** Expand the native Rust chat system (`src/server/services/chat/`) to achieve data-model and API feature parity with Chatwoot's core omnichannel capabilities.

  **Tasks for Implementer:**
  1.  **Database Schema Expansion:** Create new PostgreSQL migrations to expand `chat_conversations`, `chat_messages`, `chat_contacts`, and add tables for `chat_labels`, `chat_conversation_labels`, `chat_canned_responses`, and `chat_macros`. Ensure `tenant_id` is present on all tables with RLS enabled.
  2.  **Rust Models & Service:** Update `src/server/services/chat/models.rs` and `src/server/services/chat/service.rs` to support the new fields and entities.
  3.  **Channel Adapters Architecture:** Implement a trait-based channel adapter architecture in Rust to support adding various channels (Web Widget first).
  4.  **API Endpoints:** Build the necessary REST/gRPC endpoints to power the Unified Inbox UI (fetching conversations, sending messages, adding internal notes, applying labels).
  5.  **Testing:** Write comprehensive unit tests for the ChatService and Playwright E2E tests for the Unified Inbox flow, simulating an owner (e.g., Maya) receiving a message and an agent drafting a reply.

  **Acceptance Criteria:**
  - The DB schema supports advanced conversation routing (assignee, status, snooze, labels).
  - Internal notes (`private` messages) can be added to conversations.
  - The codebase contains the structural foundation for adding multiple channel adapters (Web Widget first).
  - All tests pass locally and on CI.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
