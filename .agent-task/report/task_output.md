issue_title: "Native Rust Omnichannel Inbox & Chat System (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OHC previously relied on Chatwoot as an external third-party service for its omnichannel customer support and chat engine. Chatwoot integration introduces external dependencies, breaks strict multi-tenant row-level security within our own DB, violates Zero-Trust/SPIFFE identity boundaries, and prevents deep, synchronous AI agent integration within our native job queues. We need a native, high-performance omnichannel inbox architecture inside `onehumancorp/mono` built in Rust. It must support multiple channels (Email, Web Widget, WhatsApp, Instagram DMs) natively while preserving our strict tenant isolation and mobile-first design. This is a critical enabler for Maya (custom cakes via IG DMs) and Carlos (lead recovery from web widget) to receive and reply to all communications in one place, natively powered by our AI.

  ## Research Report
  Benchmarking against Chatwoot (`https://github.com/chatwoot/chatwoot` source code audit):
  - **Data Models:** Chatwoot relies heavily on `Account` (Tenant), `Inbox`, `Conversation`, `Message`, `Contact`, and `Channel::*` (Adapters).
  - **Aggregate Root:** `Conversation` serves as the aggregate root for messages between a `Contact` and an `Inbox`.
  - **Channels:** `Channel` adapters (e.g., `Channel::WebWidget`, `Channel::Whatsapp`, `Channel::Email`) define channel-specific settings and webhook URLs via single-table inheritance or polymorphic associations.
  - **Real-time Engine:** WebSockets handle real-time delivery (`ActionCable` in Chatwoot). OHC will use Rust async WebSockets (e.g., `axum` or `tungstenite`) backed by Redis Pub/Sub for distributed node delivery.
  - **Agent Automation:** Chatwoot uses `agent_bot` and `macro` for automation via webhooks. OHC will instead route these directly into the native PostgreSQL `SKIP LOCKED` AI Job Queue for our AI departments (Operations, CS, Sales) to handle autonomously and synchronously.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      INBOX ||--o{ CHANNEL : configured_by
      INBOX ||--o{ CONVERSATION : receives
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      CONVERSATION ||--o{ AI_JOB : triggers

      TENANT {
          uuid tenant_id
      }
      INBOX {
          uuid id
          string name
          uuid tenant_id
      }
      CHANNEL {
          uuid id
          string type
          jsonb config
      }
      CONTACT {
          uuid id
          string name
          string identifier
      }
      CONVERSATION {
          uuid id
          uuid inbox_id
          uuid contact_id
          string status
      }
      MESSAGE {
          uuid id
          uuid conversation_id
          text content
          string message_type
      }
  ```

  ### Mobile UX Flow (375px First)
  - **Unified Inbox Screen**: Displays a scrollable list of active conversations across all channels in a UniFi-style card layout. Each card shows Contact Name, Channel Icon, Time, and Preview. Swipe left to 'Resolve', swipe right to 'Snooze'. Touch targets are 44x44px.
  - **Conversation Screen**: Sticky header with Contact Name & Channel. Scrollable message history with translucent glass materials. Bottom native keyboard input area with Quick Replies and an AI 'Draft Reply' button.
  - **AI Agent Integration**: Inbound messages trigger the AI job queue. When the Customer Assistant agent replies, it appears as a "Drafted by Assistant" pending message (distinct visual state) for the owner to approve with a single tap, or it replies automatically based on tenant config.

  ### Key Design Decisions
  - **Rust Native Services**: Replace Ruby/Rails models with native Rust models in `src/server/ohc/domain/chat`.
  - **Tenant Isolation**: Every table (`inboxes`, `conversations`, `messages`, `contacts`) MUST have `tenant_id` and enforce RLS (Row Level Security).
  - **Extensible Channels**: Use a trait/adapter pattern in Rust to easily add new channels (WhatsApp, Instagram, Web) later without altering the core messaging pipeline.
  - **Direct AI Coordination**: Unlike Chatwoot's webhook-based bots, OHC's AI agents directly watch the unified inbox via our Postgres queue and Redis distributed locks (`ohc:lock:{tenant_id}:conversation:{id}`), reducing latency and complexity.

  ## Implementation Prompt
  Implement the core data model and Rust API layer for the native OHC Omnichannel Inbox.
  1. Create PostgreSQL migrations defining `inboxes`, `channels`, `contacts`, `conversations`, and `messages` tables. You must ensure `tenant_id` is present on all tables and PostgreSQL Row-Level Security (RLS) is applied to isolate data per tenant.
  2. Implement Rust domain structs and repository adapters for these entities within `src/server/ohc/domain/chat`.
  3. Create internal gRPC/REST endpoints for:
     - Creating/listing Inboxes.
     - Creating/listing Conversations.
     - Sending/Receiving Messages within a Conversation.
  4. Ensure the message creation service publishes an event (e.g., via Redis Pub/Sub or Postgres LISTEN/NOTIFY) that the AI Job Queue can subscribe to, triggering the Customer Assistant for drafting replies.
  5. Apply 100% unit test coverage for all new Rust models and controllers.
  6. Acceptance Criteria: A new message can be created via API, assigned to an inbox and conversation, properly isolated by tenant, and triggers an event observable by the AI queue. The owner persona (e.g., Maya) can see all messages from any channel in one unified endpoint.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
