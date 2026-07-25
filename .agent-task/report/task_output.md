issue_title: "Implement Native Rust Omnichannel Chat System"
issue_description: |
  ## Problem Statement
  OneHumanCorp currently lacks a native, high-performance omnichannel chat system. While previous iterations relied on external services, these introduced latency, synchronization issues, and violated our absolute data residency and multi-tenant isolation principles. For our core personas—like Maya (baker managing IG DMs), Carlos (handyman handling SMS leads), and Fatima (food cart operator responding to pre-order queries)—fragmented communications lead to missed opportunities. OHC requires a highly scalable, strictly isolated, natively integrated Rust-based chat engine to unify all customer interactions (Instagram, WhatsApp, SMS, Web Chat, Email) into a single actionable owner feed.

  ## Research Report
  Our audit of the legacy open-source platform's source code reveals a mature but monolithic Ruby-on-Rails architecture. Key findings from its `app/models` include:
  - **Core Entities**: Accounts (Tenants), Inboxes, Channels (WebWidget, FB, Twitter, API), Contacts, Conversations, and Messages.
  - **Real-time Layer**: ActionCable (WebSockets) handles event broadcasting.
  - **Extensibility**: Webhooks and Agent Bots to automate responses.
  - **Shortcomings for OHC**: It lacks strict row-level security (RLS) enforcement at the DB level, relies heavily on background Sidekiq workers with potential Ruby-induced latency, and its UI is not natively built for our 375px mobile-first owner requirement.

  Comparing this with our needs and modern high-performance messaging (e.g., Discord's Rust services, Stripe's isolated processing), we need a Rust-based async system (using Tokio) leveraging PostgreSQL with strict RLS for multi-tenancy, and Redis for pub/sub and distributed locking.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL_ADAPTER : configured_with
      INBOX ||--o{ CONVERSATION : tracks
      CONVERSATION ||--o{ MESSAGE : contains
      CONVERSATION }o--|| CONTACT : associated_with
      TENANT ||--o{ CONTACT : manages

      TENANT {
          uuid id PK
          string name
      }
      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
      }
      CHANNEL_ADAPTER {
          uuid id PK
          uuid inbox_id FK
          string provider_type
          jsonb credentials
      }
      CONVERSATION {
          uuid id PK
          uuid inbox_id FK
          uuid contact_id FK
          string status
      }
      MESSAGE {
          uuid id PK
          uuid conversation_id FK
          uuid sender_id
          string content
          timestamp created_at
      }
      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string identifier
      }
  ```

  ### UI Wireframes & Mobile UX Flow
  - **375px Mobile First View**:
    - *Home/Unified Inbox*: A scrollable list of active `Conversations` with unread badges, sorted by urgency/latest activity. Transparent glass styling on list item cards.
    - *Conversation Detail*: A full-height chat interface. Header contains the contact name and context (e.g., "Maya's custom cake lead"). Bottom sticky input area with native mobile keyboard support and an "AI Draft" toggle.
    - *Omnichannel Indication*: Small icon badges (IG, WA, SMS) on the avatar to indicate the channel source.
  - **User Journey (Maya's Use Case)**: Maya receives an IG DM. Push notification arrives. She taps, opening the Conversation Detail. The AI Customer Assistant has already drafted a response based on her cake menu. She taps "Send", and the Rust backend queues the message out to the IG channel adapter.

  ### AI Agent Integration Points
  - **Work Triage Agent**: Listens to the `conversation_created` and `message_received` event streams via Redis Pub/Sub. Classifies intent (e.g., "Lead", "Support", "Spam").
  - **Customer Relationship Agent**: Automatically generates `Draft Messages` based on tenant context (knowledge base, menu, past interactions) and attaches them to the Conversation state for the owner to approve or send.
  - **Operations Agent**: If the message contains a booking intent, it extracts parameters and proposes an actionable "Schedule Visit" card within the chat UI.

  ### Key Design Decisions
  - **Strict Multi-Tenancy**: Every table (Inbox, Conversation, Message, Contact) MUST have a `tenant_id` and utilize PostgreSQL RLS.
  - **Native Rust**: Microservice built with `axum` for HTTP API and WebSocket handling, `sqlx` for asynchronous RLS-aware database access.
  - **Redis Pub/Sub**: For real-time WebSocket fan-out and AI job queue triggering.
  - **Stateless WebSockets**: Horizontal scaling enabled by routing WebSocket events through Redis Pub/Sub so clients can connect to any Rust node.

  ## Implementation Prompt
  Implement the foundation of the Native Rust Omnichannel Chat System inside `src/server/services/chat/`.
  1. Define the SQL migrations for `inboxes`, `channel_adapters`, `contacts`, `conversations`, and `messages`, ensuring all tables have `tenant_id` and RLS policies enabled.
  2. Create the Rust `axum` routes for CRUD operations on these entities.
  3. Implement the WebSocket endpoint `/ws/chat` that authenticates via SPIFFE/SPIRE (or current auth mechanism), subscribes to Redis, and pushes new messages to the client.
  4. Ensure 100% unit test coverage for the new Rust modules and implement at least 5 Playwright E2E tests validating the 375px mobile inbox flow, confirming real data flows from DB to UI without any mock data.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, chat]
assignees: []
