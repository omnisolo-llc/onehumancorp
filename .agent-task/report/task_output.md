issue_title: "Native Rust Omnichannel Chat: Core Data Models & Infrastructure Design"
issue_description: |
  ## Native Rust Omnichannel Chat: Core Data Models & Infrastructure Design

  **Problem Statement:**
  OneHumanCorp (OHC) historically relied on Chatwoot as an external dependency for its omnichannel chat and customer support functionality. To guarantee the "OneHumanCorp Promise"—that an owner interacts with a deeply integrated, single assistant—and to meet performance, security, and mobile-first requirements, Chatwoot is being 100% retired. We need a native Rust implementation of Chatwoot's core omnichannel architecture built directly into the OHC monolith. This foundational task designs the data models, invariants, and overall system architecture for the native Rust omnichannel engine.

  **Research Report:**
  I audited the Chatwoot Ruby on Rails codebase to reverse-engineer its core data architecture. The critical components of Chatwoot's data model are:
  -   **Inboxes**: Represent a specific channel endpoint (e.g., a specific WhatsApp number, a Facebook Page, a Web Widget).
  -   **Contacts**: The end-user communicating with the business.
  -   **Conversations**: A unified thread of communication between a Contact and the Business (via an Inbox).
  -   **Messages**: The individual atomic units of communication within a Conversation.
  -   **Channels**: Polymorphic associations linking an Inbox to its specific underlying provider configuration (e.g., `channel_whatsapp`, `channel_web_widget`).

  This data architecture needs to be replicated natively in Rust within the `onehumancorp/mono` repository, utilizing PostgreSQL with row-level security (RLS) for tenant isolation, and designed for high-concurrency WebSocket real-time updates.

  **Design Doc (Architecture):**

  1.  **Data Model (PostgreSQL & SQLx):**
      -   All tables must enforce strict multi-tenancy using `tenant_id` and PostgreSQL Row-Level Security (RLS).
      -   **`inboxes` table:**
          -   `id` (UUID, primary key)
          -   `tenant_id` (UUID, indexed, RLS)
          -   `name` (String)
          -   `channel_type` (Enum: WebWidget, WhatsApp, SMS, Email, etc.)
          -   `channel_id` (UUID, foreign key to specific channel config tables)
      -   **`contacts` table:**
          -   `id` (UUID, primary key)
          -   `tenant_id` (UUID, indexed, RLS)
          -   `name` (String, optional)
          -   `email` (String, optional)
          -   `phone_number` (String, optional)
          -   `identifier` (String, unique per tenant, for external ID linking)
      -   **`conversations` table:**
          -   `id` (UUID, primary key)
          -   `tenant_id` (UUID, indexed, RLS)
          -   `inbox_id` (UUID, foreign key to `inboxes`)
          -   `contact_id` (UUID, foreign key to `contacts`)
          -   `status` (Enum: Open, Resolved, Pending, Snoozed)
          -   `assignee_id` (UUID, optional, foreign key to system users/agents)
      -   **`messages` table:**
          -   `id` (UUID, primary key)
          -   `tenant_id` (UUID, indexed, RLS)
          -   `conversation_id` (UUID, foreign key to `conversations`)
          -   `content` (Text)
          -   `message_type` (Enum: Incoming, Outgoing, Template, Activity)
          -   `sender_type` (Enum: Contact, User, AgentBot)
          -   `sender_id` (UUID, polymorphic ID)

  2.  **Architecture Diagram (Mermaid):**
      ```mermaid
      erDiagram
          TENANT ||--o{ INBOX : has
          TENANT ||--o{ CONTACT : has
          TENANT ||--o{ CONVERSATION : has
          INBOX ||--o{ CONVERSATION : routes
          CONTACT ||--o{ CONVERSATION : participates
          CONVERSATION ||--o{ MESSAGE : contains

          INBOX {
              uuid id PK
              uuid tenant_id FK
              string name
              enum channel_type
              uuid channel_id
          }
          CONTACT {
              uuid id PK
              uuid tenant_id FK
              string name
              string email
              string phone
          }
          CONVERSATION {
              uuid id PK
              uuid tenant_id FK
              uuid inbox_id FK
              uuid contact_id FK
              enum status
          }
          MESSAGE {
              uuid id PK
              uuid tenant_id FK
              uuid conversation_id FK
              text content
              enum message_type
              enum sender_type
              uuid sender_id
          }
      ```

  3.  **AI Integration & Mobile Flow:**
      -   **AI Triage:** New incoming messages trigger the `Operations Assistant` to evaluate priority and context.
      -   **Mobile-First UX:** The primary interface for these models will be the OHC Mobile App. A unified "Inbox" view (375px wide) must efficiently load `Conversations` sorted by `last_activity_at`, with unread badges derived from `Messages` state.
      -   **Real-time:** The Rust backend will need to push invalidation events or partial updates via WebSocket when new `messages` are inserted.

  **Implementation Prompt:**
  Implement the foundational PostgreSQL schema and Rust database models for the Native Omnichannel Chat system.
  1. Create a new Rust module (`src/server/ohc/chat` or similar) for the chat domain.
  2. Write SQLx migrations to create the `inboxes`, `contacts`, `conversations`, and `messages` tables. Ensure `tenant_id` is present on all tables and configure Row-Level Security (RLS) policies to enforce tenant isolation.
  3. Define the corresponding Rust `struct` models representing these entities.
  4. Implement basic CRUD operations for these models using `sqlx`, ensuring all queries implicitly or explicitly enforce `tenant_id` filtering.
  5. Write comprehensive unit tests for the data access layer, specifically testing RLS and tenant boundary enforcement.
  Acceptance Criteria: Migrations apply cleanly, Rust models compile, and tests prove that a query for Tenant A cannot access data for Tenant B.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
