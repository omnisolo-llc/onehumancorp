issue_title: "[Architecture] Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  ## Title: Architecture Design for Native Rust Omnichannel Chat System

  ## Problem Statement
  The OHC platform must provide owners and operators (e.g., Maya, Carlos, Priya) with a unified inbox to triage messages, bookings, and customer requests across all channels (Web, WhatsApp, Email, Instagram). Previously, OHC relied on an external third-party service (Chatwoot) which has now been 100% retired to reduce operational complexity, enforce absolute multi-tenant Zero Trust (SPIFFE/SPIRE) security, and improve latency. We need a native Rust omnichannel chat system inside the OHC monolith that replicates the core capabilities of Chatwoot without the bloat, designed explicitly for mobile-first small business workflows and AI agent interactions.

  ## Research Report & Feature Benchmarking
  I cloned and audited the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) to understand its architecture and data models. Key findings:
  1. **Data Models:** Chatwoot centers around `Conversations` (with `status`, `snoozed_until`, `assignee_id`), `Messages` (polymorphic sender: contact or agent), `Contacts`, and `Inboxes`.
  2. **Channel Adapters:** It uses a flexible `Channel` model (e.g., `Channel::WebWidget`, `Channel::Whatsapp`, `Channel::Email`) connected to an `Inbox`.
  3. **Real-time:** WebSockets power real-time updates to the unified frontend.
  4. **Omnichannel:** All channels funnel into a unified `Conversation` model, abstracting away the provider specifics.
  5. **Gap Analysis:** Chatwoot is built for enterprise support teams (SLA policies, macros). OHC needs this optimized for a single owner/operator (and AI agents) where chat leads directly to commerce actions (deposits, bookings, quotes) without complex team routing.

  ## Design Doc
  ### High-Level Architecture Diagram
  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : "has"
      Tenant ||--o{ Contact : "has"
      Inbox ||--o{ Channel : "configures"
      Channel ||--o{ Conversation : "creates"
      Contact ||--o{ Conversation : "participates in"
      Conversation ||--o{ Message : "contains"

      Inbox {
          uuid id
          uuid tenant_id
          string name
      }
      Channel {
          uuid id
          uuid inbox_id
          string provider_type
          jsonb config
      }
      Conversation {
          uuid id
          uuid contact_id
          uuid inbox_id
          string status
          datetime last_activity_at
      }
      Message {
          uuid id
          uuid conversation_id
          string content
          string sender_type
      }
  ```

  ### Mobile UX Flow (375px First)
  - **Screen 1: Unified Triage Feed:** A clean, UniFi-style dashboard card layout showing unread conversations across all channels (Instagram, WhatsApp, Web). Uses OHC Premium Token library with translucent glass headers, big tap targets (>44x44px), and clear status indicators (unread vs action required).
  - **Screen 2: Conversation View:** Standard chat bubbles with native mobile keyboard behavior. Bottom sheet action menu to trigger AI tools ("Draft Quote", "Request Deposit", "Book Appointment") without leaving the chat.
  - **Screen 3: Contact Context:** Sliding drawer showing previous orders, preferences, and automated notes maintained by the Customer Relationship Assistant.

  ### AI Agent Integration Points
  - **Work Triage Agent:** Hooks into the message creation flow. Evaluates incoming messages to prioritize them in the owner's feed and group related inquiries.
  - **Customer Assistant Agent:** Subscribes to the unified inbox event bus. Auto-drafts replies based on tenant context (e.g., business hours, specific offerings) and saves them as pending messages for owner approval.

  ### Key Design Decisions
  - **Rust + SQLx + Postgres RLS:** Strict row-level security based on `tenant_id` for all database interactions.
  - **Unified Conversation Model:** All incoming webhooks (e.g., WhatsApp Meta Webhook) map to a standard `Conversation` + `Message` schema so the frontend only has to build one UI.
  - **Async Event Bus:** Messages trigger background jobs via PostgreSQL `SKIP LOCKED` for AI triage and WebSocket broadcasts, ensuring the HTTP request doesn't block on AI generation or external API calls.

  ## Implementation Prompt
  **Role:** Implementer Agent
  **Objective:** Build the core database schema, models, and service layer for the Native Rust Omnichannel Chat System based on the Chatwoot audit, replacing the retired dependency.
  **CUJ:** An owner receives a message via a channel (e.g., WhatsApp or Web Widget). The webhook hits our API, creates a Contact (if new), creates a Conversation, and saves the Message. The system then broadcasts an event, and the AI agent drafts a suggested reply.
  **Acceptance Criteria:**
  1. Define Rust models for `Inbox`, `Channel`, `Contact`, `Conversation`, and `Message` in `src/server/services/chat/models.rs`.
  2. Implement the `ChatService` in `src/server/services/chat/service.rs` with methods for `create_conversation`, `add_message`, and `get_tenant_conversations`.
  3. Ensure 100% unit test coverage for the service layer logic.
  4. Ensure all SQLx queries explicitly filter by `tenant_id` to enforce multi-tenant isolation.
  5. The API architecture must cleanly separate channel-specific webhook processing from the unified conversation logic.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
