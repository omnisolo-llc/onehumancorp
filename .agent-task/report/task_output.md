issue_title: "Architecture Design: Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  ### Problem Statement
  Currently, OneHumanCorp (OHC) relies on external dependencies like Chatwoot for omnichannel customer messaging. For our core personas (Maya, Carlos, Priya, Leo, Fatima) who run their entire business from a single assistant, relying on an external service creates latency, breaks multi-tenant data isolation guarantees, and prevents deep, seamless AI integration. An external chat tool feels like a disconnected piece of software rather than an integrated assistant.

  ### Research Report
  - **Market Context**: Platforms like Shopify (Shopify Inbox) and Wix have successfully integrated native chat tools directly linked to their commerce graph.
  - **Chatwoot Source Code Audit**: An audit of Chatwoot's architecture reveals a robust omnichannel model involving `Inboxes`, `Conversations`, `Messages`, `Contacts`, and `ChannelAdapters` (Web Widget, API, Email, SMS, WhatsApp).
    - `Inboxes` belong to `Accounts` (`tenant_id` equivalent in OHC) and handle configurations like `csat_config` and `greeting_enabled`.
    - `Conversations` belong to `Inboxes`, handle `status`, tracking (`agent_last_seen_at`), and are linked to `Contacts`.
    - `Messages` belong to `Conversations`, with properties for `content`, `content_type`, `message_type`, and `sentiment`.
    - Real-time delivery is powered by WebSockets, and structured controllers handle agent assignment.
  - **Platform Need**: We must retire Chatwoot completely and implement a native Rust omnichannel chat system within `onehumancorp/mono`. This system must handle real-time WebSockets, multi-tenant Postgres rows, and seamless interaction with the AI Customer & Operations Assistants.

  ### Design Doc
  **Architecture Overview**
  The native chat system will be built as a high-performance Rust service connecting via gRPC to the core Go backend.

  **Data Model & Invariants (ER Diagram)**
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CONVERSATION : contains
      CONVERSATION ||--o{ MESSAGE : contains
      CONTACT ||--o{ CONVERSATION : initiates
      CHANNEL_ADAPTER ||--o{ INBOX : routes_to

      TENANT {
          uuid id
          string name
      }
      INBOX {
          uuid id
          uuid tenant_id
          string name
          jsonb config
      }
      CONVERSATION {
          uuid id
          uuid inbox_id
          uuid contact_id
          uuid tenant_id
          string status
      }
      MESSAGE {
          uuid id
          uuid conversation_id
          uuid sender_id
          uuid tenant_id
          text content
          string content_type
      }
  ```
  *Multi-Tenant Isolation*: Strict row-level security (RLS) is enforced on all tables by `tenant_id`.

  **AI Department Coordination**
  The AI Customer Assistant intercepts incoming messages via PostgreSQL `SKIP LOCKED` queues. It maintains context, drafts replies securely (using the Redlock cross-agent lock `ohc:lock:{tenant_id}:conversation:{conversation_id}`), and executes actions upon owner approval.

  **Mobile UX Flow (375px First)**
  - **Layout**: Unified inbox list occupying the full 375px width. Touch targets for conversations are 60px high.
  - **Interaction**: Swiping right on a conversation assigns it to an AI agent. Tapping opens the chat thread. The AI drafted response appears in a translucent glass module above the native mobile keyboard.
  - **Visuals**: Adheres to the OHC Premium Token library (Apple/Ubiquiti-style), with clear status tokens (e.g., Unread, AI Draft Ready).

  ### Implementation Prompt
  **Goal**: Implement the native Rust omnichannel chat system to fully replace Chatwoot.
  **Task**:
  1. Build the Rust data models (`Inbox`, `Conversation`, `Message`, `Contact`, `ChannelAdapter`) with strict `tenant_id` invariants.
  2. Implement the WebSocket server in Rust for real-time message delivery.
  3. Create the multi-tenant PostgreSQL schema with Row-Level Security (RLS).
  4. Develop the Flutter UI (starting at 375px) for the Work Triage unified inbox, featuring translucent glass styling and AI draft capabilities.
  **Acceptance Criteria**: The owner can receive a message from a customer, view the AI-drafted reply, approve it, and send it back seamlessly—all within the native OHC app without external Chatwoot calls. UI must render flawlessly on a 375px mobile screen.

  ### Priority
  P0

  ### Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
