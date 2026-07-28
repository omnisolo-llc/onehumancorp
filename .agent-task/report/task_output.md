issue_title: "Native Rust Omnichannel Chat: Core Data Models & Schema Design"
issue_description: |
  # Native Rust Omnichannel Chat Architecture - OHC

  ## Problem Statement
  One Human Corp (OHC) needs a native, high-performance omnichannel inbox system to power its Customer Success and Ambassador AI agents, eliminating the external dependency on Chatwoot. Small business owners (Maya, Carlos, Fatima) receive customer communications across scattered channels (Instagram DMs, WhatsApp, SMS, Web Chat) and need a unified inbox where our native AI agents can seamlessly retrieve full context, draft replies, and dispatch messages automatically. By rewriting Chatwoot's core domain model natively in Rust, OHC maintains total data sovereignty, lower latency for real-time WebSockets, zero third-party platform limitations, and deep integration with our tenant-isolated architecture.

  ## Research Report
  **Chatwoot Source Audit:**
  We audited the open-source Chatwoot Ruby on Rails models and identified the critical structural pillars of an omnichannel inbox:
  1.  **Account / Tenant**: The isolation boundary for all data.
  2.  **Inbox & Channel**: An Inbox groups conversations. A Channel (e.g., `Channel::WebWidget`, `Channel::Whatsapp`, `Channel::Api`) defines the provider-specific configurations and credentials.
  3.  **Conversation**: The stateful session between a customer (Contact) and the business, storing status (`open`, `resolved`, `snoozed`), assignee, and metadata.
  4.  **Message**: Individual chat bubbles within a Conversation. Includes `message_type` (incoming, outgoing, activity, template), `content_type` (text, image, button), attachments, and external provider message IDs for syncing.
  5.  **Contact**: The end customer communicating with the business.

  **OHC Advantage**: Chatwoot relies heavily on ActiveRecord callbacks and Sidekiq background jobs. OHC's Rust implementation will leverage asynchronous Tokio tasks, SQLx for strictly-typed and tenant-isolated PostgreSQL queries (Row Level Security), and zero-copy JSON parsing, yielding massive performance gains for real-time chat.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      INBOX ||--o{ CHANNEL : configures
      INBOX ||--o{ CONVERSATION : tracks
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains

      TENANT {
          uuid id PK
          string name
      }
      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
          boolean enable_auto_assignment
      }
      CHANNEL {
          uuid id PK
          uuid inbox_id FK
          string provider_type
          jsonb credentials
      }
      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string name
          string email
          string phone_number
      }
      CONVERSATION {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status
          timestamp snoozed_until
      }
      MESSAGE {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          uuid sender_id FK
          string content
          string message_type
          jsonb external_source_ids
      }
  ```

  ### Mobile UX Flow (375px First)
  - The architecture directly supports the **Unified Agent Feed**.
  - When an incoming `Message` is saved, an event is emitted. The Ambassador agent processes the `Conversation` history and drafts a `Message` (status: `draft`).
  - The mobile UI queries `Messages` with status `draft`. It displays a card: "Draft reply to Sarah ready."
  - Tapping "Approve" simply updates the `Message` status to `queued` and triggers the dispatch worker.

  ### AI Agent Integration Points
  - **The Ambassador (Customer Success Agent)**: Listens for DB insert events on the `MESSAGE` table where `message_type == 'incoming'`. It reads the `CONVERSATION` context and `CONTACT` history to generate a pending outbound `MESSAGE`.
  - **Memory & Context**: The AI agent queries the `MESSAGE` table for the specific `conversation_id` to build its conversational prompt context.

  ### Key Design Decisions
  - **Tenant Isolation (RLS)**: Every single table must have a `tenant_id` column to strictly enforce PostgreSQL Row Level Security (RLS) in the OHC platform.
  - **UUIDs Everywhere**: Use UUIDv7 for all primary keys to ensure chronologically sortable, globally unique identifiers suitable for distributed systems and offline-first mobile sync.
  - **Flexible Channels**: The `CHANNEL` table uses a generic `provider_type` (enum) and a `jsonb` field for `credentials`, allowing us to add WhatsApp, Instagram, or Web Widgets without schema migrations.
  - **Message Types**: Differentiate `message_type` enum natively: `Incoming`, `Outgoing`, `Draft`, `Template`, `Activity` (system notes).

  ## Implementation Prompt
  **User-Facing Outcome:** Establish the foundational PostgreSQL database schema and Rust (SQLx) data models required to support a native omnichannel inbox, replacing the legacy Chatwoot dependency.

  **CUJ & Acceptance Criteria:**
  1.  Create PostgreSQL migration files (up/down) defining the new tables: `inboxes`, `channels`, `contacts`, `conversations`, and `messages`.
  2.  Every table MUST include a `tenant_id` column with an appropriate foreign key and index to support OHC's multi-tenant Row Level Security.
  3.  Define the corresponding Rust structs in the core domain module using `sqlx` FromRow derivations.
  4.  Implement a basic repository trait and struct (`PostgresInboxRepository`) with at least the following methods:
      - `create_conversation(tenant_id, inbox_id, contact_id) -> Result<Conversation>`
      - `add_message(tenant_id, conversation_id, content, message_type) -> Result<Message>`
      - `get_conversation_messages(tenant_id, conversation_id) -> Result<Vec<Message>>`
  5.  Write unit tests using the test database pool to verify tenant isolation (ensure a query for Tenant A cannot read Tenant B's conversations). Do not implement API routes or UI in this ticket.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
