issue_title: "[Research] Architect Native Rust Omnichannel Chat System to Replace legacy chat system"
issue_description: |
  # Problem Statement
  OHC aims to provide an integrated "Owner Work Assistant" capable of receiving messages from multiple platforms (Instagram, WhatsApp, Email, Web Widget) and presenting them in a unified triage interface. Previously, legacy chat system was considered for this. However, legacy chat system as an external dependency is 100% RETIRED according to our OHC Engineering Standards. We need a native Rust omnichannel customer support & chat engine built directly into our monorepo (`onehumancorp/mono`) to guarantee multi-tenant row-level security, reduce operational complexity, and seamlessly coordinate with our AI agents without external API hops.

  # Research Report
  Our analysis of the legacy chat system open-source repository (`https://github.com/legacy chat system/legacy chat system`) reveals the core architectural components needed for a modern unified inbox:

  *   **Core Entities**: Account (Tenant), Inbox, Channel (Web, Email, API, WhatsApp, etc.), Contact, Conversation, Message, AgentBot.
  *   **Communication Flow**: Inbound webhooks/API calls are translated by "Channel Adapters" into normalized `Message` and `Conversation` models.
  *   **Real-time Updates**: WebSockets (ActionCable in legacy chat system) broadcast message events to active subscribers.
  *   **AI Integration Points**: legacy chat system uses `AgentBot` entities. OHC will need a robust hook system to allow our specialized AI agents (Customer Assistant, Operations Assistant) to intercept, draft, or automatically reply to messages.
  *   **Extensibility**: Macro support and automation rules are critical for SMB operational efficiency (e.g., auto-assigning tags based on keywords).

  # Design Doc
  ## Architecture
  *   **Service Layer (Rust)**:
      *   `src/server/chat/`: New Rust module for chat functionality.
      *   `Channel Adapters`: Traits and implementations for different channels (e.g., `InstagramAdapter`, `WhatsAppAdapter`, `WebWidgetAdapter`). These normalize incoming payloads into our internal `Message` format.
      *   `WebSocket Hub`: A high-performance Rust WebSocket server (likely using `tokio` and `axum` or `tungstenite`) to push real-time updates to connected clients (web/mobile).
      *   `AI Agent Hook System`: A pub/sub or synchronous interceptor pattern where incoming messages are evaluated by our local LLM pipelines (via the existing `src/server/agents/` module) before being persisted or broadcast, enabling features like "auto-draft replies."
  *   **Data Model (PostgreSQL + RLS)**:
      *   `chat_inboxes` (id, tenant_id, name, channel_type)
      *   `chat_conversations` (id, tenant_id, inbox_id, contact_id, status)
      *   `chat_messages` (id, tenant_id, conversation_id, sender_type, sender_id, content, created_at)
      *   *Crucially, all tables will enforce Row-Level Security (RLS) using `tenant_id`.*
  *   **Frontend (Flutter/PWA)**:
      *   Unified Inbox View: A 375px-optimized screen showing a unified feed of conversations across all channels.
      *   Conversation View: Chat interface with clear indicators of the channel source, AI drafted replies (differentiated visually, e.g., translucent glass styling), and quick action buttons.

  ## Mobile UX Flow (375px first)
  1.  **Work Triage Home**: Owner opens app, sees "3 New Messages (Instagram, Web)".
  2.  **Unified Inbox Screen**: List of active conversations, sorted by urgency/unread status. Clear icons indicate the channel (e.g., a small Instagram logo).
  3.  **Conversation Screen**:
      *   **Top**: Customer name, channel, status.
      *   **Middle**: Message history. If an AI agent has drafted a reply, it appears in a styled, translucent "Draft" card at the bottom.
      *   **Bottom**: Action bar. "Approve Draft", "Edit Draft", or manual text input.

  ## AI Agent Integration Points
  *   **Work Triage Agent**: Analyzes incoming messages and groups them (e.g., "3 cake inquiries").
  *   **Customer Assistant Agent**: Subscribes to the `message.created` event. If the message needs a reply, it generates a draft and saves it to the conversation context, triggering a UI update.

  # Implementation Prompt
  *Implement the foundational native Rust omnichannel chat system to replace external legacy chat system dependencies. This includes defining the core database schema (Inboxes, Conversations, Messages) with strict `tenant_id` Row-Level Security. Create the foundational Rust models and a unified API endpoint to fetch active conversations for a tenant. Ensure the API response is structured to support our AI agent drafting capabilities. Do not build specific channel adapters (WhatsApp/Instagram) yet; focus on the core normalized internal models and the API layer that the frontend unified inbox will consume. Implement at least one comprehensive E2E test verifying the creation and retrieval of a conversation within a tenant context.*

  # Priority
  P0

  # Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
