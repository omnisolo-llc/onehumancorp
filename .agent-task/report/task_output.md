issue_title: "Architecture Design: Native Rust Omnichannel Chat System"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) requires an integrated, high-performance omnichannel chat system. Historically, businesses stitch together multiple tools to handle WhatsApp, web widget, Instagram DMs, etc., leading to fragmentation. Our goal is to build a native Rust chat system within OHC, completely replacing any external dependencies like Chatwoot, while providing AI triage and agent capabilities directly over these channels. This is essential for non-technical operators (like Maya and Carlos) who need a unified inbox that requires zero configuration.

  ## Research Report
  We conducted an architectural audit of Chatwoot's Ruby on Rails source code, specifically reviewing its models, channels, controllers, and real-time infrastructure. Chatwoot centers around a deeply relational model: `Account`, `Inbox`, `Channel` (WhatsApp, WebWidget, etc.), `Contact`, `Conversation`, and `Message`.

  Key Insights from the Chatwoot Audit:
  - **Inboxes and Channels:** Accounts have multiple Inboxes, each tied to a specific Channel type. This abstraction allows the core `Conversation` and `Message` models to remain channel-agnostic.
  - **Conversations:** Link a `Contact` with an `Inbox`.
  - **Messages:** Handle various types (incoming, outgoing, template) and content types.
  - **Real-time Engine:** Uses WebSockets for live chat widgets and internal dashboards.

  For OHC, building this natively in Rust provides a massive performance advantage, especially for background AI agent processing. We will leverage our existing Postgres RLS (tenant_id) to ensure strict data isolation.

  ## Design Doc
  ### Architectural Design (Rust + Postgres)

  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      INBOX ||--o{ CONVERSATION : has
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains

      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
          string channel_type "whatsapp | web_widget | instagram"
          jsonb config
          datetime created_at
      }

      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string identifier "e.g., phone or email"
          string name
          datetime created_at
      }

      CONVERSATION {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status "open | resolved | snoozed"
          datetime created_at
      }

      MESSAGE {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          string content
          string message_type "incoming | outgoing | template"
          datetime created_at
      }
  ```

  ### Mobile UX Flow (375px First)
  1. **Unified Inbox View:** A sticky bottom navigation bar with an "Inbox" tab. Tapping it shows a consolidated list of `Conversations`, categorized by "Action Needed" vs "Resolved".
  2. **Conversation View:** A chat interface showing messages in bubbles. AI agent drafts appear with a translucent styling (macOS Translucent Glass) and an "Approve & Send" button.
  3. **Contact Context Sheet:** Swiping left on a conversation reveals a drawer with customer details, order history, and AI-generated summaries, enabling quick context without leaving the chat.

  ### AI Agent Integration Points
  - **Work Triage:** When a new `Message` arrives, the Triager agent classifies intent, updates `Conversation` priority, and summarizes the request.
  - **Customer & Relationship Assistant:** Automatically generates draft responses based on past context, placing them as pending `Messages` for the owner to approve.
  - **Operations Coordination:** If a message implies a booking or order inquiry, agents interact with the scheduling/order services and inject interactive quote/booking cards into the chat flow.

  ## Implementation Prompt
  **Goal:** Implement the foundational Rust data models, migrations, and CRUD API for the native Omnichannel Chat system in `src/server/ohc/chat/`.

  **Tasks:**
  1. Create Postgres migrations for `inboxes`, `contacts`, `conversations`, and `messages` tables, ensuring they all include `tenant_id` and have Row Level Security (RLS) enabled.
  2. Implement the corresponding Rust struct models using SQLx.
  3. Build the backend gRPC/REST endpoints for querying and mutating conversations and messages.
  4. Ensure all new endpoints include comprehensive unit and integration tests under `bazel test //...`.
  5. The API should be fully tested via Playwright if a simple UI test fixture is added.

  **Acceptance Criteria:**
  - Database schema enforces multi-tenant isolation via RLS.
  - CRUD operations function correctly and pass all Rust tests.
  - No external chat dependencies are introduced.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture]
assignees: []
