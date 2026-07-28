issue_title: "Architecture: Native Rust Omnichannel Chat Engine (Replacing Chatwoot)"
issue_description: |
  ## Title: Architecture: Native Rust Omnichannel Chat Engine

  ## Problem Statement
  OneHumanCorp (OHC) owners need a single, unified inbox to interact with customers across various channels (Instagram DMs, WhatsApp, SMS, Web Chat, Email, etc.). Previously, OHC relied on an external third-party service, Chatwoot, which introduced integration complexity, latency, external dependency risk, and compromised multi-tenant isolation guarantees. A native omnichannel chat architecture is required to bring these capabilities entirely in-house. It must provide robust real-time communication (WebSockets), support omnichannel data models (inboxes, conversations, messages, channels), and seamlessly integrate with OHC's AI agents to draft replies, execute operations, and extract context directly from the conversation stream.

  ## Research Report
  - **Context**: External Chatwoot dependency is 100% RETIRED. We are replacing it with a native implementation in Rust, leveraging OHC's existing multi-tenant data architecture.
  - **Chatwoot Source Code Audit**:
    - **Core Data Models**:
      - `Inbox`: Aggregates conversations for a specific channel or team.
      - `Conversation`: Represents a thread between a `Contact` and `Assignee`/`Team`. Tracks `status` (open, resolved, snoozed), `snoozed_until`, `priority`, and SLAs.
      - `Message`: Represents an individual message within a conversation. Supports `content_type` (text, attachment, template), `message_type` (incoming, outgoing, activity), and `private` notes.
      - `Contact`: Represents the customer across channels, linked via `contact_inbox`.
    - **Real-time Architecture**: Chatwoot relies on ActionCable (WebSockets) to push real-time updates (message creation, typing status, presence).
    - **Channel Adapters**: Chatwoot abstracts channels (Web Widget, API, Facebook, WhatsApp, Email, Line, SMS) via polymorphic `channel` associations on the `Inbox`.
  - **Competitor Benchmarking**:
    - Shopify Inbox: Deeply integrated with store data (products, orders), allowing agents to easily send product cards or checkout links.
    - Front / Missive: Excellent at unified team collaboration, collision detection, and private internal notes inside threads.
  - **Opportunity for OHC**: A native Rust implementation means OHC AI agents (Customer Assistant, Sales Assistant) can tap directly into the event stream, instantly drafted AI replies, intercept operations intents (e.g., "book an appointment"), and maintain Zero-Trust tenant isolation without third-party API hops.

  ## Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      INBOX ||--o{ CHANNEL : configures
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains
      CONVERSATION }o--o{ AI_AGENT : watched_by

      INBOX {
          uuid id
          uuid tenant_id
          string name
          boolean enable_auto_assign
      }

      CHANNEL {
          uuid id
          string provider
          jsonb credentials
      }

      CONVERSATION {
          uuid id
          uuid inbox_id
          uuid contact_id
          string status
          timestamp snoozed_until
      }

      MESSAGE {
          uuid id
          uuid conversation_id
          uuid sender_id
          string sender_type
          string content
          string content_type
          boolean is_private_note
      }
  ```

  ### Core Components
  1. **Omnichannel Core Service (Rust)**:
     - Handles CRUD for Inboxes, Contacts, Conversations, and Messages.
     - Implements multi-tenant RLS via PostgreSQL (all tables have `tenant_id`).
  2. **WebSocket Gateway (Rust)**:
     - Manages persistent connections to the OHC Mobile and Web clients.
     - Subscribes to Redis Pub/Sub topics for real-time message dispatching (`ohc:chat:tenant_{id}:conversation_{id}`).
  3. **Channel Adapter Workers (Rust/Go)**:
     - Background consumers that adapt incoming webhooks (e.g., from Meta for Instagram/WhatsApp) into internal `Message` entities.
  4. **AI Agent Hooks**:
     - Uses PostgreSQL `SKIP LOCKED` job queues to notify the Customer Assistant of new incoming `Messages`. The AI agent can inject a `Message` with `is_private_note = true` containing a suggested draft, or directly respond based on tenant configuration.

  ### Mobile UX Flow (375px)
  1. **Unified Feed**: The main screen lists active conversations across all channels, prioritized by SLA or urgency. Each item shows an avatar, the channel icon (e.g., IG, Mail), and a snippet of the latest message.
  2. **Conversation View**: Tapping a conversation opens the chat interface. A distinct area at the bottom shows the AI agent's suggested reply (a Translucent Glass card) which the owner can "Send", "Edit", or dismiss.
  3. **Context Drawer**: Swiping left from the right edge (or tapping a header icon) reveals the Contact's history, recent orders, and associated tasks.
  4. **Action Bar**: Below the input field, quick actions (Send Quote, Request Payment, Share Product) are easily reachable with a thumb.

  ### Key Design Decisions
  - **Native Real-Time**: Utilizing Rust's `tokio` and `tungstenite` for high-concurrency WebSocket management ensures low latency and minimal resource overhead compared to ActionCable.
  - **AI First Integration**: Treating AI agents as specialized "participants" or background workers that can insert private drafts (`is_private_note: true`) natively integrates AI without cluttering the customer-facing message history.
  - **Strict Multi-Tenancy**: The database schema strictly enforces `tenant_id` at every level (Inbox, Conversation, Message) to guarantee Zero-Trust data isolation.

  ## Implementation Prompt
  **User Persona**: Maya, the baker, receives an Instagram DM asking about vegan cake availability. She needs her OHC app to immediately notify her, show the message in her unified inbox, and present a pre-drafted AI reply confirming she offers vegan options.

  **Critical User Journey (CUJ)**:
  1. The system ingests an external message (simulated webhook).
  2. A new `Conversation` and `Message` are created in the database.
  3. The WebSocket server pushes the new message event to the connected frontend client.
  4. The AI Customer Assistant worker detects the new message and inserts a drafted response as a private note in the same conversation.
  5. The frontend UI displays the conversation with the new message and the AI's suggested draft.

  **Acceptance Criteria**:
  - Implement the core database schema for `Inbox`, `Conversation`, and `Message` in the backend service, ensuring multi-tenant RLS (row-level security).
  - Implement a basic WebSocket endpoint that clients can connect to, and which broadcasts new messages to active connections for that tenant/conversation.
  - Implement a background worker (or stub) that simulates the AI agent drafting a reply when a new customer message arrives.
  - Implement the unified inbox UI and conversation view in the Flutter frontend, displaying incoming messages and the AI draft, with responsive 375px mobile-first layouts.
  - Write Playwright E2E tests covering the creation of a message via API, its real-time appearance in the UI, and the display of the AI drafted response. No external Chatwoot APIs are to be mocked or used.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
