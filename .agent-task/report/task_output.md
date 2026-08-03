issue_title: "Architecture: Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  # Problem Statement
  OneHumanCorp (OHC) owners need to interact with customers across various channels (WhatsApp, Web Widget, Instagram, Email) directly from their 375px mobile device. Previously, OHC relied on an external Chatwoot integration for this capability. Chatwoot has been fully retired as an external service. To provide a lightning-fast, secure, and natively AI-integrated experience, OHC must build its own multi-tenant omnichannel chat engine natively in Rust. This system must eliminate third-party dependencies, reduce latency, and enable seamless integration with OHC AI agents for automated proactive drafting (The Ambassador).

  # Research Report
  **Chatwoot Source Code Audit:**
  An extensive audit of the `chatwoot/chatwoot` repository (v3.x) revealed the following core architectural paradigms:
  - **Data Models:** `Conversation`, `Message`, `Inbox`, `Contact`, `Account` (Tenant), and `Channel::*` polymorphic associations (e.g., WhatsApp, WebWidget, Email).
  - **Inboxes & Routing:** Messages arrive via webhooks or WebSockets, get assigned to an `Inbox` (which maps to a specific `Channel`), and create/update a `Conversation`.
  - **Real-time:** Real-time sync achieved via ActionCable (WebSockets).

  **Competitive Analysis:**
  - **Shopify Inbox:** Basic aggregation but lacks deep omnichannel identity resolution across WhatsApp and Instagram.
  - **Zendesk/Intercom:** Extremely robust but enterprise-heavy; not suited for small business owners on mobile.
  - **OHC Native Rust Advantage:** By building this natively in Rust inside `onehumancorp/mono`, we guarantee zero-trust multi-tenancy (RLS + SPIFFE/SPIRE), predictable low latency, and direct synchronous access to AI agent orchestration (KAIROS) without external API overhead.

  # Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[WhatsApp Webhook] --> B[Rust Actix API Gateway]
      C[Instagram Webhook] --> B
      D[Web Widget WebSocket] --> E[Rust WebSocket Server]
      B --> F{Omnichannel Router / Inbox Service}
      E --> F
      F --> G[(PostgreSQL - Tenant RLS)]
      F --> H[Event Bus / Redis]
      H --> I[AI Agent Orchestrator]
      I --> |Drafts Reply| F
      F --> J[Mobile App UI 375px]
  ```

  ### Mobile UX Flow (375px First)
  - **Inbox View:** Unified list of active conversations categorized by status (Unread, Action Required, Bot Handled). Each row displays an avatar, channel icon (e.g., WhatsApp), customer name, and a snippet of the latest message.
  - **Conversation View:** Real-time chat interface showing the full history. The AI (The Ambassador) places drafted responses in an interactive "Glassmorphism" card directly above the composer.
  - **Action Flow:** The owner taps "Send Draft" (1-tap) or "Edit" (opens native keyboard).
  - **Visuals:** Clean Apple/Ubiquiti-style hierarchy, restrained translucent materials, and robust touch targets (min 44x44px).

  ### AI Agent Integration Points
  - **The Ambassador:** Subscribes to the `message.created` event via Redis/Event Bus. When a message arrives from a customer, the agent queries the unified `Contact` history and drafts a response as an internal note or a pending AI-drafted message inside the `Conversation`.

  ### Key Design Decisions
  - **Rust Microservice/Crates:** The chat engine will be modularized as a native Rust crate within the mono repo, providing high-performance webhook consumption and WebSocket broadcasting.
  - **Row Level Security (RLS):** Every table (`inboxes`, `conversations`, `messages`, `contacts`) will enforce strict tenant isolation (`tenant_id`) at the database level.
  - **Polymorphic Channels:** Adopt Chatwoot's scalable channel model where an `Inbox` belongs to a specific channel provider configuration (e.g., `Channel::WhatsApp`, `Channel::WebWidget`), allowing seamless addition of new channels later.

  # Implementation Prompt

  **User-Facing Outcome:**
  As an owner (e.g., Carlos), when a customer messages my WhatsApp Business number, I instantly receive a push notification and see the message in the OHC mobile app. The AI has already drafted a response based on the customer's previous service history. I can approve and send the response with a single tap, keeping all communication centralized.

  **CUJ & Acceptance Criteria:**
  1. Implement Rust database schemas and models for `Inbox`, `Channel`, `Contact`, `Conversation`, and `Message` with `tenant_id` RLS enforced.
  2. Implement webhook ingest endpoints for at least one channel (e.g., WhatsApp or a generic API channel) that correctly parses payloads and creates `Message` and `Conversation` records.
  3. Implement a WebSocket endpoint for real-time message delivery to the mobile UI.
  4. Ensure any incoming message automatically triggers an internal event that a mock or real AI agent can consume to draft a reply.
  5. UI Requirement: Display the inbox and conversation threads flawlessly on a 375px width (Playwright E2E tests required), with no fake data.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []