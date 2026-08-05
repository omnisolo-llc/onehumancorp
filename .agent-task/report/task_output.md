issue_title: "[Native Chat] Core Rust Omnichannel Inbox & Conversation Architecture"
issue_description: |
  # Native C******* Replacement: Omnichannel Inbox & Conversation

  ## Problem Statement
  OHC needs to fully retire its dependency on the external C******* service and replace it with a high-performance, native omnichannel chat system written in Rust, directly integrated into the `onehumancorp/mono` repository. Small business owners (like Maya the Baker or Carlos the Handyman) need a unified inbox that brings together SMS, WhatsApp, Instagram DMs, Email, and Web Chat without relying on third-party SaaS chat products. It must be built around OHC's core multi-tenant Row Level Security (RLS) architecture.

  ## Research Report
  Based on an audit of the `c*******/c*******` source code (`app/models/*`), C*******'s core architecture centers around the following key entities:
  - `Account` (maps to OHC `Tenant`)
  - `Inbox` (the unified endpoint for a specific channel)
  - `Channel::*` (the specific adapters like `Channel::Api`, `Channel::WebWidget`, `Channel::Sms`)
  - `Conversation` (the thread of messages between a contact and the tenant)
  - `Message` (the individual text/attachment payload)
  - `Contact` (the external customer)
  - `ContactInbox` (the linking table that maps a Contact's specific identity, e.g., phone number, to a specific Inbox)

  To achieve parity natively in Rust within OHC, we must design a matching, but modernized, schema and service layer. We will use Rust (with Axum/Tonic and SeaORM/Diesel or sqlx) to implement this. The architecture must enforce strict multi-tenant isolation (Zero Trust) via `tenant_id` on every table and leverage PostgreSQL RLS.

  ## Design Doc
  ### Data Model & Invariants
  1.  **`inboxes` table**:
      - `id` (UUID, PK)
      - `tenant_id` (UUID, FK, required for RLS)
      - `name` (String, e.g., "Main SMS Support")
      - `channel_type` (Enum: `WebWidget`, `API`, `SMS`, `Email`, `WhatsApp`)
      - `settings` (JSONB)
  2.  **`contacts` table (Existing in OHC, needs integration)**:
      - `id` (UUID)
      - `tenant_id` (UUID)
      - `name`, `email`, `phone_number`, etc.
  3.  **`contact_inboxes` table**:
      - `id` (UUID, PK)
      - `tenant_id` (UUID)
      - `contact_id` (UUID, FK)
      - `inbox_id` (UUID, FK)
      - `source_id` (String, e.g., the specific phone number or external ID for this channel)
  4.  **`conversations` table**:
      - `id` (UUID, PK)
      - `tenant_id` (UUID)
      - `inbox_id` (UUID, FK)
      - `contact_id` (UUID, FK)
      - `status` (Enum: `Open`, `Resolved`, `Pending`, `Snoozed`)
      - `assignee_id` (UUID, FK to Users)
  5.  **`messages` table**:
      - `id` (UUID, PK)
      - `tenant_id` (UUID)
      - `conversation_id` (UUID, FK)
      - `contact_id` (UUID, nullable, if sent by customer)
      - `user_id` (UUID, nullable, if sent by agent)
      - `content` (Text)
      - `message_type` (Enum: `Incoming`, `Outgoing`, `Template`, `Activity`)
      - `content_type` (Enum: `Text`, `Form`, `Article`, `Cards`)
      - `private` (Boolean, internal note vs public message)

  ### Architecture Overview
  - **Service Layer**: A new Rust microservice (`chat_engine`) or crate within the monolith that exposes gRPC/REST APIs for creating inboxes, sending messages, and querying conversations.
  - **Real-time Sync**: Will require WebSocket integrations or Server-Sent Events (SSE) to push new `Message` events to the Flutter frontend instantly. (To be detailed in a separate issue).
  - **AI Integration**: The `chat_engine` will emit events (`MessageCreated`) to the PostgreSQL `SKIP LOCKED` job queue. The OHC Customer Assistant Agent will dequeue these, draft replies, and use internal APIs to insert `draft` or `outgoing` messages back into the conversation.

  ### ER Diagram
  ```mermaid
  erDiagram
    TENANT ||--o{ INBOX : has
    TENANT ||--o{ CONTACT : has
    TENANT ||--o{ CONVERSATION : has
    TENANT ||--o{ MESSAGE : has

    INBOX ||--o{ CONTACT_INBOX : "receives from"
    CONTACT ||--o{ CONTACT_INBOX : "connects to"

    INBOX ||--o{ CONVERSATION : groups
    CONTACT ||--o{ CONVERSATION : initiates

    CONVERSATION ||--o{ MESSAGE : contains

    INBOX {
      UUID id PK
      UUID tenant_id FK
      String name
      Enum channel_type
    }
    CONTACT {
      UUID id PK
      UUID tenant_id FK
      String name
    }
    CONTACT_INBOX {
      UUID id PK
      UUID tenant_id FK
      UUID contact_id FK
      UUID inbox_id FK
      String source_id
    }
    CONVERSATION {
      UUID id PK
      UUID tenant_id FK
      UUID inbox_id FK
      UUID contact_id FK
      Enum status
    }
    MESSAGE {
      UUID id PK
      UUID tenant_id FK
      UUID conversation_id FK
      Text content
    }
  ```

  ### Mobile UX Flow (375px first)
  1.  **Unified Inbox View**: A list view showing active `Conversations` sorted by recent activity. Each row shows the Contact name, a snippet of the latest message, a channel icon (SMS, Web, etc.), and an unread indicator.
  2.  **Conversation Detail View**: A standard chat UI. Bottom input bar (native keyboard). Messages bubble up. AI drafted replies appear as distinct, actionable cards above the input bar ("Approve & Send", "Edit").

  ## Implementation Prompt
  **Goal:** Implement the core database schema (migrations) and foundational Rust backend models/CRUD operations for the native OHC Omnichannel Inbox.

  **Acceptance Criteria:**
  1.  Create PostgreSQL up/down migrations defining `inboxes`, `contact_inboxes`, `conversations`, and `messages`. All tables MUST include `tenant_id` and enforce RLS policies restricting access to the current tenant.
  2.  Implement the corresponding Rust data models (using `sqlx` or the repo's chosen ORM).
  3.  Implement basic repository/service methods in Rust to:
      - Create an inbox for a tenant.
      - Start a new conversation for a contact in an inbox.
      - Add a message (incoming or outgoing) to a conversation.
      - Query open conversations for a tenant's inbox.
  4.  Provide 100% unit test coverage for the Rust models and repository methods, ensuring cross-tenant data leakage is impossible (test with multiple tenant IDs).

  **Note:** Do not implement the WebSocket layer or the external channel adapters (Twilio/WhatsApp) in this task. Focus strictly on the core internal unified data model and CRUD operations.

  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
