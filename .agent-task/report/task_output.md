issue_title: "Native Rust Omnichannel Inbox & Chat Engine (Chatwoot Replacement)"
issue_description: |
  ## Mission Queue Protocol Brief

  **Title**: Implement Native Rust Omnichannel Inbox & Chat Engine

  **Problem Statement**:
  Currently, OneHumanCorp (OHC) is retiring its dependency on the external third-party Chatwoot service to reduce external coupling and improve native multi-tenant performance. Our non-technical owner/operator personas, such as Maya (baker using Instagram DMs) and Carlos (handyman using SMS/WhatsApp), require a unified, lightning-fast inbox to view and respond to all customer inquiries. Without a native omnichannel engine, OHC cannot seamlessly integrate AI-assisted drafts and background operations with real-time customer messaging. We need to build a native Rust replacement that provides 100% feature parity with Chatwoot's core messaging architecture.

  **Research Report**:
  An extensive audit of the Chatwoot source code (https://github.com/chatwoot/chatwoot) was conducted.
  - Chatwoot’s core data model revolves around `Accounts` (Tenants), `Inboxes`, `Channels` (Web Widget, API, Email, Facebook Page, Twitter, Twilio SMS, WhatsApp, Line, Telegram), `Conversations`, `Messages`, and `Contacts`.
  - Real-time messaging is handled via WebSockets (ActionCable in Rails) broadcasting event payloads.
  - OHC will replicate this model natively in Rust, leveraging our existing PostgreSQL infrastructure with strict multi-tenant Row-Level Security (RLS) and Redis for pub/sub WebSocket coordination.
  - Industry competitors like Shopify Inbox and Wix Inbox heavily rely on edge-cached static assets and efficient WebSocket connections, which our Rust implementation will easily exceed in performance.

  **Design Doc**:

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL_ADAPTER : configured_with
      INBOX ||--o{ CONVERSATION : contains
      CONVERSATION ||--o{ MESSAGE : contains
      CONTACT ||--o{ CONVERSATION : initiates
      MESSAGE }o--|| CONTACT : sent_by

      TENANT {
          uuid id
          string name
      }
      INBOX {
          uuid id
          uuid tenant_id
          string name
      }
      CHANNEL_ADAPTER {
          uuid id
          string channel_type
          json credentials
      }
      CONVERSATION {
          uuid id
          uuid inbox_id
          uuid contact_id
          string status
      }
      MESSAGE {
          uuid id
          uuid conversation_id
          string content
          string message_type
      }
      CONTACT {
          uuid id
          uuid tenant_id
          string name
          string email
          string phone_number
      }
  ```

  ### Mobile UX Flow (375px first)
  1. **Unified Inbox Screen**: The app opens to a clean list of active conversations. Each row shows the customer name, avatar, snippet of the last message, channel icon (e.g., Instagram, SMS), and time elapsed.
  2. **Conversation Thread**: Tapping a row opens the conversation. 375px layout focuses entirely on the chat bubbles. A sticky bottom input bar allows native keyboard text entry, with an "AI Draft" floating action button directly above the input.
  3. **Context Drawer**: Swiping left from the right edge reveals a hidden context pane showing the customer's purchase history, active bookings, and AI-generated notes.

  ### AI Agent Integration Points
  - **Customer & Relationship Assistant**: Listens to the Redis pub/sub feed for `message.created` events. If a message is from a customer, the AI automatically drafts a contextual reply, marking the draft internally so it appears as a suggested response in the owner's UI.
  - **Work Triage**: Analyzes incoming messages to detect intent (e.g., "I need a quote"). It can automatically convert a conversation into a Task or Quote Draft in the background.

  **Implementation Prompt**:
  To the Implementer Agent:
  Your task is to implement the core backend Rust services and database schema for the Native Omnichannel Inbox, completely replacing Chatwoot.
  1. Define the SQL migrations for `inboxes`, `channel_adapters`, `contacts`, `conversations`, and `messages`, ensuring Row-Level Security (`tenant_id`) is strictly enforced on every table.
  2. Implement the Rust gRPC/REST APIs for listing conversations, fetching messages, and creating new messages.
  3. Build a WebSocket server handler in Rust using Redis Pub/Sub to broadcast real-time message events to connected clients.
  4. Ensure zero external dependencies on Chatwoot.
  5. The UI must present a 375px-optimized unified inbox view adopting macOS Translucent Glass styling, where the owner can view and reply to a message.
  6. All Critical User Journeys (CUJ) must be covered by Playwright E2E tests simulating a real business owner receiving and replying to a customer inquiry.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture]
assignees: []
