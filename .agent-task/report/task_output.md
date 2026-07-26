issue_title: "Architect Native Rust Omnichannel Chat System to Replace External Chat Service"
issue_description: |
  ## Title: Architect Native Rust Omnichannel Chat System to Replace External Chat Service

  ## Problem Statement
  OHC is retiring the external legacy chat dependency to bring omnichannel customer support natively into the platform. Non-technical owner/operators (like Maya the baker and Carlos the handyman) need a unified inbox where they can seamlessly interact with customers across Instagram DMs, WhatsApp, SMS, and Web Chat, without dealing with third-party integrations, external service limits, or fragmented data silos. By building this natively in Rust, we ensure high performance, strict multi-tenant isolation, and deep AI integration across all work contexts, allowing the assistant to triage messages, draft replies, and automate workflows securely.

  ## Research Report
  - **Source Code Audit**: An audit of the prior external chat solution reveals a strong architecture based around `Accounts` (Tenants), `Inboxes`, `Channels` (Adapters for Web Widget, Email, API, Twitter, Facebook, etc.), `Conversations`, and `Messages`.
  - **Core Entities**:
    - `Conversation`: The central unit of engagement.
    - `Message`: Individual messages within a conversation.
    - `Contact`: The customer identity across channels.
    - `Inbox`: Aggregates messages from a specific channel for an account.
    - `Channel`: Specific channel configurations (e.g., WhatsApp, Email).
  - **Real-time Capabilities**: Relies on WebSockets for real-time dispatch of events to clients, agents, and external webhooks.
  - **AI & Automation**: Current external models lack deep integration with OHC’s native agents (like Operations or Finance). Native implementation will allow AI agents to securely draft proposals or trigger bookings based on chat context.

  ## Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      TENANT ||--o{ CONTACT : has
      TENANT ||--o{ CONVERSATION : has
      INBOX ||--o{ CHANNEL_ADAPTER : configured_with
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      CONVERSATION ||--o{ TASK : triggers

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
      CHANNEL_ADAPTER {
          uuid id PK
          uuid inbox_id FK
          jsonb credentials
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
      }
      MESSAGE {
          uuid id PK
          uuid conversation_id FK
          text content
          string sender_type
          timestamp created_at
      }
  ```

  ### Mobile UX Flow (375px First)
  - **Unified Inbox Screen**: A clean, single-column list of active conversations categorized by status (e.g., Unassigned, Mine, All).
  - **Conversation View**: Tapping a conversation opens a chat interface with the customer's history. A translucent glass header shows the customer's name and channel icon (e.g., Instagram, Web).
  - **AI Drafting**: A highly visible floating action button or inline suggestion area where the AI Assistant proposes drafted replies or next actions (e.g., "Create Quote", "Send Payment Link").
  - **Touch Targets**: All interactive elements (send button, attach file, AI suggestions) are at least 44x44px.

  ### AI Agent Integration Points
  - **Work Triage**: AI listens to incoming messages and routes or tags conversations based on urgency and topic.
  - **Customer & Relationship Assistant**: Automatically drafts replies based on historical context, tenant knowledge base, and tone.
  - **Operations Assistant**: Extracts dates and requests from the chat to seamlessly propose booking tasks or calendar events directly in the chat view.

  ### Key Design Decisions
  - **Native Rust Implementation**: Microservices for WebSocket real-time messaging, inbox management, and channel webhooks.
  - **Data Isolation**: Strict multi-tenant isolation via `tenant_id` on all tables, enforced at the database level with PostgreSQL RLS.
  - **API-First**: REST/gRPC endpoints for mobile and desktop clients to ensure a consistent experience across all devices.

  ## Implementation Prompt
  Implement the backend core data models and service layer for the native OHC omnichannel chat system. Your tasks include:
  1. Set up the PostgreSQL schema for `inboxes`, `channels`, `contacts`, `conversations`, and `messages`, ensuring Row-Level Security (RLS) is applied using `tenant_id`.
  2. Implement the gRPC/REST API endpoints to list, create, and update conversations and messages.
  3. Create a unified, 375px-first mobile Inbox UI in Tauri/Flutter that connects to these endpoints. Ensure it adheres to the premium OHC Translucent Glass visual design and UniFi layout patterns.
  4. Integrate the native AI Assistant to automatically suggest a drafted reply when a new message is loaded.

  Acceptance Criteria:
  - A real non-technical user (e.g., Maya) can open the mobile view, see a list of conversations, click into one, and send a message.
  - All data is isolated by `tenant_id`.
  - The UI is fully functional on a 375px wide screen with no horizontal scrolling.
  - The AI Assistant correctly suggests a reply based on the message content.
  - 100% unit test and E2E Playwright coverage for the complete flow.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
