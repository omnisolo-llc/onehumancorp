issue_title: "Native Rust Omnichannel Inbox & Chat Engine (Chatwoot Replacement)"
issue_description: |
  ### Problem Statement
  OneHumanCorp (OHC) is retiring its dependency on external third-party services like Chatwoot to provide a more integrated, performant, and secure omnichannel chat system for owner/operators. Our core personas (Maya, Carlos, Priya, Leo, Fatima) need a unified inbox that aggregates DMs (Instagram, WhatsApp, SMS, web) without juggling multiple apps. Relying on an external Chatwoot deployment fragments our multi-tenant data model, degrades the native 375px mobile experience, and complicates Zero-Trust SPIFFE/SPIRE isolation. We need a native Rust-based omnichannel chat system built directly into OHC to coordinate messages and AI assistant actions seamlessly.

  ### Research Report & Chatwoot Source Code Audit
  I have conducted an exhaustive source code audit of Chatwoot (`https://github.com/chatwoot/chatwoot`) to baseline the architecture required for our native Rust implementation. Key structural insights:
  - **Data Models**: Chatwoot centers around `accounts` (tenant), `inboxes` (channels), `conversations`, `messages`, and `contacts`.
  - **Conversations & Messages**: `conversations` track state (`status`, `assignee_id`, `snoozed_until`) and maintain a `contact_last_seen_at`. `messages` store `content`, `message_type`, `private` (for internal notes), and `content_type`.
  - **Channel Adapters**: Inboxes rely on channel-specific tables (`channel_facebook_pages`, `channel_whatsapp`, `channel_web_widgets`, etc.) to map external APIs to internal routing.
  - **Real-time WebSockets**: Required for live syncing of new messages, typing indicators, and presence.

  **Platform Differentiation for OHC**: Unlike Chatwoot, which is built for support teams, OHC's implementation must be **Assistant-First**. The AI agent (Work Triage & Customer Assistant) is the primary actor, drafting replies and tagging intent before the human owner ever opens the app.

  ### Design Doc

  #### 1. Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      INBOX ||--o{ CONVERSATION : routes
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains
      INBOX ||--o| CHANNEL_ADAPTER : uses
      MESSAGE }|--|| AI_INTENT : analyzed_by

      TENANT {
          uuid id
          string name
      }
      INBOX {
          uuid id
          string name
          string channel_type
          boolean ai_auto_draft
      }
      CONVERSATION {
          uuid id
          string status
          timestamp last_activity_at
      }
      MESSAGE {
          uuid id
          string content
          boolean is_private_note
          string sender_type
      }
      CHANNEL_ADAPTER {
          string provider
          json credentials
      }
  ```

  #### 2. Mobile UX Flow (375px First)
  - **Unified Inbox View**: A single feed with high-contrast, translucent glass cards. Unread messages have a distinct accent color. AI-drafted replies show a subtle "Draft ready" badge.
  - **Conversation View**: Native messaging interface (iMessage/WhatsApp style). At the bottom, instead of just a keyboard, the AI suggests 2-3 quick actions (e.g., "Send Payment Link", "Confirm Appt").
  - **Interactivity**: Swiping right on a conversation archives it. Swiping left reveals "Assign to Staff" or "Mark Urgent".
  - **Offline/Flaky Network**: SQLite/IndexedDB caching. Messages sent offline appear slightly faded with a "Sending..." indicator until acknowledged by the backend.

  #### 3. AI Agent Integration Points
  - **Work Triage Hook**: When a `MESSAGE` is created by a contact, a Rust background job triggers the Work Triage agent to summarize intent and assign priority.
  - **Customer Assistant Hook**: Auto-generates a `MESSAGE` with `is_private_note = true` containing a suggested reply for the owner to approve with one tap.
  - **Knowledge Assistant**: Injects past context into the conversation view (e.g., "This customer previously bought 2 custom cakes").

  #### 4. Key Design Decisions
  - **Native Rust**: High performance, strict memory safety, and easy integration with OHC's core event bus.
  - **Zero-Trust**: Every request to the Inbox Service must be validated via SPIFFE/SPIRE for the specific `tenant_id`.
  - **PostgreSQL Row-Level Security**: Enforce `tenant_id` isolation directly at the database schema level.

  ### Implementation Prompt
  **To the Implementer Swarm Agent:**
  Your task is to implement the foundational native Rust omnichannel chat backend and the corresponding Flutter UI components for the Unified Inbox.
  1. **Backend (Rust)**: Define the proto definitions (gRPC) and Diesel/SQLx schemas for `Inbox`, `Conversation`, `Message`, and `Contact` enforcing `tenant_id` RLS. Implement the REST/gRPC API for fetching conversations and sending messages. Set up the foundational WebSocket hub for real-time events.
  2. **Frontend (Flutter)**: Build the Unified Inbox screen targeting a 375px width. Use the OHC Premium Token library to design translucent glass message cards. Implement the conversation detail view.
  3. **AI Handshake**: Create a mockable background trait/interface where incoming messages trigger an event for the AI assistant to draft a reply.

  **Acceptance Criteria:**
  - A test owner persona (e.g., Maya) can open the app, view a seeded list of 3 conversations, tap one, and send a new message.
  - 100% unit test coverage on Rust models and Flutter UI state.
  - At least 5 Playwright E2E tests validating the Inbox journey (empty state, message list, conversation view, send message, offline state mock).
  - ZERO mock data in production code; all UI data must flow from the Rust backend via seeded DB records.

  ### Priority
  P0 - Critical Path (Chatwoot Retirement Mandate)

  ### Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
