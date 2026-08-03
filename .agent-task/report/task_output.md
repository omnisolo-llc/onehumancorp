issue_title: "Architect Native Rust Omnichannel Chat to Retire Chatwoot Dependency"
issue_description: |
  # Mission Queue Protocol: Native Omnichannel Chat Architecture

  ## Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" simply aggregate messages without context. OHC currently aims to build an AI-first "Ambassador" agent but relies on an unfinished or missing native Rust implementation (based on Chatwoot's legacy architecture). We must natively build the foundational models, API endpoints, and webhook receivers for a unified inbox that supports our mobile-first 375px UX and Agentic Workflows.

  ## Research Report
  **Findings:**
  - **Chatwoot Architecture:** Chatwoot uses a robust multi-tenant model involving `Account`, `Inbox`, `Channel`, `Contact`, `ContactInbox`, `Conversation`, `ConversationParticipant`, `Message`, and `Attachment`.
  - **OHC Requirement:** The `chatwoot` external service is 100% RETIRED. OHC must implement these features natively in Rust.
  - **Current State:** OHC has basic stubs in `src/server/services/chat/models.rs` and `src/server/services/chat/service.rs`. However, it lacks comprehensive database schemas (missing `chat_inboxes`, `chat_channels`, etc., in DB migrations), complete RLS tenant isolation, channel-specific adapter interfaces (like Meta/WhatsApp webhooks), and real-time WebSocket capabilities.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ CHAT_INBOX : owns
      TENANT ||--o{ CHAT_CHANNEL : owns
      TENANT ||--o{ CHAT_CONTACT : owns
      CHAT_INBOX ||--o{ CHAT_CHANNEL : contains
      CHAT_INBOX ||--o{ CHAT_CONVERSATION : hosts
      CHAT_CONTACT ||--o{ CONTACT_INBOX : has
      CHAT_INBOX ||--o{ CONTACT_INBOX : has
      CHAT_CONTACT ||--o{ CHAT_CONVERSATION : participates
      CHAT_CONVERSATION ||--o{ CHAT_MESSAGE : contains
      CHAT_MESSAGE ||--o{ CHAT_ATTACHMENT : has
  ```

  ### Mobile UX Flow (375px First)
  - **Unified Feed:** Messages from WhatsApp, Insta DM, etc., appear in a single scrollable feed with standard Translucent Glass cards.
  - **Agent Draft Context:** Instead of just showing the message, the UI displays an "Agent Draft" card under the unread message, proposing a reply based on customer history, with 1-tap "Approve" or "Edit".
  - **Real-time Updates:** Incoming messages update the UI immediately without full page reloads.

  ### AI Agent Integration Points
  - **The Ambassador (Operations/CS):** Subscribes to the `work_item` or `message_created` event bus. Upon receiving a message, queries the `customer_profile` and unified conversation history, drafts a response, and inserts it into `agent_draft` or `action_required`.

  ## Implementation Prompt
  **Objective:** Implement the foundational database schema and Rust backend services for OHC's Native Omnichannel Chat, mirroring Chatwoot's core data models but optimized for OHC's multi-tenant architecture and AI agent integration.

  **Critical User Journey (CUJ):**
  A customer sends a message via WhatsApp (simulated via API). The system receives it, identifies the `ChatContact`, creates a `ChatConversation` (if one doesn't exist), and stores the `ChatMessage`. The owner opens the mobile app (375px) and sees the message in their unified inbox.

  **Acceptance Criteria:**
  1.  **Database Migration:** Create a new migration file (e.g., `src/server/db/migrations/20260702_native_chat_tables.sql`) implementing the core tables: `chat_inboxes`, `chat_channels`, `chat_contacts`, `contact_inboxes`, `chat_conversations`, `conversation_participants`, `chat_messages`, and `chat_attachments`.
  2.  **Row Level Security (RLS):** Ensure strict tenant isolation using `tenant_id` on all new tables with `ENABLE ROW LEVEL SECURITY`.
  3.  **Rust Models:** Update `src/server/services/chat/models.rs` to reflect the complete schema (using `sqlx` and `sea-orm` as appropriate for the repo).
  4.  **Rust Service:** Expand `src/server/services/chat/service.rs` with robust CRUD operations for the new models, including handling atomic conversation creation and message insertion.
  5.  **Testing:** Achieve 100% unit test coverage for the new service methods and verify the schema migration works using `bazel test //...`.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
