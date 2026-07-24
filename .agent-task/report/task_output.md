issue_title: "Native Rust Omnichannel Chat Engine (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  Small business owners (Maya, Carlos, Priya, Leo, Fatima) need to communicate with their customers across multiple channels (Instagram DMs, WhatsApp, SMS, Email, Web Widget) without constantly switching apps. Previously, OHC relied on an external third-party Chatwoot integration for this capability. However, Chatwoot has been 100% RETIRED as an external dependency to enforce Zero-Trust security, strict multi-tenant isolation, and reduce operational overhead. We need a native Rust omnichannel customer support & chat engine built directly into `onehumancorp/mono` that achieves feature parity with Chatwoot's core capabilities, particularly focusing on omnichannel data models, WebSocket real-time messaging, and unified inboxes.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Chatwoot Architecture Audit:** A review of Chatwoot's source code (`https://github.com/chatwoot/chatwoot`) reveals core entities: `Conversation`, `Message`, `Contact`, `Inbox`, and various `Channel` adapters (API, Email, Facebook Page, Instagram, SMS, Telegram, Twilio, Web Widget, WhatsApp).
  - **WebSocket Messaging:** Chatwoot relies heavily on ActionCable for real-time WebSocket communication to update the dashboard instantly when messages arrive or status changes.
  - **Multi-Tenancy:** Chatwoot uses `account_id` extensively across its models (`messages`, `conversations`, `contacts`, `inboxes`) to enforce multi-tenancy.
  - **OHC Native Implementation:** Our native Rust implementation must replicate these core data models (Conversation, Message, Contact, Inbox, Channel) with strict row-level security (tenant isolation). We will implement a high-performance WebSocket server in Rust (using Tokio/Tungstenite) to handle real-time message delivery to the Flutter frontend.
  - **AI Agent Integration:** The native chat engine will serve as the foundation for "The Ambassador" (Customer Success Agent), intercepting messages to draft proactive replies before the owner even sees them.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Channels: IG, WhatsApp, SMS, Web] -->|Webhooks/API| B(Omnichannel Gateway - Rust)
      B --> C[Channel Adapters]
      C --> D{Message Router & Processor}
      D -->|Store| E[(PostgreSQL - Unified Customer Graph DB)]
      D -->|Trigger| F[Event Mesh / NATS]
      F --> G[The Ambassador AI Agent]
      G -->|Draft Reply| E
      F --> H[WebSocket Server - Rust Tokio/Tungstenite]
      H -->|Real-time Update| I[Flutter Mobile App - 375px]
      I -->|Owner Approves Draft| D
      D --> C
      C -->|Send| A
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Unified Inbox Feed (Mobile):** A sleek, translucent glassmorphism list of active conversations. Each row shows the customer name, channel icon (e.g., IG, WhatsApp), a snippet of the latest message, and a status badge (e.g., "Draft Ready", "Waiting on Customer").
  - **Conversation View:** Tapping a conversation opens the chat timeline. The AI-drafted reply sits pinned at the bottom in a visually distinct "Action Required" card.
  - **Interaction:** The owner can tap a prominent "Approve & Send" button, or tap "Edit" to modify the draft using the native mobile keyboard.
  - **Offline/Flaky Network:** Changes (like approving a draft) are queued locally on the mobile device and synced opportunistically. The UI instantly reflects the "Sending..." state.

  ### AI Agent Integration Points
  - **The Ambassador (Customer Success Agent):** Subscribes to the NATS event mesh for `message.created` events. It analyzes the message context, queries the customer's history from the PostgreSQL DB, and generates a draft response. The draft is saved as a `Message` record with `status: draft`.

  ### Key Design Decisions
  - **Native Rust & Tokio:** Replaces Ruby on Rails with high-performance Rust async tasks and WebSockets for lower latency and better resource utilization.
  - **Strict Multi-Tenancy:** `tenant_id` is mandatory on every table (`conversations`, `messages`, `contacts`, `inboxes`) and enforced at the database level using RLS (Row Level Security).
  - **Draft-First Paradigm:** Unlike Chatwoot where agents manually reply, the OHC system defaults to the AI proactively drafting the reply.

  ## Implementation Prompt
  **User-Facing Outcome:** As an owner (e.g., Maya), I receive customer messages from Instagram and WhatsApp in a single feed within the OHC app. When I open a new message, an AI has already drafted a perfect reply based on the customer's history. I just tap "Approve" to send it via the native Rust backend.
  **CUJ & Acceptance Criteria:**
  1. Define Rust data models (Structs & Diesel/SQLx schemas) for `Conversation`, `Message`, `Contact`, `Inbox`, and `Channel` with strict `tenant_id` enforcement.
  2. Implement a high-performance WebSocket server using Tokio/Tungstenite that allows the Flutter client to connect, authenticate (SPIFFE/SPIRE/JWT), and receive real-time message events.
  3. Create an API endpoint (`POST /api/v1/webhooks/omnichannel`) to ingest simulated incoming messages and route them to the correct inbox.
  4. Ensure the ingest pipeline triggers an asynchronous event (via NATS or in-memory channel) that could be consumed by an AI agent to generate a draft.
  5. Provide Playwright E2E tests: A script simulates an incoming webhook message. The Flutter/Web UI (connected via WebSocket) instantly displays the new message in the inbox feed without a manual page refresh.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []