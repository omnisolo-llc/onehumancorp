issue_title: "[Architecture] Native Rust Omnichannel Chat Engine to Replace Chatwoot"
issue_description: |
  ## Problem Statement
  OHC requires a seamless, unified, and native real-time chat experience for owner/operators like Maya and Carlos, who need to unify Instagram DMs, SMS, WhatsApp, and Web Chat into a single triage feed. Previously, we relied on Chatwoot, but as per our architectural mandate, Chatwoot is 100% RETIRED as a third-party dependency. We must build our own high-performance, multi-tenant omnichannel customer support and chat engine natively in Rust to achieve tighter integration with our AI agents and backend systems, eliminating external dependencies.

  ## Research Report
  - **Market Context**: Platforms like Shopify Inbox, Meta Business Suite, and Chatwoot offer unified inbox capabilities. Chatwoot provides an open-source model but relies heavily on Ruby on Rails, Sidekiq, and external moving parts.
  - **Codebase Audit**: Our Rust backend under `src/server` supports gRPC/Axum and PostgreSQL. We need to introduce real-time WebSocket capabilities, channel adapters (e.g., for Meta APIs, SMS/Twilio), and a robust data model for unified conversations.
  - **Chatwoot Benchmarking**: Reviewing Chatwoot's architecture reveals a structured approach to Inboxes, Contacts, Conversations, Messages, and Channel Adapters. We can map these entities into Rust `structs` backed by Postgres, with Valkey (Redis) for real-time pub/sub across multi-tenant boundaries.

  ## Design Doc
  ### Architecture Diagram (Mental/Mermaid Model)
  - **Entities**:
    - `Tenant`: Top-level boundary.
    - `Contact`: The external user interacting via a channel.
    - `Inbox`: Logical grouping of channels.
    - `ChannelAdapter`: Configurations for Web, API, Instagram, WhatsApp.
    - `Conversation`: A unified thread between a contact and the owner (or AI).
    - `Message`: Immutable chat records with attachments and rich payloads.
  - **Real-Time Engine**: Axum WebSockets attached to a Valkey Pub/Sub backbone (`ohc:chat:tenant_id:conversation_id`).
  - **AI Coordination**: Agent "listener" hooks into the Pub/Sub stream to auto-draft replies or auto-triage for the Work Triage agent.

  ### Mobile UX Flow (375px First)
  - **Unified Inbox Screen**: A clean, iOS-style list of active conversations, clearly badged with the source channel icon (e.g., WhatsApp, Insta, Web). Translucent glass headers and swipe-to-archive actions.
  - **Chat Thread View**: Mobile-first message bubbles with integrated tap-to-pay or quote widgets directly inline. 44x44px target touch inputs for quick replies and AI-draft suggestions.

  ### AI Agent Integration Points
  - **Customer Assistant Agent**: Subscribes to new incoming messages. If a new lead asks about pricing, the agent uses tenant memory to draft a suggested reply, which appears inline for the owner with a "Send Draft" button.
  - **Operations Agent**: Parses incoming messages for booking requests and highlights calendar availability inline.

  ## Implementation Prompt
  **Goal**: Implement the core data model and initial Axum-based WebSocket infrastructure for the new native Rust Omnichannel Chat system.
  **CUJ**:
  1. Owner logs into OHC on mobile (375px).
  2. Opens the "Inbox" tab.
  3. Receives a real-time WebSocket message (via web widget simulation).
  4. The UI instantly updates the conversation list without polling.
  **Acceptance Criteria**:
  - Define `Conversation`, `Message`, and `Inbox` models in Rust (PostgreSQL + Row-Level Security).
  - Implement Axum WebSocket handler that subscribes to Valkey channels per-tenant.
  - Implement at least 5 E2E Playwright tests verifying real-time message delivery between two browser instances.
  - Do NOT implement specific external channels (like Meta API) yet; start with a mock/internal Web Chat adapter.
  - Ensure 100% unit test coverage for the new Rust module.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
