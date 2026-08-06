issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  # Problem Statement

  OneHumanCorp previously integrated with Chatwoot as an external service for omnichannel messaging. However, maintaining Chatwoot as an external service creates friction for small business owners, breaks the unified assistant-first experience, and adds unnecessary operational overhead. We need a native Rust implementation embedded within the OHC platform to handle multi-tenant omnichannel communication (Instagram, WhatsApp, Email, etc.) seamlessly, fulfilling our promise of an integrated, simple, and intelligent Work Assistant. Chatwoot is being fully retired in favor of this custom Rust solution.

  # Research Report

  **Findings & Competitive Analysis:**
  - **Chatwoot Source Code Audit:** Chatwoot uses a robust schema with entities like `conversations`, `messages`, `inboxes`, `contacts`, and `channel_adapters`. We must replicate this functionality efficiently in Rust.
  - **Competitor Platforms:** Platforms like Shopify Inbox and Wix Inbox aggregate messages but lack deep AI integration to proactively draft contextual responses using a unified identity graph.
  - **OHC Opportunity:** By building our own Rust-based chat system, we can tightly integrate it with our AI (The Ambassador) to automatically draft responses, update customer profiles, and trigger operational workflows (e.g., booking an appointment from a DM) without jumping between tools.

  # Design Doc

  ### Architecture Diagram

  ```mermaid
  graph TD
      A[External Channels: IG, WA, Email] -->|Webhooks/API| B(Rust Channel Adapters)
      B --> C[Rust Omnichannel Gateway Service]
      C --> D[Customer Identity Resolution]
      D --> E[(Multi-Tenant PostgreSQL DB)]
      C --> F[Event Mesh / Redis Queue]
      F --> G[The Ambassador AI Agent]
      G -->|Draft Contextual Reply| E
      G --> H[Action Required Queue]
      H --> I[Mobile Frontend Feed]
      I -->|Owner Approves| J[Rust Dispatcher]
      J --> A
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Unified Inbox View:** A clean, macOS Translucent Glass styled list of active conversations, categorized by priority or channel.
  - **Conversation Thread:**
      - Top section: Customer context (e.g., past orders, appointments).
      - Middle section: Message history with clear distinction between customer messages, AI drafts, and sent messages.
      - Bottom section: Input area with a pre-populated AI draft.
  - **Mobile Interaction:** Swipe to archive/resolve. 1-tap "Approve & Send" for AI drafts. Native keyboard integration for manual editing.

  ### AI Agent Integration Points
  - **The Ambassador (Customer Success Agent):** Listens to incoming message events, fetches customer context, and drafts a reply.
  - **The Manager (Operations Agent):** Can be invoked by The Ambassador if a message requires checking inventory or availability.

  ### Key Design Decisions
  - **Native Rust Implementation:** Eliminates reliance on Chatwoot, improving performance, security (Zero-Trust), and integration.
  - **Strict Multi-Tenancy:** All database tables (`conversations`, `messages`, `inboxes`, etc.) must include `tenant_id` with Row-Level Security (RLS) enabled.
  - **Proactive AI Drafting:** The system defaults to drafting responses for owner approval, rather than waiting for the owner to start typing.

  # Implementation Prompt

  **User-Facing Outcome:** As an owner, I see all my customer messages (from IG, WhatsApp, Email) in one unified feed within the OHC app. When I tap a message, I see the customer's history and an AI-drafted reply that I can send with one tap. I never have to log into a separate chat tool.

  **CUJ & Acceptance Criteria:**
  1.  Implement the database schema for the Rust Omnichannel Chat System (e.g., `conversations`, `messages`, `inboxes`, `contacts`, `channel_adapters`) with strict multi-tenant isolation (`tenant_id`).
  2.  Build the core Rust backend services (Omnichannel Gateway, Channel Adapters, Dispatcher) to handle incoming messages and outgoing replies.
  3.  Integrate the system with the Event Mesh and The Ambassador AI Agent to automatically generate draft replies for new messages.
  4.  Develop the mobile-first frontend UI (Unified Inbox, Conversation View) following the macOS Translucent Glass and UniFi layout guidelines.
  5.  Ensure the frontend displays AI drafts clearly and allows 1-tap approval or manual editing.
  6.  Write comprehensive unit tests (100% coverage) for the Rust backend and Playwright E2E tests covering the full flow from receiving a webhook to sending an approved reply.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
