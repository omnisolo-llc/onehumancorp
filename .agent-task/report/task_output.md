issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  **Problem Statement**
  The current platform documentation and codebase indicate the mandate to retire the third-party Chatwoot service in favor of a native, high-performance omnichannel chat and customer support system built natively in Rust. Non-technical owners like Maya, Carlos, and Priya need unified inbox capabilities spanning SMS, WhatsApp, Instagram, and web chat that scale properly, handle multi-tenancy securely, and integrate directly with OHC agents for auto-replies without leaving the OHC platform.

  **Research Report**
  As mandated, an exhaustive audit of the `chatwoot/chatwoot` source code was performed. The Chatwoot architecture relies heavily on separate channel models (`Channel::Api`, `Channel::Email`, `Channel::FacebookPage`, `Channel::Sms`, `Channel::WebWidget`, `Channel::Whatsapp`, etc.) which are polymorphic to a central `Inbox` entity. Inboxes connect to `Conversation` models, which aggregate `Message` entities from multiple channels. Contacts represent the users reaching out across these channels.

  Competitors like Shopify Inbox, Meta Business Suite, and Zendesk all centralize omnichannel messaging into a unified thread model, enabling AI agent orchestration over the top. OHC requires a matching set of data models and real-time (WebSocket) infrastructure to replicate these capabilities at the edge, using Rust and PostgreSQL.

  **Design Doc**
  - **Architecture Overview (Rust & PostgreSQL)**:
    - **Models**:
      - `Inbox`: Configuration for a unified entry point, polymorphic relation to channels.
      - `Channel`: Trait or enum encompassing WebWidget, WhatsApp, SMS, Email, etc.
      - `Contact`: Unified customer profile, mergeable across identifiers (email, phone).
      - `Conversation`: A continuous thread between a `Contact` and an `Inbox`.
      - `Message`: Individual messages within a conversation.
    - **API & Real-time**:
      - gRPC internal services for AI agent hooks (e.g., "The Promoter Agent" and "The Ambassador Agent").
      - REST API for client webhooks (Twilio, WhatsApp Cloud API).
      - WebSocket (axum/tokio-tungstenite) for real-time frontend updates (PWA/Flutter).
    - **Multi-tenancy**:
      - All entities must strictly enforce `tenant_id` via PostgreSQL Row Level Security (RLS).
  - **Mobile UX Flow (375px first)**:
    - **Unified Inbox Screen**: A simple list view of conversations sorted by most recent. Unread indicators and channel icons (e.g., WhatsApp, Web) shown on each row.
    - **Conversation Detail Screen**: Standard chat interface. Messages from the contact on the left, agent/owner replies on the right. An input area with an "AI Draft" button prominently displayed.
  - **AI Agent Integration Points**:
    - Webhook or internal event bus triggers when a new `Message` is created.
    - AI agents can create a `Message` with `status: draft` for owner approval, or auto-send based on tenant settings.

  **Implementation Prompt**
  Implement the core Rust backend microservice (or module within the monolith) for the OHC Omnichannel Chat System. This must include the fundamental PostgreSQL schemas for Inboxes, Contacts, Conversations, and Messages with RLS multi-tenancy enforced. Build the REST endpoints to receive messages from a simulated external channel (e.g., Web Widget) and a WebSocket server to push real-time updates to connected clients. Ensure that the AI agent orchestration layer can easily hook into the message creation lifecycle to draft automated replies. Do not build the entire Flutter frontend in this PR, but provide the necessary backend contracts and a minimal HTML/JS test client to verify the WebSocket behavior.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
