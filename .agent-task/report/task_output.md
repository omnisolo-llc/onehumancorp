issue_title: "Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  # Mission Queue Protocol: Native Rust Omnichannel Chat System

  ## Problem Statement
  Currently, OHC relies on Chatwoot as an external third-party service for omnichannel customer support and messaging (WhatsApp, Email, Instagram, Facebook, Web Widget, etc). This introduces latency, breaks our strict multi-tenant Data/RLS models, prevents deep AI agent integration, and complicates the system architecture. We need to fully retire Chatwoot and build a native, high-performance omnichannel chat system in Rust directly within the `onehumancorp/mono` repository that our AI assistants can leverage to triage, draft, and respond to customers.

  ## Research Report
  - **Source Code Audit (Chatwoot):** Chatwoot's architecture relies on several core models:
    - `Account`, `AccountUser`, `User` (Tenancy & Auth)
    - `Contact`, `ContactInbox` (Customer identities across channels)
    - `Inbox`, `Channel::*` (Routing and channel configuration)
    - `Conversation` (Thread of messages)
    - `Message` (Individual chat bubbles, attachments, statuses)
  - **OHC Equivalents:**
    - Tenancy: `tenant_id` isolation using PostgreSQL Row-Level Security (RLS).
    - Identity: `customer_profile`, `customer_memory_context` tables.
    - Inbox/Routing: The `work_item` table and `agent_draft` tables recently added in `20260701_omnichannel_tables.sql`.
  - **Competitor Systems Audit:** Shopify Inbox and Stripe heavily use WebSockets for real-time edge connections, integrating deeply into their unified backend. By moving to Rust, we can handle thousands of concurrent WebSocket connections for web widgets with minimal memory overhead, using Redis for pub/sub message broadcasting across our distributed services.

  ## Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      TENANT ||--o{ CUSTOMER_PROFILE : has
      TENANT ||--o{ INBOX : manages
      INBOX ||--o{ CHANNEL : configures
      CUSTOMER_PROFILE ||--o{ CONTACT_INBOX : links
      INBOX ||--o{ CONVERSATION : contains
      CONTACT_INBOX ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      MESSAGE ||--o| AGENT_DRAFT : drafted_by

      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
          boolean enable_auto_assignment
      }
      CHANNEL {
          uuid id PK
          uuid inbox_id FK
          string provider_type "whatsapp, facebook, api, widget"
          jsonb credentials
      }
      CONVERSATION {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_inbox_id FK
          string status "open, resolved, snoozed"
      }
      MESSAGE {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          text content
          string sender_type "customer, agent, ai"
          string status "sent, delivered, read, failed"
      }
  ```

  ### Mobile UX Flow (375px First)
  1. **Work Command Center (Home):** The owner opens the app. A clean, Apple/Ubiquiti-style unified feed shows "3 New Inquiries" (WhatsApp, Insta, Web).
  2. **Conversation View:** Tapping an inquiry slides in the chat view.
     - Sticky top header (Customer Name, Channel Icon, Status).
     - Scrollable message list with translucent glass bubbles. AI-drafted replies appear as "ghosted" bubbles with a spark icon.
     - Bottom input bar: Native mobile keyboard, attachment button, and a prominent "Approve AI Draft" button.
  3. **AI Context:** Swiping left from the chat reveals the Customer Context card (past orders, lifetime value, previous tickets).

  ### AI Agent Integration Points
  - **Operations Agent (Triage):** Listens to new `MESSAGE` inserts. Auto-categorizes and tags the `CONVERSATION`.
  - **Customer & Relationship Agent (Drafter):** Uses RAG (Retrieval-Augmented Generation) on past `customer_profile` and `message` history to generate an `AGENT_DRAFT`.
  - **Decisions Agent:** Monitors response times and sentiment, providing daily summaries.

  ### Key Design Decisions
  - **Complete Chatwoot Retirement:** No more external webhooks to Chatwoot. All incoming webhooks (WhatsApp Cloud API, Meta Graph API) hit our Rust `src/server/integrations/chat` API directly.
  - **Native Rust Real-time:** Use `gorilla/websocket` (Go) or Rust equivalent async WebSockets (`tokio` + `tungstenite` or Axum) for the Web Widget, with Redis Pub/Sub for cross-node broadcasting.
  - **Zero Trust & Security:** Strict RLS on all new tables (`inbox`, `channel`, `conversation`, `message`). Every query must include the `tenant_id` invariant.

  ## Implementation Prompt
  **To the Implementer:**
  Implement the core database schema and the Rust backend API for the new Native Omnichannel Chat System.
  1. Create a migration in `src/server/db/migrations` to add `inbox`, `channel`, `contact_inbox`, `conversation`, and `message` tables. Ensure strict PostgreSQL RLS policies are applied to all tables using `app.current_tenant_id`.
  2. Implement the corresponding Rust models, repository layers, and CRUD API endpoints in `src/server/integrations/chat` and `src/server/api`.
  3. Create an incoming webhook handler for WhatsApp Cloud API that parses incoming text messages and inserts them into the `message` and `conversation` tables.
  4. Ensure 100% unit test coverage for the new services and verify with `bazel test //...`.
  Do NOT integrate Chatwoot SDKs. This is a complete native replacement.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
