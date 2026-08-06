issue_title: "Architecture & Implementation: Native Omnichannel Chat & Web Widget Platform"
issue_description: |
  # Native Omnichannel Chat & Web Widget Platform

  ## Problem Statement
  OneHumanCorp (OHC) is replacing the legacy solution with a high-performance, multi-tenant omnichannel customer support & chat engine built natively in Rust. Our business owners (Maya, Carlos, Priya, Leo, Fatima, Nora) need a seamless, non-technical way to unify conversations across channels (Web, Email, Instagram, WhatsApp, SMS). Currently, the application relies on an external integration which is being retired. We must build the native Rust microservices, the database schemas (PostgreSQL row-level security enabled), the channel adapters, and the web chat widget to provide a complete "in-house" experience.

  ## Research Report
  - **Competitor Analysis:** Solutions like Shopify Inbox and Intercom provide robust multi-channel capabilities but often suffer from heavy UI overhead for the end-user.
  - **Source Code Audit:** We need a robust approach with detailed models for `conversations`, `messages`, `contacts`, `inboxes`, and `channel_web_widgets`.
  - **OHC Missing Capability:** A native Rust engine that mirrors this capability with strict multi-tenant isolation, utilizing our Zero-Trust architecture, and allowing for AI agent interventions (e.g., auto-replying for Maya's cake requests).

  ## Design Doc (Architecture)

  ### Architecture Diagram (Mermaid)
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      INBOX ||--o{ CHANNEL : configures
      CHANNEL ||--o{ CONVERSATION : routes
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains
      TENANT {
          uuid id
          string name
      }
      INBOX {
          uuid id
          uuid tenant_id
          string name
      }
      CHANNEL {
          uuid id
          uuid inbox_id
          string channel_type "WEB_WIDGET, WHATSAPP, EMAIL"
          jsonb config
      }
      CONTACT {
          uuid id
          uuid tenant_id
          string name
          string email
          string phone
      }
      CONVERSATION {
          uuid id
          uuid tenant_id
          uuid inbox_id
          uuid contact_id
          string status "OPEN, CLOSED, SNOOZED"
      }
      MESSAGE {
          uuid id
          uuid conversation_id
          uuid tenant_id
          string content
          string sender_type "CONTACT, AGENT, SYSTEM"
      }
  ```

  ### Mobile UX Flow (375px First)
  - **Owner View:**
    - A centralized "Unified Inbox" screen with a list of active conversations, clearly labeled by channel (e.g., a WhatsApp icon for Maya's cake inquiry).
    - Tapping a conversation opens a chat view. Native keyboard integration with a prominent "Reply" bar.
    - An AI suggestion pill appears above the keyboard ("Drafting reply based on menu...").
  - **Customer View (Web Widget):**
    - A sticky FAB (Floating Action Button) on the owner's storefront.
    - Expands into a compact chat window. Requires no login (session cookie/local storage based visitor ID).

  ### AI Agent Integration Points
  - **Customer & Relationship Assistant:** Listens to new `CONVERSATION_CREATED` and `MESSAGE_RECEIVED` events on the internal queue. Generates draft replies stored as `MESSAGE` with `status: DRAFT` or auto-sends based on tenant configuration.

  ### Key Design Decisions
  - **Data Isolation:** All tables include a `tenant_id` and PostgreSQL Row-Level Security (RLS) is strictly enforced.
  - **Real-time:** WebSocket connections are routed through the unified message bus, delivering events to connected clients instantly.
  - **Rust Native:** The backend services for the channels and inbox will be written in Rust within `src/server/integrations` and `src/server/ohc`, replacing external webhooks.

  ## Implementation Prompt
  **Goal:** Implement the foundational database schema, Rust domain models, and the core gRPC/REST API for the Native Omnichannel Inbox.

  **Acceptance Criteria:**
  1. Define the PostgreSQL schema migrations for `inboxes`, `channels`, `contacts`, `conversations`, and `messages`, ensuring RLS is enabled on `tenant_id`.
  2. Implement the Rust domain models and repository layer using `sqlx` (or the existing DB framework in `src/server/db`).
  3. Create the API endpoints (or gRPC services) for:
     - Creating an Inbox & configuring a Web Widget channel.
     - Creating a Contact and initiating a Conversation.
     - Sending a Message within a Conversation.
  4. Build a basic pre-compiled JS snippet for the Web Widget that can be embedded in a static HTML page, which connects to the message API.
  5. 100% Unit test coverage on the Rust backend logic.
  6. At least 5 Playwright E2E tests verifying the creation of a channel, initiating a chat from the widget, and the owner receiving the message in the unified inbox API.

  ## Priority: P0 (Critical Path for Native Architecture Transition)
  ## Estimated Scope: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, rust, chat]
assignees: []
