issue_title: "[Native Chatwoot Replacement] Implement Native Rust Omnichannel Chat System"
issue_description: |
  # Native Rust Omnichannel Chat System - Chatwoot Replacement

  ## Problem Statement
  OneHumanCorp's mandate strictly requires that we RETIRE Chatwoot as an external third-party service/dependency and build our own high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust inside `onehumancorp/mono`. Currently, the system lacks a fully native, deep-integrated omnichannel engine that can handle multi-tenant inboxes, conversational routing, SLA policies, WebSocket real-time messaging, and multi-channel adapters (Web, Instagram, WhatsApp, Email, API) with the same feature set as Chatwoot, while strictly adhering to OHC's Zero-Trust multi-tenant isolation and mobile-first owner workflows. Maya, Carlos, Priya, Leo, and Fatima all need an inbox that just works, with AI agents automatically triaging messages and drafting replies, without OHC relying on external dependencies for core communication.

  ## Research Report
  - **Chatwoot Codebase Audit:** Analyzed the `chatwoot/chatwoot` repository (v3.x).
    - **Core Data Models:**
      - `Account` (matches OHC `tenant`), `User` (matches OHC `team_member`/`user`), `Inbox`, `Channel` (STI for `Channel::WebWidget`, `Channel::Email`, `Channel::Api`, etc.), `Contact`, `ContactInbox`, `Conversation`, `Message`, `Notification`.
    - **Architecture:** Rails MVC, ActionCable for WebSockets, Sidekiq for background jobs (emails, webhooks).
    - **Key Capabilities Needed in OHC:**
      - Unified Inbox routing (assigning conversations to agents or teams).
      - Multi-channel support (Web Widget, API, Email, Social).
      - Real-time updates via WebSockets.
      - Agent Presence and Typing Indicators.
      - Macros, Canned Responses, SLA Policies, and Automation Rules.
  - **OHC Architecture Alignment:**
    - OHC uses Rust (Axum/Tonic), PostgreSQL (RLS), and Valkey.
    - We must replicate Chatwoot's data models into Rust structs and PostgreSQL tables with strict `tenant_id` RLS policies.
    - ActionCable WebSockets must be replaced by Axum WebSockets backed by Valkey Pub/Sub for horizontal scalability.
    - Sidekiq jobs must be replaced by OHC's PostgreSQL `SKIP LOCKED` job queue.

  ## Design Doc
  ### Architecture & Data Model

  ```mermaid
  erDiagram
      tenant ||--o{ inbox : "owns"
      tenant ||--o{ omni_contact : "owns"

      inbox {
          uuid id PK
          uuid tenant_id FK
          string name
          uuid channel_id
          string channel_type
      }

      channel_web_widget {
          uuid id PK
          uuid tenant_id FK
          string website_url
          string widget_color
      }

      omni_contact {
          uuid id PK
          uuid tenant_id FK
          string name
          string email
          string phone_number
          string identifier
      }

      contact_inbox {
          uuid id PK
          uuid tenant_id FK
          uuid contact_id FK
          uuid inbox_id FK
          string source_id
      }

      omni_conversation {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_inbox_id FK
          uuid assignee_id FK
          string status
      }

      omni_message {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          uuid contact_id FK
          string sender_type
          uuid sender_id FK
          string content
          string message_type
      }

      inbox ||--o{ channel_web_widget : "has one (polymorphic)"
      omni_contact ||--o{ contact_inbox : "has many"
      inbox ||--o{ contact_inbox : "has many"
      contact_inbox ||--o{ omni_conversation : "has many"
      inbox ||--o{ omni_conversation : "has many"
      omni_conversation ||--o{ omni_message : "contains"
  ```

  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC_Axum_API
      participant Valkey_PubSub
      participant OHC_Agent_Job_Queue

      Customer->>OHC_Axum_API: Send Message (Web Widget)
      OHC_Axum_API->>Database: Save omni_message
      OHC_Axum_API->>Valkey_PubSub: Publish `message.created`
      Valkey_PubSub->>OHC_Axum_API: Broadcast to subscribed owners
      OHC_Axum_API->>OHC_Agent_Job_Queue: Enqueue AI Triage Job
      OHC_Agent_Job_Queue-->>Database: Agent drafts reply
  ```

  - **Inboxes & Channels:**
    - `inbox` table: `id`, `tenant_id`, `name`, `channel_id`, `channel_type` (Enum: WebWidget, API, Email, WhatsApp, Instagram).
    - `channel_*` tables (e.g., `channel_web_widgets`, `channel_api`): Specific config for each channel.
  - **Contacts & Conversations:**
    - `omni_contact` table: `id`, `tenant_id`, `name`, `email`, `phone_number`, `identifier`.
    - `contact_inbox` table: Links a contact to a specific inbox.
    - `conversation` table: `id`, `tenant_id`, `inbox_id`, `contact_inbox_id`, `status` (open, resolved, pending), `assignee_id`.
  - **Messages:**
    - `message` table: `id`, `tenant_id`, `conversation_id`, `contact_id`, `sender_type` (Contact, Agent, AI), `sender_id`, `content`, `message_type` (incoming, outgoing, template).
  - **Real-time & AI Integration:**
    - **Valkey Pub/Sub:** Event schema for `conversation.created`, `message.created`, `presence.update`.
    - **AI Triage Agent:** Hooks into `message.created` to auto-draft replies or auto-resolve basic queries.

  ### Mobile-First UX Flow (375px)
  - **Unified Inbox Screen:**
    - A single list view showing conversations across all channels.
    - Avatars indicate source (Instagram icon, Web icon).
    - Bold for unread, translucent background for AI-drafted (pending approval).
  - **Conversation Thread Screen:**
    - Sticky header with contact name and status toggle.
    - Scrollable message list. Native keyboard integration.
    - Action drawer (swipe up) for Macros, Canned Responses, and AI Drafts.

  ### Multi-Tenant & Security Constraints
  - EVERY new table MUST have a `tenant_id` UUID column.
  - EVERY new table MUST have `ENABLE ROW LEVEL SECURITY`.
  - EVERY new table MUST have a policy: `USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid)`.

  ## Implementation Prompt
  **To the Implementer:**
  Implement the core database schema, Rust domain models, and primary Axum API endpoints for the native OHC Omnichannel Chat System (Chatwoot replacement).
  1. Create a database migration for the core tables: `omni_inbox`, `omni_channel_web_widget`, `omni_contact`, `omni_contact_inbox`, `omni_conversation`, and `omni_message`. Ensure strict RLS policies on all tables.
  2. Implement the corresponding Rust entities and repositories in `src/server/domain/` or `src/server/services/inbox/`.
  3. Implement the Axum HTTP REST endpoints for creating an inbox, listing conversations, and sending a message.
  4. Write comprehensive unit tests and a Playwright E2E test verifying a user can create a Web Widget inbox and receive a message.
  Ensure the implementation is perfectly isolated per tenant and hides all technical complexity from the business owner.

  ## Priority
  P0 (Critical)

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, omnichannel, rust, chatwoot-replacement]
assignees: []
