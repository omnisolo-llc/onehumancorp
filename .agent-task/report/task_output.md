issue_title: "[Research] Architect Native Rust Omnichannel Chat (Chatwoot Replacement)"
issue_description: |
  **Problem Statement**
  Currently, OHC relies on integrating external customer support components, or lacks a deeply integrated omnichannel customer communication system. The directive requires 100% retiring Chatwoot as an external service and implementing a high-performance native Rust omnichannel chat system within OHC. Small-business owners like Maya (baker managing Instagram DMs) and Carlos (handyman fielding quotes via SMS) need a unified, multi-tenant inbox that automatically coordinates with AI agents to draft replies, track context, and schedule follow-ups without leaving the application.

  **Research Report**
  After cloning and analyzing the `chatwoot` repository, their core schema supports multi-tenant operations via an `accounts` table linked to `users`, `inboxes`, `channels`, and `conversations`.

  Key Chatwoot architectural concepts to replicate natively:
  - **Inboxes/Channels**: Representing connection points (e.g. `channel_email`, `channel_facebook_pages`, `channel_sms`, `channel_web_widgets`, `channel_whatsapp`).
  - **Conversations**: Central entity holding a sequence of `messages`, associated with an `inbox` and `contact`.
  - **Messages**: Core data payload, tracking attachments, status, and sender type (agent, contact, bot).
  - **Contacts**: End customers interacting across channels.
  - **Agents & Agent Bots**: Assignable entities for a conversation (including AI bots for automated replies).
  - **Real-time Delivery**: WebSocket streaming of events (like `message.created`) to connected UI clients.

  By bringing this into the `ohc-mono` backend (Rust), we can leverage our existing Redis (for real-time pubsub), PostgreSQL (multi-tenant RLS), and AI Agent jobs system.

  **Design Doc**
  - **Data Model (Rust/PostgreSQL)**
    - `tenant_id` on every table (RLS enforced).
    - `ohc_chat_inboxes`: Represents an intake channel (e.g., Email, SMS, Web Widget, WhatsApp).
    - `ohc_chat_contacts`: External customers.
    - `ohc_chat_conversations`: Threads linking a contact and an inbox. Tracks status (open, closed, pending).
    - `ohc_chat_messages`: Individual messages in a conversation. Tracks `sender_type` (Customer, Agent, AI Bot).
    - `ohc_chat_channel_configs`: Stores credentials/webhooks for third-party integrations (Twilio, Meta, etc.).

  - **Architecture Details**
    - **API Layer**: Expose gRPC/REST endpoints for inbox management, sending messages, and fetching conversation history.
    - **Real-time WebSockets**: Implement a WebSocket handler (using `tokio-tungstenite`/`axum` ws) to push events to the Flutter frontend. Use Redis PubSub (`ohc:chat:events:{tenant_id}`) for horizontally scaling the WS servers.
    - **AI Agent Integration**: Implement PostgreSQL `SKIP LOCKED` job queue workers that trigger on `message.created` where `assigned_to` is an AI agent. The AI agent drafts a reply and inserts it as a pending message, which the owner can approve or the system can auto-send.
    - **Mobile UX Flow (375px)**: A unified "Inbox" tab. A scrollable list of conversations showing unread badges and channel icons. Tapping a conversation opens a chat view with native mobile keyboard support, a text input area, an "AI Suggest" button, and quick-action tools (e.g., "Send Payment Link").

  - **Key Design Decisions**
    - Implement everything in the `src/server/integrations/chat` module (or a dedicated `ohc_chat` crate).
    - Enforce tenant isolation via our existing auth context.
    - Design a trait `ChannelAdapter` to easily plug in different providers (SMS, Web Widget) down the line.

  **Implementation Prompt**
  Implement the core database schema, API service layer, and WebSocket streaming infrastructure for the new native Rust Omnichannel Chat system.
  - Define the data models for Inboxes, Contacts, Conversations, and Messages.
  - Build the gRPC/REST controllers to manage these entities.
  - Implement a WebSocket endpoint to push new messages to connected clients.
  - Ensure all database queries strictly enforce `tenant_id` isolation.
  - Write comprehensive unit tests for the data access layer and service logic, ensuring 100% coverage.
  - Provide a Playwright E2E test verifying a user can create an inbox, simulate receiving a message, and view it in the UI.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
