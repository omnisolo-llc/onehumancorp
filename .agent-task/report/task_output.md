issue_title: "Native Rust Omnichannel Chat: Unified Inbox Architecture & Real-Time Messaging"
issue_description: |
  # Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" simply aggregate messages without context. To make matters worse, relying on third-party systems like Chatwoot introduces external dependencies, increased latency, and a fragmented data model that makes it hard to unify customer context seamlessly.

  As part of the initiative to retire external Chatwoot dependencies, we need a native Rust omnichannel chat system for OneHumanCorp. This system will securely handle Meta webhooks, WebSocket real-time widget chats, and multi-tenant isolation, giving owners a lightning-fast, zero-friction unified inbox experience on a 375px mobile display.

  # Research Report
  **Findings & Competitive Analysis:**
  - **Chatwoot Audit:** Analyzed standard omnichannel models including Inboxes, Contacts, Conversations, Messages, ChannelAdapters (e.g., WebWidget, API, WhatsApp, Email), and real-time WebSocket events. Chatwoot heavily relies on Redis pub/sub and ActionCable for WebSockets.
  - **OHC Opportunity:** By building natively in Rust within `onehumancorp/mono`, we can achieve microsecond latencies, strictly enforce Row Level Security (RLS) via `tenant_id` at the database and API level, and integrate directly with OHC's "Ambassador Agent" for AI-drafted replies. This completely removes the overhead of maintaining an external Ruby-on-Rails/Postgres/Redis stack just for chat.
  - **Competitors (Shopify Inbox, Wix Inbox):** Generally slower and less capable of AI-driven proactive replies out of the box. A native high-performance Rust backend combined with an offline-tolerant Flutter UI provides a significant competitive advantage.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[WhatsApp / Instagram DM] -->|Webhook| B(Rust Omnichannel Gateway)
      C[Web Chat Widget] -->|WebSocket| B
      B --> D{Tenant Isolation & Auth Layer}
      D -->|Validation| E[Rust Core Chat Engine]
      E -->|Persistence| F[(PostgreSQL with RLS)]
      E -->|Pub/Sub| G[Redis Event Bus]
      G --> H[WebSocket Broadcaster]
      H --> I[OHC Mobile Shell 375px]
      E --> J[The Ambassador Agent]
      J -->|Context & AI Draft| F
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Unified Inbox Feed (Mobile):** The default screen displays a vertically scrollable list of active conversations. Each card shows the customer's name, avatar, last message snippet, channel icon (e.g., WhatsApp, Instagram), and a distinct visual indicator if an AI-drafted reply is pending.
  - **Interaction:** Tapping a conversation opens the chat view. The header displays customer context (e.g., past orders). The main area is the message timeline. If an AI draft exists, it floats above the input bar as a "glassmorphic" card.
  - **Action:** 1-Tap "Send Draft" button. Or tap into the native keyboard to edit.
  - **Visual Design:** OHC Premium Token library with Apple/Ubiquiti-style hierarchy. Clean spacing, readable typography.

  ### AI Agent Integration Points
  - **The Ambassador Agent:** Subscribes to the `message.created` event via the Redis event bus. Upon new incoming messages, it retrieves the customer's identity graph and recent context, generates a drafted reply, and persists it as a pending message draft in the conversation.
  - **The Manager Agent:** Monitors operational keywords (e.g., "cancel order", "book appointment") to tag conversations or proactively surface relevant operational actions (e.g., calendar booking UI) to the owner.

  ### Key Design Decisions
  - **Rust + WebSockets:** High-performance async WebSocket handling natively in Rust to handle thousands of concurrent tenant connections efficiently.
  - **Strict Multi-Tenancy:** Every query and WebSocket subscription must enforce `tenant_id`. No cross-tenant data leakage is structurally possible.
  - **Zero-Trust & Security:** Webhooks from Meta/WhatsApp are signature-verified. WebSocket connections require valid SPIFFE/SPIRE-backed or session-authenticated tokens.

  # Implementation Prompt
  **User-Facing Outcome:**
  As an owner (e.g., Maya the baker), when a customer messages me on WhatsApp, my OHC app immediately pings me. I open the app on my phone to see the message in my unified inbox, alongside a perfectly crafted AI reply that I can approve and send with one tap.

  **CUJ & Acceptance Criteria:**
  1. Define the core Rust data models: `Inbox`, `Conversation`, `Message`, and `Contact`, strictly enforcing `tenant_id`.
  2. Implement a unified Rust HTTP/WebSocket handler that can accept webhooks from external channels and manage persistent WebSocket connections for real-time mobile/web clients.
  3. Wire up the internal pub/sub (Redis) to broadcast `message.created` events to connected WebSocket clients.
  4. Ensure all database writes for the chat module go through PostgreSQL with Row Level Security (RLS) enabled.
  5. Provide Playwright E2E tests: A test that simulates an incoming webhook, verifies the message is stored correctly, and confirms the message is broadcasted to a connected mock WebSocket client representing the owner's mobile view.

  # Priority
  P0

  # Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []