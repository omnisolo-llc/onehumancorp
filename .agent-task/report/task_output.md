issue_title: "Implement Custom Rust Omnichannel Chat System"
issue_description: |
  # Research Report: Custom Rust Omnichannel Chat System

  ## Problem Statement
  Currently, OHC relies on Chatwoot as an external third-party service for omnichannel customer support and chat functionality. However, based on the engineering standards, Chatwoot must be retired and replaced with a native Rust implementation to ensure performance, multi-tenant isolation, and alignment with OHC's architectural vision.

  ## Research Findings
  After auditing the Chatwoot source code (`https://github.com/chatwoot/chatwoot`), the following core data models and relationships were identified as critical for a minimum viable native Rust chat system:

  1.  **Account/Tenant**: The root of multi-tenancy.
  2.  **Inbox**: A channel instance (e.g., a specific email address, a web widget, a WhatsApp number) connected to an Account.
  3.  **Conversation**: A thread of messages between a Contact and Agents within an Inbox.
  4.  **Message**: The actual content sent in a Conversation.
  5.  **Contact**: The end-user communicating with the business.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      TENANT ||--o{ CONTACT : has
      TENANT ||--o{ CONVERSATION : has
      TENANT ||--o{ MESSAGE : has
      INBOX ||--o{ CONVERSATION : receives
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
  ```

  ### Key Design Decisions
  - **Language**: Rust, integrated into `onehumancorp/mono`.
  - **Multi-Tenancy**: Strict row-level isolation using `tenant_id` on all tables (`inboxes`, `conversations`, `messages`, `contacts`).
  - **API**: Provide basic gRPC/REST endpoints to create and list inboxes, conversations, and messages.
  - **Real-time (Future)**: The architecture should support future WebSocket integration for real-time messaging, but the MVP will focus on the data model and core API.

  ### AI Agent Integration
  The system must expose hooks or interfaces where AI agents (like the Customer & Relationship Assistant) can read new messages, understand conversation context, and draft replies on behalf of the owner.

  ### Mobile UX flow
  1. Owner opens OHC app on phone.
  2. Owner taps the "Inbox" icon in bottom navigation bar.
  3. Owner sees a list of active conversations across all channels (Instagram, WhatsApp, Email, Web Widget).
  4. Owner taps on a conversation to view the full message thread.
  5. Owner can quickly type a reply or use an AI-generated draft to respond.
  6. Owner can mark the conversation as "Resolved" or assign it to another team member.

  ### UI wireframes (375px)
  - Inbox List Screen:
    - Top bar: Search and Filter by channel/status.
    - List view: Conversation cards with Contact name, snippet of last message, channel icon, and timestamp.
    - Bottom bar: Standard navigation (Home, Inbox, Offers, More).
  - Conversation Thread Screen:
    - Top bar: Contact name and Back button.
    - Scrollable area: Message bubbles (left for Contact, right for Owner).
    - Bottom area: Message input field with attachment icon and "Send" button. AI draft suggestion button appears above the input field if available.

  ## Implementation Prompt
  Implement the foundational data model and API for the native Rust omnichannel chat system to replace Chatwoot.

  1.  **Database Schema**: Design and implement the PostgreSQL schema for `inboxes`, `conversations`, `messages`, and `contacts`. Ensure strict multi-tenant isolation (`tenant_id`).
  2.  **Rust Data Models**: Create the corresponding Rust structs/entities in the backend module.
  3.  **API Endpoints**: Implement the core API endpoints (e.g., via gRPC/REST) to:
      - Create and list Inboxes for a Tenant.
      - Create and list Conversations within an Inbox.
      - Send and retrieve Messages in a Conversation.
  4.  **Testing**: Write comprehensive unit tests for the data models and API handlers, ensuring 100% coverage.

  The user-facing outcome is that OHC can natively manage chat data without relying on external Chatwoot services, laying the groundwork for the unified owner inbox UI.

  ## Priority & Scope
  - **Priority**: P0
  - **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
