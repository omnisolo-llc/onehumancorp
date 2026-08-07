issue_title: "Research: Architect Custom Rust Omnichannel Chat to Replace Chatwoot"
issue_description: |
  ## Problem Statement
  OHC currently relies on external Chatwoot services for its omnichannel customer support & chat engine. This violates the core architectural mandate to replace Chatwoot with a high-performance, native Rust implementation inside `onehumancorp/mono`. We need a custom Rust multi-tenant chat system that matches Chatwoot's features (omnichannel data models, controllers, channels, WebSocket real-time messaging, inboxes) but integrated directly into OHC.

  ## Research Report
  - **Codebase Discovery**: The search `grep -rn "chatwoot" src` returned empty, indicating Chatwoot integration is either entirely absent or abstracted away, but the mandate states it must be explicitly built in Rust.
  - **Source Auditing**: Chatwoot (`https://github.com/chatwoot/chatwoot`) relies heavily on Rails models, PostgreSQL, Redis (Sidekiq queues), and ActionCable for WebSockets.
  - **Target Persona**: Maya the baker and Carlos the handyman need an integrated inbox that works flawlessly on a 375px phone screen, combining Instagram DMs, email, and SMS natively within OHC without an external Chatwoot dependency.

  ## Design Doc
  - **Architecture**:
    - **Rust Backend**: New module `src/server/domain/omnichannel` (or similar) to handle Conversations, Messages, Contacts, and Channels.
    - **WebSockets**: Utilize existing Rust asynchronous ecosystem (e.g., Tokio, Axum/Tungstenite) to provide real-time updates.
    - **Database**: Expand the `Tenant` PostgreSQL schema (enforcing Row Level Security via `tenant_id`) to store `conversations` and `messages`.
    - **Frontend**: Flutter PWA matching the macOS-style Translucent Glass and UniFi layout, prioritizing a mobile-first (375px) unified inbox view.

  ### Diagrams

  #### Architecture & ER Diagram
  ```mermaid
  erDiagram
      Tenant {
          uuid tenant_id PK
          string name
      }
      Contact {
          uuid contact_id PK
          uuid tenant_id FK
          string name
          string phone
          string email
      }
      Channel {
          uuid channel_id PK
          uuid tenant_id FK
          string type
          string provider_id
      }
      Conversation {
          uuid conversation_id PK
          uuid tenant_id FK
          uuid contact_id FK
          uuid channel_id FK
          string status
      }
      Message {
          uuid message_id PK
          uuid conversation_id FK
          uuid tenant_id FK
          string content
          timestamp created_at
      }

      Tenant ||--o{ Contact : "has"
      Tenant ||--o{ Channel : "has"
      Tenant ||--o{ Conversation : "has"
      Contact ||--o{ Conversation : "participates_in"
      Channel ||--o{ Conversation : "hosts"
      Conversation ||--o{ Message : "contains"
  ```

  #### Sequence Diagram (Message Received)
  ```mermaid
  sequenceDiagram
      participant External as External Source (Instagram/SMS)
      participant API as OHC Ingress API
      participant ChatEngine as Rust Chat Engine
      participant DB as PostgreSQL (RLS enabled)
      participant AI as AI Agent (Triager)
      participant WS as WebSocket Hub
      participant Client as Owner (Flutter App)

      External->>API: Webhook (New Message)
      API->>ChatEngine: Process Message payload
      ChatEngine->>DB: Lookup Tenant & Contact
      ChatEngine->>DB: Insert Message (tenant_id)
      ChatEngine->>AI: Trigger Context Analysis
      AI-->>ChatEngine: Suggested Reply Draft
      ChatEngine->>DB: Save Draft Suggestion
      ChatEngine->>WS: Broadcast New Message Event (tenant_id)
      WS->>Client: Real-time UI Update
  ```

  - **Mobile UX Flow**:
    - Inbox view lists ongoing conversations from various channels (Instagram, SMS, Web).
    - Tapping a thread opens a native-feeling chat interface.
    - AI integration points seamlessly draft replies based on agent context.
  - **AI Integration**: AI agents monitor the inbox for incoming messages, applying tags or drafting responses for owner approval before sending.

  ## Implementation Prompt
  - Create a native Rust implementation of a multi-tenant omnichannel chat engine to replace Chatwoot functionality.
  - Implement the core data models (`Conversation`, `Message`, `Inbox`, `Channel`) in Rust using the existing database ORM/SQL layer.
  - Ensure all database queries enforce row-level tenant isolation using `tenant_id`.
  - Design the API endpoints and WebSocket handlers for real-time message delivery.
  - Build the corresponding Flutter PWA unified inbox screen, ensuring it is fully functional and responsive down to a 375px width.
  - The UI must adhere to the macOS-style Translucent Glass and UniFi dashboard design tokens.
  - The implementation must include extensive E2E Playwright tests simulating a full conversation lifecycle.

  ## Priority
  P0 (Mandated Architectural Upgrade)

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
