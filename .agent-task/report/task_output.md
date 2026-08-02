issue_title: "Native Rust Omnichannel Chat: Data Model & Agent Routing System Design"
issue_description: |
  # Native Rust Omnichannel Chat: Data Model & Agent Routing System Design

  ## Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" simply aggregate messages without context and require manual typing.

  Previously, OHC used Chatwoot for this functionality. However, Chatwoot has been fully retired and removed from the platform. We are now replacing it with a native, high-performance, multi-tenant Rust architecture inside `onehumancorp/mono`. This research issue outlines the foundational data models and AI agent routing mechanisms needed to achieve full Chatwoot feature parity and beyond, focusing specifically on how The Ambassador agent interacts with this new native inbox.

  ## Research Report
  - **Chatwoot Source Code Audit:** Reviewed `https://github.com/chatwoot/chatwoot` source code (specifically `db/schema.rb`).
    - Key tables: `inboxes`, `conversations`, `messages`, `contacts`, `channel_*`.
    - Key concepts: Conversations belong to an Inbox and a Contact. Messages belong to a Conversation. Sender types include Contact (customer), User (agent/owner), and System (bot).
  - **Current OHC State:** `src/server/services/chat/` contains a nascent native Rust implementation (`models.rs`, `service.rs`). `src/server/services/inbox/` contains another older implementation (`unified_threads`, `unified_messages`). We need to solidify the canonical native Chatwoot replacement models in `src/server/services/chat/` and integrate them with the webhook ingestion paths and AI routing.
  - **The Ambassador Agent:** The Customer Success Agent (The Ambassador) must be able to read these native `chat_messages`, query the customer's identity graph, and draft replies directly into the `chat_messages` table (or an `action_required` queue) for the owner to approve.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram DM / WhatsApp / Email] -->|Webhook| B(Omnichannel Gateway - webhook.rs)
      B --> C{Customer Identity Resolution Engine}
      C -->|Lookup/Create| D[(chat_contacts)]
      B --> E[Chat Service Layer]
      E -->|Find/Create| F[(chat_conversations)]
      E -->|Insert| G[(chat_messages)]
      G --> H[Event Mesh / Job Queue]
      H --> I[The Ambassador Agent]
      I -->|Query Context| D
      I -->|Draft Reply| J[(unified_triage_actions / chat_messages draft)]
      J --> K[Mobile App Feed 375px]
      K -->|1-Tap Approve| L[Omnichannel Dispatcher]
      L --> A
  ```

  ### Data Model (Native Rust `chat_*` tables)
  - `chat_inboxes`: tenant_id, name, channel_type (e.g., 'whatsapp', 'instagram').
  - `chat_contacts`: tenant_id, name, email, phone, external_identifier.
  - `chat_conversations`: tenant_id, inbox_id, contact_id, status (open/resolved), assignee_id (human or AI agent).
  - `chat_messages`: tenant_id, conversation_id, sender_type (customer, agent, bot), content, status (sent, delivered, read, draft).
  - **Crucial Multi-Tenant Rule:** Every table MUST have `tenant_id` and PostgreSQL RLS must be enforced.

  ### Mobile UX Flow (375px First)
  - **Feed:** The owner opens the app. The top card says "New WhatsApp Message from Sarah".
  - **Action:** Tapping it reveals the thread. The Ambassador agent has already drafted a response based on Sarah's past orders.
  - **Approve:** A prominent "Send Draft" button. No manual typing required.

  ### AI Agent Integration Points
  - When a `chat_message` with `sender_type = 'customer'` is inserted, an event (`message_triage`) is fired to the AI Job Queue.
  - The orchestrator routes this to The Ambassador.
  - The Ambassador reads the `chat_conversation` history, queries the product catalog/calendar, and generates a draft.

  ## Implementation Prompt
  **User-Facing Outcome:** When a customer sends a message on any channel, the owner sees a unified thread in their mobile app with a pre-drafted, context-aware AI response ready for 1-tap approval.

  **CUJ & Acceptance Criteria:**
  1. Extend `src/server/services/chat/models.rs` and the corresponding SQL schema migrations to fully match the necessary Chatwoot capabilities (e.g., adding `status` or `is_draft` to `chat_messages`, ensuring `chat_contacts` handles omnichannel identity).
  2. Implement the API endpoints in `src/server/api/chat/` to support fetching conversations and messages for the Next.js frontend, reading from the canonical `chat_*` tables.
  3. Refactor `src/server/api/inbox/webhook.rs` to insert incoming messages into the new `chat_messages` / `chat_conversations` tables instead of the legacy `omni_inbox_messages` or `unified_threads`.
  4. Ensure the `message_triage` background job triggers The Ambassador agent, which successfully drafts a reply linked to the `chat_conversation_id`.
  5. Add Playwright E2E tests simulating an incoming webhook, verifying the database insertion, and verifying the drafted reply appears in the UI.
  6. Ensure 100% unit test coverage for the new Rust service layer methods.

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
