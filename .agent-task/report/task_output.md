issue_title: "Architecture Design: Native Rust Omnichannel Chat System"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) is replacing its legacy third-party Rails chat provider with a high-performance Native Rust Omnichannel Chat System. This change is crucial to providing non-technical owner/operators (such as Maya the baker or Carlos the handyman) an unfragmented work assistant that invisibly centralizes all customer communications (Instagram DMs, WhatsApp, SMS, Web Widget, Email) in a single feed without needing technical expertise or external integrations. OHC must own the entire communication stack to guarantee real-time updates, strict multi-tenant isolation, and deep integration with the Operations, Sales, and Knowledge AI Assistant capabilities natively within `onehumancorp/mono`.

  ## Research Report
  - **Codebase & Legacy Source Audit:** We have successfully inspected the legacy open-source Ruby on Rails chat system codebase (specifically models like `conversation.rb`, `contact.rb`, `account.rb` and the `/channel` directory for integrations like SMS, WhatsApp, Web Widget, Twitter).
  - **Market Position:** Competitors like Shopify Sidekick or Zendesk handle these disjointedly or push them to third parties. OHC's differentiation is an 'owner-centered' single inbox. Removing external chat dependencies removes points of failure and unifies the data schema for OHC Agents.
  - **Core Discoveries:**
    - Multi-tenant data structures must employ `tenant_id` at every level (Account -> Inbox -> Contact -> Conversation -> Message).
    - Diverse channel handlers (Email, SMS, Web Widget, WhatsApp, Instagram, FB Page, Line, Telegram) require unified standard interfaces.
    - Real-time updates via WebSockets are paramount for the web and mobile PWA clients.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT {
          uuid id PK
          string name
          jsonb settings
      }
      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
          string channel_type
      }
      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string name
          string email
          string phone_number
      }
      CONVERSATION {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status
      }
      MESSAGE {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          string content
          string message_type
      }
      CHANNEL_ADAPTER {
          uuid id PK
          uuid inbox_id FK
          string provider
          jsonb credentials
      }

      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : manages
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : holds
      INBOX ||--o| CHANNEL_ADAPTER : configured_via
  ```

  ### Mobile UX Flow (375px First)
  - **Inbox Screen:** A clean list view showcasing recent conversations grouped by priority. Uses translucent glass material style. The `tenant_id` filters content implicitly.
  - **Conversation View:** Full-screen chat interface. Sender messages left-aligned, owner messages right-aligned. A prominent bottom sheet provides AI Draft suggestions ("Customer Assistant" capability).
  - **Handoff:** The conversation header includes an "Actions" menu (converted to Quote/Booking/Task).

  ### AI Agent Integration Points
  - **Customer & Relationship Assistant:** Subscribes to incoming `MESSAGE` events. Automatically drafts replies for the owner to approve, maintaining memory of the `CONTACT`.
  - **Work Triage:** Aggregates `CONVERSATION` statuses to surface urgent items in the primary owner feed.
  - **Data Observability:** All agent handoffs are traceable through system-generated messages injected into the `CONVERSATION` timeline, tagged invisibly from standard user views but accessible in an 'owner audit' view if needed.

  ### Key Design Decisions
  - **Native Rust & SeaORM:** Implement the data layer using `sea-orm` within `src/server/integrations/chat` to enforce strict PostgreSQL Row Level Security (RLS) via `tenant_id`.
  - **WebSocket Orchestration:** Leverage `axum` and `tokio-tungstenite` to establish durable connections for real-time `MESSAGE` syncing to the Flutter PWA.
  - **Unified Channel Trait:** Define a Rust Trait (`ChannelAdapter`) that individual channel implementations (WhatsApp, SMS, Web Widget) must implement for receiving/sending messages.

  ## Implementation Prompt
  **Goal:** Build the foundational Native Rust Omnichannel Chat System data layer and service API.
  **CUJ:** An owner (e.g., Maya) opens the OHC mobile app (375px view) and sees a unified inbox containing an incoming web widget message and an Instagram DM, all correctly isolated to her workspace.
  **Acceptance Criteria:**
  1. Implement the database schema (SeaORM entities) for `Tenant`, `Inbox`, `Contact`, `Conversation`, and `Message` ensuring strict `tenant_id` filtering.
  2. Create gRPC/REST API endpoints (Axum) to list inboxes, start a conversation, and send/receive messages.
  3. Implement a generic `ChannelAdapter` trait and a simple in-memory mock or stub for a Web Widget channel to verify the end-to-end flow.
  4. Integrate WebSockets for real-time message delivery to connected clients.
  5. Deliver 100% test coverage for all new models and service methods.
  6. E2E Playwright tests must verify the creation of a conversation and the rendering of messages in the UI.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, core-platform]
assignees: []
