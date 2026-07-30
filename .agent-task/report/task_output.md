issue_title: "Implement Native Rust Omnichannel Chat System"
issue_description: |
  ## Title: Implement Native Rust Omnichannel Chat System (Chatwoot Replacement)

  ## Problem Statement
  Small business owners need a unified inbox to manage customer inquiries across channels (WhatsApp, Web Widget, Instagram DMs, etc.) without logging into multiple tools. Currently, our architecture relied on Chatwoot, which is fully retired as an external dependency. We have a critical architectural gap where we lack a native, high-performance, multi-tenant Rust backend to handle omnichannel messaging, enforce strict Row Level Security (RLS) via `tenant_id`, and seamlessly integrate with OHC AI agents for automated reply drafting.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Legacy Chatwoot Architecture**: We audited Chatwoot's source code (schema, webhooks, API). Chatwoot relies heavily on ActiveRecord models with tables for `conversations`, `messages`, `inboxes`, `contacts`, and `channel_adapters`. Our native Rust implementation needs equivalent data models but strictly isolated via `tenant_id` RLS in PostgreSQL.
  - **Shopify Inbox / Wix Inbox**: Both aggregate messages but lack proactive AI workflows for the owner. They are reactive.
  - **OHC Opportunity**: By building natively in Rust within `onehumancorp/mono`, we can achieve lightning-fast WebSockets for Web Widget chat, ultra-low latency webhook processing for Meta (WhatsApp/Instagram), and deeply integrate "The Ambassador" (Customer Success Agent) to draft replies by directly querying the tenant's product catalog and customer order history.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[WhatsApp Webhook] --> B(Rust Omnichannel API)
      C[Web Widget WebSocket] <--> B
      B --> D{Tenant Auth & RLS}
      D --> E[(PostgreSQL)]
      E --> |Listen/Notify| F[Redis Pub/Sub]
      F --> C
      B --> G[AI Agent Triage Queue]
      G --> H[The Ambassador Agent]
      H --> |Query Catalog & History| E
      H --> |Draft Reply| I[Action Required Queue]
      I --> J[OHC Mobile App 375px]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Agent Feed**: A single feed card titled "1 New WhatsApp Message from Sarah".
  - **Unified View (Mobile)**: Tapping the card opens a unified chat view.
    - *Top half*: Customer identity and past orders (e.g., "Bought Vegan Cake 2 months ago").
    - *Bottom half*: AI-drafted reply based on context.
  - **Interaction**: The owner reviews the draft and taps a large "Approve & Send" button (min 44x44px touch target) which dispatches the message back out through the native Rust webhook API.

  ### AI Agent Integration Points
  - **The Ambassador (Customer Success Agent)**: Listens to new `Message` events on the internal queue. Performs RAG against the unified customer graph and product catalog to propose replies.
  - **Operations Agent (The Manager)**: Triggers if a message requests a booking or order status, verifying inventory and appending data to the draft.

  ### Key Design Decisions
  - **Native Rust & PostgreSQL RLS**: Eliminates Chatwoot dependency. Ensures zero-trust multi-tenancy. Every query enforces `tenant_id`.
  - **Event-Driven AI Triage**: Incoming messages immediately enter an AI evaluation queue (via `SKIP LOCKED`) so drafts are ready *before* the owner opens the app.
  - **Mobile-First Approval**: Transitioning from a manual "typing" inbox to an "approval" inbox designed for a 375px screen.

  ## Implementation Prompt
  **User-Facing Outcome:** When a customer messages a business on WhatsApp, the owner receives a push notification and sees an actionable card in their OHC mobile app. The card contains the customer's message, their order history, and a pre-drafted reply. The owner taps "Approve & Send," and the message is instantly delivered via WhatsApp.

  **CUJ & Acceptance Criteria:**
  1. Create the database schemas (Conversations, Messages, Inboxes, Contacts) in PostgreSQL with `tenant_id` and RLS enforced.
  2. Implement the native Rust backend endpoints to receive external webhooks (e.g., simulated WhatsApp) and WebSocket connections for a Web Widget.
  3. Implement the background worker (using `SKIP LOCKED` pattern) that routes incoming messages to "The Ambassador" agent for draft generation.
  4. Create a unified chat UI card in the Flutter/PWA frontend (375px mobile-first) that displays the draft and includes an "Approve & Send" button.
  5. **Verification**: Write at least 5 Playwright E2E tests verifying the end-to-end flow: receiving a mock webhook, generating a draft, user approving in the UI, and the outbound message being dispatched.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
