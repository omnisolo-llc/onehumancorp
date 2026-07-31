issue_title: "Native Rust Omnichannel Chat: Core Data Models & Schema Design"
issue_description: |
  ## Problem Statement
  OHC is retiring the external Chatwoot dependency to bring the omnichannel chat capability directly into our native Rust/PostgreSQL platform (`onehumancorp/mono`). To achieve this, we need to architect and implement the core data models and database schemas for our native Rust Omnichannel Chat system. The system needs to support multiple channels (WhatsApp, Web Widget, etc.), multi-tenant isolation (via Row Level Security), and agent assignment, matching the feature set of Chatwoot but built natively in our stack.

  ## Research Report
  Our research into Chatwoot's architecture (`https://github.com/chatwoot/chatwoot`) reveals several key entities that power an omnichannel inbox system:
  1.  **Inboxes**: Represent a specific communication channel (e.g., a specific WhatsApp number or a Web Widget on a specific domain).
  2.  **Conversations**: Represent a thread of messages between a contact and the business/agent. They belong to an Inbox and a Contact.
  3.  **Messages**: Individual messages within a Conversation.
  4.  **Contacts**: The customers interacting with the business.
  5.  **Channel Adapters**: Specific channel configurations (e.g., WhatsApp configuration, Web Widget configuration) that an Inbox wraps.

  Our implementation must adopt these concepts but enforce strict multi-tenant isolation using `tenant_id` on every table and Row Level Security (RLS) policies.

  ## Design Doc

  ### Architecture
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      TENANT ||--o{ CONVERSATION : owns
      TENANT ||--o{ MESSAGE : owns

      INBOX ||--o{ CONVERSATION : contains
      INBOX ||--|{ CHANNEL_WHATSAPP : "polymorphic config"
      INBOX ||--|{ CHANNEL_WEB_WIDGET : "polymorphic config"

      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains

      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
          string channel_type "whatsapp | web_widget"
          uuid channel_id "polymorphic reference"
          boolean is_active
      }

      CONVERSATION {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          uuid assignee_id FK "optional agent"
          string status "open | resolved | snoozed"
          datetime last_activity_at
      }

      MESSAGE {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          uuid sender_id "contact or agent"
          string sender_type "contact | agent | system"
          text content
          jsonb metadata "attachments, provider specific IDs"
          datetime created_at
      }

      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string name
          string email
          string phone_number
          jsonb custom_attributes
      }
  ```

  ### Multi-Tenancy & Zero Trust
  - Every table **MUST** include a `tenant_id` UUID column.
  - Every table **MUST** have PostgreSQL Row Level Security (RLS) enabled.
  - RLS policies must restrict all `SELECT`, `INSERT`, `UPDATE`, and `DELETE` operations to `tenant_id = current_setting('app.current_tenant_id')::uuid`.

  ### Mobile-First UX Flow (375px)
  While this task focuses on the backend schema, the data models must support a high-performance mobile inbox UI:
  - **Inbox List View**: Efficiently querying `CONVERSATION` ordered by `last_activity_at` with unread counts and contact names.
  - **Chat View**: Streaming `MESSAGE` rows for a specific `CONVERSATION` over WebSockets.

  ## Implementation Prompt
  Implement the database migrations and SeaORM (or equivalent Rust ORM) entity definitions for the core native omnichannel chat system.

  **Acceptance Criteria:**
  1.  Create PostgreSQL migration files (SQL) to define the `inboxes`, `contacts`, `conversations`, and `messages` tables.
  2.  Ensure every table has a `tenant_id` column.
  3.  Implement Row Level Security (RLS) policies for every new table, restricting access based on `tenant_id`.
  4.  Create corresponding Rust entity structs using the project's standard ORM (e.g., SeaORM) in the `src/server/integrations/chat` module (or appropriate models module).
  5.  Write unit tests to verify that the RLS policies correctly isolate data between different `tenant_id`s. (e.g., A query as Tenant A should not see Tenant B's conversations).
  6.  Ensure all schema changes and queries adhere to high-performance indexing strategies (e.g., indexing `(tenant_id, inbox_id)`, `(tenant_id, contact_id)`, `(tenant_id, conversation_id, created_at)`).

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
