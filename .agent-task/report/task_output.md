issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) is replacing its reliance on Chatwoot as an external dependency with a native, high-performance omnichannel chat system built in Rust. Chatwoot has been fully retired from the architecture to reduce complexity, eliminate third-party constraints, and seamlessly integrate messaging deeply into OHC's multi-tenant structure and agent capabilities. The new system needs to support real-time interactions, seamlessly integrate with various messaging channels, and offer full feature parity while maintaining OHC's design standards.

  ## Research Report
  Based on an audit of the `chatwoot` source repository (`https://github.com/chatwoot/chatwoot`), the core architecture requires handling of multiple channels (e.g., email, SMS, Instagram, WhatsApp, web widgets). The core models driving this include Accounts, Inboxes, Conversations, Contacts, and Messages. Chatwoot relies on Ruby on Rails with PostgreSQL and Sidekiq for background jobs.

  Our system will replace this with a native Rust implementation embedded within the `onehumancorp/mono` architecture, specifically in the `src/server/ohc/` module. The architecture will follow OHC's `tenant_id` row-level security for PostgreSQL and employ real-time WebSocket communication, a robust data schema for omnichannel support, and seamless coordination with the OHC AI agents.

  **Key Requirements derived from Chatwoot:**
  - **Inboxes**: Represent channels (e.g., Web Widget, WhatsApp, Email).
  - **Conversations**: Represent threads of messages linked to a Contact and Inbox.
  - **Messages**: The actual message payload, handling various content types (text, attachments).
  - **Contacts**: The customer entity engaging via the channel.
  - **Real-time updates**: WebSocket-driven frontend notifications for new messages, typing indicators, and status changes.

  ## Design Doc
  **Architecture Overview:**
  - **Backend (Rust)**:
    - Located in `src/server/ohc/chat`.
    - Exposes a gRPC service defined via `src/proto/chat.proto` (and augmenting `inbox.proto`).
    - Connects to PostgreSQL with strict `tenant_id` isolation.
    - Uses Redis for Pub/Sub to broadcast real-time events across instances to WebSockets.
  - **Data Model (Entity-Relationship)**:
    ```mermaid
    erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CONVERSATION : contains
      TENANT ||--o{ CONTACT : manages
      CONTACT ||--o{ CONVERSATION : participates_in
      CONVERSATION ||--o{ MESSAGE : contains

      INBOX {
        uuid id PK
        uuid tenant_id FK
        string name
        string channel_type
        jsonb config
      }
      CONVERSATION {
        uuid id PK
        uuid tenant_id FK
        uuid inbox_id FK
        uuid contact_id FK
        string status
        datetime last_activity_at
      }
      MESSAGE {
        uuid id PK
        uuid tenant_id FK
        uuid conversation_id FK
        string content
        string message_type
        uuid sender_id
        datetime created_at
      }
      CONTACT {
        uuid id PK
        uuid tenant_id FK
        string name
        string email
        string phone_number
      }
    ```
  - **AI Agent Integration**:
    - **Customer Assistant Agent**: Automatically drafts replies to incoming messages by observing the `MessageCreated` event via the Redis queue.
    - **Operations Assistant Agent**: Triggers actions (e.g., booking a service or checking inventory) based on conversation context.

  **Mobile UX Flow (375px First):**
  - **Inbox View**: A clean, unified list of conversations across all channels. High-contrast unread indicators. Follows macOS translucent glass design.
  - **Conversation View**: Native-feeling chat interface. Large tap targets for sending messages, attachments, and quick AI draft approval.
  - **Contact Pane**: A swipeable right pane revealing customer context (past orders, preferences) seamlessly populated by the AI.

  ## Implementation Prompt
  Implement the native Rust omnichannel chat system to replace Chatwoot.

  **Backend Tasks:**
  - Create the protobuf definitions for the chat system (`ChatService`, `Inbox`, `Conversation`, `Message`, `Contact`) in `src/proto/chat.proto` and integrate them into `src/server/ohc/chat`.
  - Implement the Rust gRPC server handling CRUD operations for Inboxes, Conversations, Messages, and Contacts. Ensure strict `tenant_id` row-level security on all database interactions.
  - Implement the WebSocket handler to broadcast new messages and conversation status changes to the frontend.
  - Set up the background job queue (using PostgreSQL `SKIP LOCKED` or Redis) to trigger AI agents when new messages arrive.

  **Frontend Tasks:**
  - Build the mobile-first (375px base) unified Inbox UI using the OHC Premium Token library.
  - Create the Conversation detail view with real-time WebSocket connectivity for seamless messaging.
  - Ensure the "AI draft reply" feature is easily accessible and requires single-tap owner approval.

  **Acceptance Criteria:**
  - E2E Playwright tests must verify that an owner can receive a message from a simulated customer, view it in the unified inbox, and reply using the UI, with updates reflected in real-time.
  - 100% Unit test coverage on the Rust chat module.
  - The UI must render correctly on a 375px viewport with no horizontal scrolling and tap targets >= 44x44px.
  - The system must function independently of any Chatwoot service.

  Estimated Scope: Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []