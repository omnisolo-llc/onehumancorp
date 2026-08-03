issue_title: "Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) is replacing its external third-party Chatwoot dependency with a native, high-performance omnichannel customer support and chat engine built entirely in Rust, operating as microservices within the `onehumancorp/mono` repository. OHC needs to ensure tight multi-tenant isolation, real-time messaging, and agentic workflows that are natively embedded into the OHC platform. Chatwoot’s ruby-based architecture was not suited for OHC’s stringent data model and Zero-Trust isolation targets, and its reliance on external hosting adds unnecessary operational complexity. We need a unified Native Rust inbox system that replicates Chatwoot's core functionality (omnichannel data models, controllers, channels, WebSocket real-time messaging) but tailored to our owner-centric persona model.

  ## Research Report
  - **Source Code Audit:** Reviewed the core domain models in Chatwoot (`app/models/inbox.rb`, `conversation.rb`, `message.rb`, `contact.rb`). Chatwoot's models separate conversations from inboxes, and tightly link messages to conversations and accounts.
  - **Data Isolation:** Chatwoot uses an `account_id` integer pattern for tenancy, but OHC requires strict row-level security using PostgreSQL `tenant_id` and Zero-Trust SPIFFE/SPIRE-backed microservices.
  - **Real-time WebSockets:** Chatwoot uses ActionCable. The Rust implementation will use a high-performance asynchronous WebSocket framework (e.g., Axum or Actix-Web with Tokio) combined with Redis Pub/Sub for real-time, multi-node message delivery.
  - **Omnichannel:** Chatwoot abstracts channels (Email, WhatsApp, Facebook, Web Widget) via `channel_id` polymorphic associations on `Inbox`. We will need an extensible `ChannelAdapter` trait in Rust to handle the variations in these integrations.

  ## Design Doc
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : "has"
      Tenant ||--o{ Contact : "has"
      Inbox ||--o{ ChannelAdapter : "configured with"
      Contact ||--o{ Conversation : "initiates"
      Inbox ||--o{ Conversation : "contains"
      Conversation ||--o{ Message : "contains"
      Message }o--|| Agent : "sent by (optional)"

      Tenant {
          uuid id PK
          string business_name
      }
      Inbox {
          uuid id PK
          uuid tenant_id FK
          string name
          boolean enable_auto_assignment
      }
      ChannelAdapter {
          uuid id PK
          uuid inbox_id FK
          string channel_type
          jsonb credentials
      }
      Contact {
          uuid id PK
          uuid tenant_id FK
          string name
          string email
          string phone
      }
      Conversation {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status
          timestamp last_activity_at
      }
      Message {
          uuid id PK
          uuid conversation_id FK
          string content
          string message_type
          timestamp created_at
      }
  ```

  ### Mobile UX Flow (375px first)
  1. **Unified Inbox List View:**
     - Clean, macOS Translucent Glass UI.
     - A simple list of active conversations categorized by "Needs Action" or "Pending".
     - Unread indicator on the left, customer avatar, and a snippet of the latest message.
     - Swipe left to snooze, swipe right to mark resolved.
  2. **Conversation View:**
     - Sticky header displaying the customer's name and channel icon (WhatsApp, Instagram, etc.).
     - Chat bubble flow: customer messages on the left, owner/AI agent messages on the right.
     - Bottom input bar with native mobile keyboard. A prominent "AI Assist" button floats above the text box to instantly draft a reply.
  3. **AI Agent Interaction:**
     - If the AI Operations Agent drafts a reply while the owner is asleep, the conversation sits in a "Drafted" state. The owner can tap "Approve & Send" or edit the text.

  ### AI Agent Integration Points
  - **Work Triage Agent:** Listens to the `MessageCreated` event via Redis Pub/Sub. Automatically categorizes the conversation and sets priority.
  - **Customer Relationship Agent:** Drafts replies based on the `Conversation` context. Injects drafts directly into the `Message` table with a state of `DRAFT_PENDING_APPROVAL`.
  - **Operations Agent:** If the conversation detects a service request or order, it triggers an automation rule to create a booking or task linked to the `Conversation`.

  ### Key Design Decisions
  - **Rust Native:** The backend will be a Rust gRPC/REST service using Axum/Tokio.
  - **Multi-Tenancy:** Every table must have `tenant_id` and strict PostgreSQL RLS applied.
  - **Idempotency:** Webhooks and message creation endpoints must require idempotency keys to handle flaky mobile networks.
  - **Asynchronous Processing:** Heavy tasks like email parsing or AI drafting are delegated to a PostgreSQL `SKIP LOCKED` job queue.

  ## Implementation Prompt
  **Goal:** Implement the foundational database schema, core Rust data models, and the basic gRPC/REST APIs for the Native Rust Omnichannel Chat System.

  **Tasks for the Implementer Agent:**
  1. Create the PostgreSQL migrations for `inboxes`, `contacts`, `conversations`, and `messages`. Ensure RLS policies are enforced on `tenant_id`.
  2. Implement the Rust Axum service definitions and entity structs for these models, maintaining strict memory safety and clean serialization.
  3. Create the CRUD endpoints for an Owner to create an `Inbox` and a `Contact`, and for a webhook/API client to initiate a `Conversation` and insert a `Message`.
  4. Scaffold the WebSocket endpoint that will eventually stream `Message` updates to the frontend.
  5. Ensure all new code has 100% unit test coverage and write a Playwright E2E test verifying a user can view an empty inbox state in the UI.

  **Acceptance Criteria:**
  - Migrations apply successfully via Bazel build.
  - The Rust service compiles and runs within the Docker Compose stack.
  - Endpoints reject requests lacking valid tenant context.
  - Tests verify that `Conversation` objects are properly linked to `Inbox` and `Tenant`.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
