issue_title: "Native Rust Omnichannel Chat System Migration"
issue_description: |
  **Title**: Architect and Implement Native Rust Omnichannel Chat System to Replace Chatwoot

  **Problem Statement**:
  Currently, OneHumanCorp (OHC) utilizes Chatwoot as an external third-party service for its omnichannel customer support and chat engine. Relying on an external service creates friction for non-technical owner/operators (such as Maya the baker or Carlos the handyman), adds points of failure, complicates multi-tenant isolation, and prevents tight integration with our native AI agent assistance (like auto-replying to Instagram DMs). OHC mandates 100% retirement of Chatwoot as an external service in favor of a native, high-performance Rust implementation built into the `onehumancorp/mono` repository, maintaining feature parity.

  **Research Report**:
  We audited the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) to benchmark features and understand its architecture. Key observations:
  - **Data Models**: Chatwoot revolves around `Accounts`, `Inboxes`, `Conversations`, `Messages`, `Contacts`, and `Channel Web Widgets`.
  - **Channels**: It supports multiple channel adapters (Web, API, Social Media).
  - **Real-time Messaging**: It uses WebSockets heavily for instantaneous message updates to agents.
  - **Platform Needs**: OHC requires row-level multi-tenant isolation (using `tenant_id`), integration with gRPC (Tonic), and the ability for our AI agents to tap directly into the chat stream invisibly to provide context and auto-reply capabilities.

  **Design Doc**:
  *Architecture Overview:*
  - A native Rust microservice will handle chat interactions using gRPC/Tonic for internal communication and Axum for REST endpoints if needed, but primarily relying on our existing API architecture.
  - PostgreSQL will store the schema: `Conversations`, `Messages`, `Contacts`, `Inboxes`. All tables will strictly enforce RLS based on `tenant_id`.
  - Real-time updates will be managed via WebSockets integrated into the native Rust backend and distributed using Redis Pub/Sub for scale.

  *Mermaid.js Diagram:*
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : manages
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates_in
      CONVERSATION ||--o{ MESSAGE : contains

      INBOX {
          uuid id
          string tenant_id
          string name
          string channel_type
          boolean is_active
      }
      CONVERSATION {
          uuid id
          string tenant_id
          uuid inbox_id
          uuid contact_id
          string status
      }
      MESSAGE {
          uuid id
          string tenant_id
          uuid conversation_id
          text content
          string sender_type
          timestamp created_at
      }
      CONTACT {
          uuid id
          string tenant_id
          string name
          string identifier
      }
  ```

  *Mobile UX Flow (375px First):*
  - The chat interface will use OHC Premium Tokens (Apple/Ubiquiti-style hierarchy, translucent materials).
  - **List View**: A unified feed of all active customer interactions prioritizing unread and urgent queries.
  - **Chat View**: Tapping a conversation opens the chat view with native mobile keyboard support. The UI will prominently display AI-suggested draft replies that the owner can approve with a single tap.
  - The design strictly avoids horizontal scrolling and ensures all touch targets are >= 44x44px.

  *AI Agent Integration:*
  - The Customer & Relationship Assistant (AI Agent) will automatically subscribe to the new message feed via the Redis event bus.
  - It will draft replies for chat/email/IG directly in the background and insert them as "pending AI drafts" in the `Messages` table, visible only to the owner for approval.

  *Key Design Decisions:*
  - **No External Dependency**: Fully replaces Chatwoot for strict data ownership and Zero-Trust isolation.
  - **Agent-First**: The chat schema inherently supports "AI" as a first-class `sender_type`.
  - **Multi-Tenant Native**: Built natively with PostgreSQL RLS and SPIFFE/SPIRE for identity validation.

  **Implementation Prompt**:
  Implement the Core Data Models and API Layer for the Native Rust Chat System.
  - **Goal**: Create the PostgreSQL schema migrations for `inboxes`, `conversations`, `messages`, and `contacts` (ensuring `tenant_id` and RLS).
  - **Goal**: Build the Rust gRPC (Tonic) service layer for creating inboxes, starting conversations, and sending/receiving messages.
  - **CUJ**: As an owner (Maya), I should be able to receive an inquiry from a custom form and view it in my OHC message feed. When I reply, it should be saved and delivered.
  - **Acceptance Criteria**:
    1. Schema migrations are present and applied.
    2. Rust gRPC services and proto definitions for Chat are implemented.
    3. 100% unit test coverage for the service logic.
    4. At least one E2E Playwright test verifies a message can be created and retrieved through the UI.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []