issue_title: "Native Rust Omnichannel Inbox & Routing Architecture"
issue_description: |
  # Native Rust Omnichannel Inbox & Routing Architecture

  ## Problem Statement
  We are fully retiring Chatwoot as an external third-party dependency. OHC currently lacks a native omnichannel messaging system that can handle WhatsApp, Email, Instagram DMs, and Web Chat concurrently while applying SLA policies, routing to AI agents, and serving a multi-tenant user base. A non-technical owner like Maya or Carlos needs a unified "Work Triage" feed that invisibly coordinates these channels. If they receive a WhatsApp message and an Instagram DM, both must arrive in a single unified inbox, be parsed for intent, and immediately trigger the appropriate AI operations assistant.

  ## Research Report
  Based on the Chatwoot source code audit (`/app/models` and `/app/controllers`), we must replicate its core data structures natively in Rust:
  - **Conversations & Messages:** Multi-tenant entities supporting robust threading.
  - **Inboxes & Channel Adapters:** Interfaces for polymorphic message consumption (WhatsApp Cloud API, IMAP/SMTP, Instagram/FB Graph, WebWidget).
  - **Contacts & Contact Inboxes:** Unified identity across channels.
  - **Routing & Agents:** Assignment rules mapping conversations to human teams or AI agents based on intent.
  - **Real-Time WebSocket Engine:** For instant updates to the desktop/mobile client.

  Chatwoot's Ruby on Rails architecture uses Active Record polymorphic associations for channel adapters. Our Rust architecture will leverage PostgreSQL with strictly typed `tenant_id` RLS for isolation, and a polymorphic trait-based `ChannelAdapter` system.

  ## Design Doc
  ### Data Model & Invariants
  *   **`Conversation`**: `id`, `tenant_id`, `inbox_id`, `contact_id`, `status` (open, pending, resolved), `assignee_id` (agent/human), `created_at`.
  *   **`Message`**: `id`, `tenant_id`, `conversation_id`, `sender_type` (contact/agent), `sender_id`, `content`, `message_type` (incoming/outgoing), `created_at`.
  *   **`Inbox`**: `id`, `tenant_id`, `name`, `channel_type` (whatsapp, email, widget), `channel_id`.
  *   **`Contact`**: `id`, `tenant_id`, `name`, `email`, `phone_number`.

  **Invariants:**
  *   All tables MUST include `tenant_id` and have PostgreSQL Row Level Security (RLS) enabled.
  *   Conversations are strictly bound to a single Inbox and Contact.

  ### Architecture Diagram (Mermaid)
  ```mermaid
  graph TD
    A[External Channels: WhatsApp, Web, Email] -->|Webhook/API| B(Gateway / Load Balancer)
    B --> C[Rust API Server: Channel Webhooks]
    C --> D[Channel Adapter System]
    D --> E[(PostgreSQL: Messages & Conversations)]
    E --> F[AI Routing & Triage Engine]
    F --> G[Background Job Queue: pg_vector/valkey]
    G --> H[Operations / CS AI Agents]
    E -.-> I[WebSocket Server]
    I -.-> J[Flutter / PWA Client]
  ```

  ### Mobile UX Flow (375px First)
  - **Unified Feed:** The primary mobile view is a combined inbox. Each conversation card shows the contact name, channel icon (WhatsApp/Insta), last message preview, and a status token (e.g., "Urgent", "AI Draft Ready").
  - **Conversation View:** Tapping a card opens the chat thread. The AI's suggested reply is pre-filled in a translucent "glass" styled text box at the bottom, ready for one-tap approval.
  - **Real-time:** The UI updates instantly via WebSocket without manual refreshing.

  ### AI Agent Integration Points
  - **Triage Hook:** On new message insert, a PostgreSQL `SKIP LOCKED` worker dequeues the event and invokes the AI Triage Agent to classify intent and urgency.
  - **Drafting Hook:** If intent requires a reply, the CS Agent is invoked to generate a draft message, stored in the DB as a pending suggestion.

  ## Implementation Prompt
  Implement the core database schema, Rust models, and backend gRPC/REST endpoints for the new Native Omnichannel Inbox.

  **CUJ:**
  As a system administrator, I need the database schema and basic CRUD API for Inboxes, Contacts, Conversations, and Messages so that channel adapters (WhatsApp, Web) can begin ingesting data into our native system.

  **Acceptance Criteria:**
  1. Define Rust structs and SQL schema for `Conversation`, `Message`, `Inbox`, and `Contact` including `tenant_id` and RLS.
  2. Implement backend endpoints to create an inbox, create a contact, start a conversation, and add a message.
  3. All code MUST have 100% unit test coverage.
  4. Ensure a clear Trait/Interface exists for future `ChannelAdapter` implementations.

  ## Priority
  P0 (Critical path for replacing Chatwoot)

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
