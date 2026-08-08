issue_title: "Implement Native Rust Omnichannel Chat System"
issue_description: |
  # Native Omnichannel Chat System

  ## Problem Statement
  Small business owners need to communicate with customers across multiple channels (Instagram, WhatsApp, Email, Web) without juggling multiple apps. Previously, this was designed around Chatwoot, but Chatwoot has been fully retired and removed from the architecture. We need a native Rust implementation of a high-performance, multi-tenant omnichannel chat engine that achieves parity with Chatwoot's core features (inbox, conversations, messages, contacts) but is built specifically for OHC's architecture (Zero-Trust, Agentic workflows, mobile-first).

  ## Research Report
  - **Codebase Audit:** Chatwoot removal is complete (verified via `docs/reports/production_agent_optimization_report.md` and `docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md`). No legacy integration remains.
  - **Chatwoot Source Benchmark:** Reviewed `chatwoot/chatwoot` models (Conversation, Message, Contact, Channel adapters).
    - Chatwoot uses a robust polymorphic channel design (`channel_type`, `channel_id`).
    - Conversations are linked to Accounts (tenants), Inboxes, and Contacts.
    - WebSockets are used for real-time delivery.
  - **Competitor Systems:** Shopify Inbox and Wix Inbox are basic aggregators. Zendesk is too complex. OHC needs an AI-first inbox where "The Ambassador" agent can draft replies natively before the human owner intervenes.

  ## Design Doc
  ### Architecture Diagram (Mermaid)
  ```mermaid
  erDiagram
    TENANT {
      uuid id PK
      string name
    }
    INBOX {
      uuid id PK
      uuid tenant_id FK
      string name
      string channel_type
    }
    CONTACT {
      uuid id PK
      uuid tenant_id FK
      string name
      string identifier
    }
    CONVERSATION {
      uuid id PK
      uuid tenant_id FK
      uuid inbox_id FK
      uuid contact_id FK
      string status
    }
    MESSAGE {
      uuid id PK
      uuid conversation_id FK
      uuid sender_id
      string content
      string message_type
    }
    TENANT ||--o{ INBOX : has
    TENANT ||--o{ CONTACT : has
    INBOX ||--o{ CONVERSATION : routes
    CONTACT ||--o{ CONVERSATION : participates
    CONVERSATION ||--o{ MESSAGE : contains
  ```

  ### Mobile UX Flow (375px First)
  1. **Unified Inbox List:** Owner opens the app and sees a unified list of conversations. Badges indicate the source channel (e.g., Insta, WhatsApp).
  2. **Conversation View:** Tapping a conversation opens a chat UI.
  3. **AI Drafts:** If "The Ambassador" agent drafted a reply, it appears as a floating, translucent glass card at the bottom of the screen with "Approve" or "Edit" buttons.
  4. **Native Keyboard:** Tapping "Edit" or the input field brings up the native mobile keyboard.

  ### AI Agent Integration
  - **The Ambassador:** Subscribes to the `message.created` event via the OHC Event Mesh. When a new user message arrives, the agent queries the customer graph and drafts a reply, inserting it as a draft `MESSAGE` linked to the `CONVERSATION`.
  - **Operations Agent:** Monitors conversations for keywords like "book", "cancel", "status" and surfaces contextual actions.

  ### Key Design Decisions
  - **Native Rust:** Implemented purely in Rust within `onehumancorp/mono`.
  - **Strict Multi-Tenancy:** Row-Level Security (RLS) in PostgreSQL enforced on all tables (`tenant_id`).
  - **Event-Driven:** Uses the existing mesh/pubsub infrastructure to distribute messages to agents and connected WebSocket clients.

  ## Implementation Prompt
  - Create the foundational database schema (migrations) for the native omnichannel chat system: Inboxes, Contacts, Conversations, and Messages. Ensure strict multi-tenant constraints.
  - Implement the Rust repository layer (`src/server/api/inbox` or similar) to handle CRUD operations for these entities.
  - Build the Axum API endpoints (REST) to fetch the unified inbox list and conversation messages for the mobile client.
  - Ensure the API returns structured data that the Flutter/Tauri mobile frontend can render into a 375px-optimized glassmorphism UI.
  - Integrate with the existing authentication middleware to enforce Zero-Trust access.
  - Do not implement the specific external channel webhooks (e.g., WhatsApp, Instagram) in this initial ticket; focus on the core internal data model and unified API first.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
