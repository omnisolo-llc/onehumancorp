issue_title: "Architecture: Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OHC requires a built-in, native omnichannel customer support and chat system. Relying on an external Chatwoot service introduces latency, security risks, split multi-tenancy, and operational overhead. We need to retire Chatwoot and build its core features natively in Rust to achieve single-platform cohesion, strict tenant isolation via row-level security, and seamless AI agent integration.

  ## Research Report
  Based on an audit of the Chatwoot source code (`https://github.com/chatwoot/chatwoot`), the core architecture revolves around:
  - **Inboxes**: The entry points for messages (e.g., Web Widget, Email, API, WhatsApp).
  - **Channels**: Implementations of specific inbox types (Channel::WebWidget, Channel::Email, etc.).
  - **Conversations**: A thread of messages between a contact and agents (human or bot), linked to an inbox.
  - **Messages**: Individual messages within a conversation, tracking sender, content, type, and status.
  - **Contacts**: The end-user communicating with the business, with attributes, location, and identifiers.
  - **Agents & Bots**: Entities that can assign themselves to conversations and reply to messages.

  For OHC to replace Chatwoot natively, we must implement these core models and their relationships in Rust, backed by PostgreSQL, ensuring strict row-level security (RLS) for our `tenant_id` pattern.

  ## Design Doc
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      TENANT ||--o{ CONVERSATION : owns
      TENANT ||--o{ MESSAGE : owns

      INBOX ||--o{ CONVERSATION : receives
      INBOX ||--o| CHANNEL_CONFIG : has

      CONTACT ||--o{ CONVERSATION : initiates
      CONTACT ||--o{ MESSAGE : sends

      CONVERSATION ||--o{ MESSAGE : contains

      AGENT ||--o{ CONVERSATION : assigned_to
      AGENT ||--o{ MESSAGE : sends
  ```

  ### Core Entities (Rust / PostgreSQL)
  1.  **Inbox**:
      -   `id` (UUID), `tenant_id` (UUID), `name` (String), `channel_type` (Enum: WebWidget, Email, API, WhatsApp), `created_at`, `updated_at`.
  2.  **Contact**:
      -   `id` (UUID), `tenant_id` (UUID), `name` (String), `email` (String, optional), `phone` (String, optional), `identifier` (String, optional), `custom_attributes` (JSONB).
  3.  **Conversation**:
      -   `id` (UUID), `tenant_id` (UUID), `inbox_id` (UUID), `contact_id` (UUID), `status` (Enum: Open, Resolved, Snoozed), `assignee_id` (UUID, optional).
  4.  **Message**:
      -   `id` (UUID), `tenant_id` (UUID), `conversation_id` (UUID), `sender_type` (Enum: Contact, Agent, Bot), `sender_id` (UUID, optional), `content` (Text), `message_type` (Enum: Incoming, Outgoing, InternalNote).

  ### Multi-Tenancy & Security
  -   All tables **MUST** have a `tenant_id` column.
  -   PostgreSQL Row Level Security (RLS) must be enabled on all tables, filtering by the current transaction's `tenant_id` (set via session variable or application-level filtering).
  -   API endpoints must strictly validate tenant access.

  ### AI Agent Integration
  -   Messages should trigger an event bus (e.g., Redis Pub/Sub or in-memory channel).
  -   AI agents (Operations, CS, Sales) can listen to these events.
  -   Agents can create draft replies (internal notes) or send direct replies (outgoing messages).
  -   Use Redis Redlock to coordinate agents and prevent concurrent replies to the same message.

  ### Mobile UX Flow (375px first)
  1.  **Work Triage View**: Conversations are integrated into the main "Work Triage" feed, ordered by priority/urgency.
  2.  **Conversation View**:
      -   Header: Contact name, status toggle, back button.
      -   Message List: Bubbles for incoming (left) and outgoing (right). Internal notes visually distinct (e.g., yellow background).
      -   Composer: Native keyboard, input field, attachment button, send button.
      -   AI suggestions appear above the composer as selectable chips.
  3.  **No horizontal scrolling**. Touch targets > 44px.

  ## Implementation Prompt
  **Role**: Implementer Agent
  **Task**: Build the foundational Rust data models and PostgreSQL schemas for the native OHC omnichannel chat system, replacing Chatwoot.
  **CUJ**:
  1. An owner (Maya) logs into OHC.
  2. A new customer (Contact) sends a message via the Web Widget (Inbox).
  3. A new Conversation and Message are created in the database.
  4. Maya sees the Conversation in her Triage feed, opens it, and sends a reply Message.

  **Acceptance Criteria**:
  - Define PostgreSQL migrations for `inboxes`, `contacts`, `conversations`, and `messages` with `tenant_id` and RLS.
  - Implement Rust structs/models for these entities using `sqlx` or `diesel` (whichever is standard in the repo).
  - Implement basic CRUD operations (create inbox, create contact, create conversation, add message).
  - Write 100% unit test coverage for the models and CRUD operations.
  - Write at least 5 Playwright E2E tests verifying the creation and display of a conversation flow in the UI (mocking the network is forbidden; use real database reads/writes).
  - Ensure all `main` branch tests pass (`bazel test //...`).

  ## Priority
  P0 (Critical)

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, chat]
assignees: []
