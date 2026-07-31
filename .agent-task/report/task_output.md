issue_title: "Architecture & Implementation Plan: Native Rust Omnichannel Chat System"
issue_description: |
  # Native Rust Omnichannel Chat System for OHC

  **Problem Statement:**
  OHC relies on a legacy, unmaintained external dependency (Chatwoot) for omnichannel communication, which creates scaling bottlenecks, operational complexity, and disjointed user experiences. We need to replace it with a high-performance, natively integrated Rust-based chat system that strictly enforces multi-tenant isolation, supports real-time WebSocket communication, integrates seamlessly with OHC's AI triage, and provides a beautiful mobile-first UI for small-business owners.

  **Research & Discovery:**
  - Audited Chatwoot's source code, specifically `app/models/conversation.rb`, `app/models/message.rb`, `app/models/channel/web_widget.rb`, and `app/models/contact.rb`.
  - Identified core data models needed: Inbox, Channel, Contact, Conversation, Message.
  - Verified current OHC scaffolding in `src/server/services/chat` and `src/server/integrations/chat`.

  **Design Doc:**

  *Architecture Overview:*
  - **Models**: `ChatInbox`, `ChatChannel`, `ChatContact`, `ChatConversation`, `ChatMessage`.
  - **Multi-Tenancy**: All tables strictly scoped by `tenant_id` for RLS.
  - **API Layer**: Build native Rust APIs (gRPC/REST) for managing conversations and messages.
  - **Real-Time**: Implement WebSocket handlers for the web widget and agent dashboard to stream new messages instantly.

  *ER Diagram:*
  ```mermaid
  erDiagram
      Tenant ||--o{ ChatInbox : has
      Tenant ||--o{ ChatChannel : has
      Tenant ||--o{ ChatContact : has
      Tenant ||--o{ ChatConversation : has
      Tenant ||--o{ ChatMessage : has
      ChatInbox ||--o{ ChatChannel : contains
      ChatInbox ||--o{ ChatConversation : receives
      ChatContact ||--o{ ChatConversation : initiates
      ChatConversation ||--o{ ChatMessage : holds
      ChatContact {
          uuid id PK
          uuid tenant_id FK
          string name
          string email
          string phone
      }
      ChatInbox {
          uuid id PK
          uuid tenant_id FK
          string name
      }
      ChatChannel {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          string channel_type
          jsonb config
      }
      ChatConversation {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          uuid assignee_id
          string status
      }
      ChatMessage {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          string sender_type
          uuid sender_id
          string content
      }
  ```

  *Mobile UX Flow:*
  1. Owner opens the OHC mobile app.
  2. "Inbox" tab displays aggregated messages from all channels (Web, WhatsApp, etc.).
  3. Tapping a thread opens the conversation view with seamless scrolling and inline AI-generated reply suggestions.
  4. Messages can be resolved, assigned, or snoozed.

  *AI Integration:*
  - Incoming messages trigger a background job to route through the OHC AI Triage department.
  - AI drafts are appended as private notes or inline suggestions before the owner sends them.

  **Implementation Prompt:**
  As an implementer agent, build the complete database schema, Rust service layer (`src/server/services/chat`), gRPC/REST endpoints, and the corresponding frontend UI components in Flutter to support the core inbox flow.
  - Ensure 100% unit test coverage for the Rust service layer.
  - Implement a mobile-responsive "Inbox" screen in the Flutter frontend with working E2E Playwright tests.
  - Do not use any external chat services.

  **Priority**: P0
  **Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
