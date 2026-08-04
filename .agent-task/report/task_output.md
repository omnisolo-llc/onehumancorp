issue_title: "Native Rust Omnichannel Chat System Architecture"
issue_description: |
  ## Problem Statement
  OHC requires a unified, high-performance omnichannel inbox that handles incoming messages (WhatsApp, Web Widget, Email, etc.) in a scalable, multi-tenant environment without relying on external services like Chatwoot. The external Chatwoot dependency is being 100% retired. Small business owners (like Maya, Carlos, Priya) need all their customer communications unified into a single assistant-led feed. They don't have time to manage multiple tabs or apps; they need a 375px mobile-friendly "work command center" where messages automatically turn into tasks, quotes, or AI-drafted replies.

  ## Research Report
  An audit of the Chatwoot open-source repository (`app/models`) reveals the core data entities required for a robust omnichannel inbox:
  - `Account` (Maps to OHC `Tenant`)
  - `Contact` (Customer entity)
  - `Inbox` (Container for specific channels)
  - `Channel::Whatsapp`, `Channel::WebWidget`, `Channel::Email`, etc. (Channel-specific configurations and credentials)
  - `Conversation` (A thread of messages between a Contact and the business)
  - `Message` (Individual chat bubbles)
  - `AgentBot` / `AgentBotInbox` (Integration points for AI assistants)

  Competitors like Shopify Inbox, WeCom, and Wix handle this by deeply integrating the unified inbox with customer records and orders. For OHC, we must adopt this deep integration natively in Rust, ensuring that every message is analyzed by the AI Job Queue for potential triage, order extraction, and intent classification.

  ## Design Doc
  ### Data Model & Invariants
  We need to replicate and adapt these models natively in Rust, ensuring strict multi-tenancy. Every table must include a `tenant_id` column to enforce Row-Level Security (RLS) in PostgreSQL.

  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : owns
      Tenant ||--o{ Contact : owns
      Tenant ||--o{ Conversation : owns
      Inbox ||--o{ Channel : has
      Inbox ||--o{ Conversation : contains
      Contact ||--o{ Conversation : participates
      Conversation ||--o{ Message : has
      Tenant ||--o{ AgentSession : owns

      Tenant {
          uuid id PK
          string name
      }
      Contact {
          uuid id PK
          uuid tenant_id FK
          string name
          string phone_number
          string email
      }
      Inbox {
          uuid id PK
          uuid tenant_id FK
          string name
      }
      Channel {
          uuid id PK
          uuid inbox_id FK
          string channel_type
          jsonb credentials
      }
      Conversation {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status
      }
      Message {
          uuid id PK
          uuid conversation_id FK
          uuid tenant_id FK
          string content
          string message_type
          timestamp created_at
      }
  ```

  ### Mobile UX Flow (375px)
  1. **Work Triage Dashboard:** The user opens the app on their phone. The home screen shows a combined list of unread messages and pending tasks, prioritized by urgency (e.g., "New custom cake inquiry from WhatsApp").
  2. **Conversation View:** Tapping an inquiry opens a chat interface. It displays the message history with a unified design language (macOS-style Translucent Glass).
  3. **AI Assistance:** At the bottom, instead of just a keyboard, there is an "AI Drafted Reply" button or inline preview, created by the Customer Assistant agent based on past context and available inventory/pricing.
  4. **Action Context:** A collapsible top drawer shows the `Contact` details, recent orders, and internal notes.

  ### AI Agent Integration Points
  - **Work Triage Agent:** Listens to a webhook/message event stream. When a new `Conversation` or `Message` is created, it assesses the intent and updates the owner's prioritized task feed.
  - **Customer Assistant Agent:** Listens to incoming `Messages`. It retrieves the `Contact`'s history, matches intent against `Knowledge & Documents`, and generates a draft reply. It coordinates with the `Operations Assistant` to verify availability before proposing dates.

  ### Key Design Decisions
  - **Rust Backend:** The core chat engine will be built in Rust for high throughput, low memory footprint, and memory safety, processing high-volume webhook events from WhatsApp/Meta and WebSocket connections from Web Widgets.
  - **Zero Trust:** Every database query must filter by `tenant_id` at the lowest repository level. No cross-tenant data leakage is permitted.

  ## Estimated Scope
  Large

  ## Implementation Prompt
  **Goal:** Implement the foundational database schema and core Rust data models for the native omnichannel chat system.

  **Acceptance Criteria:**
  1. Create database migrations for `inboxes`, `channels`, `contacts`, `conversations`, and `messages`. Every table MUST include a `tenant_id` column and be configured for Row-Level Security (RLS).
  2. Implement the corresponding Rust entity models and repository layers with strict `tenant_id` scoping in every query.
  3. Implement basic CRUD APIs (gRPC and REST) to create an Inbox, associate a Channel (e.g., WhatsApp stub), create a Contact, and send/receive a Message.
  4. Ensure 100% unit test coverage for the repository and service layers.
  5. The API must trigger a background job (or emit an event) when a new message is received, allowing the Work Triage agent to pick it up.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
