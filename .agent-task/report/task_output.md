issue_title: "[Platform Architecture] Native Rust Omnichannel Unified Inbox System"
issue_description: |
  # Native Rust Omnichannel Unified Inbox System (Chatwoot Replacement)

  ## Problem Statement
  Currently, OneHumanCorp (OHC) relies on Chatwoot for omnichannel customer support and messaging. However, treating Chatwoot as an external dependency creates several friction points for our core owner/operator personas (e.g., Maya, Carlos, Priya):
  - **Latency and Synchronization**: Synced external state leads to message delay or dropped context during critical sales and support conversations.
  - **Complex Multi-Tenancy**: Maintaining OHC's strict Row-Level Security (RLS) multi-tenant data boundaries requires complex integration overhead with an external third-party service.
  - **Mobile Inconsistency**: Bringing a third-party chat widget and inbox into the native OHC Flutter application results in an inconsistent UI/UX, failing the premium Translucent Glass design standard and 375px mobile-first promise.
  - **AI Agent Integration Limitation**: Deep AI capabilities (e.g., automated quote drafting for Carlos, order parsing for Fatima) are constrained by an external system's webhook and API rate limits.

  ## Research Report & Findings
  An audit of the Chatwoot source code (`app/models/conversation.rb`, `app/models/message.rb`, `app/models/inbox.rb`) revealed that a unified inbox centers on a few core data models:
  - **Account/Tenant**: The scope for all data.
  - **Inbox & Channel**: Represents where messages come from (Web Widget, Email, WhatsApp, Instagram).
  - **Conversation**: The thread containing messages between a contact and the owner/assignee.
  - **Message**: The individual payload (text, image, interactive templates).

  Leading platforms (Shopify Sidekick, Wix) deeply embed messaging natively to trigger backend workflows directly. A native Rust messaging engine inside `onehumancorp/mono` can offer zero-latency WebSocket updates, leverage our existing PostgreSQL RLS, seamlessly weave into the OHC Flutter mobile app, and enable AI agents (Sales, Operations, Customer Service) to act instantly on incoming messages without network hops.

  ## Design Doc
  ### High-Level Architecture (Mermaid.js)
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
      }
      CONVERSATION {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status
      }
      MESSAGE {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          string content_type
          text content
      }
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONVERSATION : owns
      INBOX ||--o{ CONVERSATION : receives
      CONVERSATION ||--o{ MESSAGE : contains
  ```

  ### Architecture Details
  - **Backend (Rust)**: Use `tokio` and `axum` to handle high-concurrency WebSocket connections and REST API endpoints.
  - **Data Storage**: PostgreSQL with strict Row Level Security (`tenant_id`) for persistence. Redis for real-time pub/sub across load-balanced application instances.
  - **Channel Adapters**: Native Rust modules to interface with Email (IMAP/SMTP), WhatsApp Cloud API, Instagram Graph API, and OHC's own web chat widget.

  ### Mobile UX Flow (375px First)
  - **Inbox Command Center**: The owner opens the app. A unified "Feed" tab aggregates all incoming messages, tasks, and alerts into one scrollable, prioritized list.
  - **Conversation View**: Tapping a message opens a clean, macOS-style translucent chat interface. Touch targets are large (44x44px). The keyboard pushes the chat up smoothly.
  - **Agent Interaction**: AI-drafted replies appear as "ghost text" or pre-filled input box suggestions, allowing Maya or Carlos to tap "Approve & Send" with one thumb.

  ### AI Agent Integration Points
  - **Work Triage AI**: Monitors the Redis pub/sub feed for new messages, parses intent, and categorizes the conversation (e.g., "Lead", "Support", "Urgent").
  - **Customer & Relationship AI**: Automatically drafts replies based on tenant context (inventory, calendar) and inserts them as pending messages.

  ## Implementation Prompt
  **Goal:** Implement the foundational Native Rust Omnichannel Chat API and Database Schema.
  **CUJ:** As an OHC system component, I need to create a Tenant Inbox, initiate a Conversation, and send a Message via a REST API, observing that a WebSocket event is broadcast to the connected client.
  **Acceptance Criteria:**
  1. Define Rust data models for `Inbox`, `Conversation`, and `Message` adhering to OHC's PostgreSQL RLS multi-tenancy.
  2. Create REST endpoints (`POST /api/v1/inboxes`, `POST /api/v1/conversations`, `POST /api/v1/conversations/:id/messages`).
  3. Implement a WebSocket route (`/api/v1/ws`) that broadcasts new messages to the relevant connected client, using Redis pub/sub for scaling.
  4. Ensure 100% test coverage for all new models and endpoints.
  5. *Note: Do not implement the external channel adapters (WhatsApp, IG) yet; focus on the internal API core.*

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
