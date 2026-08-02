issue_title: "Migrate away from Chatwoot to Custom Rust Omnichannel Chat System"
issue_description: |
  # Research Report: Chatwoot Retirement and Native Rust Omnichannel Chat Implementation

  ## Context and Opportunity
  Following the `P0` mandate in the project's requirements, Chatwoot must be completely retired as an external third-party service, dependency, or integration. OHC requires its own high-performance, multi-tenant omnichannel customer support and chat engine, built natively in Rust inside `onehumancorp/mono`. We have checked out Chatwoot's source code and audited its architecture to inform this design.

  ## Problem Statement
  External chat integrations like Chatwoot introduce latency, cross-boundary authentication complexity, multi-tenancy risk, and operational overhead that violates the strict "Zero Trust & Security" and multi-tenant isolation rules of the OHC platform. For our non-technical owner/operators (Maya, Carlos, Priya, Leo, Fatima), they need instant, resilient, offline-tolerant communication natively built into the OHC app, avoiding third-party data silos.

  ## Research Report & Design Decisions
  We audited the Chatwoot source code (`https://github.com/chatwoot/chatwoot`), specifically focusing on:
  - **Data Models**: `Account`, `Inbox`, `Conversation`, `Message`, and `Contact`.
  - **Features**: Omnichannel capabilities, agent routing, macros, canned responses, and webhooks.

  ### Chatwoot Data Model Mapping to OHC
  We will replicate the core models natively in Rust using PostgreSQL:
  - `Account` -> `Tenant` (Already exists in OHC, mapped via `tenant_id`)
  - `Inbox` -> `ChannelInbox` (Represents a connection, e.g., IG DM, WhatsApp, Email)
  - `Conversation` -> `Conversation` (A thread between a contact and the business)
  - `Message` -> `Message` (Individual chat bubbles, text/attachments, internal notes)
  - `Contact` -> `CustomerContact` (The end user interacting with the business)

  ### Key Design Decisions
  1. **Strict Multi-Tenancy**: All database tables will strictly enforce row-level security using `tenant_id`. Every query must include the tenant context.
  2. **High-Performance Messaging**: The Rust backend will leverage WebSockets for real-time message delivery, designed to be resilient for low-end mobile devices and slow networks (critical for personas like Fatima).
  3. **AI Integration**: AI Agents (like the Customer Assistant) will natively hook into the `Conversation` flow, capable of drafting replies or auto-responding based on tenant settings, operating on the same data plane without API hop overhead.
  4. **Mobile Parity UX**: The frontend implementation (Flutter/PWA) must follow the "grandmother test", ensuring clean, translucent glass UI components that are fully usable on a 375px screen, showing unread badges and immediate visual feedback.

  ### Architecture Diagram
  ```mermaid
  erDiagram
      Tenant ||--o{ ChannelInbox : owns
      Tenant ||--o{ CustomerContact : has
      ChannelInbox ||--o{ Conversation : routes
      CustomerContact ||--o{ Conversation : participates
      Conversation ||--o{ Message : contains
      Message ||--o{ Attachment : includes

      Tenant {
          uuid tenant_id PK
          string name
      }
      ChannelInbox {
          uuid id PK
          uuid tenant_id FK
          string channel_type "email, whatsapp, ig_dm, native"
          boolean is_active
      }
      CustomerContact {
          uuid id PK
          uuid tenant_id FK
          string name
          string phone
          string email
      }
      Conversation {
          uuid id PK
          uuid tenant_id FK
          uuid channel_inbox_id FK
          uuid customer_contact_id FK
          string status "open, resolved, snoozed"
          datetime last_activity_at
      }
      Message {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          string sender_type "agent, customer, system"
          string content
          boolean is_private_note
          datetime created_at
      }
  ```

  ### Implementation Prompt (For Implementer Agents)
  **Objective:** Build the core backend data models, gRPC/REST APIs, and frontend UI for the native Rust Omnichannel Inbox, replacing Chatwoot.

  **CUJ (Critical User Journey):**
  1. Maya (the baker) opens the OHC mobile app (375px viewport).
  2. She navigates to the "Messages" or "Inbox" tab.
  3. She sees a unified list of active conversations (Instagram DMs, website chat).
  4. She taps a conversation, sees the message history, and types a reply.
  5. The UI updates instantly (optimistic UI), and the native Rust backend persists the message and broadcasts it via WebSocket.

  **Acceptance Criteria:**
  - Rust models, migrations, and repository logic for `ChannelInbox`, `CustomerContact`, `Conversation`, and `Message` are implemented with strict `tenant_id` isolation.
  - API endpoints (gRPC/REST) are available to list conversations, fetch messages, and send messages.
  - The UI provides a premium, responsive (375px-first), Apple/Ubiquiti-style interface for the unified inbox.
  - 100% unit test coverage for new Rust code and Frontend logic.
  - E2E Playwright tests verifying the CUJ (Maya sending a message).
  - All existing `bazel test //...` run green.

  **Estimated Scope**: Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, chat]
assignees: []
