issue_title: "Native Rust Omnichannel Chat: Chatwoot Feature Parity"
issue_description: |
  # Problem Statement
  OneHumanCorp relies on the concept of an Omnichannel Inbox to manage communication between small business owners and their clients across multiple platforms (WhatsApp, Instagram, Email, SMS, etc). As a core instruction, we must completely retire Chatwoot as an external third-party dependency. OHC requires a native, high-performance, multi-tenant Rust-based Omnichannel Inbox integrated directly into our infrastructure to provide a unified experience without relying on external services. The owners need to manage this inbox effortlessly from their mobile phones.

  # Research Report
  - **Codebase & External Dependencies Audit**: Chatwoot is mandated to be 100% RETIRED.
  - **Chatwoot Source Code Audit**: Investigated the Chatwoot source repo (`https://github.com/chatwoot/chatwoot`), specifically `app/models/conversation.rb`, `app/models/message.rb`, `app/models/inbox.rb` and `app/models/channel/`. Found comprehensive data models for `Conversations`, `Messages`, `Inboxes`, and multiple `Channels` (SMS, Email, WhatsApp, Instagram, Telegram).
  - **Gap Analysis**: OHC's current Rust monolithic backend has a stub for `ohc.inbox` and some proto definitions (`src/proto/inbox.proto` contains `OmniMessage` and `Conversation`), but lacks the extensive models, channel adapters, and webhooks routing to completely replace Chatwoot's features. We need to implement native Rust services mirroring Chatwoot's core conversational architecture while integrating tightly with our multi-tenant PostgreSQL system and Zero Trust/SPIFFE identity.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[External Channels: WhatsApp, SMS, IG, Email] -->|Webhooks| B(Rust Channel Adapters & Webhook Gateway)
      B --> C{Omnichannel Routing & Validation Engine}
      C -->|Schema Validation| D[Unified Multi-Tenant DB: PostgreSQL]
      C -->|Pub/Sub Events| E[Redis/NATS Event Mesh]
      E --> F[AI Customer Success Agent - The Ambassador]
      F -->|Context & RAG| G[Owner Context & Catalog DB]
      F -->|Drafts Reply| D
      E --> H[Frontend WebSocket Gateway: tokio-tungstenite]
      H --> I[OHC Mobile App / PWA 375px]
      I -->|Approve/Edit Draft| C
  ```

  ### Mobile UX Flow
  - **Unified Feed (Mobile 375px):** First screen shows a clean "Unified Inbox" feed utilizing OHC Premium Tokens (translucent glass, Apple/Ubiquiti hierarchy).
  - **Conversation View:** Tapping a thread reveals the multi-channel history. Distinct icons indicate the channel source (e.g., IG, WhatsApp) for each message.
  - **AI Drafting Integration:** A prominent translucent card at the bottom of the conversation view shows an AI-drafted reply.
  - **Quick Actions:** Single tap "Approve & Send" or a secondary "Edit" action that brings up the native mobile keyboard.

  ### AI Agent Integration Points
  - **The Ambassador (Customer Success Agent):** Hooks into the incoming message stream via the NATS event mesh. Queries the tenant's catalog, user order history, and past conversations to auto-generate a `draft_reply` within the `OmniMessage` struct.
  - **State Transitions:** Updates the `OmniMessage.status` from `unread` to `draft_ready` to prompt the owner for approval.

  ### Key Design Decisions
  - **Data Isolation:** All models (`Conversations`, `Messages`, `Inboxes`) will enforce strict tenant boundary using PostgreSQL `tenant_id` Row Level Security (RLS).
  - **Rust Native Handlers:** Use `axum` for HTTP webhook endpoints and `tokio-tungstenite` for WebSocket connections to the frontend.
  - **No Third Party Chats:** Fully decouple from Chatwoot, ensuring everything is natively hosted within the OHC mono-repo.

  # Implementation Prompt
  **User-Facing Outcome:** A small business owner (like Maya or Carlos) can receive messages from Instagram DMs, WhatsApp, and SMS all in one unified OHC feed. When they open a new message, they find a perfectly drafted response ready to be sent with a single tap, completely powered by OHC's internal Rust architecture.

  **CUJ & Acceptance Criteria:**
  1. Define and migrate PostgreSQL tables matching the native Rust implementation of `Inboxes`, `Conversations`, and `Messages` based on the proto definitions, enforcing `tenant_id` Row-Level Security.
  2. Implement an `axum`-based Webhook Gateway capable of receiving simulated incoming messages for at least two channels (e.g., Email, SMS).
  3. Ensure the gateway parses the payload, validates it, and inserts it into the DB under the correct tenant, producing a NATS event.
  4. Build the AI listener (The Ambassador) that consumes the incoming message event, drafts a response, and updates the DB with a draft status.
  5. Provide Playwright E2E tests: Simulate an incoming webhook, log in as the tenant owner, navigate to the Unified Inbox (at 375px viewport), verify the message and AI draft appear, and approve the draft.

  # Priority
  P0

  # Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
