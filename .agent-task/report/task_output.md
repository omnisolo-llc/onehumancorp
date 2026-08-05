issue_title: "Implement Custom Rust Omnichannel Chat System"
issue_description: |
  **Problem Statement:**
  OneHumanCorp (OHC) currently lacks a native omnichannel chat and customer support engine. Relying on external third-party services like Chatwoot for customer chat violates the core engineering mandate to keep OHC an integrated, assistant-first experience. Non-technical owner/operators (like Maya the baker or Carlos the field service owner) need to unify Instagram DMs, WhatsApp messages, emails, and website chats into a single actionable feed without managing external software, separate chat agent logins, or brittle third-party integrations.

  **Research Report:**
  As mandated by the standard, external Chatwoot integration is 100% RETIRED. I have cloned and benchmarked the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) to understand its architecture:
  - **Models Evaluated:** Evaluated models include `Channel::Api`, `Channel::WebWidget`, `Conversation`, `Message`, `Contact`, and `Inbox`.
  - **Core Concepts:**
    - `Inbox`: Represents a channel (Web, Email, SMS, WhatsApp) connected to the system.
    - `Conversation`: A thread of messages tied to an Inbox and Contact.
    - `Message`: Individual messages within a conversation.
    - `Contact`: The customer interacting with the business.
  - **Why Native Rust?** A native implementation in Rust, deeply integrated into OHC's multi-tenant architecture, ensures lower latency, unified authentication (SPIFFE/SPIRE), row-level tenant isolation, and direct compatibility with the existing OHC Assistant (AI Triage). It allows owners to respond to all channels directly within the OHC PWA.

  **Design Doc:**
  We need to build a native Rust multi-tenant omnichannel chat engine in `onehumancorp/mono`.
  - **Core Entities:**
    - `ChatInbox`: Represents the intake channel (e.g., WebWidget, API, Email). Must include `tenant_id` for isolation.
    - `ChatConversation`: A thread linking a `Contact` to a `ChatInbox`.
    - `ChatMessage`: Individual messages in a conversation.
    - `ChatContact`: The external user.
  - **Channel Connectors:**
    - Start with a "Web Widget" channel that exposes a secure token and configuration (colors, welcome text, pre-chat form).
    - API endpoints to ingest messages from external webhooks (e.g., Twilio for SMS/WhatsApp, Meta for Instagram).
  - **Integration with OHC:**
    - The "Work Triage" agent will monitor new `ChatConversation`s and generate tasks or draft replies based on the `ChatMessage` content.
    - WebSocket or SSE connections to stream real-time updates to the OHC Flutter/PWA frontend.
  - **Multi-Tenancy:**
    - All database tables must have `tenant_id` and enforce PostgreSQL Row Level Security (RLS) policies.

  **Implementation Prompt:**
  1. **Schema & Database:** Create database migrations for `chat_inboxes`, `chat_conversations`, `chat_messages`, and `chat_contacts` with `tenant_id` and RLS enabled.
  2. **Rust Backend (gRPC/REST):** Implement the core CRUD APIs in Rust (using the existing OHC backend framework, e.g., tonic for gRPC or axum for REST) to manage inboxes, start conversations, and send/receive messages.
  3. **Web Widget Support:** Implement endpoints to serve the configuration for a Web Widget channel (similar to Chatwoot's `Channel::WebWidget`).
  4. **Frontend Integration:** Update the OHC Flutter App (PWA) to display an "Inbox" or "Messages" view that unifies these conversations, allowing the owner to read and reply.
  5. **Verification:** Write unit tests with 100% coverage and Playwright E2E tests verifying an owner can configure a web widget, receive a message from a simulated customer, and reply to it in the OHC UI.

  **Priority:** P0 (Critical - Foundational feature for Work Triage and Customer Assistant)
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
