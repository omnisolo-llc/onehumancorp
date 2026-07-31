issue_title: "Design: Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  # Problem Statement
  OHC requires a high-performance, multi-tenant omnichannel customer support and chat engine natively in Rust. The current or previous reliance on third-party services like Chatwoot introduces dependencies, scaling constraints, and potential lack of deep integration with the core platform. A native solution is needed to align with OHC's vision of an autonomous, highly integrated AI work assistant.

  # Research Report
  **Findings & Competitive Analysis:**
  - **Chatwoot Source Code Audit:** Chatwoot uses Ruby on Rails with complex ActiveRecord models (`Account`, `Inbox`, `Conversation`, `Message`, `Contact`, `Channel::*`).
  - **Data Models:** The core models in Chatwoot revolve around a multi-tenant `Account`, which has many `Inboxes`. Each Inbox maps to a specific `Channel` (WhatsApp, Instagram, Web Widget, etc.). `Conversations` belong to an Inbox and a `Contact`, and consist of `Messages`.
  - **WebSockets & Real-time:** Real-time events are broadcasted using ActionCable.
  - **OHC Opportunity:** By building natively in Rust, we can achieve significantly better performance, memory safety, and seamless integration with our AI Job Queue (PostgreSQL `SKIP LOCKED`) and Redis distributed locks. We can leverage our existing `Tenant` model for multi-tenancy and SPIFFE/SPIRE for zero-trust identity. The Rust implementation will feature explicit multi-tenant isolation rules, faster WebSocket handling (e.g., via Tokio/Tungstenite), and direct access to our core business graphs.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Web Widget / Instagram DM / WhatsApp] -->|Webhooks / WebSockets| B(Omnichannel Gateway - Rust)
      B --> C{Tenant Router & Auth}
      C --> D[Unified Inbox Service]
      D --> E[(PostgreSQL - RLS Enabled)]
      D --> F[Redis - Pub/Sub & Locks]
      D --> G[AI Ambassador Agent]
      G -->|Proactive Drafts| D
      F -->|Real-time Updates| H[Mobile/Web Client - 375px First]
  ```

  ### Mobile UX Flow (375px First)
  - **Unified Inbox View:** A clean, Unifi-style glassmorphism interface on a 375px mobile screen. A simple list of active conversations, clearly badged by channel (e.g., small WhatsApp or Instagram icon).
  - **Conversation View:** Standard chat interface. Messages from the customer on the left, agent/owner replies on the right. AI drafts appear seamlessly in the input field as "Suggestions" waiting for a 1-tap approval.
  - **Offline/Flaky Network Handling:** Optimistic UI updates. Messages sent while offline show a subtle loading spinner, stored locally, and retried automatically.

  ### AI Agent Integration Points
  - **The Ambassador:** Subscribes to the unified inbox stream. For every new incoming message, it queries the tenant's product catalog and customer history, then proposes a draft reply.
  - **Action Required Queue:** High-confidence drafts bypass the standard inbox list and appear as priority actionable cards in the owner's main feed.

  ### Key Design Decisions
  - **Rust Backend:** Use Actix-Web or Axum (matching standard OHC stack) for the API and WebSocket server.
  - **Row Level Security (RLS):** Enforce tenant isolation strictly at the database level using `tenant_id` on all tables (`inboxes`, `conversations`, `messages`, `contacts`).
  - **Zero-Trust:** Inter-service communication (e.g., Gateway to Agent) secured via SPIFFE/SPIRE.

  # Implementation Prompt
  **User-Facing Outcome:** As an owner, I see all my customer messages from Instagram, WhatsApp, and the web widget in one native, blazing-fast inbox inside the OHC app. My AI assistant pre-drafts replies, saving me hours.

  **CUJ & Acceptance Criteria:**
  1. Implement the Rust data model (Entites: `Inbox`, `Channel`, `Contact`, `Conversation`, `Message`) with strict RLS (`tenant_id`).
  2. Implement the API endpoints for listing inboxes, fetching conversations, and sending/receiving messages.
  3. Implement a WebSocket endpoint for real-time message broadcasting to clients.
  4. Build the Flutter UI components adhering to the 375px mobile-first, translucent glass design system.
  5. Provide Playwright E2E tests: Simulate an incoming webhook, verify the message appears in the UI via WebSocket, and the owner can send a reply.

  # Priority
  P0

  # Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
