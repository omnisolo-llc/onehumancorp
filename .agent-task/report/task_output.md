issue_title: "Native Rust Omnichannel Chat: Architecture & Data Models"
issue_description: |
  # Problem Statement

  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" simply aggregate messages without context. To resolve this and completely replace the external dependency, OHC needs a high-performance, native Rust omnichannel chat engine. This engine will not only aggregate messages but also tie into the `Customer Identity Resolution Engine` to power our AI agents (e.g., The Ambassador).

  # Research Report

  **Findings & Competitive Analysis:**
  - **Source Audit:** We have audited the legacy external system's source code, focusing on its omnichannel data models (`Conversation`, `Message`, `Inbox`, `ChannelAdapter`, `Contact`), controllers, webhooks, and WebSocket architecture. It relies on Ruby on Rails and PostgreSQL.
  - **OHC Architecture:** We are replacing the external dependency with a Rust-based microservice within `onehumancorp/mono`. This ensures better performance, tighter integration with our agent ecosystem (via Redis and Kafka/Event Mesh), and stricter multi-tenant isolation.
  - **Core Entities:** The new architecture needs robust entities with strict RLS (Row Level Security) per `tenant_id`.

  # Design Doc

  ### Architecture Diagram

  ```mermaid
  graph TD
      A[Instagram DM / WhatsApp / Email] -->|Webhook| B(Omnichannel Gateway - Rust)
      B --> C{Message Router}
      C -->|Lookups| D[Unified Customer Graph DB]
      C --> E[Inbox Engine]
      E --> F[Event Mesh / Kafka]
      F --> G[The Ambassador Agent]
      G -->|Draft Reply| H[Action Required Queue]
      H --> I[Mobile App Feed 375px]
      I -->|Approve| J[Omnichannel Dispatcher - Rust]
      J --> A

      subgraph Rust Omnichannel Chat Engine
      B
      C
      E
      J
      end
  ```

  ### Mobile UX Flow (375px First)
  - The native engine will power the unified inbox UI.
  - **Conversations List:** Fast, infinite-scroll list of active conversations across all channels.
  - **Message View:** Real-time updates via WebSockets. Unified context view showing past orders and AI-drafted replies.

  ### AI Agent Integration Points
  - The Chat Engine publishes `MessageReceived` events to the Event Mesh.
  - The Ambassador Agent listens to these events, fetches context, and drafts a reply, seamlessly interacting with the native Rust chat system rather than external APIs.

  ### Key Design Decisions
  - **Rust Native:** Build inside `src/server/integrations/chat/` or a dedicated Rust crate in the workspace.
  - **Multi-Tenancy:** Every entity (`Conversation`, `Message`, `Contact`, `Channel`) MUST have a `tenant_id` for RLS.
  - **Extensibility:** Use a trait-based `ChannelAdapter` system in Rust to easily add new channels (WhatsApp, Web Widget, IG).

  # Implementation Prompt

  **User-Facing Outcome:** The system can ingest, store, and route omnichannel messages entirely within OHC's native stack, powering the unified inbox and AI agents without relying on third-party services.

  **CUJ & Acceptance Criteria:**
  1. Define and implement the core Rust struct definitions and trait interfaces for `Conversation`, `Message`, `Contact`, `Inbox`, and `ChannelAdapter`.
  2. Implement strict multi-tenant isolation (`tenant_id` on all models).
  3. Create an initial in-memory or SQLx-backed repository layer for these entities.
  4. Implement a unified `MessageRouter` that can accept an incoming payload, resolve the `Contact`, and create a `Message` in the correct `Conversation`.
  5. Ensure 100% test coverage for the new Rust module.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
