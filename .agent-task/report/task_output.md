issue_title: "Retire Legacy Omnichannel Inbox: Architect Native Rust Inbox & Agent Routing"
issue_description: |
  ## Problem Statement
  OHC currently lacks a unified, native messaging layer to handle the diverse communications of its personas (Maya's Instagram DMs, Carlos's text messages, Priya's website chats). The previous external messaging platform is being entirely retired as an external service due to complexity, lack of proper OHC multi-tenant integration, and the need for agents to seamlessly coordinate and act on conversations. We need a native Rust omnichannel inbox embedded directly into OHC to capture, triage, and route all customer interactions without relying on external third-party software.

  ## Research Report
  - **Context:** The prompt strictly mandates the complete retirement of the legacy messaging platform as an external service/dependency. OHC must implement its own high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust inside `ohc-mono`.
  - **Source Code Audit Findings (`https://github.com/chatw-o-o-t/chatw-o-o-t`):**
    - **Data Model:** Core entities include Accounts (Tenants), Inboxes, Channels (WebWidget, SMS, Email, API, WhatsApp, etc.), Conversations, Messages, Contacts, and Users.
    - **Messaging/Real-time:** The previous platform uses ActionCable (WebSockets) for real-time messaging updates to the agent dashboard and web widgets.
    - **Routing/Automation:** Rule-based routing (Macros, Canned Responses) and AI integration points exist.
  - **Platform Gap Analysis:** OHC currently lacks these native tables and services. To serve our personas effectively, we need to ingest messages (e.g., via Twilio/Meta webhooks), store them securely with tenant isolation, and broadcast updates to the mobile-first OHC frontend where both human owners and AI agents can respond.

  ## Design Doc

  ### Architecture
  We will introduce a new `omnichannel` domain within `src/server/ohc/domain/`.

  **Data Model & Invariants:**
  - `Inboxes`: Logical grouping of channels (e.g., "Sales", "Support"). Multi-tenant isolated (`tenant_id`).
  - `Channels`: Configurations for specific providers (Twilio SMS, Web Widget, WhatsApp). Polymorphic design.
  - `Contacts`: Customers interacting with the channels.
  - `Conversations`: A threaded discussion between a Contact and the Inbox (Owner/Agent).
  - `Messages`: Immutable records of communication (Text, Attachments) within a Conversation.

  **AI Department Coordination:**
  - **Work Triage:** An AI agent listens to new `Conversations` and categorizes them, creating an initial summary or draft response.
  - **Customer Assistant:** Drafts replies based on tenant context.

  **Mobile-First UX Flow (375px first):**
  - **Unified Inbox View:** A clean, Unifi-style list view of active conversations. Each row shows contact avatar, last message snippet, time, and channel icon.
  - **Conversation Thread:** Tapping a conversation opens a standard chat UI. Input field at the bottom, auto-expanding. "AI Draft" translucent glass overlay appears if an agent has prepared a response.
  - **Real-time:** WebSockets ensure instantaneous updates without pull-to-refresh.

  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL : configures
      INBOX ||--o{ CONVERSATION : tracks
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains
      TENANT {
          uuid id
      }
      INBOX {
          uuid id
          uuid tenant_id
          string name
      }
      CHANNEL {
          uuid id
          uuid inbox_id
          string provider_type
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
          string sender_type
      }
  ```

  ### Implementation Prompt
  **Goal:** Implement the foundation of the native Rust Omnichannel Inbox to replace the legacy system.
  **CUJ:**
  1. Owner configures a new "Web Widget" channel for their inbox via the API.
  2. A simulated customer sends a message to the Web Widget API.
  3. The system creates a Contact, Conversation, and Message.
  4. The Owner fetches the active conversations and sees the new message.

  **Tasks for Implementer:**
  - Create the protobuf definitions for `Inbox`, `Channel`, `Conversation`, `Message`, and `Contact`.
  - Implement the gRPC API endpoints for listing conversations and sending messages.
  - Create the underlying PostgreSQL schema with strict `tenant_id` row-level security.
  - Implement a basic service layer in `src/server/ohc/domain/omnichannel` to handle message creation and conversation tracking.
  - Write at least one E2E test verifying the creation of a conversation and message via the API.

  **Acceptance Criteria:**
  - Protobufs and API layer exist.
  - DB schema is created with tenant isolation.
  - API allows sending a message (creating a conversation if none exists) and retrieving conversations.
  - All unit and E2E tests pass (`bazel test //...`).

  **Estimated Scope:** Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture]
assignees: []
