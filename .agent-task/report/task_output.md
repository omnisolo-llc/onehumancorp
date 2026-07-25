issue_title: "Native Rust Omnichannel Inbox Data Model & Multi-Tenancy Architecture"
issue_description: |
  # Native Rust Omnichannel Inbox Data Model & Multi-Tenancy Architecture

  ## Problem Statement
  Currently, OHC lacks a unified, native omnichannel inbox. We are retiring the external cw_platform dependency and need to replace it with a native, highly performant Rust implementation that supports multi-tenancy seamlessly. Small business owners like Maya (baker) or Carlos (handyman) receive messages across Instagram, WhatsApp, SMS, and Email. Managing these manually is prone to errors, missing messages, and slow response times. A unified system needs to aggregate these conversations securely per tenant, providing a solid foundation for our AI agents (like The Ambassador) to read context and draft replies.

  ## Research Report
  **Findings & cw_platform_Analysis:**
  - Audited the open-source cw_platform repository (`https://github.com/cw_platform/cw_platform/tree/develop/app/models`).
  - cw_platform's core data models revolve around: `Account` (Tenant), `Inbox`, `Channel` (adapters like Twilio, WhatsApp, Facebook), `Conversation`, `Message`, `Contact`, and `User`.
  - cw_platform utilizes extensive polymorphic associations for channels (e.g., `channelable_type` and `channelable_id` on the Inbox model).
  - Multi-tenancy in cw_platform is typically handled at the application level, scoping queries by `account_id`.

  **OHC Native Rust Target Architecture:**
  - We will implement a native Rust microservice/module mimicking the core capabilities of cw_platform but strictly enforcing row-level multi-tenant isolation via our existing PostgreSQL Row Level Security (RLS) standards.
  - Core entities needed: `inboxes`, `channels` (with JSONB config for specific channel metadata or specific tables), `conversations`, `messages`, and `contacts`.
  - We must ensure real-time capabilities via WebSocket or Server-Sent Events (SSE) will be supported by this data model (though the pub/sub implementation is a subsequent step).
  - The model must support our AI "Teammate" agents seamlessly querying conversation history.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      TENANT ||--o{ CONTACT : has
      INBOX ||--o{ CHANNEL : configures
      INBOX ||--o{ CONVERSATION : contains
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
          boolean is_active
      }
      CHANNEL {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          string channel_type "e.g., whatsapp, instagram, email"
          jsonb credentials
      }
      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string name
          string email
          string phone_number
          string avatar_url
      }
      CONVERSATION {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status "open, resolved, snoozed"
          timestamp last_activity_at
      }
      MESSAGE {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          string content
          string message_type "incoming, outgoing, internal_note"
          uuid sender_id "nullable, FK to user/agent if outgoing"
          jsonb attachments
          timestamp created_at
      }
  ```

  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC_Gateway as Omnichannel Gateway
      participant DB as DB (Conversations & Messages)
      participant AI as The Ambassador (AI Agent)
      participant UI as OHC Mobile App

      Customer->>OHC_Gateway: Send Message (e.g., WhatsApp)
      OHC_Gateway->>DB: Insert Message, Update Conversation
      DB-->>OHC_Gateway: Message Saved
      OHC_Gateway->>AI: Trigger "New Message" Event
      AI->>DB: Query Conversation History
      DB-->>AI: Return Context
      AI->>DB: Insert Draft Reply (status: "draft")
      DB-->>AI: Draft Saved
      UI->>DB: Fetch "Action Required" Feed
      DB-->>UI: Return Draft Reply
      UI->>UI: Owner Taps "Approve Reply"
      UI->>OHC_Gateway: Dispatch Reply
      OHC_Gateway->>Customer: Deliver Message
  ```

  ### Mobile UX Flow (375px First)
  - *This specific task focuses on the Backend Data Model, but here is the UX context:*
  - The mobile feed will display a unified list of `Conversations`.
  - Tapping a conversation fetches the `Messages` ordered by `created_at`.
  - The UI must render different message types (incoming customer, outgoing owner, AI drafted, internal note).

  ### AI Agent Integration Points
  - **The Ambassador Agent:** Will query the `conversations` and `messages` tables (filtered by `tenant_id` and `contact_id`) to build RAG context for generating drafted replies.
  - The Agent will insert draft messages with a specific `status` (e.g., `draft`) into the `messages` table, which the mobile client will surface as "Action Required".

  ### Key Design Decisions
  - **Row Level Security (RLS):** Every single table (`inboxes`, `channels`, `conversations`, `messages`, `contacts`) MUST have a `tenant_id` column and have PostgreSQL RLS enabled to guarantee cross-tenant isolation.
  - **Native Rust:** Implemented using our standard stack (SQLx/Diesel, depending on the repo standard) within the `src/server` directory.
  - **Channel Extensibility:** Instead of deep polymorphic SQL relations for channels, we use a single `channels` table with a `channel_type` enum and a `credentials` JSONB column. This simplifies the Rust schema and allows easy addition of new channels (Twilio, Line, etc.) without schema migrations.

  ## Implementation Prompt
  **User-Facing Outcome:** The foundational database schema and Rust entity models are established to support a native, highly-scalable unified inbox. This replaces external dependencies and guarantees strict multi-tenant data isolation.
  **CUJ & Acceptance Criteria:**
  1. Create PostgreSQL migration scripts (up and down) defining the tables: `inboxes`, `channels`, `contacts`, `conversations`, and `messages`.
  2. Ensure every table includes a `tenant_id` column and explicit Row Level Security (RLS) policies enforcing tenant isolation.
  3. Implement the corresponding Rust struct models and SQLx/Diesel repository traits in `src/server/db/` or the appropriate domain module.
  4. Write comprehensive unit tests verifying that CRUD operations work correctly and that RLS prevents accessing data across different `tenant_id`s.
  5. *Do NOT implement the REST/gRPC API layer or WebSocket handlers in this step.* Focus purely on the data model, migrations, and repository access layer.

  **Estimated Scope:** Medium
  **Priority:** P0

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
