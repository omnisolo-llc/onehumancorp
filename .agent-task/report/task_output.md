issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  # Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales.

  OHC previously considered using Chatwoot, but as per the architectural guidelines, Chatwoot is 100% RETIRED as an external service. OHC needs a native, highly scalable, multi-tenant Rust-based omnichannel customer support and chat engine built directly into `onehumancorp/mono`.

  # Research Report
  **Findings & Competitive Analysis:**
  - **Chatwoot Source Code Audit:** Chatwoot uses Rails with complex `Conversation`, `Message`, `Inbox`, and `Contact` models. It relies heavily on WebSockets (ActionCable) and background jobs (Sidekiq) for real-time messaging and integrations (WhatsApp, Twitter, Facebook).
  - **Shopify Inbox & Wix Inbox:** Good aggregation but lack proactive AI drafting based on full customer history. They are reactive rather than autonomous.
  - **OHC Opportunity:** A native Rust implementation provides massive performance gains, lower memory footprint, and tighter integration with OHC's "Teammate" AI (The Ambassador). By building this natively in Rust, we can leverage Rust's strict concurrency guarantees and multi-tenant row-level security directly within our existing Bazel monorepo structure.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[External Webhooks: WhatsApp/Insta/Email] -->|Ingress| B(Rust Omnichannel Gateway Service)
      B --> C{Tenant Router & Auth}
      C --> D[Rust Inbox Microservice]
      D --> E[(PostgreSQL unified DB)]
      D --> F[Redis Pub/Sub]
      F --> G[WebSocket Gateway]
      G --> H[Flutter Mobile Client 375px]
      D --> I[AI Event Mesh]
      I --> J[The Ambassador Agent]
      J -->|Drafts Reply| D
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Inbox View:** A clean, bottom-nav accessible inbox aggregating all channels. Badges for unread messages.
  - **Conversation Thread:** Chat bubbles style. AI-drafted responses appear as a distinct "Pending Draft" card at the bottom, above the keyboard, with a glowing 1-tap "Approve & Send" button.
  - **Customer Context Panel:** Swipe left (or a top header tap) reveals the unified customer identity graph (past orders, notes, lifetime value).
  - **Visual Design:** Glassmorphism cards, blurred background to maintain focus, native keyboard integration if editing.

  ### AI Agent Integration Points
  - **The Ambassador:** Subscribes to the `message.created` event via the event mesh. If it's an inbound message, it queries the customer's omnichannel identity graph (purchase history, past bookings) via RAG. It then drafts a highly personalized reply and inserts it as a `Message` with status `draft`.

  ### Key Design Decisions
  - **Rust Native:** Implement the core domain models (Account, Inbox, Conversation, Message, Contact) as a new Rust crate/service within the Bazel workspace.
  - **WebSocket Realtime:** Use a Rust async WebSocket server (e.g., `axum` + `tokio-tungstenite`) linked to Redis Pub/Sub for cross-node message fanout.
  - **Strict Multi-Tenancy:** Enforce `tenant_id` on every query within the Rust data access layer, matching our PostgreSQL RLS policies.
  - **Proactive Drafting:** Move from read-reply to read-approve. The AI drafts the response before the user opens the app.

  # Implementation Prompt
  **User-Facing Outcome:**
  As a business owner, I can open the OHC app, go to my Unified Inbox, and see messages from WhatsApp and Email in one thread. My AI assistant has already drafted a highly accurate reply based on the customer's history. I tap "Approve", and the Rust backend instantly dispatches it to the correct external channel.

  **CUJ & Acceptance Criteria:**
  1. Create the core Rust microservice (`src/server/chat_engine`) with basic multi-tenant CRUD endpoints for Inboxes, Conversations, Contacts, and Messages.
  2. Implement an initial webhook ingestion endpoint in Rust that normalizes incoming payloads into a unified `Message` struct.
  3. Implement a WebSocket endpoint in Rust that streams new messages to connected clients based on their `tenant_id`.
  4. Provide unit tests (100% coverage) for the Rust models and handlers.
  5. Provide Playwright E2E tests: A user logs in, receives a simulated inbound message via WebSocket, the UI displays it, the AI draft appears, the user taps "Approve", and a simulated outbound dispatch occurs.
  6. Ensure all code builds cleanly via `bazel build //...` and tests pass via `bazel test //...`.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture]
assignees: []
