issue_title: "Native Rust Omnichannel Chat: Core Data Model & Multitenancy (Chatwoot Migration)"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) currently relies on a third-party, non-native omnichannel integration (Chatwoot). This breaks our "Zero Trust, Native Rust" architectural goal, introduces latency, and fractures our multitenancy model. For non-technical operators like Maya (the home baker) and Carlos (the field service owner), customer inquiries from Instagram, WhatsApp, and Web Widgets are the lifeblood of their business. They need an integrated inbox where they can instantly see customer context, draft replies via AI agents, and send quotes directly within the chat stream.

  The gap: We need a native Rust-based omnichannel chat architecture directly integrated into OHC's multitenant datastore, ensuring sub-millisecond local reads, reliable real-time WebSockets, and seamless integration with OHC AI agents.

  ## Research Report & Architectural Audit
  After reviewing the Chatwoot source code (`https://github.com/chatwoot/chatwoot`), the core architecture revolves around the following hierarchy:
  1.  **Account (Tenant)**: The isolation boundary.
  2.  **Inbox**: A conceptual bucket for incoming conversations. Linked to a specific "Channel".
  3.  **Channel (e.g., WebWidget, API, WhatsApp, Instagram)**: The source connector configuration.
  4.  **Contact**: The customer communicating with the business.
  5.  **Conversation**: The ongoing thread between a Contact and an Inbox (Agent/Business).
  6.  **Message**: Individual pieces of communication within a Conversation.

  **Competitive Analysis**:
  *   **Shopify Inbox / Wix Inbox**: Deeply tied to the storefront. They blur the line between a chat message and a commerce event (e.g., "Add to Cart" appears as a system message in the chat). OHC must support this natively.
  *   **Chatwoot (Legacy)**: Excellent data model for pure support, but disconnected from OHC's transactional context (quotes, deposits, bookings).

  **Our Goal**: Implement a high-performance, multitenant equivalent in Rust using our existing PostgreSQL backend and Rust-based service architecture, allowing AI agents to seamlessly observe and interact with `Conversation` events.

  ## Design Doc

  ### Architecture Diagram (Mermaid)
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      TENANT ||--o{ CONTACT : has
      INBOX ||--|| CHANNEL_WEB_WIDGET : configuration
      INBOX ||--|| CHANNEL_API : configuration
      INBOX ||--o{ CONVERSATION : routes
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      CONVERSATION }o--o| AGENT : assigned_to

      TENANT {
          uuid id PK
          string name
      }
      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
          string channel_type
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
          string status
          timestamp last_activity_at
      }
      MESSAGE {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          uuid sender_id
          string sender_type
          string content
          string message_type
      }
  ```

  ### Core Architectural Decisions
  1.  **Strict Multitenancy**: Every table MUST have `tenant_id` and Row-Level Security (RLS) policies applied. No cross-tenant data leakage is permissible.
  2.  **Native Rust Implementation**: The chat domain will be built as a new module within our Rust backend (`onehumancorp/mono`), migrating away from external APIs.
  3.  **Polymorphic Channels**: `inboxes` will have a `channel_type` (e.g., `Channel::WebWidget`, `Channel::Api`) and point to specific configuration tables (or JSONB structures) for that channel.
  4.  **Event-Driven AI Integration**: Every new `MESSAGE` created triggers an asynchronous event (via PostgreSQL `SKIP LOCKED` job queue or internal event bus) that the "Customer & Relationship Assistant" AI department can consume to draft replies or extract intent.

  ### Mobile UX Flow (375px First)
  1.  **Work Triage View**: The operator (Maya) opens the app. A unified "Inbox" card shows "3 New Messages (2 Instagram, 1 Web)".
  2.  **Conversation View**: Tapping a conversation opens a standard chat UI. Crucially, the top of the screen shows the *Customer Context* (e.g., "Returning customer, last ordered: Custom Cake").
  3.  **AI Assistant Action**: A floating translucent glass button suggests "AI Drafts Reply". Tapping it streams the AI response into the input box for Maya to approve.

  ## Implementation Prompt (For Implementer Agent)
  **Objective**: Implement the foundational PostgreSQL schema and core Rust data structures for the Native OHC Omnichannel Chat system.

  **Critical User Journey (CUJ)**:
  As a developer integrating the system, I need the database schema and Rust structs to represent Inboxes, Contacts, Conversations, and Messages with strict tenant isolation, so that I can begin building the API and WebSocket layers.

  **Acceptance Criteria**:
  1.  Create PostgreSQL migration files for `inboxes`, `contacts`, `conversations`, and `messages` tables.
  2.  All tables MUST include `tenant_id` (UUID) and setup Row Level Security (RLS) ensuring `tenant_id = current_setting('app.current_tenant')`.
  3.  Implement corresponding Rust structs/models in the backend workspace.
  4.  Write comprehensive unit tests ensuring data integrity and verifying that RLS policies prevent cross-tenant access.
  5.  (No UI work is required for this foundational data model task, but ensure structs are serialized appropriately for future JSON REST/WebSocket use).

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
