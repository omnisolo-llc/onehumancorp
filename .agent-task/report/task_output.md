issue_title: "Build Native Rust Omnichannel Inbox & Chat Engine (Replacing Chatwoot)"
issue_description: |
  # Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" simply aggregate messages without context. OHC needs a high-performance native Rust omnichannel engine that not only aggregates messages but actively integrates with our AI agents (like The Ambassador) to draft replies, maintaining context from past orders and interactions. We are retiring the Chatwoot external dependency and building this natively to ensure deep integration, zero-trust security, and real-time mobile performance.

  # Research Report
  **Findings & Competitive Analysis:**
  - **Chatwoot Source Code Audit:** Chatwoot relies on Ruby on Rails, ActionCable for WebSockets, and sidekiq for background jobs. Its schema (`conversations`, `messages`, `inboxes`, `contacts`) is robust but not optimized for native rust embedding or our specific AI-first multi-tenant requirements.
  - **OHC Opportunity:** By building natively in Rust within `onehumancorp/mono`, we can achieve microsecond latencies, strictly enforce our `tenant_id` Row-Level Security at the database driver level, and seamlessly route incoming webhooks directly into our AI Job Queue (PostgreSQL `SKIP LOCKED`).
  - **Market Position:** Unlike Shopify Inbox or Wix Inbox, our native engine will feed directly into "The Ambassador" agent to proactively draft replies based on the unified customer graph, changing the owner's workflow from "read-reply" to "read-approve".

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[External Webhooks: WhatsApp/IG/Email] -->|Ingest| B(Rust API Gateway)
      B --> C{Webhook Verification & Normalization}
      C --> D[PostgreSQL: Inboxes, Contacts, Conversations, Messages]
      D --> E[Redis Pub/Sub: Real-time Events]
      E --> F[Rust WebSocket Server]
      F --> G[Flutter PWA / Mobile App 375px]
      D --> H[AI Job Queue]
      H --> I[The Ambassador Agent]
      I -->|Draft Reply| D
      I -->|Push Notification| F
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Inbox List (Mobile):** A clean, unread-first list of conversations. Badges indicate source channel (e.g., small WhatsApp icon).
  - **Conversation View:** Standard chat interface. Incoming messages on the left.
  - **AI Integration (The Magic):** If The Ambassador agent has drafted a reply, it appears pre-filled in the composer with a translucent glass background and a primary "Approve & Send" button.
  - **Visual Design:** OHC Premium Token library. Clean Apple/Ubiquiti-style hierarchy.

  ### AI Agent Integration Points
  - **The Ambassador:** Subscribes to the `message.created` event (via Redis/Job Queue). Contextualizes the message against the Contact's history, generates a draft, and inserts it into the `messages` table with `status = drafted`.

  ### Key Design Decisions
  - **Multi-Tenant Isolation:** Every table (`inboxes`, `channels`, `contacts`, `conversations`, `messages`) MUST have a `tenant_id` and strict foreign key constraints.
  - **Unified Data Model:** Replicate the core structure of Chatwoot (Account/Tenant -> Inbox -> Channel -> Conversation -> Message) but strongly typed in Rust using SQLx or Diesel.
  - **Real-time First:** All message mutations must broadcast events via Redis to connected WebSocket clients to ensure the 375px mobile UI is instantly responsive.

  # Implementation Prompt
  **User-Facing Outcome:** The owner opens their OHC app on their phone (375px) and sees a unified stream of messages from WhatsApp, Instagram, and web chat. They tap a message and see a perfectly drafted reply from their AI assistant, ready to send with one tap.
  **CUJ & Acceptance Criteria:**
  1. Define and implement the PostgreSQL database schema for the native chat engine (Inboxes, Contacts, Conversations, Messages) with strict `tenant_id` isolation.
  2. Implement the Rust backend services (gRPC/REST) to create inboxes, manage contacts, start conversations, and send/receive messages.
  3. Implement a webhook ingestion endpoint capable of receiving external messages, normalizing them, and creating new `Message` records.
  4. Implement a basic WebSocket broadcasting mechanism (via Redis) to push new message events to clients.
  5. Provide Playwright E2E tests: A user logs in, navigates to the unified inbox, sees a newly received (mocked incoming) message, and sends a reply via the UI. Ensure the UI responds correctly at the 375px mobile breakpoint.

  **Note:** Do not implement specific vendor API integrations (e.g., actual Meta WhatsApp API calls) yet. Focus on the core engine, data models, API endpoints, and the UI.

  # Priority
  P0 (Critical)

  # Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
