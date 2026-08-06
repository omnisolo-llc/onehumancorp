issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  ## Problem Statement
  OneHumanCorp currently lacks a native Rust omnichannel customer support & chat engine to replace Chatwoot as an external dependency. This limits our ability to provide a deeply integrated, highly performant, and secure "unified inbox" for our users (like Maya, Carlos, Priya, Leo, and Fatima) that natively works with our existing agents.

  ## Research Report
  - Audited Chatwoot's Ruby on Rails source code (models: Inbox, Conversation, Message, Contact, Channel, User, AgentBot, Webhook, Attachment).
  - Investigated the data models and core components required for omnichannel communication:
    - **Inbox**: Configures channels (email, widget, API) and assignment rules.
    - **Conversation**: Links a Contact to an Inbox, tracking status, assignee, and SLA.
    - **Message**: Represents individual communications (text, attachments, template messages) within a Conversation.
    - **Contact**: Represents the external user communicating with the business.
  - OHC's native implementation in Rust should leverage PostgreSQL (with row-level security for multi-tenancy) and provide a performant API and WebSocket layer for real-time messaging, completely replacing any reliance on an external Chatwoot service.

  ## Design Doc
  ### Architecture
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : configures
      INBOX ||--o{ CHANNEL : has
      TENANT ||--o{ CONTACT : manages
      CONTACT ||--o{ CONVERSATION : initiates
      INBOX ||--o{ CONVERSATION : tracks
      CONVERSATION ||--o{ MESSAGE : contains
      USER ||--o{ CONVERSATION : assigned_to
      AGENT_BOT ||--o{ CONVERSATION : assigned_to

      TENANT {
          uuid id PK
          string name
      }
      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
          boolean auto_assignment
      }
      CHANNEL {
          uuid id PK
          uuid inbox_id FK
          string channel_type
          json config
      }
      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string name
          string email
          string phone
      }
      CONVERSATION {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          uuid assignee_id FK "nullable"
          string status
          datetime last_activity_at
      }
      MESSAGE {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          uuid sender_id FK
          string sender_type "user|contact|bot"
          text content
          string message_type "incoming|outgoing|template"
      }
  ```

  ### Mobile UX Flow (375px First)
  1.  **Unified Inbox View:** A clean list of active conversations across all channels. Unread messages are bolded.
  2.  **Conversation View:** Tapping a conversation opens the chat thread. The input area uses native mobile keyboards.
  3.  **Agent Integration:** A small "AI Draft" button allows the user to instantly generate a reply based on business context.

  ### AI Agent Integration Points
  -   **Draft Generation:** The `CustomerAssistant` agent can automatically draft replies based on previous context and business data.
  -   **Auto-Triage:** The `OperationsAssistant` can categorize incoming messages, extract intent, and assign them to the appropriate human or bot queue.

  ## Implementation Prompt
  Implement the core native Rust chat engine data models and initial CRUD APIs for `Inbox`, `Conversation`, `Message`, and `Contact`. Ensure strict multi-tenant isolation using the `tenant_id` pattern. Design the API to be consumed by our Tauri frontend and eventually support real-time WebSocket updates.

  **Acceptance Criteria:**
  - `Inbox`, `Conversation`, `Message`, and `Contact` entities are defined in Rust (e.g., using `sea-orm` or `sqlx` depending on repo norms).
  - PostgreSQL migrations are created.
  - CRUD API endpoints are implemented for these entities.
  - 100% unit test coverage for the new models and endpoints.
  - E2E Playwright test simulating a simple message flow in the UI.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
