issue_title: "Architecture & Implementation Plan: Native Rust Omnichannel Chat System"
issue_description: |
  # Native Rust Omnichannel Chat System

  ## Problem Statement
  As mandated by the engineering standards, OneHumanCorp (OHC) must retire Chatwoot as an external dependency and implement its own high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust. This system is critical for personas like Maya (baker managing Instagram DMs), Carlos (handyman fielding SMS inquiries), and Fatima (food cart taking WhatsApp pre-orders). They need a unified inbox that aggregates customer communications across platforms, allows AI agent intervention, and feels instantaneous on a 375px mobile screen.

  ## Research Report
  - **Chatwoot Audit**: Analyzed the Chatwoot Ruby on Rails codebase. Core entities include `conversations`, `messages`, `contacts`, `inboxes`, and various `channel_*` tables (Email, Facebook, Instagram, SMS, Telegram, WebWidget, WhatsApp).
  - **Key Learnings**: Chatwoot uses a robust polymorphic design for channels where an `inbox` links to a specific `channel_type` and `channel_id`. It relies heavily on WebSockets for real-time updates to the dashboard and widget.
  - **OHC Gap**: OHC currently lacks this native Rust backend infrastructure to support omnichannel messaging, multi-tenant isolation, and real-time AI agent integration.

  ## Design Doc

  ### Data Model & Invariants
  We will implement a native Rust microservice (or module within `onehumancorp/mono`) using PostgreSQL and Redis (for pub/sub and distributed locks).

  **Core Entities:**
  - `Account` (Tenant): The root of multi-tenancy. All tables must have an `account_id` with Row Level Security (RLS) enabled.
  - `Contact`: Represents a customer across channels. Fields: `id`, `account_id`, `name`, `email`, `phone_number`, `identifier`, `custom_attributes`.
  - `Channel`: Trait/Interface in Rust. Concrete implementations (tables): `channel_email`, `channel_sms`, `channel_whatsapp`, `channel_web_widget`, `channel_instagram`.
  - `Inbox`: Aggregates channels. Fields: `id`, `account_id`, `name`, `channel_type`, `channel_id`.
  - `Conversation`: A thread of messages. Fields: `id`, `account_id`, `inbox_id`, `contact_id`, `status` (open, resolved, snoozed), `assignee_id`.
  - `Message`: Individual messages within a conversation. Fields: `id`, `account_id`, `conversation_id`, `content`, `message_type` (incoming, outgoing, template), `sender_type` (contact, user, agent_bot), `sender_id`.

  ### Architecture Diagram (Mermaid)
  ```mermaid
  erDiagram
      ACCOUNT ||--o{ INBOX : owns
      ACCOUNT ||--o{ CONTACT : owns
      ACCOUNT ||--o{ CONVERSATION : owns
      INBOX ||--o{ CONVERSATION : contains
      INBOX ||--|| CHANNEL : has
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains

      INBOX {
          uuid id
          uuid account_id
          string name
          string channel_type
          uuid channel_id
      }
      CONVERSATION {
          uuid id
          uuid account_id
          uuid inbox_id
          uuid contact_id
          string status
      }
      MESSAGE {
          uuid id
          uuid account_id
          uuid conversation_id
          text content
          string message_type
          string sender_type
          uuid sender_id
      }
  ```

  ### Mobile UX Flow (375px First)
  1. **Unified Inbox List**: Owner opens OHC app on phone. Sees a prioritized list of conversations (Instagram DMs, SMS, Web Widget). Unread/urgent messages have a translucent red badge.
  2. **Conversation View**: Tapping a thread opens a native-feeling chat interface. Sticky header with customer name and context (e.g., "Maya - Custom Cake Inquiry").
  3. **AI Drafting**: A floating action button or inline suggestion shows "AI Draft Ready". Tapping it populates the input field with a generated reply based on past context and policies.
  4. **Quick Actions**: Horizontal scrollable chips above the keyboard for "Request Payment", "Send Booking Link", "Mark Resolved".

  ### AI Agent Integration
  - **Operations Agent**: Monitors incoming messages via a Redis Pub/Sub stream (`ohc:events:message_created`).
  - If a message asks about pricing or availability, the agent acquires a distributed lock (`ohc:lock:{account_id}:conversation:{conversation_id}`) to prevent human collision, drafts a reply, and saves it as a draft message.
  - The UI updates in real-time via WebSockets to show the owner the suggested draft.

  ## Implementation Prompt
  **Role**: Implementer Agent
  **Task**: Build the foundational Rust API and data layer for the native Omnichannel Chat System.
  **CUJ**: A small business owner (Carlos) receives a web widget message, sees it in his unified inbox, and replies.
  **Acceptance Criteria**:
  - Implement PostgreSQL schemas for `inboxes`, `channels` (Web Widget as MVP), `contacts`, `conversations`, and `messages`.
  - Ensure strict multi-tenancy using `account_id` and Row Level Security.
  - Build gRPC/REST endpoints to create a conversation and send a message.
  - Implement a WebSocket handler (using Axum or similar Rust framework) to broadcast `message.created` events to connected clients for a specific account.
  - Write 100% unit test coverage for the Rust data layer and API handlers.
  - Create Playwright E2E tests simulating Carlos receiving and replying to a message in the mobile UI.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
