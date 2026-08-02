issue_title: "Native Rust Omnichannel Chat: Core Data Models & Channel Adapters"
issue_description: |
  # Problem Statement
  OneHumanCorp (OHC) is transitioning away from using Chatwoot as an external dependency for its omnichannel inbox features. We need a native, high-performance replacement built in Rust, hosted inside our monolithic repository (`onehumancorp/mono`).

  Owners (like Maya the baker and Carlos the handyman) currently face friction managing customer communications across SMS, WhatsApp, Email, and Instagram. They need a unified inbox that aggregates all messages seamlessly and allows AI agents (like The Ambassador) to draft replies using a unified history context. Relying on an external system limits our ability to enforce strict multi-tenancy rules and integrate tightly with our custom AI Agent pipeline.

  # Research Report
  - **Chatwoot Source Code Audit**: Investigated the `app/models` and `app/models/channel` directories in the Chatwoot Ruby on Rails repository.
  - **Key Entities Identified**:
    - `Account` (Tenant)
    - `Inbox` (Collection of conversations for a specific channel)
    - `Contact` (Customer representation)
    - `Conversation` (A thread of messages between a Contact and an Account)
    - `Message` (Individual piece of communication)
    - `Channel::*` (Specific adapters like `Channel::Api`, `Channel::Email`, `Channel::Whatsapp`, etc.)
  - **OHC Requirement**: We need to replicate these core models in Rust using an appropriate ORM or raw SQL (e.g., SQLx) with strict multi-tenant isolation (`tenant_id` on every table). The architecture must support async event processing and WebSocket-based real-time updates for the frontend.

  # Design Doc
  ## Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      TENANT {
          uuid id PK
          string name
      }
      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string name
          string email
          string phone_number
          jsonb custom_attributes
      }
      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
          string channel_type
          jsonb channel_credentials
      }
      CONVERSATION {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status "open, resolved, snoozed"
          datetime last_activity_at
      }
      MESSAGE {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          string content
          string message_type "incoming, outgoing, bot"
          datetime created_at
      }

      TENANT ||--o{ CONTACT : has
      TENANT ||--o{ INBOX : configures
      TENANT ||--o{ CONVERSATION : owns
      TENANT ||--o{ MESSAGE : stores
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates_in
      CONVERSATION ||--o{ MESSAGE : includes
  ```

  ## UI Wireframes & Mobile UX Flow (375px First)
  - **Home Screen (Inbox List)**: A clean, 375px-wide feed displaying "open" conversations across all channels (WhatsApp, Instagram, etc.). Each item shows the contact name, the latest message preview, the channel icon, and the time.
  - **Conversation View**: Tapping a conversation opens a full-screen chat interface. Translucent glass app bar at the top with the contact's name. A vertically scrolling message history.
  - **Message Draft Area**: At the bottom, a 44px minimum height text input field with native keyboard support. A primary "Send" button. If an AI agent drafted a reply, the draft text appears slightly greyed out with "Approve" or "Edit" buttons.
  - **Empty States**: If no messages exist, display a friendly illustration indicating "You're all caught up!"

  ## Key Design Decisions
  - **Multi-Tenancy**: Every entity will include a `tenant_id` column. We will enforce Row-Level Security (RLS) in PostgreSQL.
  - **Channel Adapters**: We will implement a `ChannelAdapter` trait in Rust. Initial implementation will focus on a generic `ApiChannelAdapter` to allow simulated webhook ingestion for testing.
  - **Event Mesh Integration**: Creating a `Message` will publish an event to our internal event mesh, triggering AI agents (like The Ambassador) to evaluate the conversation and draft responses.

  ## AI Agent Integration Points
  - The `Message` creation handler will invoke the `Ambassador` agent if the `status` is `open` and `message_type` is `incoming`.
  - The agent will query the `Conversation` history and `Contact` details to generate a draft reply, which is inserted as a `MESSAGE` with `message_type = bot` and a pending approval state.

  # Implementation Prompt
  **User-Facing Outcome**: As an owner, I want a robust backend system that captures all incoming messages from various channels into a unified database, ensuring my data is secure and isolated from other businesses. I should also be able to interact seamlessly with these messages on my phone.

  **Tasks for Implementer**:
  1. Define the PostgreSQL schema for the entities: `contacts`, `inboxes`, `conversations`, and `messages`. Ensure every table includes a `tenant_id` UUID column and that RLS policies are applied.
  2. Create Rust structs and database interaction logic (using SQLx or SeaORM, depending on current stack standards) for these entities within a new module `src/omnichannel` or similar.
  3. Implement a generic `ApiChannelAdapter` trait and a concrete webhook endpoint that can receive simulated external messages, create a `Contact` (if new), a `Conversation` (if new), and insert a `Message`.
  4. Write unit tests to verify CRUD operations and strict multi-tenant isolation (ensure a query for Tenant A cannot read Tenant B's messages).

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
