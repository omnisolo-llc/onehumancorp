issue_title: "Architectural Gap: Native Rust Multi-Tenant Omnichannel Chat Engine (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  Currently, OneHumanCorp (OHC) lacks a native, high-performance, multi-tenant omnichannel customer support and chat engine. Chatwoot as an external dependency is 100% retired. Small-business owners like Maya (baker), Carlos (handyman), and Priya (boutique operator) need a unified inbox that consolidates Instagram DMs, WhatsApp, SMS, and Web Chat into a single, cohesive interface. Relying on an external tool breaks the "one assistant" promise and creates data silos, preventing the AI from seamlessly triaging work, drafting replies, and coordinating operations.

  ## Research Report
  - **Chatwoot Source Code Audit**: An audit of the `chatwoot/chatwoot` repository (v3.3+) reveals a robust Ruby on Rails architecture focused on `Account` (tenant), `Inbox`, `Channel`, `Conversation`, `Message`, and `Contact`. It uses WebSockets (ActionCable) for real-time delivery and Sidekiq for background processing (webhooks, email processing).
  - **Competitor Systems**: Tools like Shopify Inbox, WeCom, and Inbox by Zendesk show that deep integration with the platform's core entities (orders, bookings, customers) is essential. An external system like Chatwoot cannot easily reference internal OHC domain objects (e.g., `StaffTask`, `Booking`, `Order`) natively.
  - **OHC Implementation Need**: OHC requires a native Rust implementation in `src/server/ohc/domain/chat` that guarantees strict row-level security (tenant isolation), integrates natively with our AI agent departments (Operations, CS, Sales), and supports high-concurrency WebSocket connections for real-time client updates.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL_ADAPTER : configured_via
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      MESSAGE }o--|| AI_AGENT_DRAFT : optionally_has

      TENANT {
          uuid id
          string name
      }
      INBOX {
          uuid id
          uuid tenant_id
          string name
          boolean greeting_enabled
      }
      CHANNEL_ADAPTER {
          uuid id
          string channel_type
          jsonb credentials
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
          string sender_type
          uuid sender_id
      }
      CONTACT {
          uuid id
          uuid tenant_id
          string name
          string phone_number
      }
  ```

  ### Mobile-First UI Wireframes & UX Flow (375px)
  - **Unified Inbox View**: A clean list view of active conversations. Each row shows the customer name, channel icon (WhatsApp, IG, Web), last message snippet, and unread indicator. Uses macOS translucent materials for the top navigation bar.
  - **Conversation Thread**:
    - **Top Bar**: Customer name, Avatar, and a "Context" button revealing past orders/bookings.
    - **Message Area**: Standard chat bubbles. System events (e.g., "AI CS Agent drafted a reply") appear inline with distinct subtle styling.
    - **Composer**: Input field with native keyboard support. A magic "Sparkle" button triggers the AI Assistant to draft a reply based on business context.

  ### AI Agent Integration Points
  - **Work Triage**: Whenever a `Message` is created by a contact, the AI Job Queue evaluates the urgency and categorizes it (e.g., Inquiry, Complaint, Booking Request).
  - **Customer Assistant (Drafting)**: Listens to new conversations and auto-generates a draft `Message` for the owner to review, leveraging the `tenant`'s memory and the `Contact`'s history.
  - **Operations Assistant**: Can insert system messages directly into the conversation thread (e.g., "Delivery route updated") without exposing internal API complexity.

  ### Key Design Decisions
  - **Multi-Tenancy**: Every table must include `tenant_id` and enforce PostgreSQL Row-Level Security (RLS) to guarantee Zero-Trust isolation.
  - **Real-Time Layer**: Utilize Rust's async ecosystem (e.g., Tokio + Axum WebSockets) combined with Redis Pub/Sub for horizontal scaling of real-time message delivery.
  - **Abstract Channels**: Implement a polymorphic `ChannelAdapter` trait/interface so that adding TikTok or Line in the future doesn't require altering the core `Conversation`/`Message` schema.

  ## Implementation Prompt
  **Role**: Implementer Agent
  **Task**: Build the core data models and service layer for the native Rust Omnichannel Chat Engine in `src/server/ohc/domain/chat`.
  **User-Facing Outcome**: As a business owner (e.g., Maya), I can receive messages from multiple channels in a single inbox within my OHC app, and my AI assistant can read these messages to draft replies.
  **CUJ**:
  1. A webhook payload arrives from an external channel (simulated via API).
  2. The system resolves the `tenant_id` and `inbox_id`.
  3. The system finds or creates a `Contact`.
  4. The system creates a `Conversation` and appends a `Message`.
  5. The real-time event is broadcasted.
  **Acceptance Criteria**:
  - Implement Rust structs, Diesel/SeaORM models (or the project's standard ORM), and database migrations for `Inbox`, `Channel`, `Contact`, `Conversation`, and `Message` ensuring `tenant_id` is present on all.
  - Implement a service function to process an incoming raw message and persist the core entities.
  - Include 100% unit test coverage for the service logic.
  - Add at least one Playwright E2E test verifying that a message created via the backend appears in the UI Unified Inbox view (mocking the external webhook trigger).
  - Ensure all `bazel test //...` checks pass perfectly.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
