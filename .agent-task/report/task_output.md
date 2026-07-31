issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  # Research Report: Custom Rust Omnichannel Chat System

  ## Problem Statement
  OneHumanCorp currently has a dependency on external third-party services like Chatwoot for omnichannel customer support and chat functionality. Relying on external services breaks our multi-tenant isolation, Zero-Trust security guarantees, and adds external operational complexity. Maya (the baker), Carlos (the handyman), and Nora (the agency principal) need to manage all customer communications (Instagram DMs, website chat, SMS) directly within OHC's unified inbox without external vendor complexity.

  ## Research Report
  Based on auditing the Chatwoot source code (`https://github.com/chatwoot/chatwoot`), a complete native Rust implementation is required to replace it. Chatwoot's architecture relies heavily on a Ruby on Rails backend, PostgreSQL for data, Redis for background jobs (Sidekiq) and pub/sub.

  ### Chatwoot Key Concepts to Replicate
  1.  **Inboxes**: Core organizational unit holding conversations. Tied to a specific channel.
  2.  **Channels**: Adapters for different platforms (Web Widget, API, Email, Facebook, Twitter, WhatsApp, SMS, Line, Telegram, etc.).
  3.  **Conversations**: A thread of messages between a contact and an agent (or bot) within an inbox.
  4.  **Messages**: Individual communication units (text, attachments, template messages) within a conversation.
  5.  **Contacts**: The end-user communicating with the business.
  6.  **Agents/Users**: The business users responding to conversations.
  7.  **Teams**: Groupings of agents for routing.
  8.  **Automations/Macros**: Pre-defined actions based on triggers.

  ### Competitor Analysis
  - **Chatwoot**: Powerful but monolithic Rails app. Not designed for our specific multi-tenant Rust architecture.
  - **Shopify Inbox**: Deeply integrated into the Shopify ecosystem. Excellent for commerce but closed.
  - **Zendesk/Intercom**: Enterprise-focused, high cost, overly complex for our SMB personas.

  ## Design Doc

  ### Architecture
  The custom omnichannel chat system will be built natively in Rust inside `onehumancorp/mono`. It will leverage our existing PostgreSQL for persistence and Redis for pub/sub and distributed locking.

  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CONVERSATION : contains
      INBOX ||--|| CHANNEL : configured_via
      CONVERSATION ||--o{ MESSAGE : contains
      CONVERSATION }|--|| CONTACT : involves
      CONVERSATION }|--o| AGENT_USER : assigned_to
      MESSAGE }|--|| CONTACT : sent_by_contact
      MESSAGE }|--|| AGENT_USER : sent_by_agent
      CONTACT }|--|| TENANT : belongs_to
  ```

  #### Core Components
  1.  **Ingestion/Webhook Service (Rust)**: Receives webhooks from external channels (Instagram, WhatsApp, etc.). Authenticates payloads, standardizes them into internal `Message` events, and publishes to Redis Pub/Sub.
  2.  **Core Chat Engine (Rust)**: Subscribes to Redis events. Handles conversation routing, assignment, state management, and persists to PostgreSQL (respecting Row-Level Security for multi-tenancy).
  3.  **WebSocket Server (Rust)**: Maintains real-time connections with the OHC frontend (Flutter/PWA) to push live updates to the unified inbox.
  4.  **AI Integration Layer (Rust)**: Hooks into conversation events to allow the Customer & Relationship Assistant (Gemini Pro) to draft replies, tag conversations, and generate summaries.

  ### Mobile UX Flow (375px)
  1.  **Unified Inbox View**: The main tab shows a list of active conversations, sorted by recency or priority. Badges indicate unread messages.
  2.  **Conversation View**: Tapping a conversation opens a familiar chat interface. The contact's context (past orders, notes) is visible in a collapsible top drawer or bottom sheet.
  3.  **AI Draft Action**: A distinct button next to the composer allows the owner to "Draft Reply with AI". The AI analyzes the context and populates the text field, which the owner can edit before sending.
  4.  **Channel Indicators**: Small icons next to messages indicate the source (e.g., an Instagram icon for DMs).

  ### AI Agent Integration Points
  - **Work Triage**: AI agents monitor the inbox for high-priority or urgent requests and elevate them to the main OHC feed.
  - **Drafting**: AI agents hook into the conversation view to provide context-aware draft replies.
  - **Contextual Memory**: AI agents summarize resolved conversations and update the `Contact` profile with new preferences or details.

  ### Key Design Decisions
  - **Native Rust**: Ensures high performance, strong typing, and alignment with OHC's backend strategy.
  - **Row-Level Security (RLS)**: Mandatory for all new tables (`inboxes`, `conversations`, `messages`, etc.) to guarantee strict multi-tenant isolation.
  - **Decoupled Ingestion**: Using Redis Pub/Sub for incoming messages ensures the core engine isn't blocked by slow channel APIs.

  ## Implementation Prompt
  Implement the foundation for the Custom Rust Omnichannel Chat System, replacing external Chatwoot dependencies.

  **Acceptance Criteria:**
  1.  Create the database schema (PostgreSQL) for `inboxes`, `channels`, `conversations`, `messages`, and `contacts`. Ensure strict Row-Level Security (`tenant_id`) is applied to all tables.
  2.  Implement the core Rust models and repository layer for these entities.
  3.  Build a basic Rust WebSocket service capable of broadcasting "new message" events to authenticated frontend clients.
  4.  Create a dummy "Web Widget" channel adapter in Rust to simulate incoming messages for testing.
  5.  Develop the Flutter frontend unified inbox UI (mobile-first, 375px) to display a list of conversations and a basic chat view.
  6.  Ensure 100% unit test coverage for the new Rust and Flutter code.
  7.  Add Playwright E2E tests verifying that a message sent via the dummy adapter appears in the Flutter unified inbox UI in real-time.

  The implementation must adhere to OHC's visual standards (Translucent Glass, UniFi layouts) and maintain Zero-Trust security guarantees.

  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
