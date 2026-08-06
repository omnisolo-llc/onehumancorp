issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  # Problem Statement
  OneHumanCorp previously integrated with Chatwoot as a third-party omnichannel messaging solution. However, relying on an external dependency breaks our core architecture principles of zero-trust, local-first performance, native integration with AI agents, and strict multi-tenant data isolation. OHC needs a native, high-performance omnichannel chat engine built in Rust to unify messages from Instagram, WhatsApp, Email, and Web into a single inbox where our internal AI agents (like The Ambassador) can natively read, context-switch, and draft replies instantly before the human operator even opens the app.

  # Research Report
  - **Chatwoot Source Audit:** We cloned and analyzed the `chatwoot/chatwoot` repository (v3.x).
    - **Models:** Chatwoot uses `Account` (Tenant), `Inbox`, `Conversation`, `Message`, `Contact`, `Channel::*` models heavily.
    - **Architecture:** It relies on ActionCable/WebSockets for real-time delivery and Sidekiq for background jobs.
    - **Gap:** Chatwoot's architecture does not natively support an integrated LLM agent loop that blocks/intercepts before the human operator sees the message in the UI feed.
  - **Competitive Advantage:** By building this natively in Rust within `onehumancorp/mono`:
    - We eliminate external latency and webhook points of failure.
    - We enforce row-level security (`tenant_id`) natively in Postgres rather than hoping Chatwoot's multi-tenancy holds up.
    - Our AI Agent queue (using Postgres SKIP LOCKED) can natively hook into `INSERT INTO messages` to immediately trigger the `Ambassador` agent to draft a reply, directly updating the `draft_id` on the conversation.
  - **Target Personas:**
    - Maya (Baker): Gets 10 Instagram DMs overnight. Wakes up, opens OHC, sees 10 "Draft Replies" ready to approve.
    - Carlos (Handyman): Gets SMS inquiries. OHC AI drafts quotes natively based on his pricing catalog.

  # Design Doc
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD
      A[External Webhooks: IG, WhatsApp, Email] -->|HTTPS POST| B(Gateway: Axum / Rust)
      B --> C{Channel Adapter Strategy}
      C -->|Normalize| D[Unified Inbox Service]
      D --> E[(Postgres: messages, conversations, contacts)]
      D --> F[AI Job Queue: SKIP LOCKED]
      F --> G[The Ambassador Agent]
      G -->|Draft Reply| E
      E -->|Real-time update| H[WebSocket / SSE Server]
      H --> I[OHC Mobile Shell 375px]
  ```

  ### Mobile UX Flow (375px First)
  - **Triage Feed:** The default view is not a traditional chat list. It is an "Action Required" feed.
  - **Card View:** A card shows the customer's avatar, channel icon (e.g., IG), their message, and a blurred/glassmorphic preview of the AI's drafted response.
  - **Interaction:**
    - 1-Tap "Send Draft".
    - Tap card body to open full conversation history and edit the draft.
  - **State Management:** When "Send" is tapped, optimistic UI marks it sent, and the background job dispatches it back through the respective `ChannelAdapter`.

  ### Key Design Decisions
  1. **Data Model:** We will replicate Chatwoot's core omnichannel domain (Inboxes, Contacts, Conversations, Messages) but strictly partition them by `tenant_id` at the database level using Postgres Row-Level Security.
  2. **Real-time:** Use Server-Sent Events (SSE) or WebSockets natively via `axum` in the Rust backend instead of Ruby ActionCable.
  3. **Agent First:** Messages do not just go to the UI. They go to the AI Job Queue first. The `Conversation` model will have a `draft_message_id` or `agent_suggestion` field.

  # Implementation Prompt
  **User-Facing Outcome:**
  As a business owner, I receive an incoming test message (e.g., simulated WhatsApp). When I log into the OHC app on my mobile browser, I see the message in my unified inbox feed, complete with an AI-generated draft response. I can tap "Approve" to send it or edit it directly. The entire flow happens within OHC without any external Chatwoot dependency.

  **Acceptance Criteria & CUJ:**
  1. **Database Schema:** Create migrations for `contacts`, `inboxes`, `conversations`, and `messages`, ensuring strict `tenant_id` isolation.
  2. **Rust Backend:** Implement Axum API endpoints to simulate receiving an incoming webhook, create the conversation/message, and expose an endpoint to fetch the inbox.
  3. **AI Hook:** Implement a background job or hook that detects a new incoming message and generates a draft reply using the configured `OHC_LLM_PROVIDER`.
  4. **Frontend UI:** Build a mobile-first (375px) React/Tauri UI component to display the unified inbox, showing the incoming message and the AI draft.
  5. **Action:** Implement the "Approve/Send" button to mark the message as sent.
  6. **Tests:** Must include Playwright E2E tests validating the complete flow from incoming message webhook to UI display and approval. No network mocking for internal APIs.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
