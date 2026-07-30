issue_title: "Implement Core Chatwoot-like Architecture in Rust"
issue_description: |
  # Problem Statement
  OneHumanCorp requires an embedded, high-performance, omnichannel customer support and chat system to replace external services like Chatwoot. Our business personas (Maya, Carlos, Priya, Leo, Fatima) need a unified inbox that brings together SMS, Email, Instagram DMs, and WhatsApp into a single stream. The current `src/server/services/chat` provides only a skeletal data model and service layer that falls far short of Chatwoot's extensive feature set (channels, auto-assignment, SLAs, custom attributes, macros, real-time WebSockets, robust analytics).

  # Research Report
  - **Chatwoot Source Code Audit**: Investigated `https://github.com/chatwoot/chatwoot` models (`Conversation`, `Message`, `Inbox`, `Channel::*`, `Contact`).
  - **Identified Gaps in Current Rust Implementation**:
    - Missing WebSockets (ActionCable equivalent) for real-time `message_created`, `conversation_updated` events.
    - Missing Channel adapters (currently just a generic `channel_type` string, needs explicit implementations for SMS, Email, WebWidget).
    - Missing rich message content types (attachments, rich text, buttons).
    - Missing conversation features: read receipts, SLAs, snoozing, priority, custom attributes.
    - Missing agent collaboration features: private notes, mentions, macros, canned responses.

  # Design Doc

  ## Architecture & Data Model (Mermaid)
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL : configures
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : starts
      CONVERSATION ||--o{ MESSAGE : contains
      USER ||--o{ MESSAGE : sends
      CONVERSATION ||--o{ CONVERSATION_PARTICIPANT : has

      INBOX {
          uuid id
          uuid tenant_id
          string name
          boolean enable_auto_assignment
          jsonb auto_assignment_config
      }
      CHANNEL {
          uuid id
          uuid tenant_id
          uuid inbox_id
          string type "web_widget|api|email|sms|whatsapp"
          jsonb config "credentials, webhooks"
      }
      CONVERSATION {
          uuid id
          uuid tenant_id
          uuid inbox_id
          uuid contact_id
          uuid assignee_id
          string status "open|resolved|pending|snoozed"
          int priority
          datetime snoozed_until
          jsonb custom_attributes
      }
      MESSAGE {
          uuid id
          uuid tenant_id
          uuid conversation_id
          string sender_type "contact|user|agent_bot|system"
          uuid sender_id
          string content
          int message_type "incoming|outgoing|template"
          boolean private_note
          jsonb attachments
      }
  ```

  ## Real-time & Agent Coordination
  - **WebSocket Layer**: Use `axum::extract::ws` integrated with a Redis pub/sub backplane (or NATS, which is in `Cargo.toml`) to broadcast events to the mobile/web clients.
  - **AI Department Coordination**:
    - The `Operations` or `Customer Service` AI Agent will subscribe to the `message_created` event on NATS.
    - If a conversation has no assignee or is flagged for AI handling, the Agent generates a draft or sends an automatic reply (e.g., "do you do vegan cakes?").
    - AI actions are recorded as `agent_bot` or `system` sender types, visible to the owner.

  ## Mobile UX Flow (375px First)
  - **Unified Inbox Screen**: A clean list view of all open conversations, prioritized by SLA or unread status. Avatars indicate the channel (e.g., an Instagram icon overlay).
  - **Conversation Thread Screen**: Chat bubbles for messages, inline display of private notes (styled differently, e.g., yellow tint). Bottom input bar switches seamlessly between "Reply to Customer" and "Add Private Note".
  - **Translucent Glass Material**: The navigation bar and floating action buttons should use the iOS/macOS translucent blur effect.

  # Implementation Prompt
  **User-Facing Outcome**: The owner can see incoming messages from various channels in a unified inbox, reply directly, add private notes for staff, and see AI agents automatically draft responses to common inquiries.

  **Acceptance Criteria**:
  1. Extend `ChatConversation`, `ChatMessage`, `ChatInbox`, and `ChatChannel` models in `src/server/services/chat/models.rs` to support the missing Chatwoot fields (e.g., status enums, private notes, priorities, custom attributes).
  2. Implement database migrations for the new schema using SQLx.
  3. Create a real-time event publisher (using NATS or Redis) in `ChatService` that emits `message_created` and `conversation_updated` events when DB mutations occur.
  4. Create an Axum WebSocket handler that allows clients to subscribe to a tenant's unified inbox events.
  5. Implement one specific Channel Adapter (e.g., Web Widget or API) that handles incoming messages and routes them to the correct Inbox and Conversation.
  6. E2E Test: A Playwright test that simulates a customer sending a message via the Web Widget channel, and verifies that the owner's UI (via WebSocket) receives the message in real-time and displays it correctly.

  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
