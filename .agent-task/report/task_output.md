issue_title: "Architecture Design: Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  ### Title
  Native Rust Omnichannel Chat System Architecture (Chatwoot Replacement)

  ### Problem Statement
  OneHumanCorp (OHC) currently relies on Chatwoot as an external dependency for its omnichannel chat and customer support functionality. Relying on an external service breaks the OHC promise of "advanced setup, integrations, and technical details hidden." It forces multi-tenant data outside our security boundary and prevents deep AI integration across tasks, bookings, and customer contexts. Maya (the baker) and Carlos (the handyman) need to respond to Instagram DMs and SMS respectively directly from OHC's mobile-first interface, without managing an external Chatwoot configuration. We must build a high-performance, native Rust omnichannel chat system within OHC's mono repo to fully replace Chatwoot, providing a unified, multi-tenant inbox for all owner interactions.

  ### Research Report
  An audit of the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) reveals its core architecture:
  - **Models**: `Account` (tenant), `User`, `Inbox`, `Conversation`, `Message`, `Contact`.
  - **Channels**: Adapters for API, Email, Facebook Page, Instagram, Line, SMS, Telegram, TikTok, Twilio, Twitter, Web Widget, WhatsApp.
  - **Real-time**: ActionCable (WebSockets) for pushing events to the UI.
  - **Routing/Rules**: Automation rules, assignment policies, SLAs, and macros.

  To build this in Rust for OHC:
  - We can leverage our existing `tenant_id` row-level security in PostgreSQL.
  - Channels can be modeled as Rust traits/adapters handling ingress (webhooks) and egress (API calls).
  - Real-time updates can be handled via WebSockets using `tokio-tungstenite` or similar, backed by Redis Pub/Sub for cross-node distribution.
  - The UI must remain clean, mirroring Apple/Ubiquiti design patterns, starting from 375px.

  ### Design Doc
  #### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : has
      INBOX ||--o{ CHANNEL : configures
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains
      CHANNEL {
          string provider "Instagram, SMS, Email, WhatsApp"
          string credentials
      }
      MESSAGE {
          string content
          string sender_type "Owner, Customer, Agent"
          timestamp created_at
      }
  ```

  #### Mobile UX Flow (375px First)
  1. **Unified Feed**: The OHC home screen shows "Work Triage". Unread messages from any channel appear as prioritized cards with Translucent Glass styling.
  2. **Conversation View**: Tapping a message opens the unified thread. The top bar shows the Contact Name and Channel (e.g., "Maya's Cakes via Instagram").
  3. **AI Drafts**: Above the native mobile keyboard, a "Drafted by AI" snippet appears, summarizing context (e.g., "They ordered a vegan cake last time").
  4. **Actions**: A floating action button allows quick creation of a Quote, Task, or Booking directly from the chat.

  #### AI Agent Integration Points
  - **Customer Assistant**: Listens to the Redis stream for new `Message` events. Drafts replies based on tenant context and saves them as pending `Message` records (visible only to the owner).
  - **Operations Assistant**: Analyzes message intent (e.g., "Can I pick up tomorrow?") and suggests calendar updates.
  - **Work Triage**: Updates the owner's daily summary when urgent messages arrive.

  #### Key Design Decisions and Why
  - **Native Rust**: Ensures high performance, minimal memory footprint, and tight integration with OHC's backend.
  - **Row-Level Security (RLS)**: PostgreSQL RLS guarantees strict multi-tenant isolation for all chat records.
  - **Single Unified Inbox**: Owners do not switch between "SMS" and "Instagram" tabs; all conversations are unified per Contact.
  - **AI-First**: The system is designed to have the AI as a first-class participant in the thread, capable of drafting and taking actions before the owner intervenes.

  ### Implementation Prompt
  **Goal**: Implement the core data models, REST API, and WebSocket infrastructure for the Native Rust Omnichannel Chat System, completely replacing Chatwoot.
  **Persona**: Maya (Home Baker) needs to receive an Instagram DM and see it in her OHC unified feed immediately.
  **Acceptance Criteria**:
  - Implement Rust structs and PostgreSQL migrations with RLS for `Inbox`, `Channel`, `Contact`, `Conversation`, and `Message`.
  - Implement a REST API for fetching conversations and sending messages.
  - Implement a WebSocket endpoint for real-time message delivery to the Flutter PWA.
  - Implement a dummy channel adapter for testing.
  - Ensure 100% unit test coverage.
  - Add at least one Playwright E2E test simulating a customer message appearing in the owner's UI.

  ### Priority
  P0

  ### Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []