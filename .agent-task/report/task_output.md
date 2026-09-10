issue_title: "Architecture & Integration Strategy for Native Rust Omnichannel Engine"
issue_description: |
  # Task: Architecture & Integration Strategy for Native Rust Omnichannel Engine

  ## Problem Statement
  Currently, OneHumanCorp requires an omnichannel inbox capable of routing user interactions across Web Chat Widget, WhatsApp, Instagram DMs, Email, SMS, Telegram, Line, and custom Webhooks. Relying on disconnected services impedes the core persona needs, such as real-time response by AI agents acting on behalf of the operator (e.g. Maya replying to Instagram DMs). We need a unified real-time engine to persist conversations, manage typing indicators, deliver cross-tenant routing safely, and present an actionable feed.

  ## Research Report
  - We analyzed the existing implementations in similar products like Tencent Workbuddy, Front, and Intercom.
  - A key missing element is a high-performance, strictly multi-tenant WebSocket and SSE real-time messaging pipeline built directly in Rust for the core services.
  - Standard open-source solutions such as axum (for WebSockets) combined with async message queues (e.g., using Redis Pub/Sub for cross-node fanout or Apache Kafka for massive persistence) can provide the necessary low-latency engine.

  ## Design Doc
  - **Architecture diagram (Mermaid.js)**:
    ```mermaid
    graph TD
    A[Client App - Flutter] --> B(Rust API Gateway & WebSocket Server);
    B --> C{Channel Router};
    C --> D[WhatsApp Adapter];
    C --> E[Instagram DM Adapter];
    C --> F[Web Chat Adapter];
    C --> G[Real-Time Engine];
    G --> H[(PostgreSQL RLS DB)];
    G --> I[(Redis Pub/Sub)];
    ```
  - **UI/UX Flow (Mobile-First 375px)**:
    - Unified 'Inbox' bottom tab.
    - Conversation list with clear channel badges (WhatsApp icon, IG icon, Web icon).
    - Chat view with unread badges, offline indicators, typing states, and AI-drafted reply suggestions.
  - **AI Agent Integration Points**:
    - The `omnisolo.integration` service and the Universal Provider Facade will listen for events from the Real-Time Engine to auto-draft responses or execute actions (e.g., creating a booking deposit link).
    - Agents use the `omnisolo.memory` service to retain customer context.

  ## Implementation Prompt
  Implement the foundation of the native Rust omnichannel engine using `axum` for WebSocket / SSE support.
  1. Define the core data models: `Conversation`, `MessageEnvelope`, and `ChannelType`, enforcing RLS by including `tenant_id`.
  2. Implement a unified WebSocket handler (`/ws/omnichannel`) in the existing Rust backend (or set up a new crate under `src/server/ohc/omnichannel` if required).
  3. Ensure events correctly route to simulated `Adapter` traits (e.g. `WhatsAppAdapter`, `WebChatAdapter`).

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
