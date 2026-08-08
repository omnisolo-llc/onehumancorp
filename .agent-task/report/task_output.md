issue_title: "Architecture Design: Native Rust Omnichannel Chat System"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) currently lacks a unified, native chat system. The mandate requires the full retirement of Chatwoot as an external dependency and the creation of a high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust. Our non-technical owner personas (like Maya, Carlos, and Priya) need a seamless, invisible omnichannel experience. They shouldn't have to manage multiple tools or know what a "webhook" is. They need unified messages from Web Widgets, WhatsApp, Instagram, SMS, and Email flowing into a single OHC inbox where AI agents can draft replies, analyze intent, and coordinate tasks invisibly.

  ## Research Report
  Based on an audit of the `https://github.com/chatwoot/chatwoot` source code and OHC requirements:
  - **Chatwoot Architecture**: Uses a polymorphic `Channel` model (e.g., `Channel::WebWidget`, `Channel::Whatsapp`, `Channel::Sms`) linked to an `Inbox`. `Conversation`s belong to an `Inbox` and a `Contact`, containing `Message`s.
  - **Data Models**: Chatwoot's core entities are `Account` (Tenant), `User` (Agent), `Contact` (Customer), `Inbox`, `Channel`, `Conversation`, and `Message`.
  - **Real-time**: Relies heavily on WebSockets (ActionCable) for live updates to the agent dashboard and web widgets.
  - **Multi-tenancy**: Uses `account_id` universally across tables for logical isolation.

  **OHC Adaptation**:
  - We will implement this natively in Rust within `onehumancorp/mono`.
  - We will use PostgreSQL with row-level security (`tenant_id`) for strict multi-tenancy.
  - We will implement generic channel adapters (Web, Email, SMS, WhatsApp) to ingest and normalize messages.
  - We will integrate this deeply with our AI agents (Customer & Relationship Assistant) to auto-draft replies and maintain context.

  ## Design Doc
  ### Architecture Diagram (Mermaid)
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      INBOX ||--o{ CHANNEL_CONFIG : configures
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains

      TENANT {
          uuid id PK
          string name
      }
      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
          boolean enable_auto_assignment
      }
      CHANNEL_CONFIG {
          uuid id PK
          uuid inbox_id FK
          string provider_type "web, whatsapp, sms, email"
          jsonb credentials
      }
      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string name
          string identifier
          jsonb custom_attributes
      }
      CONVERSATION {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status "open, resolved, snoozed"
          uuid assignee_id
      }
      MESSAGE {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          string sender_type "contact, agent, bot"
          uuid sender_id
          text content
          string message_type "incoming, outgoing, private_note"
          timestamp created_at
      }
  ```

  ### Mobile UX Flow (375px First)
  - **Unified Inbox View**: A simple, unified feed of open conversations. Avatar of the contact, latest message snippet, and channel icon (e.g., WhatsApp, Web).
  - **Conversation View**: Chat-bubble interface. Action bar at the bottom with options to type, use AI draft, or send a pre-defined offer/quote.
  - **AI Integration**: AI-drafted responses appear as translucent, suggested text bubbles that the owner can tap to approve and send. No technical jargon.

  ### AI Agent Integration Points
  - **Work Triage**: On new incoming `Message`, a background task triggers the AI to analyze intent, summarize, and route the conversation.
  - **Customer Assistant**: Listens to new messages and drafts suggested replies based on context (previous orders, FAQs).
  - **Background Jobs**: Use PostgreSQL `SKIP LOCKED` for processing incoming webhooks (e.g., WhatsApp, SMS) and dispatching them to the correct `Inbox`.

  ### Key Design Decisions
  - **Strict Multi-Tenancy**: Every table must have `tenant_id` and RLS policies enforced.
  - **Polymorphic Sender/Channel**: `Message.sender_type` distinguishes between customers, human agents, and AI bots. `CHANNEL_CONFIG` dictates how outgoing messages are delivered.
  - **Rust Ecosystem**: Use `axum` for HTTP/Webhooks, `tokio` for async processing, `sqlx` for database interactions, and standard WebSockets for real-time UI updates.

  ## Implementation Prompt
  **Goal**: Implement the core data models and service layer for the Native Rust Omnichannel Chat System.
  **CUJ**: An owner (e.g., Maya) receives a message from a customer via a Web Widget. The message appears in her unified inbox. She reads it and replies.
  **Tasks for Implementer**:
  1. Create the PostgreSQL migrations for `inboxes`, `channel_configs`, `contacts`, `conversations`, and `messages`, ensuring `tenant_id` and RLS are strictly applied.
  2. Implement the Rust data models (structs) corresponding to these tables in the backend service.
  3. Implement the core repository methods (CRUD) for these models, ensuring all queries are scoped by `tenant_id`.
  4. Create a basic REST API or gRPC service to list conversations and create a message in a conversation.
  5. Include comprehensive unit tests for the repositories and services. Do NOT prescribe exact function signatures; design them according to idiomatic Rust and project standards. Ensure all tests pass.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, chat]
assignees: []
issue_scope: Large
issue_scope: Large
