issue_title: "Implement Native Rust Omnichannel Inbox & Chat Engine"
issue_description: |
  # Native Rust Omnichannel Chat System Architecture

  ## Problem Statement
  OHC previously relied on Chatwoot as an external third-party service for its omnichannel customer support and chat functionality. Chatwoot has been 100% retired. This creates a critical architectural gap: Maya (the home baker) and Carlos (the field service owner) need a unified inbox to triage Instagram DMs, WhatsApp messages, and website web-widget chats in real-time. Without a native, high-performance omnichannel inbox, the OHC AI agents cannot intercept, draft replies, or auto-route messages, and the business owner is forced to context-switch across multiple disparate apps, breaking the "One Human Corp" promise.

  ## Research Report
  I checked out and audited the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) to understand its architecture.
  Chatwoot's core domain models consist of:
  - **Inboxes**: Centralized configurations for different communication channels (e.g., WhatsApp, Web Widget, API).
  - **Channels**: Specific polymorphic channel adapters (`channel_whatsapp`, `channel_web_widgets`, `channel_instagram`) holding channel-specific credentials and configurations.
  - **Contacts**: The end-customer sending the messages.
  - **Conversations**: A stateful message thread tied to an `Inbox` and a `Contact`, with statuses (`open`, `snoozed`, `resolved`).
  - **Messages**: Immutable message records referencing external source IDs, with content types, attachments, and sender polymorphism (User/Agent vs. Contact).

  Comparing this to OHC's needs: we need a multi-tenant, Rust-based engine that enforces strict row-level security (RLS) by `tenant_id`. It must expose WebSockets for real-time web widget communication, and HTTP webhooks for Meta (WhatsApp/Instagram) integration.

  ## Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      INBOX ||--o{ CHANNEL_ADAPTER : configured_by
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      MESSAGE }o--|| AGENT : drafted_by

      TENANT {
          uuid tenant_id PK
          string name
      }
      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
          boolean enable_auto_assignment
      }
      CHANNEL_ADAPTER {
          uuid id PK
          uuid inbox_id FK
          string type "WhatsApp, WebWidget, Instagram"
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
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status "open, snoozed, resolved"
          datetime last_activity_at
      }
      MESSAGE {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          string sender_type "Contact, User, Bot"
          uuid sender_id
          text content
          string status "sent, delivered, read"
      }
  ```

  ### Mobile UX Flow (375px first)
  1. **Triage Feed (Home):** The owner opens the app. A notification bubble shows "3 New Messages".
  2. **Unified Inbox:** Tapping the bubble opens a unified list of conversations, intermingling WhatsApp, Web Widget, and Instagram DMs. Each list item shows the Contact name, channel icon, and snippet of the latest message or AI draft.
  3. **Conversation View:** Tapping a conversation opens the chat interface. It uses native mobile keyboards. The top shows the Contact's name. The bottom has an input field.
  4. **AI Assistant Integration:** Above the input field, a glowing "AI Draft" button appears if the Customer & Relationship Assistant has pre-drafted a reply based on the context (e.g., quoting a cake price).
  5. **Action:** The owner taps "Send" (or approves the AI draft), and the message is instantly routed through the Rust backend to the appropriate channel.

  ### AI Agent Integration Points
  - **Work Triage:** When a `MESSAGE` is inserted, an asynchronous Rust background job (using PostgreSQL `SKIP LOCKED` or Redis queues) is triggered. The Triage agent evaluates if the message is urgent.
  - **Customer & Relationship Assistant:** Subscribes to the same event queue. It reads the `CONVERSATION` history, fetches the `CONTACT`'s past orders or preferences, and inserts a `MESSAGE` with `sender_type = Bot` and `status = drafted`. This draft is pushed to the owner's UI via WebSocket.

  ### Key Design Decisions
  - **Native Rust Implementation:** High concurrency for WebSockets and fast webhook processing.
  - **Tenant Isolation:** Every table (`inboxes`, `contacts`, `conversations`, `messages`) MUST have a `tenant_id` column to enforce PostgreSQL RLS.
  - **Unified Message Model:** A single `messages` table handles all channels, using `sender_type` (Contact/User/Bot) to determine alignment in the UI, keeping the schema simple but flexible.

  ## Implementation Prompt
  **Role:** Backend Implementer (Rust) & Frontend Implementer (Flutter/PWA)
  **Objective:** Implement the foundational Native Rust Omnichannel Chat System based on the Chatwoot architecture, and wire it up to the mobile-first OHC Flutter/Next UI.
  **CUJ:** Maya receives a message on her web widget. She sees it in her OHC app's unified inbox and replies from her phone.
  **Acceptance Criteria:**
  - Create the PostgreSQL schema for `inboxes`, `contacts`, `conversations`, and `messages` in the Rust backend module (`src/server/integrations/chat`). Ensure `tenant_id` RLS is applied.
  - Implement the Rust gRPC/REST endpoints to list conversations and messages.
  - Implement the WebSocket handler in Rust for real-time web widget messages.
  - Build the Mobile-First (375px) Unified Inbox UI in the frontend, featuring translucent glass styling and UniFi dashboard layouts.
  - Add at least five Playwright E2E tests verifying the end-to-end chat flow (widget -> inbox -> reply).
  - Provide 100% unit test coverage for the new Rust models and API handlers.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
