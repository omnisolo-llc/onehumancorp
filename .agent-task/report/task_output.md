issue_title: "[Architecture] Native Rust Omnichannel Chat & Inbox System (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  Currently, OHC lacks a native omnichannel chat and unified inbox system, previously relying on external services like Chatwoot (which is now strictly 100% RETIRED). For non-technical owners like Maya (baker managing IG DMs) or Carlos (handyman texting clients), disjointed communication across Instagram, WhatsApp, SMS, and Web Widget causes missed leads, delayed responses, and broken context. They need a single, unified "Work Triage" inbox that consolidates all customer interactions, natively integrated into OHC's backend and AI assistant capabilities.

  ## Research Report: Chatwoot Architecture Benchmarking
  An audit of the Chatwoot open-source repository (`https://github.com/chatwoot/chatwoot`) reveals its core structural strengths that OHC must replicate in native Rust:

  1. **Omnichannel Abstraction (`app/models/channel/`)**: Chatwoot abstracts providers (WhatsApp, Facebook, Twitter, Twilio SMS, Email, Line, Web Widget) into a common interface. Messages map to a unified schema regardless of the source.
  2. **Conversation & Message Core (`app/models/conversation.rb`, `message.rb`)**:
     - `Conversation` aggregates an `account_id` (tenant), `inbox_id`, `contact_id`, and `assignee_id`. It tracks state (`status: open/resolved`), priority, snoozed timestamps, and custom attributes.
     - `Message` links to a conversation, tracking `content_type` (text, attachment, template), `message_type` (incoming, outgoing, private note), and `sender_type`.
  3. **Real-time Engine**: Chatwoot uses ActionCable (WebSockets) for real-time inbox updates to the agent UI.
  4. **Agent Automation (Macros/Rules)**: Chatwoot triggers rules (like auto-assigning or auto-replying) based on message creation events.

  **OHC Market Differentiation**: Unlike Chatwoot which targets support teams, OHC targets the *business owner*. The inbox isn't just for chatting; it's a "Work Triage" queue where AI agents (Sales, Ops, Support) intercept messages, draft proposals, create bookings, or generate payment links directly within the thread.

  ## Design Doc: OHC Native Rust Inbox Architecture

  ### 1. Data Model & Invariants (Multi-Tenant)
  - Every entity is strictly scoped by `tenant_id` (Row-Level Security).
  - Built natively in Rust within `onehumancorp/mono`.

  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : "owns"
      Tenant ||--o{ Contact : "owns"
      Inbox ||--o{ ChannelAdapter : "has"
      Contact ||--o{ Conversation : "participates in"
      Inbox ||--o{ Conversation : "contains"
      Conversation ||--o{ Message : "contains"

      Inbox {
          uuid id
          uuid tenant_id
          string name
          boolean is_ai_triage_enabled
      }

      ChannelAdapter {
          uuid id
          uuid inbox_id
          string provider_type "whatsapp, ig_dm, sms, web_widget"
          jsonb provider_credentials
          string status
      }

      Contact {
          uuid id
          uuid tenant_id
          string name
          string phone_number
          string email
          jsonb external_ids
      }

      Conversation {
          uuid id
          uuid tenant_id
          uuid inbox_id
          uuid contact_id
          string status "open, snoozed, resolved"
          timestamp last_activity_at
      }

      Message {
          uuid id
          uuid tenant_id
          uuid conversation_id
          string content
          string message_type "incoming, outgoing, template, internal_note"
          string sender_type "contact, owner, ai_agent"
          jsonb attachments
      }
  ```

  ### 2. AI Department Coordination
  - **Work Triage (Redis/Postgres Job Queue)**: When an incoming webhook hits a `ChannelAdapter`, the message is persisted. A `NewMessageEvent` is published to the `ai_job_queue` (SKIP LOCKED).
  - **Customer Assistant Agent**: Dequeues the event, reads the conversation history, and drafts a reply (stored as `message_type: internal_note, sender_type: ai_agent, status: draft`).
  - **Sales/Ops Integration**: The AI can attach UI components (e.g., a "Confirm Booking" or "Pay Deposit" card) as rich `content` payload in the draft.

  ### 3. Mobile-First UX Flow (375px)
  - **The "Triage" Screen (Home)**: A unified list of unread conversations across all channels. Each row shows the contact avatar, platform icon (e.g., green WhatsApp icon), snippet, and an AI-generated tag (e.g., "Needs Quote").
  - **The Thread View**: A familiar iMessage-style chat.
    - Incoming messages on the left.
    - Owner/Agent replies on the right.
    - *AI Drafts*: Highlighted in a translucent glass container at the bottom with a "Send" or "Edit" button.
  - **Performance/Offline**: The Flutter app uses a local SQLite cache for the inbox. Messages read/write to local first, syncing to the Rust backend via REST + WebSocket/Server-Sent Events when online.

  ### 4. Zero Trust & Security
  - **Webhooks**: Provider webhooks (e.g., Meta Graph API) are validated via signature verification before processing.
  - **Tenancy**: `tenant_id` is enforced at the database level (RLS) and verified via SPIFFE/SPIRE identity tokens on every request.

  ## Implementation Prompt (For Implementer Agent)
  **Objective**: Implement the foundational database schema, Rust gRPC/REST service, and initial Flutter UI for the OHC Native Omnichannel Inbox.

  **Acceptance Criteria**:
  1. **Backend (Rust)**: Define the Postgres schemas (Migrations) for `inboxes`, `channel_adapters`, `contacts`, `conversations`, and `messages`, strictly enforcing `tenant_id` RLS.
  2. **API Layer**: Expose endpoints to create a conversation, send a message, and list conversations for an inbox.
  3. **AI Webhook Hook**: Implement a basic event listener that triggers a background job when a new message is received.
  4. **Frontend (Flutter)**: Build the 375px "Triage" inbox list screen and the individual Conversation Thread screen using OHC Premium Token design (translucent glass, clean hierarchy). Implement a simulated AI draft state in the UI.
  5. **Verification**: Write at least 5 Playwright E2E tests verifying the complete flow from creating a conversation (simulating an incoming webhook) to the owner approving and sending an AI drafted reply in the UI. No mocked network responses for internal API calls.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, backend, frontend]
assignees: []
