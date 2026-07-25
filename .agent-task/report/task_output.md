issue_title: "Implement Native Rust Omnichannel Chat (Support Engine)"
issue_description: |
  ## Problem Statement
  OHC currently relies on external systems for customer support and omnichannel chat. To ensure strict multi-tenant isolation, Zero-Trust security (SPIFFE/SPIRE), high performance, and deep integration with our AI Work Assistant capabilities without external dependencies, we must retire third-party chat services completely. OHC needs a native, highly performant Rust-based omnichannel chat engine that works seamlessly on low-data 375px mobile viewports for users like Maya, Carlos, and Fatima.

  ## Research Report
  I performed a codebase audit to benchmark the core architectural features needed for omnichannel support. We must implement the following key domain models:
  - **Account**: The core multi-tenant boundary.
  - **User & Contact**: Users (agents) and Contacts (customers).
  - **Inbox & Channel**: Routing endpoints. Channels include Web Widget, API, Email, SMS, WhatsApp, Instagram, etc.
  - **Conversation & Message**: The core interaction models with support for attachments, macros, canned responses, and CSAT surveys.
  - **Real-time WebSockets**: Leveraging Tokio/Tungstenite (Rust).

  Unlike existing solutions, OHC requires native AI agent (Customer & Relationship Assistant) intervention that can silently draft replies, auto-categorize intents, and escalate when confidence is low. OHC also requires row-level multi-tenancy enforced at the database level (`tenant_id` on every table) and Redlock for distributed coordination among AI sub-agents.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : owns
      Tenant ||--o{ Contact : owns
      Inbox ||--o{ Channel : configures
      Inbox ||--o{ Conversation : contains
      Contact ||--o{ Conversation : initiates
      Conversation ||--o{ Message : has
      Conversation }|--|| Agent : assigned_to

      Tenant {
          uuid id
          string name
      }
      Inbox {
          uuid id
          uuid tenant_id
          string name
          boolean is_active
      }
      Channel {
          uuid id
          uuid inbox_id
          string type
          json config
      }
      Conversation {
          uuid id
          uuid inbox_id
          uuid contact_id
          string status
      }
      Message {
          uuid id
          uuid conversation_id
          string content
          string sender_type
          uuid sender_id
      }
  ```

  ### UI Wireframes & 375px Mobile Flow
  - **Inbox View (375px):** A clean, un-cluttered feed of active conversations. Adheres to OHC Premium Token library with translucent glass materials. Sticky top header. Each item in the list shows customer name, channel icon (e.g., IG, WhatsApp), unread indicator, and last message snippet.
  - **Conversation View (375px):** Native mobile keyboard optimization. Messages flow from bottom to top. An AI suggestion chip floats above the text input ("AI Draft: Yes, we do vegan cakes...").
  - **UX Flow:** Maya opens the OHC app. Taps "Messages". Taps the unread IG DM from a customer. Reads the AI-drafted reply. Taps "Approve & Send". The message is sent to IG via the native Rust channel adapter.

  ### Mobile UX Flow
  - **Low-Data Tolerance:** Read paths cache locally. Writes (messages) are optimistically appended to the UI with a pending state, then synced in the background. Flaky network retries are handled silently.
  - **Touch Targets:** All message bubbles, approval buttons, and navigation elements are minimum 44x44px.

  ### AI Agent Integration Points
  - **Customer & Relationship Assistant:** Listens to the `message_created` event (via PostgreSQL `SKIP LOCKED` job queue). If the message is from a customer, the agent retrieves context (past orders, CRM notes), drafts a reply, and inserts a `Message` with `status = draft`.
  - **Notification Agent:** Determines if the owner needs immediate push notification based on urgency (e.g., catering request for tomorrow).

  ### Key Design Decisions
  - **Backend:** Natively implemented in Rust (`onehumancorp/mono/src/server/chat`). Uses `tokio` for async runtime, `axum` for HTTP/WS, and `sqlx` for PostgreSQL.
  - **Data Isolation:** All queries must implicitly filter by `tenant_id` via Row-Level Security (RLS) in PostgreSQL.
  - **Real-time:** WebSockets managed by Rust, broadcasting events (message created, typing indicator) to connected clients via Redis Pub/Sub.

  ## Implementation Prompt
  **To the Implementer:**
  Implement the native Rust omnichannel chat backend and the corresponding Flutter/PWA UI.
  - **Goal:** Replace third-party services with a 100% native Rust chat engine integrated with our AI assistant.
  - **CUJ:** As Maya (a baker), I want to receive an Instagram DM in my OHC inbox, see an AI-drafted reply, and tap "Send" to respond, so that I can handle customer inquiries quickly from my phone.
  - **Acceptance Criteria:**
    1. Rust API endpoints exist for creating inboxes, listing conversations, and sending messages.
    2. Real-time WebSocket connection pushes new messages to the UI.
    3. UI is fully responsive on 375px viewports (no horizontal scroll).
    4. Playwright E2E test added demonstrating a customer message arriving and the owner sending a reply.
    5. Zero mocked data in the UI; everything flows through the real Rust backend and Postgres.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
