issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  ## Problem Statement
  OHC currently relies on external systems (or lacks a deep, native integration) for omnichannel chat and customer support. Our personas—especially Maya (baker managing Instagram DMs), Carlos (handyman handling WhatsApp/SMS inquiries), and Fatima (food cart taking varied pre-orders)—need a unified inbox that brings all customer communications into one place. Relying on an external service like Chatwoot introduces latency, state synchronization issues, limits multi-tenant isolation guarantees, and complicates our AI agents' ability to intercept, understand, and auto-reply to messages directly within the platform. We need a native Rust implementation of a high-performance omnichannel inbox directly in `onehumancorp/mono`.

  ## Research Report
  - **Chatwoot Architecture Assessment:** We audited the Chatwoot source code (`https://github.com/chatwoot/chatwoot`). The core primitives in Chatwoot include:
    - `Inbox`: The central hub for a specific channel (e.g., an Instagram account or a WhatsApp number) belonging to an account.
    - `Conversation`: A thread of messages associated with a Contact and an Inbox. Includes metadata like status (open, resolved, snoozed), assignee, labels, and SLAs.
    - `Message`: The individual piece of communication, categorized by type (incoming, outgoing, template, activity).
    - `Contact`: The end-user communicating across channels.
    - **Channel Adapters**: Chatwoot relies heavily on polymorphic associations (e.g., `channel_id` and `channel_type` in `inboxes`) to route to specific channel implementations (e.g., `Channel::Whatsapp`, `Channel::Email`, `Channel::WebWidget`).
  - **OHC Gaps:** OHC needs these same primitives but built natively in Rust. Our AI agent workflows (the "Customer Assistant" and "Work Triage" agents) require direct, low-latency access to the `Conversation` and `Message` tables to read context and draft replies. A native implementation ensures that data never leaves our multi-tenant PostgreSQL boundary until sent to the customer, adhering to our zero-trust and Spiffe/Spire models.

  ## Design Doc
  ### Architecture
  We will introduce a new `omnichannel` module within `src/server/domain/omnichannel/` (and corresponding tables/services) that replicates the core functionality of Chatwoot in Rust.

  **Data Model (Mermaid.js Entity-Relationship)**
  ```mermaid
  erDiagram
      TENANT {
          uuid id PK
          string name
      }
      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string name
          string email
          string phone_number
          jsonb custom_attributes
      }
      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
          string channel_type "e.g., WEB_WIDGET, WHATSAPP, INSTAGRAM"
          jsonb channel_config
      }
      CONVERSATION {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status "OPEN, RESOLVED, SNOOZED"
          uuid assignee_id "Optional human or agent"
          timestamp last_activity_at
      }
      MESSAGE {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          uuid sender_id "Polymorphic: Contact or User/Agent"
          string sender_type "CONTACT, USER, AGENT"
          string message_type "INCOMING, OUTGOING, ACTIVITY"
          text content
          jsonb external_source_ids "e.g., FB message ID"
      }

      TENANT ||--o{ CONTACT : "owns"
      TENANT ||--o{ INBOX : "owns"
      TENANT ||--o{ CONVERSATION : "owns"
      TENANT ||--o{ MESSAGE : "owns"
      INBOX ||--o{ CONVERSATION : "contains"
      CONTACT ||--o{ CONVERSATION : "participates in"
      CONVERSATION ||--o{ MESSAGE : "contains"
  ```

  ### Mobile UX Flow (375px)
  1. **Unified Inbox List:** Owner opens the app and sees the "Messages" tab. A clean, list view showing recent conversations with preview text, unread badges, and channel icons (e.g., a tiny Instagram logo next to Maya's customer).
  2. **Conversation View:** Tapping a conversation opens a standard chat interface. The interface includes:
     - Messages bubbled by sender.
     - A sticky text input area at the bottom.
     - An "AI Draft" floating action button (or inline suggestion) that generates a reply based on the context and the owner's previous actions.
  3. **Context Panel (Drawer):** Swiping from the right (or tapping a "details" icon) reveals the Contact's history, previous orders, and custom notes.

  ### AI Agent Integration
  - **Work Triage Agent:** Hooks into the `Message` creation event. When a new incoming message arrives, it evaluates priority and updates the `Conversation` status or assigns it to the `Customer Assistant` agent.
  - **Customer Assistant Agent:** Reads the `Conversation` history and drafts a response in the background. It creates a `Message` with a special state (e.g., `DRAFT`) that the owner can review, edit, and send.

  ## Implementation Prompt
  **Goal:** Implement the core backend data models and gRPC/REST APIs for the native Rust omnichannel chat system.

  **Critical User Journey (CUJ):**
  1. A tenant owner (like Maya) creates a new `Inbox` representing her Web Widget.
  2. A new `Contact` initiates a chat.
  3. A `Conversation` is created for that contact in the inbox.
  4. The contact sends an incoming `Message`.
  5. The owner views the conversation and sends an outgoing `Message`.

  **Acceptance Criteria:**
  - Create the PostgreSQL schema migrations for `contacts`, `inboxes`, `conversations`, and `messages`, ensuring rigorous row-level security (`tenant_id`).
  - Implement the Rust struct models and Axum/Tonic API endpoints to support CRUD operations on these entities.
  - Integrate a basic WebSocket or Server-Sent Events (SSE) mechanism for real-time message delivery to the UI.
  - Write comprehensive unit tests for the domain logic and integration tests ensuring tenant isolation.
  - *Note:* Do not prescribe exact function signatures; design the system to fit seamlessly into the existing `src/server/domain` and API architecture.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
