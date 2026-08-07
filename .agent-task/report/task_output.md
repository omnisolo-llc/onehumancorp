issue_title: "Native Rust Omnichannel Chat System Architecture"
issue_description: |
  ## Research Report: Retiring Chatwoot and Building a Native Rust Omnichannel Chat System

  ### 1. Problem Statement
  OHC requires a high-performance, omnichannel customer support and chat engine to unify messages (DMs, emails, webchat, WhatsApp, etc.) for our business owners (Maya, Carlos, Priya, Leo, Fatima) into a single, cohesive interface.
  Historically, this was achieved by integrating with an external third-party service, Chatwoot.
  However, as OHC scales, relying on a third-party Ruby-on-Rails service introduces multi-tenancy complexities, increased latency, lack of native AI agent coordination, and architectural fragmentation. The core requirement now mandates the complete retirement of Chatwoot as an external dependency and the construction of a **100% native Rust omnichannel chat system within `onehumancorp/mono`**.

  ### 2. Research Report
  - **Codebase & External Source Analysis**: I checked out the `https://github.com/chatwoot/chatwoot` source code. Chatwoot relies on Rails Active Record patterns:
    - **Inbox**: `app/models/inbox.rb` (ties an Account to a Channel, tracks settings like `working_hours`, `csat_config`, etc.)
    - **Conversation**: `app/models/conversation.rb` (threads of messages between a contact and agents)
    - **Message**: `app/models/message.rb` (individual messages, handles content types, private vs public notes, source tracking)
    - **Contact**: `app/models/contact.rb` (customers interacting with the business)
    - **Channels**: e.g., `Channel::Email`, `Channel::Whatsapp`, `Channel::WebWidget`, `Channel::Api` (adapters linking external APIs like WhatsApp Cloud to the Inbox).
  - **OHC Architecture Mandate**: Our system must replicate this logical data model but adapt it for our **Rust backend** using PostgreSQL with strict multi-tenant row-level security (`tenant_id`). It must integrate directly with our distributed lock mechanism (Redis Redlock), our AI job queue (`SKIP LOCKED` PG pattern), and our native agentic departments (Operations, Customer Service).
  - **Competitor/Industry Alignment**: Similar to Shopify Ping or Intercom's architecture, OHC needs edge-friendly web sockets, high-throughput message ingestion, and agent-first prompt contexts where the AI can seamlessly intercept or draft replies in real-time.

  ### 3. Design Doc: Native Rust Chat Architecture
  - **Data Model & Invariants**:
    - `conversations`: `id`, `tenant_id`, `contact_id`, `inbox_id`, `status` (open, pending, closed), `created_at`, `updated_at`.
    - `messages`: `id`, `tenant_id`, `conversation_id`, `sender_type` (contact, agent, bot), `sender_id`, `content`, `message_type` (incoming, outgoing, private_note), `created_at`.
    - `inboxes`: `id`, `tenant_id`, `name`, `channel_type` (email, webwidget, whatsapp, api), `channel_config` (JSONB).
    - `contacts`: `id`, `tenant_id`, `identifier`, `name`, `email`, `phone_number`.
    - Strict Multi-Tenancy: All tables require `tenant_id` and PostgreSQL RLS policies ensuring rows are only accessible by the correct `tenant_id`.

  - **AI Agent Integration**:
    - When a new `Message` is inserted via a channel webhook or API, it triggers an AI Job (via PG `SKIP LOCKED`).
    - The **Customer & Relationship Assistant** is invoked to analyze the message context and draft a response (`Message` with `message_type = private_note` or `outgoing` in draft status).
    - The business owner reviews and approves the draft via the mobile "Approval Interface Paradigm."

  - **Mobile UX Flow (375px First)**:
    - The "Unified Agent Feed" surfaces cards for new conversations.
    - Tapping a card opens a full-screen, bottom-up sheet conversation view.
    - AI drafts appear as distinct, highlighted bubbles with an inline "Approve & Send" or "Edit" action.
    - Native mobile keyboard and touch-friendly targets (> 44px).

  - **Architecture Diagram (Mental Model for Implementers)**:
    - [External Webhook/Client] -> (Rust HTTP API/WebSocket) -> [Message Ingestion/Validation] -> (PostgreSQL `messages` insert)
    - (PostgreSQL insert) -> [AI Job Queue] -> (Rust Worker) -> [LLM Provider] -> (Draft Message Insert)
    - (WebSocket Broadcast) -> [Flutter Client] -> (Unified Agent Feed Update)

  ### 4. Implementation Prompt for Swarm Agents
  **Task**: Implement the core native Rust chat data models and API endpoints for OHC, replacing external Chatwoot dependencies.

  **Objective**: Create the PostgreSQL migrations, Rust Tonic gRPC/Axum REST API endpoints, and corresponding business logic to support Inboxes, Conversations, and Messages with strict row-level multi-tenant isolation.

  **CUJ**:
  1. A multi-tenant business owner creates a "Web Widget" Inbox via the API.
  2. A new customer (Contact) initiates a Conversation in that Inbox.
  3. The customer sends a Message.
  4. The system stores the Message and triggers a background event (which eventually triggers the AI Assistant).
  5. The API allows fetching the Conversation history with pagination.

  **Acceptance Criteria**:
  - Do NOT use or reference Chatwoot external services.
  - Rust models and DB migrations for `inboxes`, `contacts`, `conversations`, and `messages` must include `tenant_id` and RLS policies.
  - Implement basic CRUD gRPC/REST endpoints for these entities.
  - 100% unit test coverage for the new Rust services.
  - Ensure all database queries strictly filter by `tenant_id` (or rely on active RLS session variables).

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
