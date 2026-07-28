issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  # Problem Statement
  OHC is mandated to completely retire Chatwoot as an external dependency. We must build a high-performance, multi-tenant omnichannel customer support and chat engine natively in Rust inside `onehumancorp/mono`. Relying on external third-party services is forbidden.

  # Research Report
  **Findings & Competitive Analysis:**
  - **Chatwoot Source Code Audit:** Chatwoot uses a monolithic Ruby on Rails backend. Key data models include Accounts, Users, Inboxes, Conversations, Messages, and Contacts. Key features include omnichannel integrations (web widget, API channels, email), macro management, SLA policies, and WebSocket real-time messaging via ActionCable.
  - **OHC Native Implementation Goal:** We must replicate this feature set in a high-performance, multi-tenant Rust architecture using Axum (HTTP) and gRPC, with strict Row-Level Security (RLS) in PostgreSQL.
  - **Differentiation:** By building this natively, we ensure zero trust multi-tenancy via SPIFFE/SPIRE, and deep integration with our existing OHC AI agents (like The Ambassador for auto-drafting replies) without external webhooks or sync latency.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Web Chat Widget / API Channel] -->|WebSocket/HTTP| B(Omnichannel Gateway - Rust)
      B --> C{Authentication & Tenant Isolation}
      C -->|Valid| D[Inbox Service - Rust]
      D --> E[PostgreSQL DB with RLS]
      D --> F[Redis Cache / PubSub]
      F -->|New Message Event| G[The Ambassador Agent]
      G -->|Draft Reply| D
      D -->|WebSocket Push| H[Mobile/Desktop Client 375px]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Unified Inbox Feed (Mobile):** A list of conversations across all channels. Unread messages bolded. Avatar icons indicate source (Web, Email, SMS).
  - **Conversation View:** Standard chat interface. Messages grouped by date. Input area with attachment options.
  - **Agent Assist Integration:** "Approve AI Draft" button prominently displayed above the input area when The Ambassador agent has prepared a response based on customer history.

  ### AI Agent Integration Points
  - **The Ambassador:** Subscribes to the internal Event Mesh for new incoming messages. Uses the tenant's product catalog and customer order history (RAG) to draft context-aware replies directly into the conversation thread as a "draft" state.

  ### Key Design Decisions
  - **Native Rust/Axum:** Ensures high concurrency and low latency for real-time chat.
  - **WebSocket over Redis PubSub:** Enables scalable real-time message delivery across multiple server instances to connected mobile/web clients.
  - **Strict Multi-Tenancy:** All database tables (`conversations`, `messages`, `inboxes`, `contacts`) must have `tenant_id` and enforce RLS policies.

  # Implementation Prompt
  **User-Facing Outcome:** As an owner, I want a blazing fast unified inbox within my OHC app that shows messages from all channels. I want my AI assistant to pre-draft replies so I can just tap "Send", without relying on any external 3rd-party chat software.

  **CUJ & Acceptance Criteria:**
  1. Create the foundational Rust data models and DB migrations (PostgreSQL) for `Inbox`, `Contact`, `Conversation`, and `Message`, ensuring strict `tenant_id` RLS.
  2. Implement an Axum-based WebSocket handler for real-time bi-directional message syncing.
  3. Implement the internal API layer (gRPC/REST) for creating messages and fetching conversation history.
  4. Build a mobile-first (375px) React/Tailwind interface in the Tauri app to display the inbox list and conversation view.
  5. Provide Playwright E2E tests: A user logs in, receives a real-time message in the UI via WebSocket, and successfully sends a reply back through the native system.

  # Priority
  P0

  # Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
