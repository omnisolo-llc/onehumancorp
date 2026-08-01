issue_title: "Implement Native Rust Omnichannel Chat Engine (Chatwoot Replacement)"
issue_description: |
  **Problem Statement**
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. The legacy reliance on third-party services like Chatwoot introduced latency, complexity, and compromised our strict Zero-Trust multi-tenant architecture. We need a native, high-performance, multi-tenant omnichannel chat engine written in Rust inside `onehumancorp/mono` that brings all customer interactions directly into the OHC assistant and perfectly pairs with our AI agents.

  **Research Report**
  - **Chatwoot Source Code Audit**: Investigated `https://github.com/chatwoot/chatwoot` (`app/models/`, omnichannel data models, controllers, channel adapters, WebSocket events). Chatwoot separates concepts into `Account` (Tenant), `Inbox`, `Channel` (WhatsApp, WebWidget, API, etc.), `Conversation`, `Message`, and `Contact`.
  - **Competitive Analysis**: Shopify Inbox and Wix Inbox aggregate messages but lack deep AI-context integration. Traditional unified inboxes require manual responses. OHC's native chat engine will act as the ingestion layer for "The Ambassador" (Customer Success Agent), enabling proactive, context-aware AI response drafting.
  - **Feature Parity Requirements**: We must natively replicate Inbox routing, Web Widget WebSockets, WhatsApp Cloud API webhooks, SLA policies, macros, and agent routing logic in Rust using strict Row Level Security (RLS) via `tenant_id`.

  **Design Doc**

  *Architecture Diagram*
  ```mermaid
  graph TD
      A[WhatsApp Webhook] -->|Ingress| B(Rust API Gateway)
      C[Instagram DM Webhook] -->|Ingress| B
      D[Web Widget WebSocket] <-->|Real-time| E(Rust WebSocket Service)
      B --> F{Omnichannel Ingestion Engine}
      E --> F
      F -->|Persist| G[(PostgreSQL with RLS)]
      F --> H[Redis Event Mesh]
      H --> I[Agent Triage System]
      I --> J[The Ambassador Agent]
      J -->|Context Lookup| G
      J -->|Draft Reply| K[Action Required Queue]
  ```

  *UI Wireframes & Mobile UX Flow (375px First)*
  - **Mobile Inbox Feed**: A clean list of active conversations. Each card shows the customer avatar, channel icon (WhatsApp, Instagram), and a preview of the latest message or drafted AI response.
  - **Conversation View (375px)**: Standard chat interface. Messages are bubbled. At the bottom, if the AI has drafted a reply, a translucent glassmorphism card appears above the text input: "Drafted by The Ambassador" with primary "Approve & Send" and secondary "Edit" buttons.
  - **UX Flow**: Owner gets push notification -> Opens app to Conversation View -> Reads customer message -> Reviews drafted AI reply -> Taps "Approve & Send" -> Native Rust backend dispatches to Meta/WhatsApp API instantly.

  *AI Agent Integration Points*
  - **The Ambassador (Customer Success Agent)**: Listens to the Redis Event Mesh for new `MessageCreated` events. Fetches conversation history and customer identity graph. Drafts a reply and persists it as an `Action Required` task tied to the conversation.
  - **Agent Routing**: The native chat engine must expose hooks to allow the Agent Triage system to intercept messages before they alert the owner.

  *Key Design Decisions*
  - **Strict Multi-Tenancy**: Every table (`inboxes`, `channels`, `conversations`, `messages`, `contacts`) MUST include `tenant_id` and have PostgreSQL RLS enabled.
  - **Rust Native**: Implemented entirely in Rust within `src/server/integrations/chat/` using Axum/Tower for APIs and Tokio for async event processing.
  - **Zero-Touch Fallback**: If AI confidence is low, the message simply appears as "Unread" without a drafted response, ensuring the owner manually intervenes.

  **Implementation Prompt**
  - **User-Facing Outcome**: As a business owner, when a customer sends a WhatsApp message, I receive it instantly in the OHC app. I see the customer's history and a pre-drafted, accurate response that I can approve with one tap.
  - **CUJ & Acceptance Criteria**:
    1. Implement Rust data models and PostgreSQL schemas for `Inbox`, `Channel`, `Conversation`, and `Message` with strict RLS.
    2. Implement REST endpoints for WhatsApp webhook ingestion and Web Widget message creation.
    3. Implement WebSocket server for real-time delivery to the owner's Flutter app.
    4. Provide Playwright E2E tests: A test webhook simulates an incoming message, the UI updates in real-time, the owner taps to reply, and the system simulates the outbound dispatch. No external dependencies (like Chatwoot) are involved.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
