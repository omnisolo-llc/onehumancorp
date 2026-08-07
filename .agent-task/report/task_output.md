issue_title: "Implement Native Rust Omnichannel Chat Engine (Chatwoot Replacement)"
issue_description: |
  ### Title
  Implement Native Rust Omnichannel Chat Engine (Chatwoot Replacement)

  ### Priority
  P0

  ### Estimated Scope
  Large

  ### Problem Statement
  The owner/operator personas (Maya the Baker, Carlos the Handyman) receive customer inquiries scattered across Instagram DMs, WhatsApp, SMS, and web widgets. Previously, OHC relied on an external third-party Chatwoot integration. This dependency is 100% RETIRED. We need a native, multi-tenant Rust omnichannel chat system inside OHC to unify these interactions into a single feed that our AI agents (Customer Assistant, Work Triage) can autonomously respond to, coordinate, and summarize, all without leaving our unified platform or adding latency.

  ### Research Report
  A deep source code audit of `github.com/chatwoot/chatwoot` reveals a robust model centered around an `Inbox` which acts as the hub for multiple `Channels` (e.g., `Channel::Whatsapp`, `Channel::WebWidget`, `Channel::Instagram`). Customers are represented as `Contacts`. A `Contact` communicating over a `Channel` to an `Inbox` creates a `Conversation`. `Conversations` consist of `Messages` (incoming, outgoing, template, private notes). Chatwoot leverages ActionCable for real-time WebSocket events. To replace this natively in Rust, we need similar data structures wrapped in strict row-level security (RLS) via PostgreSQL to maintain `tenant_id` isolation. Unlike Chatwoot, our native engine will treat AI Agents (e.g., Customer Assistant) as first-class responders who draft replies or execute automation immediately upon message reception.

  ### Design Doc

  #### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : "creates"
      INBOX ||--o{ CHANNEL : "configures (Web, IG, WA)"
      TENANT ||--o{ CONTACT : "owns"
      INBOX ||--o{ CONVERSATION : "contains"
      CONTACT ||--o{ CONVERSATION : "participates"
      CONVERSATION ||--o{ MESSAGE : "has"
      CONVERSATION ||--o{ PARTICIPANT : "includes (Agents/Staff)"

      TENANT {
          uuid id
      }
      INBOX {
          uuid id
          uuid tenant_id
          string name
      }
      CHANNEL {
          uuid id
          uuid inbox_id
          string channel_type "web_widget | instagram | whatsapp | email"
          jsonb config
      }
      CONTACT {
          uuid id
          uuid tenant_id
          string name
          string email
          string phone_number
          jsonb custom_attributes
      }
      CONVERSATION {
          uuid id
          uuid inbox_id
          uuid contact_id
          string status "open | snoozed | resolved"
          uuid assignee_id
      }
      MESSAGE {
          uuid id
          uuid conversation_id
          string content
          string message_type "incoming | outgoing | internal_note"
          uuid sender_id
      }
  ```

  #### UI Wireframes & Mobile UX Flow (375px First)
  1. **Work Triage Feed**: The primary mobile screen (375px). A unified list of all active `Conversations` across all channels. Unread messages have a bold typography token and a translucent accent badge.
  2. **Conversation View**: Tapping a thread slides in the chat view.
     - **Header**: Contact name, platform icon (e.g., Instagram), and a translucent "Resolve" button.
     - **Body**: Chat bubbles. Customer messages on the left, Agent/AI drafts on the right. AI drafts appear with a distinct premium token tint (e.g., subtle purple or blue translucent glass) and "Approve/Edit" actions.
     - **Footer**: Native mobile keyboard-friendly text input. A prominent '+' button for attachments/quotes.
  3. **Interactivity & Offline**: The UI uses Flutter/PWA with a local SQLite cache. Messages sent offline appear slightly faded with a "pending" spinner until the WebSocket/REST sync confirms receipt.

  #### AI Agent Integration Points
  - **Message Hook**: On every incoming `Message` insert, an event is published to the `AI Job Queue` (PostgreSQL `SKIP LOCKED`).
  - **Customer Assistant**: Picks up the job, retrieves the `Conversation` history and `Contact` context, and evaluates if a reply can be drafted (e.g., "Do you do vegan cakes?").
  - **Drafting**: The AI inserts a `Message` with `message_type = "internal_note"` or a specific `draft` status, which streams to the UI via WebSockets for the owner to approve with one tap.

  #### Key Design Decisions
  - **Row-Level Security (RLS)**: Every table (`inboxes`, `channels`, `conversations`, `messages`, `contacts`) MUST have a `tenant_id` column with PostgreSQL RLS enforced.
  - **First-Class AI Context**: Instead of simple webhooks, the AI directly queries the Rust API for semantic matching of past messages and company knowledge (Knowledge Assistant).
  - **WebSockets**: Native Rust WebSocket support (e.g., using `axum` or `tokio-tungstenite`) to push real-time `message.created` and `conversation.updated` events directly to the Flutter clients.

  ### Implementation Prompt
  **User-Facing Outcome**: Implement the core data model, API endpoints, and Rust service layer for the native omnichannel chat engine. Maya the Baker should be able to receive a webhook-driven simulated Instagram message, view it in her unified inbox, and see an AI-drafted reply without needing a third-party Chatwoot account.

  **CUJ & Acceptance Criteria**:
  1. Implement the database migrations for `inboxes`, `channels`, `contacts`, `conversations`, and `messages` ensuring `tenant_id` and RLS are applied.
  2. Create Rust CRUD services (using `sqlx` or the repo's preferred DB layer) for these entities.
  3. Implement a webhook ingestion endpoint for a dummy channel (e.g., `web_widget` or generic API) that creates a `Contact`, `Conversation`, and `Message`.
  4. Ensure 100% unit test coverage for the new Rust services.
  5. Create a Playwright E2E test verifying a message ingested via the API appears correctly in the (simulated/actual) Work Triage UI without mocked data.
  6. Hide all technical jargon (webhooks, channels) from the primary UI; just show "Messages".

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
