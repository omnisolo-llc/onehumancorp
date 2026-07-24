issue_title: "Implement Native Rust Omnichannel Inbox & Messaging Core"
issue_description: |
  # Title
  Implement Native Rust Omnichannel Inbox & Messaging Core (Chatwoot Replacement)

  # Problem Statement
  OHC currently relies on or needs an omnichannel communication hub to aggregate DMs, emails, and webchats into a unified owner inbox. To guarantee data sovereignty, multi-tenant row-level isolation, and strict Zero-Trust security without external dependencies, we must retire any reliance on external services like Chatwoot. We need a native, high-performance omnichannel inbox built in Rust within `onehumancorp/mono` that achieves feature parity with Chatwoot's core messaging flow.

  # Research Report
  Based on an audit of the Chatwoot source code (`https://github.com/chatwoot/chatwoot`), the core architecture relies on:
  - **Inboxes**: The entry point for messages, linked to specific Channels (e.g., Web, API, Email, Facebook).
  - **Conversations**: A thread of messages between a Contact and an Inbox/Agent.
  - **Messages**: The individual payloads (text, attachments, structured data) within a Conversation.
  - **Contacts**: The unified identity of the customer across channels.

  Chatwoot uses PostgreSQL and relies on a traditional MVC structure with background jobs for webhooks and SLA enforcement. For OHC, we will adopt these domain concepts but implement them in a high-concurrency, asynchronous Rust backend (using Axum and sqlx) with strict multi-tenant row-level security (`tenant_id` on every table).

  # Design Doc
  ## Architecture diagram (Mermaid.js)
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains

      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
          string channel_type
          jsonb channel_config
      }

      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string name
          string identifier
          jsonb attributes
      }

      CONVERSATION {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status
      }

      MESSAGE {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          uuid sender_id FK
          string sender_type
          text content
          string message_type
          jsonb additional_attributes
      }
  ```

  ## UI wireframes or screen flow description (375px first)
  1. **Mobile Inbox List**: A clean, touch-friendly list (44x44px targets) of unified conversations, badged with unread counts.
  2. **Conversation Thread**: WhatsApp-style message bubbles, with a unified composer at the bottom. Support native keyboard handling.
  3. **Contact Details Sheet**: A bottom sheet that pulls up context on the customer (past orders, CRM data, tags) directly in the conversation view.

  ## Mobile UX flow
  - User taps the "Inbox" tab from the OHC unified dashboard.
  - Loading state uses skeleton screens, avoiding fake data.
  - List of active conversations ordered by recent activity.
  - Tapping a conversation transitions to the chat view.
  - If a network failure occurs, the UI displays a subtle offline banner while optimistic UI queues messages locally for retry.

  ## AI agent integration points
  - **Work Triage Agent**: Automatically assigns tags or priority flags to incoming messages based on intent (e.g., "urgent inquiry", "support request").
  - **Customer Assistant Agent**: Drafts potential replies presented as translucent suggestion chips above the composer.
  - **Auto-responder**: Uses tenant context to automatically resolve basic inquiries (e.g., store hours) without owner intervention.

  ## Key design decisions and why
  - **Rust + SQLx**: Ensures high performance and type-safe database queries.
  - **Multi-Tenant RLS**: `tenant_id` must be physically present and enforced via RLS on every table (`messages`, `conversations`, `inboxes`, `contacts`) to guarantee isolation.
  - **Event-Driven Architecture**: Use NATS or Postgres SKIP LOCKED for a job queue to dispatch webhooks, notify the AI agent of new messages, and handle push notifications.
  - **Translucent Glass UI**: Follow the OHC design system with Apple/Ubiquiti-style materials for message bubbles and floating suggestion chips.

  # Implementation Prompt
  Implement the core native Rust omnichannel messaging domain.
  1. Define the SQL migrations for `inboxes`, `contacts`, `conversations`, and `messages`, ensuring `tenant_id` is present on every table with Row-Level Security enabled.
  2. Implement the Rust service layer (e.g., `src/server/integrations/chat` or a new `messaging` crate) with endpoints to:
     - Create an inbox
     - Create a contact
     - Start a conversation
     - Send a message to a conversation
     - Fetch conversations and messages for an inbox
  3. Build the Flutter/Web UI components for the Inbox list and Conversation view, adhering strictly to the mobile-first (375px) constraint and translucent glass design system.
  4. Ensure 100% unit test coverage for the Rust backend and comprehensive Playwright E2E tests for the chat flow. Do NOT mock the API in E2E tests.

  # Priority
  P0

  # Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
