issue_title: "Native Rust Omnichannel Chat System for OHC"
issue_description: |
  # Native Rust Omnichannel Chat System for OHC

  ## Problem Statement
  OneHumanCorp (OHC) is replacing Chatwoot with a custom, native Rust omnichannel chat system to align with the core architectural requirements (multi-tenant SaaS, high performance, and owner-centered UI). Small business owners like Maya, Carlos, and Priya need a seamless, integrated inbox that works intuitively without external dependencies or complicated setups. The system needs to support real-time interactions, varied channel adapters (e.g., Web, Instagram, WhatsApp), and seamless integration with OHC's AI agents.

  ## Research Report
  - **Goal:** Replace Chatwoot with a fully native Rust implementation.
  - **Reference:** Chatwoot's source code architecture (`app/models/`, `app/controllers/api/v1/`, `app/channels/`, `app/workers/`).
  - **Key Capabilities required from Chatwoot analysis:**
    - Real-time WebSocket communication for messaging.
    - Multi-tenant architecture (`tenant_id` on all tables, RLS in Postgres).
    - Omnichannel adapters: Web Widget, Email, API-based channels (WhatsApp, IG).
    - Robust data models: Inbox, Conversation, Message, Contact.
    - AI Agent integration: Support for AI agents drafting replies and taking actions seamlessly.
  - **Competitors:** Zendesk, Intercom, GoHighLevel (which offers integrated SMS/Email/Chat). OHC's advantage will be native AI agent integration directly into the chat flow, not just as a side panel.

  ## Design Doc
  ### High-Level Architecture
  The system will be implemented as a Rust crate/service within the OHC mono-repo, leveraging the existing Bazel build system.

  - **Language:** Rust
  - **Database:** PostgreSQL (with Row Level Security for multi-tenancy).
  - **Real-time:** WebSockets (using `tokio` and `axum` or `actix-web`).
  - **AI Integration:** Direct hooks into the OHC AI Job Queue (PostgreSQL `SKIP LOCKED`).

  #### Data Model (Mermaid ER Diagram)
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      TENANT ||--o{ CONTACT : has
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains

      TENANT {
          uuid id
          string name
      }
      INBOX {
          uuid id
          uuid tenant_id
          string name
          string channel_type
      }
      CONTACT {
          uuid id
          uuid tenant_id
          string name
          string email
          string phone
      }
      CONVERSATION {
          uuid id
          uuid tenant_id
          uuid inbox_id
          uuid contact_id
          string status
      }
      MESSAGE {
          uuid id
          uuid tenant_id
          uuid conversation_id
          string content
          string message_type
          string sender_type
      }
  ```

  ### Mobile UX Flow (375px first)
  1. **Work Feed / Inbox:** The owner opens the app. A unified list of active conversations is shown. Unread messages have a distinct dot.
  2. **Conversation View:** Tapping a thread opens the chat. The UI resembles iMessage/WhatsApp.
  3. **AI Drafts:** If an AI agent has proposed a reply, it appears slightly faded in the input box with an "Approve & Send" button.
  4. **Actions:** The "+" menu allows quickly creating a quote, requesting payment, or booking a calendar slot directly in the chat.

  ### Key Design Decisions
  - **Native Rust:** Ensures performance, memory safety, and seamless integration with existing backend components.
  - **Multi-Tenant RLS:** Security at the database level guarantees data isolation between different owners.
  - **Unified Data Model:** Consolidates all communication channels into a single `Conversation`/`Message` schema to simplify the frontend and AI agent logic.

  ## Implementation Prompt
  **Role:** Backend/Full-stack Rust Engineer
  **Task:** Implement the core Native Rust Omnichannel Chat system for OHC, replacing Chatwoot.
  **CUJ:** As a business owner (e.g., Maya the Baker), I want to receive messages from different channels (starting with a Web Widget adapter) in one unified inbox and reply to them, so that I can manage all customer communications in one place.
  **Acceptance Criteria:**
  1. Implement the database schema (Inboxes, Contacts, Conversations, Messages) with strict PostgreSQL Row Level Security (RLS) based on `tenant_id`.
  2. Create a Rust REST API for basic CRUD operations on these entities.
  3. Implement a WebSocket server in Rust to broadcast new messages to connected clients (owners) in real-time.
  4. Create a basic internal Web Widget adapter to ingest messages.
  5. Provide 100% test coverage for the new Rust code (unit tests) and a Playwright E2E test verifying the flow from widget message submission to it appearing in the owner's inbox.
  6. Ensure all UI elements follow the OHC Premium Token library (macOS Translucent Glass, 375px mobile-first).

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
