issue_title: "Native Rust Omnichannel Inbox (Chat system Replacement)"
issue_description: |
  # Problem Statement
  OHC requires a high-performance omnichannel inbox that handles real-time messaging from multiple channels (Instagram DMs, WhatsApp, SMS, Web Chat, Email). Previously, Chat system was used as a 3rd-party dependency. To ensure zero-trust isolation, multi-tenancy, and deep AI agent integration, OHC needs a native Rust implementation of Chat system's core capabilities, tailored for our specific non-technical owner/operator personas like Maya (baker) and Carlos (handyman).

  # Research Report
  - **Market Context**: Traditional SMB owners face high friction when managing disconnected customer inquiries. Chat system provides a robust omnichannel architecture (Channels, Inboxes, Conversations, Messages, Contacts) but relies on Ruby on Rails.
  - **Architecture Learnings from Chat system**:
    - Abstraction of `Channel` (e.g., `Channel::Whatsapp`, `Channel::Email`).
    - The `Inbox` unifies multiple channels for a single agent/team.
    - WebSockets handle real-time UI updates via pub/sub.
  - **Competitive Landscape**:
    - *Shopify Inbox*: Heavily e-commerce focused but weak on service-based bookings (which Carlos needs).
    - *Wix Inbox*: Good aggregation, lacks true autonomous AI drafting based on a unified identity graph.
  - **OHC Specific Needs**: Native Rust integration allows our AI Agents (e.g., Operations Agent, Customer Success Agent) to intercept, draft, and optionally auto-reply to messages directly via an Event Mesh without external roundtrips.

  # Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[External Webhooks: WhatsApp, IG, SMS] -->|HTTP POST| B(Gateway & Signature Verification)
      B --> C{Channel Adapters}
      C --> D[Conversation Manager]
      D --> E[(Tenant-Isolated PostgreSQL)]
      D --> F(Redis Pub/Sub)
      F --> G[WebSocket Server]
      G --> H[Mobile/Tauri UI]
      F --> I[Event Mesh]
      I --> J[AI Agent Service - The Ambassador]
      J -->|Drafts Reply| D
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Mobile Inbox Feed**: A single feed combining all channels. Each card shows the customer name, channel icon (e.g., WhatsApp), and message preview.
  - **Thread View (Translucent Glass)**:
    - Sticky header with customer context and recent purchase/booking data.
    - Message bubbles.
    - AI "Draft Ready" overlay above the composer: The AI pre-drafts responses based on context.
    - Native keyboard support with an easy 1-tap "Approve & Send Draft" button.
  - **Advanced Routing**: Hidden by default. If needed, a toggle allows escalating a conversation or re-assigning it.

  ### AI Agent Integration Points
  - Incoming messages publish a `message.created` event.
  - The AI Agent (`The Ambassador`) subscribes to this, retrieves the `Contact`'s omnichannel history, and creates a `Message` with type `draft`.
  - The UI listens for `draft.created` via WebSocket to instantly show the owner what the AI suggests.

  ### Key Design Decisions
  - **Rust Native**: Guarantees high concurrency for WebSockets and reduces operational complexity (no separate Ruby/Rails stack).
  - **Tenant Isolation**: Row-Level Security (RLS) on `conversations` and `messages` tables ensuring data cannot leak between tenants.
  - **Proactive Drafts over Auto-Reply**: To build trust with owners (like Maya), AI drafts the reply for human approval first by default, with an option to enable auto-reply for specific intents.

  # Implementation Prompt
  - **Outcome**: A native Rust API that accepts incoming webhook messages, normalizes them into a unified `Conversation` model, triggers a local AI agent to draft a response, and pushes updates via WebSockets to a mobile-first UI.
  - **CUJ**: An Instagram DM arrives -> Webhook received -> Rust backend normalizes to `Message` -> AI drafts reply -> Owner opens app and sees the pre-drafted reply -> Taps "Approve" -> Message sent back via Instagram.
  - **Acceptance Criteria**:
    - Implement `Inbox`, `Conversation`, and `Message` models in Rust with RLS.
    - Create a webhook endpoint that correctly authenticates and ingests mock Instagram/WhatsApp payloads.
    - Integrate a WebSocket server that broadcasts new messages to connected UI clients.
    - Include Playwright E2E tests: A webhook is triggered, and a connected UI client instantly reflects the new message and the subsequent AI draft.

  # Priority
  P0

  # Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
