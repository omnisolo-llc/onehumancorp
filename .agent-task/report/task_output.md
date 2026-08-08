issue_title: "[Research] OHC Native Rust Omnichannel Inbox & Chat Engine (Replacing Chatwoot)"
issue_description: |
  # Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales.

  OHC previously relied on Chatwoot as an external third-party service for omnichannel customer support. However, relying on an external dependency violates our core architecture goals (multi-tenant isolation, Zero Trust, and single-binary deployment). We need a **native Rust implementation** of an omnichannel chat engine within the `onehumancorp/mono` repository that fully replicates Chatwoot's functionality but is deeply integrated into OHC's AI agent ecosystem and mobile-first UX.

  # Research Report
  **Findings & Competitive Analysis:**
  - **Chatwoot Source Audit:** Chatwoot's architecture relies on Ruby on Rails, ActionCable (WebSockets), and PostgreSQL. It abstracts channels (Email, Facebook, Twitter, WhatsApp, API) via "Channel Adapters". Conversations belong to an "Inbox", which belongs to an "Account" (Tenant).
  - **Shopify Inbox & Wix Inbox:** Good aggregation, but AI features are limited.
  - **Zendesk/Intercom:** Enterprise-grade and too complex for a single-person SMB.
  - **OHC Native Advantage:** By building this in Rust natively within OHC, we eliminate inter-service network latency, simplify deployment (one binary), and allow deep integration with our "Teammate" AI philosophy (e.g., The Ambassador Agent can intercept, draft, and route messages natively without webhooks).

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[External Channels: IG, WA, Email] -->|Webhooks / Polling| B(Channel Adapters - Rust)
      B --> C{Message Router & Identity Resolver}
      C --> D[Unified Conversation DB - Postgres]
      C --> E[Event Bus / WebSockets - Rust]
      E --> F[The Ambassador Agent]
      E --> G[Mobile Client - 375px]
      F -->|Reads Context| D
      F -->|Drafts Reply| D
      G -->|Approves Reply| B
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Unified Inbox Feed (Mobile):** Top card shows "1 New Message from Sarah (Insta DM)".
  - **Interaction:** Tapping the card opens a conversation view. Top half shows customer context (Sarah bought a vegan cake 2 months ago). Bottom half shows the AI-drafted reply.
  - **Action:** A prominent primary button "Send Draft" and a secondary "Edit".
  - **Visual Design:** Glassmorphism cards, blurred background to maintain focus, native keyboard integration.
  - **Settings (Advanced):** Hidden behind "Advanced", options to configure channel webhooks, SLA policies, and canned responses.

  ### AI Agent Integration Points
  - **The Ambassador Agent:** Subscribes to new message events via the native Rust Event Bus. Uses RAG against the tenant's product catalog and customer history to draft highly personalized replies natively in the DB.

  ### Key Design Decisions
  - **Native Rust Channels:** Replicate Chatwoot's `Channel::Base` pattern in Rust via traits (`ChannelAdapter`).
  - **WebSocket Real-time:** Use Rust native WebSockets (e.g., `axum` or `tungstenite`) to push events to the Flutter mobile client.
  - **Data Model Parity:**
    - `Tenant` (1) -> (M) `Inbox`
    - `Inbox` (1) -> (1) `Channel` (e.g., Channel::Whatsapp, Channel::Email)
    - `Inbox` (1) -> (M) `Conversation`
    - `Conversation` (1) -> (M) `Message`
    - `Tenant` (1) -> (M) `Contact` (resolved across channels)

  # Implementation Prompt
  **User-Facing Outcome:** As an owner (Maya), I can connect my Instagram and email directly inside the OHC app. When a customer messages me anywhere, it appears in a unified mobile feed with an AI-drafted reply ready to send in 1 tap, powered entirely by OHC's native backend without relying on external services like Chatwoot.

  **CUJ & Acceptance Criteria:**
  1. Define Rust data models (Entities, DTOs) and PostgreSQL schema migrations for the core chat domain: `inboxes`, `channels`, `conversations`, `messages`, and `contacts`, enforcing strict `tenant_id` RLS.
  2. Implement a `ChannelAdapter` trait and at least one mock/test channel adapter (e.g., `Channel::API`) to ingest external messages.
  3. Implement the `MessageRouter` to resolve identities and save incoming messages to the DB.
  4. Provide a REST API and WebSocket endpoint in the Rust server for the mobile client to fetch conversations and receive real-time message updates.
  5. Provide Playwright E2E tests: A test uses a webhook/API endpoint to simulate an incoming message, then the Playwright browser logs in, navigates to the Inbox UI, and verifies the message appears in the feed.

  **Priority:** P0 (Critical Infrastructure Replacement)
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
