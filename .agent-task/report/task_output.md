issue_title: "Architecture & Implementation Plan: Native Rust Omnichannel Chat System"
issue_description: |
  # Problem Statement
  OHC previously relied on Chatwoot as an external third-party service for omnichannel customer support and chat. This violates our new core mandate for zero reliance on Chatwoot and complete ownership of the conversational data and agent integration layer. Our business owners (Maya, Carlos, Priya, Leo, Fatima, Nora, Jun) need a unified "Work Triage" and "Customer & Relationship Assistant" experience natively within OHC without cross-service latency, sync issues, or separate admin portals. They need incoming messages from Instagram, WhatsApp, SMS, Web Widget, and Email unified in one Assistant-first interface on their phones, where AI agents can draft replies and coordinate tasks invisibly.

  # Research Report
  **Findings from Chatwoot Source Audit:**
  - **Data Models:** Chatwoot uses extensive models like `account`, `user`, `inbox`, `conversation`, `message`, `contact`, `channel_adapter`, etc. The core entity relationship is Account -> Inbox -> Conversation -> Message, with Contacts attached to Conversations.
  - **Channels:** Specific models exist for `channel/api`, `channel/email`, `channel/facebook_page`, `channel/instagram`, `channel/line`, `channel/sms`, `channel/telegram`, `channel/tiktok`, `channel/twilio_sms`, `channel/twitter_profile`, `channel/web_widget`, `channel/whatsapp`.
  - **Real-time:** WebSockets (ActionCable) are used heavily for real-time presence, typing indicators, and message delivery.
  - **Automation:** Uses `automation_rule`, `macro`, `canned_response` for SLA and auto-responses.

  **Competitive Analysis:**
  - *Shopify Inbox:* Simple, integrated directly into the merchant app, highly coupled with store data (cart context in chat).
  - *Wix Inbox:* Aggregates form submissions, emails, and live chat.
  - *OHC Differentiator:* OHC is "Assistant-First". The AI agent (Work Triage) needs to sit *between* the channel and the human owner. Messages arrive -> AI categorizes, drafts reply, links to CRM context (orders, bookings) -> Owner approves or edits on 375px mobile screen.

  # Design Doc

  **Architecture Diagram:**
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      INBOX ||--o{ CHANNEL_ADAPTER : configured_with
      TENANT ||--o{ CONTACT : has
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates_in
      CONVERSATION ||--o{ MESSAGE : contains
      MESSAGE ||--o{ ATTACHMENT : includes

      TENANT {
          uuid tenant_id PK
      }
      INBOX {
          uuid inbox_id PK
          uuid tenant_id FK
          string name
      }
      CHANNEL_ADAPTER {
          uuid channel_id PK
          uuid inbox_id FK
          string provider_type
          jsonb config
      }
      CONVERSATION {
          uuid conversation_id PK
          uuid inbox_id FK
          uuid contact_id FK
          string status
          timestamp last_activity_at
      }
      MESSAGE {
          uuid message_id PK
          uuid conversation_id FK
          uuid sender_id
          string sender_type
          text content
          string message_type
      }
  ```

  **System Architecture:**
  1. **Core Service (Rust):** Implement `ohc-chat-engine` as a module inside `onehumancorp/mono`. Use SeaORM for multi-tenant Row-Level Security (RLS) models (`Tenant`, `Inbox`, `Channel`, `Conversation`, `Message`).
  2. **Channel Adapters (Rust):** Build native async adapters using `reqwest` and webhooks for Instagram Graph API, WhatsApp Business API, Twilio SMS, and a native Web Widget (WebSocket based).
  3. **Real-time Engine:** Use Tokio-Tungstenite and async-nats (already in Cargo.toml) to handle PubSub for WebSocket connections. When a webhook arrives -> parse -> save to DB -> publish to NATS -> broadcast to connected clients (owner's Flutter app).
  4. **AI Agent Integration:**
     - Hook into the message creation pipeline via Postgres `SKIP LOCKED` job queue.
     - On new inbound message: Job triggers `Customer & Relationship Assistant` agent.
     - Agent reads conversation history, fetches contact context, and drafts a reply.
     - Draft is saved as a `Draft` message type, pushed via WebSocket to UI.
  5. **Zero Trust & Multi-Tenancy:**
     - `tenant_id` must be on every single table.
     - API endpoints must validate `tenant_id` from the JWT (SPIFFE/SPIRE context) against the resource's `tenant_id`.

  **Mobile UX Flow (375px First):**
  - **Home (Work Triage):** Top card shows "3 New Messages - 2 Action Required".
  - **Inbox List:** Unified feed. Avatar, contact name, channel icon (e.g., Insta), truncated message. Unread items have a bold dot (OHC Premium Token).
  - **Conversation Thread:** Clean translucent glass header. Bubbles for messages. If AI drafted a reply, it appears as a distinct translucent card at the bottom above the native keyboard with "Send" or "Edit" buttons.
  - **Offline Tolerance:** Writes (sending a message) are persisted to local SQLite (Flutter) and synced in the background.

  **AI Agent Integration Points:**
  - **Pre-Processing Queue:** NLP triage to label intent (Inquiry, Support, Booking Request).
  - **Draft Generator:** Gemini Pro context-aware prompt using prior conversation and business knowledge base.
  - **Next Action Suggester:** Emits structured events (e.g., "Suggest sending payment link for $50").

  # Implementation Prompt
  **User-Facing Outcome:** The owner (e.g., Maya the baker) opens the OHC mobile app and sees a unified inbox of all Instagram, WhatsApp, and Web widget messages. She can read full conversations and see AI-drafted replies ready for her approval.
  **CUJ:**
  1. Customer sends a WhatsApp message.
  2. Webhook hits OHC backend, creates Message and Conversation.
  3. AI agent automatically drafts a reply based on business context.
  4. Maya opens the app (375px mobile view), taps the unread conversation, sees the AI draft, taps "Send", which dispatches the reply back via WhatsApp API.
  **Acceptance Criteria:**
  - Native Rust API endpoints for Inbox, Conversation, and Message CRUD.
  - SeaORM entities implemented with strict `tenant_id` multi-tenancy.
  - WebSocket endpoint for real-time message delivery.
  - One mock-free, real E2E Playwright test simulating a webhook payload and asserting the message appears in the UI and a reply can be sent.
  - No Chatwoot dependencies or code references exist.
  - 100% unit test coverage for the Rust models and handlers.

  # Priority
  P0

  # Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []