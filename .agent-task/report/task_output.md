issue_title: "Architecture: Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  # Native Rust Omnichannel Chat System

  ## Problem Statement
  OneHumanCorp (OHC) is an assistant-first and owner-centered application. Our business personas (Maya, Carlos, Priya, Leo, Fatima, Nora, Jun) all need unified communications (Instagram DMs, SMS, WhatsApp, Email, Web Chat) to manage their daily work. Previously, OHC relied on an external dependency (Chatwoot), but Chatwoot has been 100% retired. This presents a critical architectural gap: OHC lacks a native, multi-tenant, high-performance omnichannel inbox capable of real-time messaging, AI agent interventions, and seamless integration with our Rust and PostgreSQL backend. Without this, users cannot coordinate customer requests into actionable tasks within a single command center.

  ## Research Report
  ### Competitor and Open Source Benchmark (Chatwoot)
  I have audited the [Chatwoot source code](https://github.com/chatwoot/chatwoot) as mandated. Chatwoot's architecture relies on:
  - **Inboxes**: A unified abstraction for different channels (e.g., Email, Twitter, Facebook, Web Widget).
  - **Conversations & Messages**: The core entities grouping multi-party messaging.
  - **Contacts**: The customer profiles linked to conversations.
  - **Channel Adapters**: Specific models that handle the ingress/egress of data for different providers.
  - **Real-time Engine**: ActionCable (WebSockets) broadcasting events across sessions.
  - **Agent Routing**: Rules to assign conversations to teams or agents.

  ### OHC Architecture Alignment
  To replicate and improve this natively in OHC:
  - We will implement this natively in **Rust** as a high-performance gRPC microservice (`ohc-chat-engine`) within `onehumancorp/mono`.
  - We will enforce **strict row-level security (RLS)** in PostgreSQL using `tenant_id` to guarantee tenant isolation.
  - We will use **Redis Pub/Sub** combined with Rust-based WebSockets (e.g., using `tokio` and `axum`) to broadcast real-time events efficiently.
  - **AI Department Coordination**: OHC AI agents (Customer Assistant, Operations Assistant) will operate as virtual agents natively subscribed to the real-time event bus, capable of analyzing inbound messages, retrieving context, drafting replies, and invoking tools (e.g., creating a booking or sending a payment link).

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      TENANT ||--o{ CONTACT : has
      TENANT ||--o{ CONVERSATION : has
      INBOX ||--o{ CHANNEL_ADAPTER : configured_with
      INBOX ||--o{ CONVERSATION : receives
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      MESSAGE ||--o{ ATTACHMENT : includes

      TENANT {
          uuid tenant_id PK
      }
      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
          string channel_type
      }
      CONVERSATION {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status
      }
      MESSAGE {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          string content
          string sender_type
          uuid sender_id
      }
  ```

  ### Mobile UX Flow (375px First)
  - **Unified Inbox List**: The user opens the app and sees a combined prioritized list of conversations from all channels. Each card (44x44px minimum touch target) displays the contact name, channel icon, last message snippet, and unread indicator.
  - **Conversation View**: Tapping a conversation opens a standard chat interface. Translucent sticky headers (macOS Translucent Glass style). A bottom input area allows typing a reply.
  - **AI Suggestions**: Inline AI drafts appear as distinct, slightly tinted message bubbles with an "Approve & Send" or "Edit" button, making it clear the AI is proposing a response, not sending it blindly.
  - **Offline/Flaky Network**: Messages sent while offline are marked as "Sending..." locally and queued. The Rust backend handles idempotency when the network is restored.

  ### AI Agent Integration Points
  - **Message Ingress**: When a new `MESSAGE` is created via the Rust API, it fires a Redis event. The **Customer Assistant Agent** consumes this event, reads the conversation history, and can either automatically reply (if confident and allowed) or draft a reply for the owner.
  - **Contextual Actions**: Agents can recognize intents (e.g., "how much for a cake?") and proactively fetch pricing from the Sales Assistant, injecting a quick-action "Send Quote" button into the owner's UI.

  ## Implementation Prompt
  **For the Implementer Agent:**
  Create the native Rust backend service and PostgreSQL schema for the OHC Omnichannel Inbox. Implement the core data models: `Inbox`, `Conversation`, `Message`, and `Contact`. Ensure every table includes a `tenant_id` and has Row-Level Security (RLS) enabled. Expose a gRPC/REST API to list conversations, send messages, and fetch contacts. Additionally, create the frontend Flutter components for the unified inbox list and conversation view, ensuring they strictly follow the 375px mobile-first design system with Translucent Glass materials and minimum 44x44px touch targets. Finally, write Playwright E2E tests demonstrating a full Critical User Journey (CUJ): an owner logs in, sees a new conversation, reads a drafted AI reply, and approves it. Do not hardcode any specific channel providers yet; use a generic "Web Widget" channel for the initial E2E flow. Ensure all Bazel tests pass.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
