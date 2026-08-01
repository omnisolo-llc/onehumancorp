issue_title: "Implement Native Rust Omnichannel Inbox & Chat System"
issue_description: |
  **Problem Statement**
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. We must completely retire the external Chatwoot dependency and implement our own lightning-fast, highly-scalable omnichannel inbox and customer support system natively in Rust inside `onehumancorp/mono`. This system must handle multi-tenant isolation, real-time WebSocket communication for web widgets, and webhook integrations for WhatsApp and Instagram.

  **Research Report**
  Findings & Competitive Analysis:
  - **Chatwoot Source Audit:** An audit of the Chatwoot source code (`app/models`) reveals core models required for feature parity: `Account` (Tenant), `Inbox`, `Conversation`, `Message`, `Contact`, `ChannelAdapter` (e.g., `Channel::Whatsapp`, `Channel::WebWidget`), `AgentBot`, and `CannedResponse`. Chatwoot uses separate tables for channels and joins them to inboxes via polymorpic associations. It heavily relies on ActionCable (WebSockets) for real-time updates.
  - **Shopify Inbox:** Aggregates chat and email but relies heavily on manual responses or basic, rigid auto-replies. It does not proactively draft contextual responses based on full customer history across all channels.
  - **Wix Inbox:** Good aggregation, but AI features are mostly limited to "improving tone" or generating generic replies, not acting as an autonomous customer success agent.
  - **OHC Opportunity:** Implement a highly scalable Rust backend that natively understands the unified customer graph. By building this natively in our repo, we enforce strict `tenant_id` RLS (Row Level Security) on every entity (Inbox, Conversation, Message) and eliminate cross-service latency between our core DB and the chat system.

  **Design Doc**

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[WhatsApp Webhook] -->|Ingest| B(Rust API / Gateway)
      C[Insta DM Webhook] -->|Ingest| B
      D[Web Widget WebSocket] -->|WSS| E(Rust Async WebSocket Handler)
      B --> F{Omnichannel Router}
      E --> F
      F -->|Save| G[(PostgreSQL - Unified Models)]
      F --> H[Event Mesh - Redpanda/Kafka]
      H --> I[The Ambassador Agent]
      I -->|Draft Reply| G
      I -->|Action Required| J[Mobile App Feed 375px]
      J -->|Approve| K[Dispatcher]
      K --> A/C/D
  ```

  ### Data Model & Invariants
  The implementation must include these core entities with strict `tenant_id` RLS:
  - `Inbox`: Represents a collection of channels.
  - `Channel::Whatsapp` / `Channel::WebWidget`: Channel-specific configurations.
  - `Contact`: The unified customer profile.
  - `Conversation`: Links a Contact to an Inbox.
  - `Message`: Individual messages within a Conversation (supports text, attachments).
  - `CannedResponse`: Reusable owner responses.

  ### Mobile UX Flow (375px First)
  - **Mobile Inbox List:** A clean, Unifi-style modular list of active conversations. Unread indicators must be highly visible.
  - **Conversation View:** Standard chat bubbles. The bottom input area supports native keyboard, quick-reply (CannedResponses), and an "AI Draft" button.
  - **AI Agent Draft Card:** A translucent glass card appears at the bottom if the Ambassador agent has prepared a response, with "Approve" (Primary) and "Edit" (Secondary) buttons.

  ### AI Agent Integration Points
  - **The Ambassador (Customer Success Agent):** Subscribes to the Event Mesh. On new incoming `Message`, it queries the `Contact`'s history and product catalog, then drafts a `Message` with status `draft`.
  - **Zero Trust:** Multi-tenant isolation is enforced at the DB level (RLS). The Rust WebSocket handler must authenticate the user via SPIFFE/SPIRE or secure session tokens before subscribing them to their tenant's `Conversation` topic.

  **Implementation Prompt**
  **User-Facing Outcome:** As an owner, I want a single lightning-fast screen in my app where I can see and reply to WhatsApp, Insta DMs, and website chat, with the AI pre-drafting replies based on the customer's history.

  **CUJ & Acceptance Criteria:**
  1. Implement the core native Rust data models (`Inbox`, `Conversation`, `Message`, `Contact`, `ChannelAdapter`) with strict PostgreSQL RLS by `tenant_id`.
  2. Implement an Actix/Axum Rust API endpoint to receive incoming webhooks (e.g., simulated WhatsApp message).
  3. Implement a secure WebSocket handler in Rust to broadcast new `Message` inserts to connected frontend clients (web widget or owner app).
  4. The system must persist incoming messages, link them to the correct `Contact` and `Conversation`, and publish a "MessageCreated" event to the Event Mesh.
  5. Provide complete unit tests (100% coverage) for the Rust models and handlers.
  6. Provide a Playwright E2E test: A simulated user sends a message to the webhook, the owner logs into the 375px mobile UI, sees the message appear in real-time via WebSocket, and successfully sends a reply.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
