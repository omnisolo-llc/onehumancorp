issue_title: "Native Rust Omnichannel Chat System Architecture"
issue_description: |
  # Native Rust Omnichannel Chat System Architecture

  ## Problem Statement
  OneHumanCorp currently lacks a fully unified, native, omnichannel customer support and chat engine. As per the engineering standards, ThirdPartyChatPlatform as an external third-party service is strictly retired. OHC must implement its own high-performance, multi-tenant omnichannel engine natively in Rust to handle WhatsApp, Instagram DMs, SMS, email, and web widget chats seamlessly within the same platform. The absence of this system fragments the user experience and forces non-technical owners to manually juggle multiple communication platforms, contradicting our mission of a single, unified "assistant-first" feed.

  ## Research Report
  - **Market Context**: Existing unified inboxes (Shopify Inbox, Wix Inbox) aggregate messages but lack deep, native context (like a user's entire purchase history) directly mapped to an AI drafting assistant out of the box. ThirdPartyChatPlatform provides a robust open-source reference for data models (inboxes, conversations, messages, channel adapters, webhooks).
  - **Codebase Constraints**: OHC enforces strict multi-tenant row-level security in PostgreSQL and uses Rust for backend services. The system must natively integrate with our "The Ambassador" AI agent to proactively draft responses instead of just reading/routing them.
  - **Benchmarking**: Inspecting ThirdPartyChatPlatform's architecture reveals key components:
    - **Channels**: Abstractions for connecting to external APIs (WhatsApp Cloud API, Meta Graph API for IG, Twilio/SMS).
    - **Inboxes**: Aggregations of channels for a tenant.
    - **Conversations & Messages**: Threading models.
    - **Real-time**: WebSockets for immediate UI updates.
  - **Design Decisions**:
    - **Zero External Dependencies**: We build this in `src/server/services/chat/` using Rust (axum/tonic) and PostgreSQL (sqlx with RLS).
    - **Mobile-First UX**: The drafted messages surface in a 375px-first "Action Required" feed, heavily leveraging glassmorphic UI components.
    - **AI Integration**: The Event Mesh will trigger "The Ambassador" on new inbound messages.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[External Webhooks: IG, WhatsApp, SMS, Email] --> B[Rust API: Ingress Service]
      B --> C{Tenant Security & Rate Limiting}
      C --> D[Omnichannel Router]
      D --> E[(PostgreSQL: Conversations, Messages)]
      D --> F[Redis / Valkey: PubSub, Cache]
      F --> G[WebSocket Server]
      G --> H[Flutter/PWA Client 375px UI]
      D --> I[Event Mesh]
      I --> J[The Ambassador Agent]
      J --> K[AI Draft Generation]
      K --> E
      K --> G
  ```

  ### Mobile UX Flow (375px First)
  - **Work Triage Feed**: The owner opens the app. The top card shows "1 New Message from Sarah (WhatsApp)".
  - **Unified Context View**: Tapping the card opens the chat. The top half displays contextual business data (Sarah's last order: #458, vegan cake).
  - **AI Drafting**: The bottom half displays an AI-generated draft: "Hi Sarah, yes we still make the vegan chocolate cake! Shall I set up an order for this weekend?".
  - **Interaction**: The user can tap "Send Draft" (1-tap action) or "Edit" (opens native mobile keyboard).
  - **Visuals**: Clean Apple/Ubiquiti-style hierarchy, translucent materials, strong spacing.

  ### AI Agent Integration Points
  - **Ingest Trigger**: Every inbound message (via webhook) fires a standard domain event.
  - **The Ambassador**: Subscribes to the event mesh. On message receipt, it queries the `Customer Identity Resolution Engine` and the tenant's product/order catalog via RAG.
  - **Draft Persistence**: The drafted response is saved as a `pending_draft` on the conversation and broadcasted via WebSockets to immediately appear in the owner's UI.

  ## Implementation Prompt
  **User-Facing Outcome**: As Maya the baker, when a customer sends an Instagram DM asking if I still sell a specific cake they bought last year, I open the OHC app to find the message in my main feed with a perfectly drafted reply ready to send in one tap. I do not need to log into Instagram or a separate chat app.

  **CUJ & Acceptance Criteria**:
  1.  **Backend Services**: Implement Rust data models and Axum/Tonic API endpoints for `Inboxes`, `Channels`, `Conversations`, and `Messages` with strict PostgreSQL Row-Level Security (RLS) based on `tenant_id`.
  2.  **Channel Adapters**: Implement the foundational trait/interface for a `ChannelAdapter` and provide a mock/webhook adapter for testing.
  3.  **Real-time Layer**: Implement a WebSocket handler that streams new messages and AI drafts to the client securely.
  4.  **UI Components**: Implement the 375px-optimized Flutter/PWA chat view displaying the conversation history, contextual customer data, and the AI draft approval card.
  5.  **E2E Testing**: Provide a Playwright E2E test where an admin logs in, an external webhook simulates an incoming message, the UI updates in real-time, the AI draft appears, and the user approves it.

  ## Priority & Scope
  - **Priority**: P0 (Critical for core product vision)
  - **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
