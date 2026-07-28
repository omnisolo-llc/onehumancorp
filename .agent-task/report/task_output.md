issue_title: "Native Rust Omnichannel Chat System Architecture"
issue_description: |
  # Problem Statement
  OHC currently lacks a native omnichannel customer support and chat engine, previously relying on external integrations like Chatwoot. To provide a seamless, zero-trust, multi-tenant work assistant for non-technical owners (like Maya the baker or Carlos the handyman), we must natively own the messaging infrastructure. Our owners need a single, unified inbox that aggregates Instagram DMs, WhatsApp, Email, and Web Chat, directly integrated with OHC's AI agents (Work Triage, Customer Assistant).

  # Research Report
  **Chatwoot Architecture Audit:**
  Based on an audit of the `chatwoot/chatwoot` repository, their core architecture revolves around:
  - **Models:** Account (Tenant), Inbox, Conversation, Message, Contact, User (Agent).
  - **Channels:** Polymorphic associations where an Inbox has a Channel (e.g., `Channel::WebWidget`, `Channel::Whatsapp`, `Channel::Email`).
  - **Real-time:** ActionCable for WebSocket events pushing updates to the frontend.
  - **Routing:** Round-robin or manual assignment of conversations to agents.
  - **Automation:** Webhooks, Macros, and SLAs.

  **OHC Gap:**
  We need a Rust-native equivalent that natively integrates with our AI Swarm (KAIROS). Instead of just routing to human agents, conversations are primarily routed to the AI Customer Assistant, which drafts replies or executes actions, asking for human approval when necessary.

  # Design Doc

  ## Architecture Diagram

  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      TENANT ||--o{ CONTACT : has
      INBOX ||--|| CHANNEL_ADAPTER : uses
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains
      MESSAGE ||--o{ ATTACHMENT : contains

      TENANT {
          uuid id PK
          string name
      }
      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
      }
      CHANNEL_ADAPTER {
          uuid id PK
          string provider_type
          json credentials
      }
      CONVERSATION {
          uuid id PK
          uuid inbox_id FK
          uuid contact_id FK
          string status
      }
      MESSAGE {
          uuid id PK
          uuid conversation_id FK
          string content
          string sender_type
      }
  ```

  ## System Architecture
  - **Backend (Rust/Axum):** Implement `src/server/services/chat/` module.
  - **Channels:** Trait-based `ChannelAdapter` allowing easy addition of Instagram, WhatsApp, and Web Widget.
  - **Real-time:** Axum WebSocket route (`/api/v1/chat/ws`) for instant message delivery and typing indicators.
  - **Persistence:** Postgres with Row-Level Security (RLS) on `tenant_id` for strict isolation.
  - **AI Integration:** When a new message arrives, a background job (Postgres `SKIP LOCKED`) triggers the `Customer Assistant` agent to analyze context, classify urgency (Work Triage), and draft a response.

  ## Mobile UX Flow (375px First)
  1. **Unified Inbox View:** A clean, Unifi-style list of active conversations. Unread items have a subtle premium badge.
  2. **Conversation Thread:** Translucent glass header showing the contact. Messages are distinct chat bubbles.
  3. **AI Assistant Drafts:** Below the latest customer message, the AI proposes a drafted response. The owner sees "Maya (AI Draft)" with a primary "Approve & Send" button and a secondary "Edit" button.
  4. **Action Context:** If the customer asks for a quote, the AI generates a quote card inline. The owner taps "Send Quote".

  ## AI Agent Integration Points
  - **Work Triage:** Intercepts incoming webhook payloads (e.g., WhatsApp message), maps them to a `Conversation`, and updates the owner's Daily Feed.
  - **Customer Assistant:** Automatically reads the `Message` history, checks memory/notes on the `Contact`, and proposes a draft response via the `Message` table with `status = "draft"`.

  # Implementation Prompt
  **User-Facing Outcome:** Owners can view and reply to all customer messages (from web, social, email) in one unified mobile-first inbox within OHC, with AI automatically drafting context-aware replies.
  **CUJ:**
  1. A customer sends a message via the Web Widget.
  2. The message appears in the owner's Unified Inbox in real-time.
  3. The AI Customer Assistant drafts a reply.
  4. The owner taps "Approve & Send".
  5. The reply is delivered to the customer in real-time.

  **Acceptance Criteria:**
  - Native Rust API endpoints for Inbox, Conversation, and Message CRUD.
  - Axum WebSocket implementation for real-time delivery.
  - Playwright E2E tests proving the CUJ end-to-end (using test-mode adapters, no mocked backend).
  - UI must follow 375px mobile-first principles with translucent glass styling.
  - 100% Rust unit test coverage.
  - Strict multi-tenant isolation via `tenant_id`.

  # Priority
  P0

  # Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []