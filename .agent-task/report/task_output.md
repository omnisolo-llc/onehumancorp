issue_title: "Native Rust Omnichannel Inbox & Chat System: Core Data Model & Multitenancy (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OHC requires a native, high-performance omnichannel support system to replace Chatwoot, which is fully retired as an external service. Owner personas (Maya, Carlos, Priya, Leo, Fatima, Nora, Jun) need to manage customer interactions across multiple channels (Instagram DMs, Web Chat, Email, WhatsApp) within the OHC platform. They need a unified inbox that is fast, reliable, and perfectly integrated with OHC's multitenant AI workflows, without juggling external systems or dealing with confusing interfaces. The core limitation currently is the lack of a native Rust data model and business logic layer to handle this scale and complexity securely within `onehumancorp/mono`.

  ## Research Report
  Based on an audit of the `chatwoot/chatwoot` source code and OHC requirements:
  - **Chatwoot Architecture**: Heavily relies on models like `Conversation`, `Message`, `Inbox`, `Contact`, `Channel::*`, heavily utilizing JSONB for flexible attributes and relying on complex ActiveRecord callbacks.
  - **OHC Gaps**: Missing Rust structs, Protobuf definitions, database schemas (PostgreSQL), and core multitenant service logic for Inboxes, Conversations, Messages, and Contacts.
  - **Competitor Analysis**: Shopify Inbox, Wix Inbox, and Zendesk all centralize messaging into a single tenant-isolated view. High scalability requires robust real-time handling (WebSockets - to be implemented in a subsequent phase) and strict tenant isolation.
  - **Decision**: We must build a foundational Rust backend in OHC that replicates Chatwoot's core entities, strictly adhering to OHC's multitenant database design (`tenant_id` on all tables, RLS) and Zero-Trust architecture.

  ## Design Doc
  ### Architecture Diagram (Mermaid)
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      TENANT ||--o{ CONVERSATION : owns
      TENANT ||--o{ MESSAGE : owns

      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates_in
      CONVERSATION ||--o{ MESSAGE : has

      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
          string channel_type
          jsonb config
          boolean is_active
          timestamp created_at
          timestamp updated_at
      }

      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string name
          string email
          string phone_number
          string avatar_url
          jsonb custom_attributes
          timestamp created_at
          timestamp updated_at
      }

      CONVERSATION {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          uuid assignee_id FK "nullable"
          string status "open, resolved, snoozed, pending"
          string priority "low, medium, high, urgent"
          jsonb custom_attributes
          timestamp created_at
          timestamp updated_at
          timestamp last_activity_at
      }

      MESSAGE {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          uuid sender_id FK "nullable, if agent/system"
          string sender_type "contact, agent, system, bot"
          text content
          string content_type "text, html, markdown"
          jsonb attachments
          string status "sent, delivered, read, failed"
          timestamp created_at
      }
  ```

  ### Mobile UX Flow & AI Integration
  - **375px Flow**: (Future Frontend PR) The unified inbox list shows Conversations grouped by status. Tapping a conversation opens the chat view.
  - **AI Agent Integration**: Agents (Customer Assistant) will subscribe to new Messages. They will read the Conversation context, draft replies, and update the Conversation status or add internal Messages (Notes) for the owner. The data model must support `sender_type: "bot"` and allow agents to draft messages pending owner approval.

  ### Key Design Decisions
  1. **Strict Multitenancy**: Every table MUST have a `tenant_id` UUID column. PostgreSQL Row-Level Security (RLS) MUST be enabled.
  2. **JSONB Extensibility**: Use JSONB for channel-specific configs in `Inbox` and custom attributes in `Contact` and `Conversation` to avoid schema bloat as new channels are added.
  3. **UUIDv7**: Primary keys should be UUIDv7 for time-based sorting and better database performance.
  4. **Status Enums**: Use PostgreSQL Enums for `conversation_status` and `message_status` to ensure data integrity.

  ## Implementation Prompt
  Implement the core database schema and Protobuf definitions for the new native Rust Omnichannel Chat system.

  **Acceptance Criteria:**
  1.  **Database Migrations (PostgreSQL)**: Create SQL migrations for `inboxes`, `contacts`, `conversations`, and `messages` tables.
      *   Ensure `tenant_id` is present on ALL tables.
      *   Enable Row Level Security (RLS) policies enforcing tenant isolation (`tenant_id = current_setting('app.current_tenant_id')::uuid`).
      *   Include appropriate indexes (e.g., on `tenant_id`, `inbox_id`, `contact_id`, `conversation_id`, `created_at`, `status`).
  2.  **Protobuf Definitions**: Create `.proto` files defining the gRPC service contracts for CRUD operations on these entities.
      *   Define messages for `Inbox`, `Contact`, `Conversation`, `Message`.
      *   Define basic RPC methods (e.g., `CreateConversation`, `ListConversations`, `SendMessage`, `ListMessages`).
  3.  **Rust Structs & Models (Optional for this initial PR, depending on scope limit)**: If scope permits, scaffold the Rust `sqlx` models and basic repository traits for these entities.
  4.  **Testing**: Write unit tests for the SQL migrations (testing constraints and RLS) and verify protobuf generation passes (`bazel build //...`).

  **Context:** Remember, this replaces Chatwoot. We are building the foundational data layer. Focus strictly on correct schema design, multitenant isolation, and clear gRPC contracts.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
