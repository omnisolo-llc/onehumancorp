issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  # Research Report & Implementation Brief

  ## Problem Statement
  OneHumanCorp (OHC) currently lacks a native Rust omnichannel customer support and chat engine, leaving a major capability deficiency. The requirement is to 100% retire external services like Chatwoot and implement matching functionality natively within `onehumancorp/mono` using a high-performance Rust backend.

  ## Research Findings
  After cloning the [Chatwoot source code](https://github.com/chatwoot/chatwoot) and auditing its architecture, it is evident that its core relies on extensive Ruby models across multiple channels (Web Widget, Email, Facebook, SMS, Twitter, Line, Whatsapp, Instagram). We must duplicate these concepts in a multi-tenant Rust architecture without using any external Chatwoot services.

  ### Competitor Systems Audit
  Leading scalable systems like Shopify Inbox and Stripe use high-concurrency event-driven architectures (like WebSockets mapped to local queues) and robust multi-tenant data isolations to offer native chat integrations without relying on third-party SaaS widgets.

  ## Design Doc
  **Architecture Overview**
  - **Backend**: Native Rust microservice/crate inside `onehumancorp/mono`.
  - **Database**: PostgreSQL with strict Row Level Security (`tenant_id` on all tables).
  - **Realtime**: WebSockets using a high-performance async runtime (Tokio) for instant message delivery.
  - **Entity Model**:
    - `Inbox`: Aggregates messages for a specific tenant.
    - `Conversation`: A thread of messages between a customer and the agent/system.
    - `Message`: Individual text/media payloads.
    - `Channel`: Adaptors for external APIs (e.g., Web Widget, WhatsApp, Email).
    - `Contact`: The end-user/customer.
  - **AI Department Coordination**: The `Customer & Relationship Assistant` handles background summarization and automated responses, reading from the `Inbox` stream.

  **Multi-Tenancy**
  All database interactions must explicitly filter by `tenant_id` and leverage PostgreSQL Row-Level Security for zero-trust data access.

  **Mobile-First UX**
  The UI component should follow the "Translucent Glass" aesthetic, optimized for a 375px viewport. Chat bubbles must be easily readable and action buttons must be at least 44x44px.

  ## Implementation Prompt
  Implement the foundation of the native Rust Omnichannel Chat engine. Focus on the core entity schemas, PostgreSQL integration, and basic Rust service layer.

  1. Create the base PostgreSQL schemas (migrations) for `inboxes`, `conversations`, `messages`, and `contacts`, enforcing multi-tenant `tenant_id` columns.
  2. Implement the Rust data model definitions in `src/server/ohc/domain/chat/mod.rs` (or equivalent).
  3. Set up the basic gRPC/REST APIs in Rust to create a message and fetch conversations for an inbox.
  4. Build a React component for the Web Chat Widget matching the OHC design system.
  5. Include full Unit (100%) and Playwright E2E tests covering a message flow from the widget to the backend.

  ## Target Persona
  **Maya (baker, 28)** & **Carlos (handyman, 42)** need an integrated inbox inside their OHC app to seamlessly reply to customer queries without opening a separate tool.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
