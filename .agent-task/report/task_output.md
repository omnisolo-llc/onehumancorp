issue_title: "Implement Native Rust Omnichannel Inbox & Chat System"
issue_description: |
  # Problem Statement
  Small business owners like Maya (baker) and Carlos (handyman) receive messages across fragmented channels—Instagram DMs, WhatsApp, SMS, Web Chat, and Email. Currently, managing these requires checking multiple apps, leading to missed leads, delayed responses, and lost revenue. Previous solutions relied on external third-party services like Chatwoot, which disconnected the chat data from our core platform's context (customer history, inventory, bookings) and introduced latency, synchronization issues, and reliability dependencies. We need a fast, native, unified inbox built directly into OHC where the owner can view and respond to all messages in one place.

  # Research Report
  **Findings & Competitive Analysis:**
  - **Shopify Inbox & Wix Inbox:** Provide basic unified messaging but lack deep integration with AI agents that can auto-draft responses based on a customer's specific purchase history across all channels.
  - **Zendesk/Intercom:** Extremely powerful but far too complex and expensive for a single-person SMB or local operator.
  - **Chatwoot Architecture Audit:**
    - Analyzed the open-source Chatwoot repository (`https://github.com/chatwoot/chatwoot`).
    - **Data Models:** Relies heavily on `Conversations`, `Messages`, `Inboxes`, `Contacts`, and `Channel` entities (e.g., `Channel::Whatsapp`, `Channel::WebWidget`).
    - **Real-time:** Uses ActionCable for WebSocket-based real-time UI updates.
    - **Webhooks:** Ingests incoming messages via provider webhooks, processes them, and routes them to the correct inbox and conversation.
  - **OHC Opportunity:** By building this natively in Rust inside `onehumancorp/mono`, we can achieve strict row-level security (multi-tenant isolation) natively. We can seamlessly inject OHC's AI (The Ambassador) to draft responses before the owner even sees the notification, providing a "read-approve" workflow rather than "read-reply."

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[External Channels: WhatsApp/IG/Email] -->|Webhook/API| B(Ingress Gateway / Webhook Handlers)
      B --> C{Channel Adapters - Rust}
      C --> D[(PostgreSQL - Unified Graph)]
      C --> E[Event Mesh]
      E --> F[WebSocket Manager]
      F -->|Real-time update| G[Mobile/Web Client]
      E --> H[The Ambassador Agent]
      H -->|Context Query| D
      H -->|Draft Reply| D
      H -->|Trigger UI update| F
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Home Screen:** "Unified Inbox" widget displaying unread counts and AI-drafted reply indicators (e.g., "3 Drafts Ready").
  - **Inbox List View:** A clean list of conversations showing customer name, channel icon (WhatsApp, IG), and message preview. Translucent Glass styling.
  - **Conversation View:** Standard chat interface but augmented. The bottom input area shows an AI-drafted response if available, with "Approve" (primary button) and "Edit" (secondary action).
  - **Mobile UX Flow:** Owner receives push notification -> Taps notification -> Opens conversation -> Sees customer context (past orders) at the top -> Reads AI drafted reply -> Taps "Approve" -> Message is dispatched via the correct channel adapter.

  ### AI Agent Integration Points
  - **The Ambassador (Customer Success):** Subscribes to the event mesh for `message.created`. Uses RAG against the tenant's context to generate a draft.
  - **The Manager (Operations):** Interacts if the message contains intent for booking or inventory checks.

  ### Key Design Decisions
  - **Native Rust Implementation:** Retire Chatwoot entirely. Implement data models (`Inbox`, `Conversation`, `Message`, `Contact`, `ChannelAdapter`) natively in Rust using SQLx and PostgreSQL to enforce `tenant_id` Row Level Security.
  - **Omnichannel Adapters:** Abstract channel specifics so the core messaging logic treats a WhatsApp message and a Web Widget message identically.
  - **Proactive AI:** Shift the paradigm from manual response to AI-assisted approval.

  # Implementation Prompt
  **User-Facing Outcome:** The owner opens their OHC app and sees all customer messages from WhatsApp, Instagram, and Web Chat in a single, unified list. For a new inquiry, a perfectly accurate reply is already drafted by the AI; the owner simply taps "Approve" to send it.

  **CUJ & Acceptance Criteria:**
  1. Define and implement the PostgreSQL database schema for the omnichannel chat system (`inboxes`, `conversations`, `messages`, `contacts`, `channel_adapters`) ensuring strict `tenant_id` isolation.
  2. Implement the Rust backend services in `src/server/integrations/chat/` to handle CRUD operations for these models.
  3. Build a WebSocket manager in Rust to push real-time updates to connected clients when new messages arrive.
  4. Implement at least one channel adapter (e.g., a mock WhatsApp webhook ingress) that converts an external payload into the unified `Message` format.
  5. Provide Playwright E2E tests: A test that simulates an incoming webhook message, verifies it appears in the UI, and allows the user to send a reply back through the adapter.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []