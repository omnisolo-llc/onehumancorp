issue_title: "Native Rust Omnichannel Chat System: Core Architecture & Data Model"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) currently relies on a retired external dependency for omnichannel chat. To deliver a lightning-fast, highly scalable, and deeply integrated owner work assistant, OHC must build its own native Rust omnichannel chat system inside `onehumancorp/mono`. This system must handle unified inboxes, WhatsApp Business, Web Widget channels, and WebSocket real-time messaging, natively supporting strict row-level security (RLS) multi-tenancy and Zero-Trust isolation. The absence of this system creates a significant gap in the platform's core capability to triage work and manage customer relationships autonomously for personas like Maya (Baker) and Carlos (Handyman).

  ## Research Report
  An exhaustive audit of the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) was conducted to benchmark omnichannel feature parity:
  - **Data Models**: Chatwoot isolates data via `account_id`. In OHC, this translates to `tenant_id` for PostgreSQL RLS. The core entities required are `Inbox`, `Channel`, `Contact`, `Conversation`, and `Message`.
  - **Channel Adapters**: Chatwoot uses polymorphic channels (e.g., `Channel::WebWidget`, `Channel::Whatsapp`, `Channel::Email`). OHC needs equivalent Rust traits and structures to handle Meta webhooks and WebSocket payloads dynamically.
  - **WebSocket Architecture**: Real-time events (typing indicators, new messages, presence) are distributed via ActionCable in Chatwoot. OHC will leverage native Rust async WebSockets (e.g., using `tokio` and `axum` or `tungstenite`) backed by Redis Pub/Sub for cross-node event broadcasting.
  - **AI Integration (Agent Bot)**: Chatwoot has an `AgentBot` concept. OHC will extend this significantly by integrating the AI Job Queue (PostgreSQL `SKIP LOCKED`) to allow the **Customer Assistant** department to auto-draft replies for Instagram DMs, WhatsApp, and Web Chat invisibly.

  ## Design Doc
  ### 1. Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : manages
      INBOX ||--o{ CHANNEL : configures
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates_in
      CONVERSATION ||--o{ MESSAGE : holds
      MESSAGE ||--|{ ATTACHMENT : includes

      TENANT {
          uuid id PK
          string name
      }
      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
      }
      CHANNEL {
          uuid id PK
          uuid inbox_id FK
          string provider_type "whatsapp, web_widget, instagram"
          jsonb credentials
      }
      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string name
          string phone_number
      }
      CONVERSATION {
          uuid id PK
          uuid inbox_id FK
          uuid contact_id FK
          string status "open, resolved, pending"
      }
      MESSAGE {
          uuid id PK
          uuid conversation_id FK
          string content
          string sender_type "contact, agent, bot"
          boolean is_private_note
      }
  ```

  ### 2. Mobile UX Flow (375px First)
  - **Unified Inbox View**: A bottom-nav anchored "Messages" screen. Shows a unified list of `Conversation` threads grouped by unread/action-required. Each thread clearly indicates the channel source (WhatsApp icon, Instagram icon).
  - **Thread View**: Native-feeling chat interface. Crucially includes a translucent "Drafted by Assistant" sticky card at the bottom if the AI has proposed a reply, with one-tap "Send" or "Edit".
  - **Contact Context Sheet**: Tapping a contact's avatar opens a bottom sheet showing their past orders, tags, and preferences, allowing the owner (e.g., Priya) to answer complex queries immediately.

  ### 3. AI Agent Integration Points
  - **Work Triage Department**: Subscribes to `MessageCreated` events via the background job queue. It evaluates incoming messages for urgency, intent (e.g., "Do you do vegan cakes?"), and routes to the Customer Assistant.
  - **Customer Assistant Department**: Queries the `Knowledge` memory for the tenant to draft context-aware replies. It inserts a `Message` with `status=draft` into the database, triggering a real-time WebSocket event to the owner's app for approval.

  ### 4. Key Design Decisions
  - **Strict Multi-Tenancy**: Every table must have `tenant_id` and PostgreSQL RLS enabled.
  - **Stateless WebSockets**: WebSocket servers must be stateless. State and pub/sub routing are delegated to Redis, ensuring OHC can scale horizontally in Kubernetes.
  - **Async Event Driven**: Webhooks from providers (Meta, Stripe) are immediately acknowledged and enqueued to the DB via `SKIP LOCKED` to prevent timeout on external APIs.

  ## Implementation Prompt
  **For the Implementer Agent:**
  Your task is to implement the foundational Rust API and Database layer for the OHC Native Omnichannel Chat system.

  **User-Facing Outcome:** As an owner (like Carlos), I can receive a message from a customer via WhatsApp or Web Widget, view it in a unified inbox, and see an AI-drafted reply ready for my approval.

  **Critical User Journey (CUJ):**
  1. A webhook payload simulates an incoming WhatsApp message.
  2. The Rust backend processes the payload, identifies the `Tenant`, `Contact`, and `Inbox`.
  3. A new `Conversation` and `Message` are persisted.
  4. An event is broadcasted via Redis Pub/Sub to active WebSocket connections.

  **Acceptance Criteria:**
  - Create the exact schema migrations for `inboxes`, `channels`, `contacts`, `conversations`, and `messages` enforcing RLS with `tenant_id`.
  - Implement a Rust Axum service (or equivalent based on the OHC stack) that exposes a POST webhook endpoint for message ingestion.
  - Implement a basic WebSocket endpoint that subscribes to a Redis channel and streams new messages to the connected client.
  - Ensure 100% unit test coverage for the domain logic and parsing.
  - Write a Playwright E2E test verifying a mock webhook POST results in a visible message in the UI (or equivalent verified backend state). Do not prescribe specific function signatures—design the modular Rust crates following OHC conventions.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
