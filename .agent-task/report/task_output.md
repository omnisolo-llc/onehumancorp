issue_title: "Build OHC Native Rust Chat Engine Architecture (Chatwoot Replacement)"
issue_description: |
  # Build OHC Native Rust Chat Engine Architecture (Chatwoot Replacement)

  ## Problem Statement
  OHC requires a high-performance, multi-tenant omnichannel customer support and chat engine natively in Rust inside `onehumancorp/mono`. Currently, the system relies on an external Chatwoot service, which introduces latency, operational complexity, and security/multi-tenancy concerns. The mandate is to completely retire Chatwoot and replicate its core features natively in OHC.
  Small business owners like Maya (baker) and Carlos (handyman) need to communicate with customers across multiple channels (Instagram, SMS, Email, etc.) from a single assistant-led flow. The current chat system lacks a native, robust, and scalable architecture to support this.

  ## Research Report
  - **Source Code Benchmarking:** We cloned and analyzed the Chatwoot source code (`https://github.com/chatwoot/chatwoot`), specifically focusing on:
    - Data Models: `Account`, `User`, `Inbox`, `Channel`, `Contact`, `Conversation`, `Message`, `Team`, `Attachment`, `Webhook`.
    - Channel Adapters: `email`, `facebook_page`, `instagram`, `line`, `sms`, `telegram`, `twitter_profile`, `web_widget`, `whatsapp`.
    - Services: `action_service`, `message_window_service`, `new_message_notification_service`.
  - **OHC Implementation:** The current OHC native Rust implementation in `src/server/services/chat/` provides very basic models (`ChatInbox`, `ChatChannel`, `ChatContact`, `ChatConversation`, `ChatMessage`) and a simple `ChatService` to perform CRUD operations. It severely lacks the depth required to replicate Chatwoot.
  - **Gaps Identified:**
    1.  **Multi-Channel Support:** The current `ChatChannel` model only stores a `channel_type` string and `config` JSON. We need robust channel adapters (Rust traits) similar to Chatwoot's approach to handle channel-specific logic (e.g., verifying webhooks, sending messages to specific provider APIs).
    2.  **Conversations & Messages:** The current models lack fields like `additional_attributes`, `custom_attributes`, `status_changed_at`, `snoozed_until`, `first_reply_created_at`, `last_activity_at`, `priority` (for conversations), and `message_type`, `content_type`, `private`, `external_source_ids` (for messages).
    3.  **Real-time Capabilities:** OHC needs a WebSocket-based real-time messaging architecture, likely using something like `tokio-tungstenite` or `actix-web-actors` (depending on the framework used) integrated with Redis Pub/Sub for distributed scaling.
    4.  **Agent & Team Assignment:** We need mechanisms to route conversations to specific agents or teams, similar to Chatwoot's assignment policies.
    5.  **Multi-Tenancy:** The current implementation uses `tenant_id`, which is a good start, but strict row-level security (RLS) in PostgreSQL is mandated and must be ensured across all chat-related tables.
    6.  **Attachments:** No support for message attachments.

  ## Design Doc
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
    TENANT ||--o{ CHAT_INBOX : owns
    TENANT ||--o{ CHAT_CONTACT : owns
    CHAT_INBOX ||--o{ CHAT_CHANNEL : contains
    CHAT_INBOX ||--o{ CHAT_CONVERSATION : receives
    CHAT_CONTACT ||--o{ CHAT_CONVERSATION : participates
    CHAT_CONVERSATION ||--o{ CHAT_MESSAGE : contains
    CHAT_MESSAGE ||--o{ CHAT_ATTACHMENT : has

    CHAT_CONVERSATION {
        uuid id PK
        uuid tenant_id FK
        uuid inbox_id FK
        uuid contact_id FK
        uuid assignee_id FK "Optional"
        string status "open, resolved, snoozed, pending"
        integer priority "Optional"
        datetime snoozed_until "Optional"
        datetime last_activity_at
    }

    CHAT_MESSAGE {
        uuid id PK
        uuid tenant_id FK
        uuid conversation_id FK
        string sender_type "contact, agent, bot, system"
        uuid sender_id "Optional"
        string message_type "incoming, outgoing, template"
        integer content_type "text, html, markdown"
        text content
        boolean private "Internal note"
    }
  ```

  ### Mobile UX Flow (375px first)
  1.  **Work Triage View:** The owner opens the app. A unified "Inbox" card shows pending conversations grouped by priority/urgency.
  2.  **Conversation View:** Tapping a conversation opens a standard chat UI. The header shows the contact's name, avatar, and channel icon (e.g., Instagram logo).
  3.  **Reply Box:** A sticky input area at the bottom. It includes options to switch between public reply and internal note (private).
  4.  **Agent AI Assistance:** A "Draft Reply" button is prominently visible, triggering the AI Customer Assistant to generate a context-aware response based on the conversation history and business knowledge.

  ### AI Agent Integration Points
  -   **Customer Assistant Agent:** Listens to `conversation_created` and `message_created` events. Automatically drafts replies for the owner's review or sends them directly if confidence is high and auto-reply is enabled.
  -   **Work Triage Agent:** Analyzes incoming messages to categorize, prioritize, and summarize the intent, updating the conversation's priority or assigning it to specific teams/agents.

  ### Key Design Decisions
  -   **Rust Trait for Channels:** Define a `ChannelAdapter` trait in Rust that all specific channel implementations (Email, WhatsApp, Instagram) must implement. This ensures a consistent interface for receiving and sending messages across different platforms.
  -   **PostgreSQL RLS:** Ensure all chat tables (e.g., `chat_conversations`, `chat_messages`) have Row-Level Security policies enforcing `tenant_id` isolation.
  -   **Redis for Real-time:** Use Redis Pub/Sub to broadcast message events to the correct WebSocket connections for active clients.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Your task is to significantly expand the native Rust Chat Engine in `src/server/services/chat/` to reach feature parity with the core capabilities of Chatwoot, focusing on multi-channel support and robust conversation/message handling.

  **Critical User Journey (CUJ):**
  Maya, the baker, receives a custom cake inquiry via an Instagram DM. The system must capture this message through an Instagram channel adapter, create/update a contact for the user, create an open conversation in her unified inbox, and display the message. Maya should be able to view the conversation, see the source is Instagram, and type a reply. The system must then route her reply back through the Instagram channel adapter to the customer.

  **Acceptance Criteria:**
  1.  **Expanded Models:** Update the `models.rs` (and corresponding DB migrations, if necessary) to include richer fields for `ChatConversation` (status enum, priority, timestamps) and `ChatMessage` (message_type, content_type, private flag).
  2.  **Channel Adapter Architecture:** Introduce a mechanism (e.g., a Rust trait or enum-based dispatcher) in the service layer to handle different channel types generically. Implement a dummy/mock adapter for a specific channel (e.g., `Email` or `Instagram`) to prove the abstraction works.
  3.  **Service Enhancements:** Update `ChatService` methods in `service.rs` to handle the new fields and complex logic (e.g., updating `last_activity_at` on the conversation when a new message is sent).
  4.  **Testing:** Achieve 100% unit test coverage for the new models and service methods. Write tests simulating the flow of receiving a message from a channel and sending a reply.
  5.  **No Mocks (except external APIs):** Use real database interactions in tests. Do not prescribe specific API endpoint structures; focus on the core Rust service and database layer architecture first.

  ## Priority and Scope
  **Priority:** P0 (critical)
  **Estimated Scope:** Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
