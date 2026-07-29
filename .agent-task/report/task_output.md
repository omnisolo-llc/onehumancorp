issue_title: "[Architecture] Implement Native Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) is retiring the external Chatwoot dependency. To fulfill our promise of being a Tencent Workbuddy-like work assistant that simplifies the daily lives of small business owners (Maya, Carlos, Priya, Leo, Fatima, Nora, Jun), we must implement a native, high-performance, multi-tenant omnichannel chat and customer support engine natively in Rust. This will provide seamless unification of customer messages (Instagram, WhatsApp, SMS, Web Widget) and operations into a single inbox without relying on third-party integrations that slow down performance or leak data context outside of OHC.

  ## Research Report
  Based on an audit of the `chatwoot` source code (specifically looking at `Conversation`, `Message`, `Inbox`, `Contact`, and `Channel::WebWidget` models), a successful Omnichannel Chat System requires:
  1. **Conversations:** Core entity tracking `status` (open, resolved, snoozed), `assignee`, `contact`, and `inbox`.
  2. **Messages:** Linked to a conversation, tracking `content_type`, `message_type` (incoming, outgoing), `private` notes, and `sender` polymorphic association.
  3. **Inboxes & Channels:** Represents the entry point for messages (e.g., WebWidget, API, Email). Needs to be strictly scoped by `account_id` (Tenant ID in OHC).
  4. **Contacts:** The customer entity interacting with the business.

  Unlike Chatwoot's Ruby on Rails architecture, OHC requires a high-performance native Rust implementation leveraging PostgreSQL row-level security for multi-tenancy, gRPC for internal APIs, and WebSocket for real-time delivery.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT {
          uuid id PK
          string name
      }
      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
          string channel_type
          jsonb config
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
          string status
          uuid assignee_id FK
      }
      MESSAGE {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          string content
          string message_type
          string sender_type
          uuid sender_id
      }

      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      TENANT ||--o{ CONVERSATION : owns
      TENANT ||--o{ MESSAGE : owns
      INBOX ||--o{ CONVERSATION : receives
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
  ```

  ### Mobile UX Flow (375px first)
  1. **Home/Triage Feed:** The user (e.g., Maya) opens the OHC app and sees a unified list of active `Conversations` that need attention (unread messages, unassigned).
  2. **Conversation View:** Tapping a conversation opens a clean, macOS Translucent Glass-styled chat interface.
  3. **Context Panel:** A drawer or collapsible top sheet reveals the `Contact` details (past orders, notes, tags) pulled from the central OHC system.
  4. **Action Area:** The user can type a reply, or tap "AI Draft" to have the AI assistant propose a response based on the context.

  ### AI Agent Integration Points
  - **Auto-Triage:** When a new `Message` arrives, an AI agent evaluates the intent, assigns a priority, and can auto-route it to the correct inbox or assignee.
  - **Draft Generation:** AI agents can propose replies (Drafts) by reading the `Conversation` history and `Contact` context.
  - **Action Extraction:** The AI can detect actionable intents in messages (e.g., "I want to book an appointment") and propose creating OHC internal tasks or bookings directly from the chat.

  ### Key Design Decisions
  - **Strict Multi-Tenancy:** Every table (`inboxes`, `contacts`, `conversations`, `messages`) MUST have a `tenant_id` column and enforce row-level security (RLS) in PostgreSQL to guarantee data isolation.
  - **Native Rust & gRPC:** Implement the backend logic as a new Rust module (`chat` or `omnichannel`) within the `src/server` tree, exposing gRPC services for the frontend and other backend components.
  - **WebSocket Real-time:** Real-time message delivery to the mobile/web clients must be handled via a scalable WebSocket layer integrated with the existing Rust server architecture.

  ## Implementation Prompt
  Implement the backend core of the native Rust Omnichannel Chat System to replace Chatwoot.
  1. Define the PostgreSQL database schema migrations for `inboxes`, `contacts`, `conversations`, and `messages`, ensuring strict multi-tenant RLS using `tenant_id`.
  2. Implement the Rust data models, repository layer, and gRPC service definitions (protobuf) for creating and managing conversations and messages.
  3. Ensure the implementation integrates with the existing OHC authentication and multi-tenancy middleware.
  4. Provide unit tests with 100% coverage and at least one E2E Playwright test (with a corresponding simple UI or API fixture if UI is not yet built) to verify a message can be created and retrieved within a tenant's context.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
