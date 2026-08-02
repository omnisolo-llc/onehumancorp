issue_title: "Architect & Implement Native Rust Omnichannel Chat System (the legacy external dependency Replacement)"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) needs a native, high-performance omnichannel chat system built in Rust to entirely replace the external dependency on the legacy external dependency. Currently, relying on an external service limits our ability to enforce strict tenant isolation, integrate deeply with our AI Agent triage system, and optimize for mobile-first low-latency operations. The owners (like Maya, Carlos, and Priya) need a seamless, real-time inbox where WhatsApp, Instagram DMs, SMS, and Web Widget chats flow into a unified interface, natively deeply integrated with OHC's internal agent orchestrator (KAIROS).

  ## Research Report
  Based on an architectural audit of the legacy external dependency's source code and industry standards (e.g., Shopify Inbox, Stripe, Intercom):
  - **Data Models:** the legacy external dependency uses a robust polymorphic channel model. Core entities include `Account` (Tenant), `Inbox` (Channel configuration), `Conversation`, `Message`, and `Contact`. This allows a unified inbox experience regardless of the origin channel.
  - **Real-Time Messaging:** the legacy external dependency uses Ruby's ActionCable for WebSockets. For our Rust native implementation, we will utilize `axum` with `tokio-tungstenite` to achieve an order of magnitude lower latency and higher concurrent connection limits, crucial for high-traffic merchants.
  - **Multi-Tenancy:** We require strict Row Level Security (RLS). Every entity must have a `tenant_id` linked securely to the authenticated owner session.
  - **AI Agent Integration:** Unlike the legacy external dependency, which treats AI as an external bot, OHC's chat system must natively treat AI agents as first-class citizens. Messages must flow into an AI Job Queue for KAIROS to triage before routing to the human owner.
  - **Mobile-First UX:** The unified inbox must work beautifully on a 375px viewport (mobile), allowing owners to quickly act (draft replies, send payment links) without horizontal scrolling.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      TENANT ||--o{ CONTACT : has
      INBOX ||--o{ CONVERSATION : receives
      CONTACT ||--o{ CONVERSATION : participates_in
      CONVERSATION ||--o{ MESSAGE : contains

      TENANT {
          uuid id PK
          string name
      }
      INBOX {
          uuid id PK
          uuid tenant_id FK
          string channel_type "whatsapp, web_widget, sms, ig_dm"
          jsonb credentials
      }
      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string name
          string phone_number
          string email
      }
      CONVERSATION {
          uuid id PK
          uuid inbox_id FK
          uuid contact_id FK
          string status "open, resolved, snoozed"
          timestamp KAIROS_triage_time
      }
      MESSAGE {
          uuid id PK
          uuid conversation_id FK
          string content
          string sender_type "contact, owner, agent"
          boolean is_private_note
          timestamp created_at
      }
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  1. **Unified Inbox List:**
     - A full-width (375px) list view.
     - Each list item displays contact avatar, channel icon (WhatsApp, Web), a snippet of the latest message, and a status token (e.g., "Agent Drafted", "Needs Action").
     - Ubiquiti UniFi-style clean cards with subtle translucent glass elements.
  2. **Conversation View:**
     - Tapping a conversation slides in the chat view.
     - Sticky bottom input field utilizing native mobile keyboards.
     - Floating Action Button (FAB) or Action Bar above the input field containing contextual "Assistant Actions" (e.g., "Generate Quote", "Request $50 Deposit").
  3. **Handoff UX:**
     - AI drafts are shown inline with a distinct visually highlighted container (e.g., a dashed border or glowing tint) requiring a tap to "Approve & Send" or "Edit".

  ### AI Agent Integration Points
  - **Ingestion:** When a webhook (e.g., WhatsApp API) receives a message, it is persisted to the DB and an event is pushed to the Redis AI Job Queue.
  - **Triage:** The Work Triage agent parses the message, identifies intent (e.g., order inquiry), and either drafts a response in the `MESSAGE` table with `sender_type="agent"` or triggers a specific capability (e.g., Sales Assistant).
  - **Owner Notification:** The WebSocket server pushes the KAIROS state update to the owner's mobile PWA, immediately updating the UI.

  ### Key Design Decisions
  - **Rust + Axum for WebSockets:** Provides optimal memory safety and performance for concurrent real-time connections compared to legacy architectures.
  - **Strict PostgreSQL RLS:** Database-level enforcement ensures cross-tenant data leaks are impossible, even if application logic fails.
  - **Polymorphic Channels:** A single `INBOX` table supports extending to new channels (Instagram, Email, etc.) without altering the core `MESSAGE` or `CONVERSATION` schemas.
  - **Offline Tolerance:** The frontend will use optimistic UI updates and robust retry mechanisms for mobile network flakiness.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Your task is to build the foundational backend and database layer for the Native Rust Omnichannel Chat System inside OHC.
  1. Define the PostgreSQL migrations for `inboxes`, `contacts`, `conversations`, and `messages`, ensuring `tenant_id` is present on all tables with `ENABLE ROW LEVEL SECURITY`.
  2. Implement the Rust service layer (e.g., using `sqlx` or `diesel`) to handle basic CRUD operations for these entities.
  3. Create an Axum WebSocket handler that allows authenticated owners to connect and receive real-time message events for their tenant.
  4. Ensure 100% unit test coverage for the new services and controllers. Provide a full Playwright E2E test verifying a mock WhatsApp incoming message reaching the Web UI via WebSockets.
  5. Adhere strictly to the mobile-first UX guidelines; ensure the API responses are optimized for low bandwidth.

  **Acceptance Criteria:**
  - Migrations execute cleanly and enforce RLS.
  - Rust API can create messages and broadcast them via WebSockets.
  - 100% test pass rate in CI.
  - No external the legacy external dependency dependencies exist in the code path.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
