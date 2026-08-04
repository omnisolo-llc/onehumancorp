issue_title: "Architecture Design: Native Rust Omnichannel Chat System"
issue_description: |
  # Native Rust Omnichannel Chat System Architecture

  ## Problem Statement
  OneHumanCorp (OHC) is replacing Chatwoot, a third-party omnichannel support tool, with a native, high-performance, multi-tenant Rust-based system. We need to eliminate external dependencies, integrate customer messaging seamlessly into OHC’s single app, and guarantee row-level security per tenant. The target audience (Maya, Carlos, Priya) requires a unified inbox on their 375px mobile screens that triages Instagram DMs, WhatsApp, and Web Chat natively inside the OHC Work Assistant.

  ## Research Report
  Based on our source code audit of Chatwoot (`https://github.com/chatwoot/chatwoot`), the core entities map as follows:
  - **Account (OHC Tenant)**: Top-level isolation.
  - **Inbox**: An entry point for messages (e.g., a specific WhatsApp number or Instagram page).
  - **Channel**: Specific channel integrations (e.g., `Channel::WebWidget`, `Channel::Whatsapp`, `Channel::FacebookPage`).
  - **Contact**: The customer communicating with the tenant.
  - **Conversation**: An ongoing thread of messages between a Contact and a Tenant via an Inbox.
  - **Message**: Individual message payloads, supporting text, attachments, and structured data.

  ### Key Design Differences from Chatwoot:
  1. **Multi-Tenancy**: Chatwoot uses `account_id` on most tables. OHC will use `tenant_id` on every table with strict PostgreSQL Row Level Security (RLS) enforcement.
  2. **Performance**: Built natively in Rust (axum or actix-web) to leverage high-performance concurrent processing and low memory footprint compared to Ruby on Rails.
  3. **AI Native**: AI agents (Operations, Customer Service) are first-class participants in the Conversation model, capable of drafting replies and categorizing conversations automatically using PostgreSQL SKIP LOCKED task queues.

  ## Design Doc
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL_ADAPTER : configured_by
      TENANT ||--o{ CONTACT : owns
      INBOX ||--o{ CONVERSATION : routes
      CONTACT ||--o{ CONVERSATION : participates_in
      CONVERSATION ||--o{ MESSAGE : contains
      MESSAGE ||--o{ ATTACHMENT : includes

      TENANT {
          uuid tenant_id PK
          string name
      }
      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
          string channel_type
          jsonb config
      }
      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string identifier
          string name
          string email
          string phone
      }
      CONVERSATION {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status "open, closed, snoozed"
          timestamp last_activity_at
      }
      MESSAGE {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          string sender_type "contact, agent, bot"
          uuid sender_id
          string content
          jsonb external_source_ids
      }
  ```

  ### Mobile UX Flow (375px First)
  1. **Home Feed**: The owner opens the app. The primary view is "Work Triage". The unified inbox is a widget showing "3 Unread Customer Messages".
  2. **Conversation List**: Tapping the widget opens the Conversation list. The layout uses UniFi-style clean cards. Each card displays the contact name, channel icon (WhatsApp, IG, Web), snippet, and time.
  3. **Thread View**: Tapping a conversation opens the thread view. Translucent glass app bar. Sticky bottom input with native mobile keyboard support.
  4. **AI Assist**: An "AI Draft" pill sits above the input field if the AI has pre-generated a suggested reply. Tapping it populates the input for review.

  ### AI Agent Integration Points
  - **Incoming Message Hook**: When a new message arrives via a webhook, it's saved and a job is enqueued in the AI Job Queue (PostgreSQL `SKIP LOCKED`).
  - **Work Triage Agent**: Evaluates the message to set the `priority` and `status` of the conversation.
  - **Customer Assistant Agent**: Reads the conversation history and tenant knowledge base, then drafts a reply. The draft is stored in the database but marked as `status: draft`.

  ## Implementation Prompt
  **Goal**: Implement the native Rust omnichannel inbox data models, repositories, and API endpoints to replace Chatwoot, ensuring strict multi-tenant isolation via `tenant_id`.

  **Critical User Journey (CUJ)**:
  1. A tenant owner (e.g., Maya) creates a new Web Widget Inbox via the OHC UI.
  2. A customer visits her site and sends a message through the widget.
  3. The system creates a Contact, Conversation, and Message in the database, isolated to her `tenant_id`.
  4. Maya opens her 375px mobile app and sees the new message in her unified inbox.

  **Acceptance Criteria**:
  - Rust models and PostgreSQL schema (with RLS policies) exist for Inbox, Contact, Conversation, and Message.
  - CRUD API endpoints exist for the models, strictly scoped by tenant identity (SPIFFE/SPIRE context).
  - E2E Playwright tests prove that a message sent via a simulated channel webhook appears in the tenant's unified inbox UI.
  - 100% Rust unit test coverage.
  - No legacy Chatwoot API calls remain.

  ## Priority: P0 (Critical)
  ## Estimated Scope: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture]
assignees: []
