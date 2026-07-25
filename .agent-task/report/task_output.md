issue_title: "Native Rust Omnichannel Chat System: Chatwoot Replacement Architecture"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) owners need a single, unified inbox to manage customer communications across all channels (Instagram DMs, email, website chat, WhatsApp). Currently, relying on an external Chatwoot dependency creates fragmentation, breaks our Zero-Trust architecture, and introduces latency. It also violates our architectural mandate to build OHC as a single native, high-performance, multi-tenant work assistant. We must fully retire the Chatwoot external dependency and build a native, high-performance, multi-tenant omnichannel chat engine in Rust inside `onehumancorp/mono`.

  ## Research Report
  An exhaustive audit of the `chatwoot/chatwoot` repository (v3) was conducted, specifically analyzing:
  - **Data Models:** `Account`, `User`, `Inbox`, `Channel`, `Conversation`, `Message`, `Contact`, `AgentBot`.
  - **Controllers & Routing:** Webhook ingestion paths for external channels.
  - **Real-time Layer:** ActionCable WebSockets for client-side message delivery.
  - **Agent Automation:** AgentBot integrations and macro automations.

  **Key Findings:**
  1. Chatwoot relies heavily on a classic Ruby on Rails monolithic architecture with Sidekiq for background jobs.
  2. Multi-tenancy is handled via an `account_id` column on almost all models.
  3. Channel adapters (e.g., `Channel::Email`, `Channel::Whatsapp`, `Channel::WebWidget`) normalize incoming messages into a standard `Message` format.

  Our Rust implementation will adopt the standardized normalized message format and channel adapter pattern but leverage Rust's strict typing, Bazel for builds, and gRPC/WebSocket for real-time delivery to our Flutter UI, completely dropping the Ruby/Rails legacy footprint.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL_ADAPTER : configured_with
      INBOX ||--o{ CONVERSATION : contains
      CONVERSATION ||--o{ MESSAGE : tracks
      CONVERSATION }o--|| CONTACT : involves
      CHANNEL_ADAPTER ||--o{ MESSAGE : receives
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
          string content
          string message_type
          timestamp created_at
      }
      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string identifier
          string name
      }
  ```

  ### System Architecture
  - **Ingestion:** Native Rust webhook handlers for Stripe, WhatsApp, IG, Email. These handlers drop raw payloads into our PostgreSQL-based job queue (`SKIP LOCKED`).
  - **Normalization:** Background workers pick up jobs, parse provider-specific formats using `ChannelAdapter` traits, and normalize them into `Message` entities tied to a `Conversation`.
  - **Real-time Delivery:** gRPC bidirectional streaming or WebSocket connections push normalized messages to the Flutter client.
  - **Multi-Tenant Isolation:** All queries strictly enforce row-level security (RLS) via `tenant_id`. Redis locks (`ohc:lock:{tenant_id}:conversation:{id}`) prevent race conditions during message ingestion.

  ### Mobile UX Flow (375px First)
  - **Unified Inbox Screen:** Clean, Ubiquiti-style list of active conversations. Avatars show channel icons (IG, Email). Unread indicators are vibrant badges.
  - **Conversation Detail:** Translucent glass header. Native mobile keyboard support. Quick-action chips for AI-drafted replies (Customer Assistant).
  - **Action Menu:** Tap a message to surface operations (Create Quote, Book Appointment) linking directly to the Operations Assistant.

  ### AI Agent Integration
  - **Customer Assistant:** Listens to the `Message` stream. When a new customer message arrives, it evaluates context and drafts a reply, saving it as a pending action for the owner.
  - **Operations Assistant:** Parses intents from chat (e.g., "I want a vegan cake on Friday") to automatically draft a `Task` or `Quote` attached to the `Conversation`.

  ## Implementation Prompt
  **To the Implementer Swarm:**
  Implement the core Rust data models and database migrations for the Native Omnichannel Chat System based on the ER diagram provided.
  1. Create the migrations for `inboxes`, `channel_adapters`, `contacts`, `conversations`, and `messages`, ensuring all tables have a `tenant_id` and PostgreSQL Row-Level Security (RLS) is enabled.
  2. Implement the Rust structs and basic CRUD operations (with RLS context) for these entities.
  3. Ensure 100% unit test coverage for the models and multi-tenant isolation logic.
  4. Integrate the new models into the Bazel build system (`BUILD.bazel`).
  *Remember: Build for the owner persona (e.g., Maya the baker) who needs a lightning-fast, unified inbox without knowing how it works. Maintain strict mobile-first API design.*

  ## Priority & Scope
  - **Priority:** P0 (Critical - Unblocks core communication and retires legacy dependency)
  - **Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
