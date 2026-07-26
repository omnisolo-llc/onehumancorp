issue_title: "Native Rust Omnichannel Chat Engine"
issue_description: |
  # Native Rust Omnichannel Chat Engine

  ## Problem Statement
  As mandated by the engineering standards, OneHumanCorp must completely retire Chatwoot as an external third-party service. OHC must implement its own high-performance, multi-tenant omnichannel customer support and chat engine natively in Rust inside the `onehumancorp/mono` repository. Small business owners (like Maya the Baker or Carlos the Handyman) need a unified inbox that brings together SMS, Email, Instagram DMs, and WhatsApp into a single, cohesive feed. They don't have the technical expertise to integrate third-party solutions, and they need AI (draft replies, context surfacing) built natively into the communication flow.

  ## Research Report
  - **Competitor Benchmark (Chatwoot)**: Based on an audit of the `https://github.com/chatwoot/chatwoot` source code, Chatwoot models data heavily around `Accounts` (Tenants), `Inboxes` (Channels), `Conversations` (Threads), `Messages`, and `Contacts`. They use a Ruby on Rails backend with Postgres and Redis, alongside a Vue frontend.
  - **System Constraints**: To meet OHC's scale and performance requirements, our implementation must be built natively in Rust.
  - **Core Architecture Gap**: OHC currently lacks native data models, APIs, and real-time (WebSocket) infrastructure to manage omnichannel conversations at scale with strict row-level multi-tenant isolation.

  ## Design Doc

  ### Architecture Diagram
  We need strict multi-tenant data isolation. The primary entities map as follows:

  ```mermaid
  erDiagram
    TENANT ||--o{ INBOX : has
    TENANT ||--o{ CONTACT : has
    TENANT ||--o{ CONVERSATION : has
    INBOX ||--o{ CONVERSATION : receives
    CONTACT ||--o{ CONVERSATION : initiates
    CONVERSATION ||--o{ MESSAGE : contains

    TENANT {
      uuid id PK
      string name
    }
    INBOX {
      uuid id PK
      uuid tenant_id FK
      string channel_type "e.g., Email, SMS, IG"
      jsonb credentials
    }
    CONTACT {
      uuid id PK
      uuid tenant_id FK
      string name
      string identifier "email or phone"
    }
    CONVERSATION {
      uuid id PK
      uuid tenant_id FK
      uuid inbox_id FK
      uuid contact_id FK
      string status "open, resolved"
    }
    MESSAGE {
      uuid id PK
      uuid conversation_id FK
      uuid sender_id "nullable (agent/ai)"
      string content
      string message_type "incoming, outgoing"
    }
  ```

  ### Real-Time Architecture & AI Agent Integration Points
  *   **WebSocket Engine**: Rust (Axum + Tokio + Tungstenite) for handling high-concurrency WebSocket connections.
  *   **Event Fan-out**: Redis Pub/Sub to fan-out messages to connected mobile/desktop clients across instances.
  *   **AI Customer Assistant**: Listens to the `message.created` stream. If the conversation is unassigned and fits criteria, the AI agent drafts a response (stored on the Message record as `draft_reply`) or auto-responds based on owner settings.
  *   **AI Operations Assistant**: Parses incoming messages for intent (e.g., "I need to reschedule") and surfaces actionable cards to the owner.

  ### Mobile UX Flow
  *   **Unified Inbox View (375px)**: A simple list view of conversations sorted by latest activity. Unread indicators and AI draft badges (e.g., "✨ Draft ready") are clearly visible.
  *   **Conversation View (375px)**: Standard chat interface. Incoming messages on the left, outgoing on the right. If an AI draft exists, it appears floating above the composer with "Approve", "Edit", or "Discard" actions.
  *   **Touch Targets**: All interactive elements (Send, Approve) must be >= 44x44px. No horizontal scrolling.

  ## Implementation Prompt
  **User-Facing Outcome**: The owner can view, manage, and reply to messages from various channels (Instagram, SMS, Email) in a unified inbox natively within OHC. AI drafts are automatically generated for incoming inquiries.
  **CUJ**:
  1. The owner (Maya) logs into OHC on her phone.
  2. She navigates to the "Inbox" tab.
  3. She sees a new message from a customer via Instagram.
  4. She taps the conversation, sees the message history, and an AI-drafted reply.
  5. She taps "Approve" to send the AI draft.

  **Acceptance Criteria**:
  *   Implement the backend foundation (Rust APIs + Database schema) for the native Omnichannel Chat Engine to replace Chatwoot, focusing on the core unified inbox logic.
  *   Ensure 100% unit test coverage for the new services and strict row-level multi-tenant isolation.
  *   Design the system to support WebSocket real-time updates and Redis pub/sub.
  *   Do NOT build the actual external channel adapters (Meta, Twilio) yet. Focus on the core engine and internal API.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
