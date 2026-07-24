issue_title: "Implement Custom Rust Omnichannel Chat System"
issue_description: |
  **Problem Statement**
  The system currently lacks a robust internal omnichannel chat capability, having retired third-party integrations like Chatwoot. Owners need a centralized inbox to manage customer interactions across multiple channels (web chat, WhatsApp, Email, etc.) smoothly from mobile and desktop.

  **Research Report**
  The OHC vision demands that AI seamlessly integrates into the owner's customer communications. We have audited the retired Chatwoot architecture (data models, channel adapters, webhooks, macros, and agent routing). To provide the required "Grandmother test" usability and robust multi-tenancy, OHC requires its own high-performance chat system natively written in Rust.

  **Design Doc**
  1.  **Architecture Diagram (Mermaid.js)**:
      ```mermaid
      graph TD
        Client[Frontend Flutter/Next.js] -->|WebSocket/gRPC| API[Rust API Server]
        Client -->|REST| API
        API -->|CRUD| DB[(PostgreSQL)]
        API -->|PubSub| Redis[(Redis)]
        External[WhatsApp/Email] -->|Webhooks| WebhookHandler[Rust Webhook Handler]
        WebhookHandler --> API
        API --> Agents[AI Agents Department]
        Agents --> API
      ```
      ```mermaid
      erDiagram
        INBOX ||--o{ CONVERSATION : contains
        CONVERSATION ||--o{ MESSAGE : contains
        INBOX {
          string id PK
          string tenant_id FK
          string name
        }
        CONVERSATION {
          string id PK
          string tenant_id FK
          string inbox_id FK
          string customer_id FK
          string channel
          string status
        }
        MESSAGE {
          string id PK
          string tenant_id FK
          string conversation_id FK
          string sender_type
          string content
          timestamp created_at
        }
      ```
  2.  **Data Models (PostgreSQL + sqlx)**:
      -   `inboxes` (tenant-scoped)
      -   `conversations` (links customer, inbox, and messages)
      -   `messages` (stores the actual chat payload, sender info)
      -   `channel_adapters` (configurations for connecting to external channels like WhatsApp, Email, or a local Web Widget)
      -   *Invariants*: Every query MUST enforce `tenant_id` row-level security.
  3.  **UI Wireframes & Mobile Flow (375px First)**:
      -   A clean, translucent glass "Inbox" view.
      -   A prioritized list of active conversations, clearly indicating the source channel (e.g., a small WhatsApp icon).
      -   Tapping a conversation opens a standard message thread.
      -   Clear AI-assisted drafting suggestions visible above the text input.
  4.  **AI Agent Integration**:
      -   When a new message arrives, the `Work Triage` agent evaluates priority.
      -   The `Customer Assistant` agent drafts suggested replies that the owner can approve or edit with one tap.

  **Implementation Prompt**
  Implement the core backend data models and standard CRUD APIs in Rust for the new Omnichannel Chat system. Ensure all database interactions respect multi-tenant row-level security. Create the corresponding foundational UI components in the frontend (Flutter/Next.js) for displaying an Inbox and a Conversation thread, optimized for a 375px mobile view.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
