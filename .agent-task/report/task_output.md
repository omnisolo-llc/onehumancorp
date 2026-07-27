issue_title: "Implement Native Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  **Title**: Implement Native Rust Omnichannel Chat System to Replace Chatwoot

  **Problem Statement**:
  OHC currently relies on external/third-party services for omnichannel messaging (or lacks a deep, multi-tenant capable native system with full omnichannel features like WebSocket real-time messaging, macros, SLA policies, etc.). Relying on Chatwoot as an external service creates data silos, breaks our strict multi-tenant row-level security model, and degrades the owner's experience with fragmented UI. Maya (baker) needs her Instagram DMs, Carlos needs his SMS quotes, and Priya needs her web-chat widget unified into a single, real-time native OHC inbox that is observable by AI agents.

  **Research Report**:
  After auditing the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) and comparing it against OHC's existing native models (`src/server/services/chat/models.rs`), significant gaps remain.
  - Chatwoot provides comprehensive data models for `Channel`, `Contact`, `Conversation`, `Message`, `Inbox`, `Webhook`, `AgentBot`, and `CannedResponse`.
  - Chatwoot handles WebSockets (ActionCable) for real-time `message.created`, `conversation.updated` events.
  - OHC's existing Rust chat service has only basic tables (`ChatInbox`, `ChatChannel`, `ChatContact`, `ChatConversation`, `ChatMessage`) without robust real-time WebSocket broadcasting, Channel Adapters (Instagram, WhatsApp, SMS, Web Widget), Macros, SLA policies, and AI agent hooks.
  - Competitors like Shopify Inbox and Wix Inbox heavily rely on native real-time chat engines deeply coupled with their inventory/order models. OHC needs this native coupling for AI agents to easily read/write context.

  **Design Doc**:
  - **Architecture Diagram (Mermaid.js)**:
    ```mermaid
    erDiagram
      Tenant ||--o{ Inbox : owns
      Inbox ||--o{ ChannelAdapter : configures
      ChannelAdapter ||--o{ Conversation : sources
      Conversation ||--o{ Message : contains
      Conversation }o--|| Contact : links_to
      Conversation ||--o{ AgentAssignment : has
      Message ||--o{ Attachment : includes
    ```
    - **Core Entities**:
      - `Inbox`: Aggregates conversations from multiple channels.
      - `ChannelAdapter`: Polymorphic configuration for Instagram, Web, SMS, WhatsApp.
      - `Conversation`: The threaded state of the customer interaction.
      - `Message`: Immutable log of communication (Agent, Customer, or AI Bot).
    - **WebSocket Real-time Messaging**:
      - Implement a Rust-based WebSocket server using `tokio-tungstenite` or Axum WebSockets.
      - Redis Pub/Sub for cross-node message broadcasting.
      - Event types: `conversation.created`, `message.created`, `contact.updated`.
    - **UI Wireframes / Mobile UX Flow**:
      - **375px Screen**: Clean Apple/Ubiquiti translucent header.
      - **Home/Feed Screen**: A unified Inbox list. Each row shows the customer avatar, channel icon (e.g., Insta), last message preview, and unread badge.
      - **Conversation Screen**: Standard chat bubbles. Bottom input bar native keyboard. Plus icon for AI Quick Replies or Attachments (Quotes, Payment Links).
      - **Action**: Tap on an incoming DM -> opens Conversation Screen -> AI agent has already drafted a translucent "Suggested Reply" floating above the keyboard.
    - **AI Agent Integration Points**:
      - **Work Triage Agent**: Subscribes to `conversation.created` and applies auto-labels.
      - **Customer Assistant Agent**: Triggered on `message.created` (when no human is actively typing) to draft a response in the background and save it as an `AgentDraft` attached to the conversation.
    - **Key Design Decisions**:
      - 100% native Rust implementation. No external Chatwoot deployment.
      - All chat tables use strict multi-tenant Row Level Security (`tenant_id`).
      - AI drafts are stored in the database, allowing human owners to approve/edit before sending.

  **Implementation Prompt**:
  As an implementer, build the native Rust Omnichannel Chat backend and Flutter frontend to replace Chatwoot.
  1. Extend the existing `src/server/services/chat/models.rs` and database schema to support Channel Adapters, Agent Drafts, and Attachments.
  2. Implement an Axum WebSocket handler for real-time messaging, using Redis Pub/Sub for broadcasting to connected clients.
  3. Create the Channel Adapter interface for incoming webhooks (e.g., Instagram/Meta webhook handler).
  4. Build the Flutter Mobile UI (375px-first) with a Unified Inbox screen and a Conversation screen featuring translucent glass styling.
  5. Ensure a "Suggested Reply" UI component is visible for AI-generated drafts.
  Acceptance Criteria: A message sent via API to the channel webhook appears instantly in the Flutter UI via WebSocket, and the AI agent background worker triggers to create a draft reply.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
