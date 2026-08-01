issue_title: "Native Rust Omnichannel Chat Engine Architecture (Legacy Chat Replacement)"
issue_description: |
  # Problem Statement
  OHC is retiring the external Legacy External Chat dependency in favor of a native, high-performance omnichannel chat system written in Rust (`src/server/integrations/chat`). We need a unified inbox that brings together WhatsApp, Instagram, Email, Web Widget, and Meta channels without relying on a third-party service. This must feel invisible to the non-technical owner (Maya, Carlos) who just wants to see all customer communication in one place.

  # Research Report
  Based on a deep audit of the `Legacy External Chat` source code (`db/schema.rb`, data models, and inbox architecture), a scalable omnichannel system requires the following core entities:
  1. **Accounts/Tenants**: Strict isolation for multi-tenancy.
  2. **Inboxes**: A unified container for channels.
  3. **Channels**: Adapters for WhatsApp, Web Widgets, API, Email, Facebook, Instagram, Twitter, etc.
  4. **Conversations**: A thread of messages linking a Contact (customer) to an Inbox.
  5. **Messages**: The individual payloads (text, attachments, etc.) within a Conversation.
  6. **Contacts**: The unified identity of a customer across multiple channels.

  Competitor systems (Shopify Inbox, Wix Chat) offer a seamless mobile-first unified inbox. To replicate Legacy External Chat's functionality natively in Rust, OHC requires a WebSocket-based real-time event system, background job queues for processing webhooks, and strict Row Level Security (RLS) in PostgreSQL.

  # Design Doc

  ## Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      TENANT ||--o{ CONTACT : has
      INBOX ||--o{ CHANNEL : configures
      CHANNEL ||--o{ CONVERSATION : sources
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      MESSAGE }|--|| CONVERSATION : belongs_to
  ```

  ## Mobile UX Flow (375px first)
  1. **Unified Feed**: The owner opens the OHC app. The first screen (375px) shows a unified list of active Conversations. Unread messages have a distinct Translucent Glass unread badge.
  2. **Conversation View**: Tapping a conversation opens a chat UI. Native mobile keyboard support is enabled. The UI shows the channel icon (e.g., WhatsApp, Web Widget) so the owner knows how the customer is communicating.
  3. **AI Agent Drafts**: Pending drafts from the Customer Service AI agent are displayed inline with a "Approve & Send" button.
  4. **Actions**: A bottom drawer provides quick actions: "Create Quote", "Request Payment", "Schedule Booking".

  ## AI Agent Integration Points
  - **Work Triage Agent**: Evaluates incoming webhook payloads and groups them into the Unified Feed.
  - **Customer & Relationship Agent**: Subscribes to the `conversation.created` and `message.created` events. Drafts replies based on tenant-scoped memory and previous contact history.
  - **Background Jobs**: PostgreSQL `SKIP LOCKED` job queue handles Meta/WhatsApp webhook verification and delivery retries.

  ## Key Design Decisions
  1. **Rust Native**: High concurrency WebSocket server (e.g. using `tokio` and `axum`) for the web widget real-time messaging.
  2. **PostgreSQL RLS**: Every table (`inboxes`, `conversations`, `messages`, `contacts`) MUST include `tenant_id` and enforce Row Level Security.
  3. **Idempotency**: All webhook handlers must process events idempotently using Redis distributed locks (`ohc:lock:{tenant_id}:webhook:{event_id}`).

  # Implementation Prompt
  **To the Implementer:**
  You are tasked with implementing the Core Data Model and Rust API for the Native Omnichannel Chat Engine, replacing Legacy External Chat.
  1. Create the PostgreSQL migration schemas for `inboxes`, `channels`, `contacts`, `conversations`, and `messages`. Apply strict `tenant_id` RLS to every table.
  2. Implement the Rust REST API endpoints for fetching conversations and sending messages in `src/server/integrations/chat/`.
  3. Implement a basic WebSocket endpoint in Rust for real-time web widget connections.
  4. The API must conform to the new `Tenant` isolation standards.
  5. Create Playwright E2E tests for the 375px mobile inbox flow showing a mock customer sending a message and the owner viewing it. Ensure no mock data exists in the UI; test data must be seeded through the API.

  # Priority
  P0

  # Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
