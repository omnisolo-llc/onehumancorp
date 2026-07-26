issue_title: "Native Rust Omnichannel Chat System Implementation (Chatwoot Replacement)"
issue_description: |
  **Title**: Native Rust Omnichannel Chat System Implementation (Chatwoot Replacement)

  **Problem Statement**:
  OHC currently lacks a native omnichannel customer support system. We have completely retired Chatwoot as an external service due to the need for tighter data integration, multi-tenancy rules, and AI agent coordination. For our personas (like Maya the baker and Carlos the handyman), they need a unified inbox that brings together Instagram DMs, WhatsApp messages, website chat, and email into a single, mobile-friendly interface where AI agents can draft replies while they sleep.

  **Research Report**:
  - The existing schema (`217_native_omnichannel_chat.sql`) outlines `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages` using PostgreSQL RLS for multi-tenant isolation.
  - Chatwoot's architecture relies on distinct Channel models (e.g., `Channel::WebWidget`, `Channel::Email`, `Channel::Api`), connecting to a unified `Inbox` and mapping to `Conversations` and `Messages`. It also handles WebSockets for real-time delivery and ActionCable for broadcasting.
  - Competitors like Shopify Inbox and Wix Inbox focus heavily on simple UI flows that don't overwhelm non-technical owners, whereas Chatwoot is built for support agents. We must simplify the UI for the owner persona, showing clear next actions and AI-drafted replies.

  **Design Doc**:
  - **Architecture diagram (Mermaid.js)**:
    ```mermaid
    erDiagram
      TENANT ||--o{ CHAT_INBOXES : owns
      CHAT_INBOXES ||--o{ CHAT_CHANNELS : contains
      CHAT_INBOXES ||--o{ CHAT_CONVERSATIONS : hosts
      CHAT_CONTACTS ||--o{ CHAT_CONVERSATIONS : initiates
      CHAT_CONVERSATIONS ||--o{ CHAT_MESSAGES : contains
      TENANT ||--o{ CHAT_CONTACTS : has
    ```
  - **Mobile UX flow (375px first)**:
    1. **Triage Feed (Home)**: Owner sees "3 new messages needing reply".
    2. **Conversation View**: Clean, translucent glass UI showing the chat thread. At the bottom, a suggested AI draft is ready to be sent or edited.
    3. **Channel Setup**: Hidden under advanced settings. A simple toggle to "Connect Instagram" or "Add Website Chat".
  - **AI agent integration points**:
    - **Customer Assistant Agent**: Listens to new `chat_messages` inserts via `SKIP LOCKED` PG job queue.
    - Drafts a response based on tenant's help documents (RAG) and previous conversation history.
    - Saves the draft to `chat_messages` with a `draft` status for owner approval.

  **Implementation Prompt**:
  Implement the backend Rust services and Flutter/Tauri UI for the native omnichannel chat system.
  - Implement gRPC endpoints for fetching conversations, sending messages, and reading inboxes.
  - Develop a real-time WebSocket layer for live message updates.
  - Implement the "Customer Assistant" AI worker that automatically drafts replies to new incoming messages.
  - Build the mobile-first UI for the conversation view with AI draft approval buttons, applying OHC Premium Token translucent glass styling.
  - Ensure 100% test coverage and full Playwright E2E coverage for the chat flow.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
